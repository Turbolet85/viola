//! The wrapper channel (architecture §Established Decisions [API Style], [Message Broker / IPC];
//! §Standard Contracts "Wrapper channel frames"): JSON-RPC 2.0, one frame per line, over the
//! per-instance interprocess local socket. Sync, but for the Tokio client behind the `tokio`
//! feature, which only `viola-mcp` enables.

mod client;
mod endpoint;
mod frame;
mod server;
#[cfg(any(test, feature = "test-support"))]
pub mod test_support;
#[cfg(feature = "tokio")]
pub mod tokio_client;

use std::io;

use serde_json::{Value, json};
use viola_core::VERSION;

pub use client::{Client, SERVER_PID_REPORTED};
pub use endpoint::{ENDPOINT_KIND, endpoint_name, endpoint_path, socket_dir};
pub use frame::{
    Params, Request, error_response, ok_response, parse_request, read_frame, write_frame,
};
pub use server::{Call, Dispatch, Server, Serving};

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
    #[error("channel reply did not arrive in time")]
    Deadline,
    #[error("channel endpoint path is not UTF-8")]
    NonUtf8Path,
}

/// The five JSON-RPC protocol fault codes; every other outcome, refusals included, travels in
/// `result`. `-32602` is a newer peer or params a method cannot take.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolError {
    Parse,
    InvalidRequest,
    MethodNotFound,
    UnsupportedVersion,
    InvalidParams,
    /// A `release` whose `params` carry `from`: only the human hands the wheel back.
    ReleaseFromDriver,
    Internal,
}

impl ProtocolError {
    pub const fn code(self) -> i64 {
        match self {
            Self::Parse => -32700,
            Self::InvalidRequest => -32600,
            Self::MethodNotFound => -32601,
            Self::UnsupportedVersion | Self::InvalidParams | Self::ReleaseFromDriver => -32602,
            Self::Internal => -32603,
        }
    }

    pub const fn message(self) -> &'static str {
        match self {
            Self::Parse => "parse error",
            Self::InvalidRequest => "invalid request",
            Self::MethodNotFound => "method not found",
            Self::UnsupportedVersion => "unsupported protocol version",
            Self::InvalidParams | Self::ReleaseFromDriver => "invalid params",
            Self::Internal => "internal error",
        }
    }

    /// The `error` member. `data` is `{supported, wrapper}` for a newer peer,
    /// `{"reason":"release-from-driver"}` for a driver's `release`, and `null` for every other fault:
    /// no other `data` is ever built.
    pub fn body(self) -> Value {
        let data = match self {
            Self::UnsupportedVersion => json!({"supported": PROTOCOL_V, "wrapper": VERSION}),
            Self::ReleaseFromDriver => json!({"reason": "release-from-driver"}),
            _ => Value::Null,
        };
        json!({"code": self.code(), "message": self.message(), "data": data})
    }

    /// The word that tells the three `-32602` faults apart on a `channel-response` line.
    pub const fn cause(self) -> Option<&'static str> {
        match self {
            Self::UnsupportedVersion => Some("unsupported-version"),
            Self::InvalidParams => Some("invalid-params"),
            Self::ReleaseFromDriver => Some("release-from-driver"),
            Self::Parse | Self::InvalidRequest | Self::MethodNotFound | Self::Internal => None,
        }
    }
}

/// A peer's JSON-RPC `error` member as the wrapper-fault `detail` a client prints: the peer's integer
/// `code`, its fault's fixed message and only `data`'s closed values, so no path or upstream text
/// crosses (security-plan §Error Handling). `None` without an integer `code`. A code outside the five
/// reads the fixed `wrapper fault`.
pub fn sanitised_fault(error: &Value) -> Option<Value> {
    let code = error["code"].as_i64()?;
    let data = &error["data"];
    let wrapper = data["wrapper"]
        .as_str()
        .filter(|text| is_version_text(text));
    let (message, data) = match code {
        -32700 => (ProtocolError::Parse.message(), Value::Null),
        -32600 => (ProtocolError::InvalidRequest.message(), Value::Null),
        -32601 => (ProtocolError::MethodNotFound.message(), Value::Null),
        -32603 => (ProtocolError::Internal.message(), Value::Null),
        -32602 => match (data["supported"].as_i64(), wrapper) {
            (Some(supported), Some(wrapper)) => (
                ProtocolError::UnsupportedVersion.message(),
                json!({"supported": supported, "wrapper": wrapper}),
            ),
            _ if data["reason"] == "release-from-driver" => (
                ProtocolError::ReleaseFromDriver.message(),
                json!({"reason": "release-from-driver"}),
            ),
            _ => (ProtocolError::InvalidParams.message(), Value::Null),
        },
        _ => ("wrapper fault", Value::Null),
    };
    Some(json!({"code": code, "message": message, "data": data}))
}

/// 1 to 64 ASCII letters, digits, `.`, `-` or `+`: a version, never a path or a sentence.
fn is_version_text(text: &str) -> bool {
    (1..=64).contains(&text.len())
        && text
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'-' | b'+'))
}

/// Span and event capture for this crate's unit tests: every span's recorded fields and every
/// line's fields (plus `level`) as JSON, on the calling thread only.
#[cfg(test)]
mod test_capture {
    use std::path::Path;
    use std::sync::{Arc, Mutex};

    use serde_json::Value;
    use tracing::span::{Attributes, Id};
    use tracing::{Event, Subscriber};
    use tracing_subscriber::layer::{Context, Layer, SubscriberExt as _};

    use crate::test_support::JsonFields as Fields;

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
    #[case::invalid_params(
        ProtocolError::InvalidParams,
        r#"{"code":-32602,"message":"invalid params","data":null}"#
    )]
    #[case::release_from_driver(
        ProtocolError::ReleaseFromDriver,
        r#"{"code":-32602,"message":"invalid params","data":{"reason":"release-from-driver"}}"#
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

    #[rstest]
    #[case::version(ProtocolError::UnsupportedVersion, Some("unsupported-version"))]
    #[case::invalid_params(ProtocolError::InvalidParams, Some("invalid-params"))]
    #[case::release_from_driver(ProtocolError::ReleaseFromDriver, Some("release-from-driver"))]
    #[case::parse(ProtocolError::Parse, None)]
    #[case::invalid(ProtocolError::InvalidRequest, None)]
    #[case::method(ProtocolError::MethodNotFound, None)]
    #[case::internal(ProtocolError::Internal, None)]
    fn protocol_error_cause_tells_the_three_invalid_params_apart(
        #[case] fault: ProtocolError,
        #[case] cause: Option<&str>,
    ) {
        assert_eq!(fault.cause(), cause);
    }

    /// Every body this build sends reads back unchanged.
    #[rstest]
    #[case::parse(r#"{"code":-32700,"message":"parse error","data":null}"#)]
    #[case::invalid(r#"{"code":-32600,"message":"invalid request","data":null}"#)]
    #[case::method(r#"{"code":-32601,"message":"method not found","data":null}"#)]
    #[case::version(
        r#"{"code":-32602,"message":"unsupported protocol version","data":{"supported":1,"wrapper":"0.1.0"}}"#
    )]
    #[case::invalid_params(r#"{"code":-32602,"message":"invalid params","data":null}"#)]
    #[case::release_from_driver(
        r#"{"code":-32602,"message":"invalid params","data":{"reason":"release-from-driver"}}"#
    )]
    #[case::internal(r#"{"code":-32603,"message":"internal error","data":null}"#)]
    fn sanitised_fault_keeps_each_wire_body(#[case] wire: &str) {
        let body: Value = serde_json::from_str(wire).expect("a body");
        let read = sanitised_fault(&body).expect("a fault");
        assert_eq!(read.to_string(), wire);
    }

    /// Free text in `message` or in a `data` object is replaced, an extra key beside a closed one is
    /// dropped, and a value of the wrong shape reads the code's plain body.
    #[rstest]
    #[case::parse(
        json!({"code": -32700, "message": "/home/x/y", "data": {"p": "canary-newname-value-e2f8"}}),
        r#"{"code":-32700,"message":"parse error","data":null}"#
    )]
    #[case::invalid(
        json!({"code": -32600, "message": "Caused by", "data": {"p": "/home/x/y"}}),
        r#"{"code":-32600,"message":"invalid request","data":null}"#
    )]
    #[case::method(
        json!({"code": -32601, "message": "canary-newname-value-e2f8", "data": "/home/x/y"}),
        r#"{"code":-32601,"message":"method not found","data":null}"#
    )]
    #[case::internal(
        json!({"code": -32603, "message": "/home/x/y", "data": {"p": "Caused by"}}),
        r#"{"code":-32603,"message":"internal error","data":null}"#
    )]
    #[case::unknown_code(
        json!({"code": -1, "message": "/home/x/y", "data": {"reason": "release-from-driver"}}),
        r#"{"code":-1,"message":"wrapper fault","data":null}"#
    )]
    #[case::free_text_on_invalid_params(
        json!({"code": -32602, "message": "/home/x/y", "data": {"p": "canary-newname-value-e2f8"}}),
        r#"{"code":-32602,"message":"invalid params","data":null}"#
    )]
    #[case::an_extra_key_beside_supported(
        json!({"code": -32602, "message": "Caused by", "data": {"supported": 1, "wrapper": "0.1.0", "p": "/home/x/y"}}),
        r#"{"code":-32602,"message":"unsupported protocol version","data":{"supported":1,"wrapper":"0.1.0"}}"#
    )]
    #[case::an_extra_key_beside_reason(
        json!({"code": -32602, "message": "Caused by", "data": {"reason": "release-from-driver", "p": "/home/x/y"}}),
        r#"{"code":-32602,"message":"invalid params","data":{"reason":"release-from-driver"}}"#
    )]
    #[case::another_reason(
        json!({"code": -32602, "data": {"reason": "/home/x/y"}}),
        r#"{"code":-32602,"message":"invalid params","data":null}"#
    )]
    #[case::a_path_as_the_wrapper(
        json!({"code": -32602, "data": {"supported": 1, "wrapper": "/home/x/y"}}),
        r#"{"code":-32602,"message":"invalid params","data":null}"#
    )]
    #[case::an_empty_wrapper(
        json!({"code": -32602, "data": {"supported": 1, "wrapper": ""}}),
        r#"{"code":-32602,"message":"invalid params","data":null}"#
    )]
    #[case::a_wrapper_past_sixty_four(
        json!({"code": -32602, "data": {"supported": 1, "wrapper": "1".repeat(65)}}),
        r#"{"code":-32602,"message":"invalid params","data":null}"#
    )]
    #[case::supported_not_an_integer(
        json!({"code": -32602, "data": {"supported": "1", "wrapper": "0.1.0"}}),
        r#"{"code":-32602,"message":"invalid params","data":null}"#
    )]
    #[case::no_message_and_array_data(
        json!({"code": -32603, "data": [1]}),
        r#"{"code":-32603,"message":"internal error","data":null}"#
    )]
    fn sanitised_fault_carries_nothing_of_the_peer_but_closed_values(
        #[case] error: Value,
        #[case] body: &str,
    ) {
        let read = sanitised_fault(&error).expect("a fault");
        assert_eq!(read.to_string(), body);
    }

    #[test]
    fn sanitised_fault_keeps_a_wrapper_of_sixty_four_version_characters() {
        let wrapper = format!("{}.-+", "aZ9".repeat(20) + "a");
        assert_eq!(wrapper.len(), 64);
        let read =
            sanitised_fault(&json!({"code": -32602, "data": {"supported": 2, "wrapper": wrapper}}))
                .expect("a fault");
        assert_eq!(read["data"]["wrapper"], wrapper.as_str());
        assert_eq!(read["data"]["supported"], 2);
    }

    #[rstest]
    #[case::no_code(json!({}))]
    #[case::string_code(json!({"code": "-32603", "message": "internal error"}))]
    #[case::float_code(json!({"code": -32603.5}))]
    fn sanitised_fault_without_an_integer_code_is_nothing(#[case] error: Value) {
        assert_eq!(sanitised_fault(&error), None);
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
