//! Test helpers shared by this crate's unit tests and the root package's tests, behind the
//! `test-support` feature, which only the root `[dev-dependencies]` enables: the release build never
//! carries it (`scripts/release-check.sh` refuses an artifact built with it).

use serde_json::{Map, Value};
use tracing::field::{Field, Visit};

/// A span's or a line's recorded fields as JSON: numbers and bools as themselves, a `Debug`-only
/// value as its `Debug` text.
#[derive(Default)]
pub struct JsonFields(pub Map<String, Value>);

impl Visit for JsonFields {
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

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use tracing::span::{Attributes, Id};
    use tracing_subscriber::layer::{Context, Layer, SubscriberExt as _};

    use super::*;

    /// Every new span's fields, recorded through `JsonFields`.
    #[derive(Clone, Default)]
    struct Spans(Arc<Mutex<Vec<Value>>>);

    impl<S: tracing::Subscriber> Layer<S> for Spans {
        fn on_new_span(&self, attrs: &Attributes<'_>, _: &Id, _: Context<'_, S>) {
            let mut fields = JsonFields::default();
            attrs.record(&mut fields);
            self.0.lock().expect("spans").push(Value::Object(fields.0));
        }
    }

    /// One field of each recorded type: each lands as its own JSON value, and only a `Debug`-only
    /// value as its `Debug` text.
    #[test]
    fn json_fields_records_each_type_as_its_json_value() {
        let spans = Spans::default();
        let subscriber = tracing_subscriber::registry().with(spans.clone());
        tracing::subscriber::with_default(subscriber, || {
            let _span = tracing::span!(
                tracing::Level::INFO,
                "fields",
                s = "text",
                u = 7u64,
                i = -3i64,
                b = true,
                d = ?[1, 2],
            );
        });
        let got = spans.0.lock().expect("spans");
        assert_eq!(
            got.as_slice(),
            [serde_json::json!({"s": "text", "u": 7, "i": -3, "b": true, "d": "[1, 2]"})]
        );
        assert!(got[0]["u"].is_u64() && got[0]["i"].is_i64() && got[0]["b"].is_boolean());
    }
}
