//! The wrapper channel (architecture §Established Decisions [API Style], [Message Broker / IPC];
//! §Standard Contracts "Wrapper channel frames"): JSON-RPC 2.0, one frame per line, over the
//! per-instance interprocess local socket. Sync only: the Tokio client joins with `viola-mcp`.

mod client;
mod endpoint;
mod frame;
mod server;

use std::io;

use serde_json::{Value, json};
use viola_core::VERSION;

pub use client::Client;
pub use endpoint::{ENDPOINT_KIND, endpoint_name, endpoint_path, socket_dir};
pub use frame::{
    Params, Request, error_response, ok_response, parse_request, read_frame, write_frame,
};
pub use server::{Dispatch, Server, Serving};

/// The protocol version this build speaks; a request whose `params.v` is newer is refused.
pub const PROTOCOL_V: u64 = 1;

/// Fixed messages only: no path, pipe name or payload reaches a `Display` (security-plan §Error
/// Handling); the OS error stays the source, for the detail file.
#[derive(Debug, thiserror::Error)]
pub enum ChannelError {
    #[error("channel endpoint is held by another process")]
    BindTaken,
    #[error("channel endpoint could not be bound")]
    Bind(#[source] io::Error),
    #[error("channel endpoint could not be reached")]
    Connect(#[source] io::Error),
    #[error("channel frame exceeds the size bound")]
    Oversize,
    #[error("channel frame could not be parsed")]
    Parse(#[source] serde_json::Error),
    #[error("channel i/o failed")]
    Io(#[source] io::Error),
    #[error("channel peer closed the connection")]
    Closed,
    #[error("channel endpoint path is not UTF-8")]
    NonUtf8Path,
}

/// The five JSON-RPC protocol faults; every other outcome, refusals included, travels in `result`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolError {
    Parse,
    InvalidRequest,
    MethodNotFound,
    UnsupportedVersion,
    Internal,
}

impl ProtocolError {
    pub const fn code(self) -> i64 {
        match self {
            Self::Parse => -32700,
            Self::InvalidRequest => -32600,
            Self::MethodNotFound => -32601,
            Self::UnsupportedVersion => -32602,
            Self::Internal => -32603,
        }
    }

    pub const fn message(self) -> &'static str {
        match self {
            Self::Parse => "parse error",
            Self::InvalidRequest => "invalid request",
            Self::MethodNotFound => "method not found",
            Self::UnsupportedVersion => "unsupported protocol version",
            Self::Internal => "internal error",
        }
    }

    /// The `error` member. `data` is `{supported, wrapper}` for a newer peer and `null` for every
    /// other fault: no other `data` is ever built.
    pub fn body(self) -> Value {
        let data = match self {
            Self::UnsupportedVersion => json!({"supported": PROTOCOL_V, "wrapper": VERSION}),
            _ => Value::Null,
        };
        json!({"code": self.code(), "message": self.message(), "data": data})
    }
}

/// Span and event capture for this crate's unit tests: every span's recorded fields and every
/// line's fields (plus `level`) as JSON, on the calling thread only.
#[cfg(test)]
mod test_capture {
    use std::path::Path;
    use std::sync::{Arc, Mutex};

    use serde_json::{Map, Value};
    use tracing::field::{Field, Visit};
    use tracing::span::{Attributes, Id};
    use tracing::{Event, Subscriber};
    use tracing_subscriber::layer::{Context, Layer, SubscriberExt as _};

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
    pub(crate) struct Captured {
        spans: Arc<Mutex<Vec<(String, Value)>>>,
        events: Arc<Mutex<Vec<Value>>>,
    }

    impl<S: Subscriber> Layer<S> for Captured {
        fn on_new_span(&self, attrs: &Attributes<'_>, _: &Id, _: Context<'_, S>) {
            let mut fields = Fields::default();
            attrs.record(&mut fields);
            let name = attrs.metadata().name().to_owned();
            self.spans
                .lock()
                .expect("spans")
                .push((name, Value::Object(fields.0)));
        }

        fn on_event(&self, event: &Event<'_>, _: Context<'_, S>) {
            let mut fields = Fields::default();
            event.record(&mut fields);
            fields.0.insert(
                "level".to_owned(),
                event.metadata().level().to_string().into(),
            );
            self.events
                .lock()
                .expect("events")
                .push(Value::Object(fields.0));
        }
    }

    impl Captured {
        pub(crate) fn events(&self, event: &str) -> Vec<Value> {
            let events = self.events.lock().expect("events");
            events
                .iter()
                .filter(|e| e["event"] == event)
                .cloned()
                .collect()
        }

        /// The one line of `event`.
        pub(crate) fn event(&self, event: &str) -> Value {
            let mut found = self.events(event);
            assert_eq!(found.len(), 1, "{event} among {}", self.text());
            found.remove(0)
        }

        pub(crate) fn span(&self, name: &str) -> Value {
            let spans = self.spans.lock().expect("spans");
            spans
                .iter()
                .find(|(n, _)| n == name)
                .map(|(_, fields)| fields.clone())
                .unwrap_or_else(|| panic!("no span {name}"))
        }

        pub(crate) fn has_span(&self, name: &str) -> bool {
            let spans = self.spans.lock().expect("spans");
            spans.iter().any(|(n, _)| n == name)
        }

        /// Every captured line, serialized.
        pub(crate) fn text(&self) -> String {
            let events = self.events.lock().expect("events");
            events.iter().map(Value::to_string).collect()
        }
    }

    /// Every thread's lines, for a test whose lines come from a server thread too: with a scoped
    /// subscriber as the only dispatcher, tracing caches a callsite first hit on another thread as
    /// disabled. The one global-subscriber test of its process (nextest runs each apart).
    pub(crate) fn capture_global() -> Captured {
        let captured = Captured::default();
        let subscriber = tracing_subscriber::registry().with(captured.clone());
        tracing::subscriber::set_global_default(subscriber).expect("first global subscriber");
        captured
    }

    pub(crate) fn capture<R>(f: impl FnOnce() -> R) -> (R, Captured) {
        let captured = Captured::default();
        let subscriber = tracing_subscriber::registry().with(captured.clone());
        (tracing::subscriber::with_default(subscriber, f), captured)
    }

    /// A test-owned endpoint outside the `viola-<h12>` namespace `viola run` binds.
    pub(crate) fn test_endpoint(dir: &Path, label: &str) -> String {
        let base = format!("viola-test-chan-{}-{label}", std::process::id());
        if cfg!(windows) {
            format!(r"\\.\pipe\{base}")
        } else {
            dir.join(format!("{base}.sock"))
                .to_string_lossy()
                .into_owned()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::parse(
        ProtocolError::Parse,
        r#"{"code":-32700,"message":"parse error","data":null}"#
    )]
    #[case::invalid(
        ProtocolError::InvalidRequest,
        r#"{"code":-32600,"message":"invalid request","data":null}"#
    )]
    #[case::method(
        ProtocolError::MethodNotFound,
        r#"{"code":-32601,"message":"method not found","data":null}"#
    )]
    #[case::version(
        ProtocolError::UnsupportedVersion,
        r#"{"code":-32602,"message":"unsupported protocol version","data":{"supported":1,"wrapper":"0.1.0"}}"#
    )]
    #[case::internal(
        ProtocolError::Internal,
        r#"{"code":-32603,"message":"internal error","data":null}"#
    )]
    fn protocol_error_body_is_the_literal_wire_object(
        #[case] fault: ProtocolError,
        #[case] wire: &str,
    ) {
        assert_eq!(fault.body().to_string(), wire);
    }

    fn io_error() -> io::Error {
        io::Error::other(r"\\.\pipe\viola-0123456789ab C:/secret/home")
    }

    #[rstest]
    #[case::bind_taken(ChannelError::BindTaken, "channel endpoint is held by another process")]
    #[case::bind(ChannelError::Bind(io_error()), "channel endpoint could not be bound")]
    #[case::connect(
        ChannelError::Connect(io_error()),
        "channel endpoint could not be reached"
    )]
    #[case::oversize(ChannelError::Oversize, "channel frame exceeds the size bound")]
    #[case::parse(
        ChannelError::Parse(serde_json::from_str::<u8>("\"canary-chain-value-5c1e\"").expect_err("not a number")),
        "channel frame could not be parsed"
    )]
    #[case::io(ChannelError::Io(io_error()), "channel i/o failed")]
    #[case::closed(ChannelError::Closed, "channel peer closed the connection")]
    #[case::non_utf8(ChannelError::NonUtf8Path, "channel endpoint path is not UTF-8")]
    fn channel_error_display_is_a_fixed_literal(#[case] error: ChannelError, #[case] shown: &str) {
        assert_eq!(error.to_string(), shown);
    }

    #[test]
    fn channel_error_keeps_the_os_error_as_its_source() {
        let error = ChannelError::Bind(io_error());
        let source = std::error::Error::source(&error).expect("source");
        assert!(source.to_string().contains("secret"));
    }

    #[test]
    fn protocol_version_is_one() {
        assert_eq!(PROTOCOL_V, 1);
    }
}
