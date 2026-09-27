//! Framing over a real endpoint (test-plan §4 viola-channel): a line of `MAX_FRAME` bytes (its
//! `\n` counted) is a frame, one byte more is refused `-32600` and the connection closes, and a line
//! that is not JSON is answered `-32700` on a connection that stays open.

use std::io::{BufRead as _, BufReader, ErrorKind, Read as _, Write as _};
use std::sync::Arc;

use interprocess::local_socket::traits::Stream as _;
use interprocess::local_socket::{GenericFilePath, Stream, ToFsName as _};
use rstest::rstest;
use serde_json::{Value, json};
use viola_channel::{Dispatch, ProtocolError, Server, Serving};

const MAX_FRAME: usize = 16 * 1024 * 1024;

struct Echo;

impl Dispatch for Echo {
    fn dispatch(&self, method: &str, _params: &Value) -> Result<Value, ProtocolError> {
        match method {
            "send" => Ok(json!({"ok": {}})),
            _ => Err(ProtocolError::MethodNotFound),
        }
    }
}

/// A test-owned endpoint, outside the `viola-<h12>` namespace `viola run` binds.
fn serve(dir: &std::path::Path, label: &str) -> (String, Serving) {
    let base = format!("viola-test-chan-{}-{label}", std::process::id());
    let endpoint = if cfg!(windows) {
        format!(r"\\.\pipe\{base}")
    } else {
        dir.join(format!("{base}.sock"))
            .to_string_lossy()
            .into_owned()
    };
    let serving = Server::bind(&endpoint)
        .expect("bound")
        .serve(Arc::new(Echo));
    (endpoint, serving)
}

fn raw(endpoint: &str) -> BufReader<Stream> {
    let name = endpoint
        .to_fs_name::<GenericFilePath>()
        .expect("endpoint name");
    BufReader::new(Stream::connect(name).expect("connect"))
}

fn send(stream: &mut BufReader<Stream>, bytes: &[u8]) -> Value {
    stream.get_mut().write_all(bytes).expect("write");
    let mut line = String::new();
    stream.read_line(&mut line).expect("reply");
    serde_json::from_str(&line).expect("one JSON reply")
}

/// A `send` request padded to exactly `len` bytes, its `\n` included.
fn request_of(len: usize) -> Vec<u8> {
    let head = r#"{"jsonrpc":"2.0","id":1,"method":"send","params":{"v":1,"pad":""#;
    let tail = "\"}}\n";
    let mut line = head.as_bytes().to_vec();
    line.resize(len - tail.len(), b'x');
    line.extend_from_slice(tail.as_bytes());
    assert_eq!(line.len(), len);
    line
}

#[rstest]
#[case::one_under(MAX_FRAME - 1)]
#[case::at_the_bound(MAX_FRAME)]
fn channel_frame_up_to_the_bound_is_answered(#[case] len: usize) {
    let dir = tempfile::tempdir().expect("tempdir");
    let (endpoint, _serving) = serve(dir.path(), &format!("f{len}"));
    let mut stream = raw(&endpoint);
    let reply = send(&mut stream, &request_of(len));
    assert_eq!(
        reply,
        json!({"jsonrpc": "2.0", "id": 1, "result": {"ok": {}}})
    );
}

/// The server reads no further than the bound, answers once and closes: the next read ends the
/// stream (a reset where the OS reports the unread byte it dropped). The frame's tail may meet that
/// close (measured on macOS: EPIPE on the write); the reply must still be there.
#[test]
fn channel_frame_one_byte_over_is_refused_and_closed() {
    let dir = tempfile::tempdir().expect("tempdir");
    let (endpoint, _serving) = serve(dir.path(), "over");
    let mut stream = raw(&endpoint);
    match stream.get_mut().write_all(&request_of(MAX_FRAME + 1)) {
        Err(e) if matches!(e.kind(), ErrorKind::BrokenPipe | ErrorKind::ConnectionReset) => {}
        written => written.expect("write"),
    }
    let mut line = String::new();
    stream.read_line(&mut line).expect("reply");
    let reply: Value = serde_json::from_str(&line).expect("one JSON reply");
    assert_eq!(
        reply,
        json!({"jsonrpc": "2.0", "id": null, "error":
            {"code": -32600, "message": "invalid request", "data": null}})
    );
    let mut rest = [0u8; 16];
    let next = stream.read(&mut rest);
    // Measured on Windows: 0. Linux reports ECONNRESET to the peer of a socket closed with an
    // unread byte (the `\n` past the bound).
    if cfg!(windows) {
        assert_eq!(
            next.expect("end of stream"),
            0,
            "the connection stayed open"
        );
    } else {
        match next {
            Ok(n) => assert_eq!(n, 0, "the connection stayed open"),
            Err(e) => assert_eq!(e.kind(), ErrorKind::ConnectionReset, "{e}"),
        }
    }
}

#[test]
fn channel_frame_not_json_is_a_parse_error_and_the_connection_stays() {
    let dir = tempfile::tempdir().expect("tempdir");
    let (endpoint, _serving) = serve(dir.path(), "parse");
    let mut stream = raw(&endpoint);
    let reply = send(&mut stream, b"{not json\n");
    assert_eq!(reply["id"], Value::Null);
    assert_eq!(reply["error"]["code"], -32700);
    let next = send(&mut stream, &request_of(80));
    assert_eq!(next["result"]["ok"], json!({}));
}
