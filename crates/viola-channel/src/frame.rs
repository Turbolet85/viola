//! Frames: one JSON object per line, the line (its `\n` included) at most `MAX_FRAME` bytes. Every
//! channel reader goes through [`read_frame`], which bounds the read before it looks for the end of
//! the line; a longer line is refused, never truncated and parsed (security-plan §Input Validation).

use std::io::{self, BufRead, Write};

use serde::Deserialize;
use serde_json::{Value, json};
use veil::Redact;
use viola_core::MAX_FRAME;

use crate::{ChannelError, ProtocolError};

/// One complete line, `\n` included, of at most `MAX_FRAME` bytes. `Oversize` when the bound is
/// reached without a `\n`; `Closed` at end of input, before any byte or mid-line (a torn frame is
/// never parsed).
pub fn read_frame(reader: &mut impl BufRead) -> Result<Vec<u8>, ChannelError> {
    read_frame_within(reader, MAX_FRAME)
}

/// `read_frame` under any bound: a line of exactly `cap` bytes, its `\n` included, is a frame.
fn read_frame_within(reader: &mut impl BufRead, cap: u64) -> Result<Vec<u8>, ChannelError> {
    let mut line = Vec::new();
    io::Read::take(&mut *reader, cap)
        .read_until(b'\n', &mut line)
        .map_err(ChannelError::Io)?;
    if line.last() == Some(&b'\n') {
        Ok(line)
    } else if u64::try_from(line.len()).is_ok_and(|len| len >= cap) {
        Err(ChannelError::Oversize)
    } else {
        Err(ChannelError::Closed)
    }
}

/// The frame and its `\n` in one `write_all`.
pub fn write_frame(writer: &mut impl Write, frame: &Value) -> Result<(), ChannelError> {
    let mut line = frame.to_string();
    line.push('\n');
    writer.write_all(line.as_bytes()).map_err(ChannelError::Io)
}

/// A request (with an `id`) or a notification. `params` is content: its `Debug` is redacted, and
/// no log line ever carries it (obs-plan §8).
#[derive(Deserialize, Redact)]
pub struct Request {
    pub jsonrpc: String,
    #[serde(default)]
    pub id: Option<u64>,
    pub method: String,
    #[serde(default)]
    #[redact(fixed = 8)]
    pub params: Value,
}

/// The envelope fields every `params` carries, read tolerantly: unknown fields are skipped and a
/// missing one is its default (architecture §Cross-cutting Mixed-version tolerance). `from` is a
/// method's own param, checked by the method (`-32602`, security-plan §Input Validation), so the
/// envelope never reads it.
#[derive(Debug, Default, Deserialize)]
pub struct Params {
    #[serde(default)]
    pub v: u64,
    #[serde(default)]
    pub sender: Option<String>,
    #[serde(default)]
    pub conn: Option<String>,
}

/// A line's request and its envelope, or the fault to answer it with and the id to answer, when
/// one was readable.
pub fn parse_request(line: &[u8]) -> Result<(Request, Params), (Option<u64>, ProtocolError)> {
    let value: Value = serde_json::from_slice(line).map_err(|_| (None, ProtocolError::Parse))?;
    let id = value.get("id").and_then(Value::as_u64);
    let invalid = (id, ProtocolError::InvalidRequest);
    let request: Request = serde_json::from_value(value).map_err(|_| invalid)?;
    if request.jsonrpc != "2.0" {
        return Err(invalid);
    }
    let params = match &request.params {
        Value::Null => Params::default(),
        Value::Object(_) => Params::deserialize(&request.params).map_err(|_| invalid)?,
        _ => return Err(invalid),
    };
    Ok((request, params))
}

pub fn ok_response(id: u64, result: Value) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "result": result})
}

/// A fault reply; `id` is `null` when the request's id could not be read.
pub fn error_response(id: Option<u64>, fault: ProtocolError) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "error": fault.body()})
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::io::Cursor;

    const CAP: usize = 16 * 1024 * 1024;

    /// A line of `len` bytes, the last one `\n`.
    fn line_of(len: usize) -> Vec<u8> {
        let mut line = vec![b'x'; len - 1];
        line.push(b'\n');
        line
    }

    #[rstest]
    #[case::one_under(CAP - 1)]
    #[case::at_the_bound(CAP)]
    fn read_frame_takes_a_line_up_to_the_bound(#[case] len: usize) {
        let mut input = line_of(len);
        input.extend_from_slice(b"{}\n");
        let mut reader = Cursor::new(input);
        assert_eq!(read_frame(&mut reader).expect("frame").len(), len);
        assert_eq!(read_frame(&mut reader).expect("next frame"), b"{}\n");
    }

    #[test]
    fn read_frame_refuses_one_byte_over_the_bound() {
        let mut reader = Cursor::new(line_of(CAP + 1));
        assert!(matches!(
            read_frame(&mut reader),
            Err(ChannelError::Oversize)
        ));
        assert_eq!(
            reader.position(),
            16_777_216,
            "the read stopped at the bound"
        );
    }

    #[rstest]
    #[case::empty(b"")]
    #[case::torn(b"{\"jsonrpc\":\"2.0\"")]
    fn read_frame_at_end_of_input_is_closed(#[case] input: &[u8]) {
        let mut reader = Cursor::new(input.to_vec());
        assert!(matches!(read_frame(&mut reader), Err(ChannelError::Closed)));
    }

    /// Written apart from the product: the first line when it ends within `cap` bytes, oversize
    /// when `cap` bytes hold no `\n`, closed when the input ends first.
    fn oracle(bytes: &[u8], cap: usize) -> Result<Vec<u8>, &'static str> {
        match bytes.iter().position(|b| *b == b'\n') {
            Some(end) if end < cap => Ok(bytes[..=end].to_vec()),
            _ if bytes.len() >= cap => Err("oversize"),
            _ => Err("closed"),
        }
    }

    fn outcome(read: Result<Vec<u8>, ChannelError>) -> Result<Vec<u8>, &'static str> {
        match read {
            Ok(line) => Ok(line),
            Err(ChannelError::Oversize) => Err("oversize"),
            Err(ChannelError::Closed) => Err("closed"),
            Err(_) => Err("other"),
        }
    }

    fn config() -> proptest::test_runner::Config {
        proptest::test_runner::Config {
            cases: 512,
            failure_persistence: Some(Box::new(
                proptest::test_runner::FileFailurePersistence::SourceParallel(
                    "proptest-regressions",
                ),
            )),
            ..proptest::test_runner::Config::default()
        }
    }

    proptest::proptest! {
        #![proptest_config(config())]

        /// Any byte string is a frame, or refused oversize exactly when its line exceeds the bound.
        #[test]
        fn read_frame_prop_frames_or_refuses_by_line_length(
            cap in 1usize..32,
            bytes in proptest::collection::vec(
                proptest::prop_oneof![proptest::strategy::Just(b'\n'), proptest::arbitrary::any::<u8>()],
                0..96,
            ),
        ) {
            let bound = u64::try_from(cap).expect("small");
            let got = outcome(read_frame_within(&mut Cursor::new(bytes.clone()), bound));
            proptest::prop_assert_eq!(got, oracle(&bytes, cap));
        }
    }

    #[test]
    fn write_frame_writes_one_line_in_one_call() {
        struct Calls(Vec<Vec<u8>>);
        impl Write for Calls {
            fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                self.0.push(buf.to_vec());
                Ok(buf.len())
            }
            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }
        let mut out = Calls(Vec::new());
        write_frame(&mut out, &json!({"a": "b\nc"})).expect("write");
        assert_eq!(out.0, [b"{\"a\":\"b\\nc\"}\n".to_vec()]);
    }

    fn parsed(line: &str) -> Result<(Request, Params), (Option<u64>, ProtocolError)> {
        parse_request(line.as_bytes())
    }

    #[test]
    fn parse_request_reads_the_envelope_and_skips_unknown_fields() {
        let (request, params) = parsed(
            r#"{"jsonrpc":"2.0","id":7,"method":"send","params":{"v":1,"sender":"0.1.0","conn":"cli-1-2-3","from":"overseer","later":true},"extra":1}"#,
        )
        .expect("request");
        assert_eq!(request.id, Some(7));
        assert_eq!(request.method, "send");
        assert_eq!(request.params["later"], true);
        assert_eq!(params.v, 1);
        assert_eq!(params.sender.as_deref(), Some("0.1.0"));
        assert_eq!(params.conn.as_deref(), Some("cli-1-2-3"));
        assert_eq!(request.params["from"], "overseer");
    }

    /// A `from` of the wrong type is the method's `-32602` to give, never the envelope's `-32600`.
    #[test]
    fn parse_request_leaves_a_mistyped_from_to_the_method() {
        for from in ["7", "[]", "{}", "true"] {
            let line = format!(
                r#"{{"jsonrpc":"2.0","id":1,"method":"last","params":{{"v":1,"from":{from}}}}}"#
            );
            let (request, _) = parsed(&line).expect("request");
            assert!(!request.params["from"].is_string(), "{from}");
        }
    }

    #[test]
    fn parse_request_takes_a_notification_without_params() {
        let (request, params) =
            parsed(r#"{"jsonrpc":"2.0","method":"hook.event"}"#).expect("notification");
        assert_eq!(request.id, None);
        assert_eq!(params.v, 0);
        assert!(params.sender.is_none() && params.conn.is_none());
    }

    #[rstest]
    #[case::not_json("not json", None, ProtocolError::Parse)]
    #[case::no_method(r#"{"jsonrpc":"2.0","id":3}"#, Some(3), ProtocolError::InvalidRequest)]
    #[case::wrong_version(
        r#"{"jsonrpc":"1.0","id":4,"method":"send"}"#,
        Some(4),
        ProtocolError::InvalidRequest
    )]
    #[case::array_params(
        r#"{"jsonrpc":"2.0","id":5,"method":"send","params":[1]}"#,
        Some(5),
        ProtocolError::InvalidRequest
    )]
    #[case::bad_v(
        r#"{"jsonrpc":"2.0","id":6,"method":"send","params":{"v":"one"}}"#,
        Some(6),
        ProtocolError::InvalidRequest
    )]
    #[case::string_id(
        r#"{"jsonrpc":"2.0","id":"7","method":"send"}"#,
        None,
        ProtocolError::InvalidRequest
    )]
    #[case::not_an_object("[1,2]", None, ProtocolError::InvalidRequest)]
    fn parse_request_faults_carry_the_readable_id(
        #[case] line: &str,
        #[case] id: Option<u64>,
        #[case] fault: ProtocolError,
    ) {
        let Err(got) = parsed(line) else {
            panic!("{line} parsed");
        };
        assert_eq!(got, (id, fault));
    }

    #[test]
    fn request_debug_redacts_params() {
        let (request, _) = parsed(
            r#"{"jsonrpc":"2.0","id":1,"method":"send","params":{"text":"canary-chain-value-5c1e"}}"#,
        )
        .expect("request");
        let shown = format!("{request:?}");
        assert!(!shown.contains("canary"), "{shown}");
        assert!(shown.contains("send"), "{shown}");
    }

    #[test]
    fn responses_carry_the_id_or_null() {
        assert_eq!(
            ok_response(9, json!({"ok": {}})).to_string(),
            r#"{"jsonrpc":"2.0","id":9,"result":{"ok":{}}}"#
        );
        assert_eq!(
            error_response(None, ProtocolError::Parse).to_string(),
            r#"{"jsonrpc":"2.0","id":null,"error":{"code":-32700,"message":"parse error","data":null}}"#
        );
        assert_eq!(
            error_response(Some(2), ProtocolError::MethodNotFound)["id"],
            2
        );
    }
}
