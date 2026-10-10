//! viola's disk state: owner-only creation, the one atomic replace, the append-only event log, the
//! snapshot envelope and the log replay behind it, the heartbeat, pid + start-time liveness, the
//! pinned exe copy and the capability stamps' bytes (architecture §Standard Contracts, §Occupied
//! Resources → Filesystem). Sync only: no Tokio.

pub mod events;
pub mod fs;
pub mod heartbeat;
pub mod liveness;
pub mod pin;
pub mod replay;
pub mod snapshot;
pub mod stamps;
pub mod strict;

use chrono::{DateTime, SecondsFormat, Utc};

/// Fixed messages only: no path and no payload reaches a `Display` (security-plan §Error Handling).
#[derive(Debug, thiserror::Error)]
pub enum StateError {
    #[error("state file i/o failed")]
    Io(#[from] std::io::Error),
    #[error("state record could not be encoded")]
    Encode(#[from] serde_json::Error),
}

/// RFC 3339 UTC with milliseconds and `Z` (architecture §Conventions).
pub fn timestamp(at: DateTime<Utc>) -> String {
    at.to_rfc3339_opts(SecondsFormat::Millis, true)
}

/// Line capture for this crate's unit tests: the fields of every line a closure wrote (plus
/// `level`) as JSON, on the calling thread only.
#[cfg(test)]
mod test_capture {
    use std::sync::{Arc, Mutex};

    use serde_json::{Map, Value};
    use tracing::field::{Field, Visit};
    use tracing::{Event, Subscriber};
    use tracing_subscriber::layer::{Context, Layer, SubscriberExt as _};

    /// Numbers and bools as themselves, a `Debug`-only value as its `Debug` text.
    #[derive(Default)]
    struct Fields(Map<String, Value>);

    impl Visit for Fields {
        fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
            self.0
                .insert(field.name().to_owned(), format!("{value:?}").into());
        }
        fn record_str(&mut self, field: &Field, value: &str) {
            self.0.insert(field.name().to_owned(), value.into());
        }
        fn record_u64(&mut self, field: &Field, value: u64) {
            self.0.insert(field.name().to_owned(), value.into());
        }
        fn record_i64(&mut self, field: &Field, value: i64) {
            self.0.insert(field.name().to_owned(), value.into());
        }
        fn record_bool(&mut self, field: &Field, value: bool) {
            self.0.insert(field.name().to_owned(), value.into());
        }
    }

    #[derive(Clone, Default)]
    struct Captured(Arc<Mutex<Vec<Value>>>);

    impl<S: Subscriber> Layer<S> for Captured {
        fn on_event(&self, event: &Event<'_>, _: Context<'_, S>) {
            let mut fields = Fields::default();
            event.record(&mut fields);
            fields.0.insert(
                "level".to_owned(),
                event.metadata().level().to_string().into(),
            );
            self.0.lock().expect("lines").push(Value::Object(fields.0));
        }
    }

    /// What `f` returned, and the lines it wrote.
    pub(crate) fn capture<R>(f: impl FnOnce() -> R) -> (R, Vec<Value>) {
        let captured = Captured::default();
        let subscriber = tracing_subscriber::registry().with(captured.clone());
        let result = tracing::subscriber::with_default(subscriber, f);
        let lines = captured.0.lock().expect("lines").clone();
        (result, lines)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone as _;

    #[test]
    fn timestamp_is_rfc3339_utc_millis_with_z() {
        let at = Utc
            .with_ymd_and_hms(2026, 9, 27, 1, 2, 3)
            .single()
            .expect("valid date")
            + chrono::Duration::milliseconds(45);
        assert_eq!(timestamp(at), "2026-09-27T01:02:03.045Z");
    }

    #[test]
    fn state_error_messages_are_fixed() {
        let io = StateError::from(std::io::Error::other("C:/secret/path"));
        assert_eq!(io.to_string(), "state file i/o failed");
        let encode =
            StateError::from(serde_json::from_str::<u8>("\"payload\"").expect_err("not a number"));
        assert_eq!(encode.to_string(), "state record could not be encoded");
    }
}
