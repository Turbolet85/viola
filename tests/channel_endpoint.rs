//! The wrapper channel over a real endpoint (test-plan §5 Module ↔ IPC; §6 Path 1): the fault
//! codes and the exclusive bind against a test-scoped server, the pipe's DACL on Windows, then a real
//! `viola run`'s endpoint and the `channel-*` lines its role file carries.

#[allow(dead_code)]
mod support;

use std::fs;
use std::io::{BufRead as _, BufReader, ErrorKind, Write as _};
use std::path::Path;
use std::sync::{Arc, mpsc};

use interprocess::local_socket::traits::Stream as _;
use interprocess::local_socket::{GenericFilePath, Stream, ToFsName as _};
use rstest::rstest;
use serde_json::{Map, Value, json};
use support::home::{
    StampedHome, Wrapper, booted_wrapper, snapshot_data, stamped_home, wait_endpoint_gone,
};
use support::hygiene::load_schema;
use support::watch::WITHIN;
#[cfg(windows)]
use viola_channel::test_support as win;
use viola_channel::{ChannelError, Client, Dispatch, ProtocolError, Server};

const MAX_FRAME: usize = 16 * 1024 * 1024;
/// The tests-owned content canary (`crates/viola-e2e/src/harness/secret_scan.rs`).
const CANARY: &str = "canary-chain-value-5c1e";

struct NoMethods;

impl Dispatch for NoMethods {
    fn dispatch(&self, _: &str, _: &Value) -> Result<Value, ProtocolError> {
        Err(ProtocolError::MethodNotFound)
    }
}

/// A test-owned endpoint, outside the `viola-<h12>` namespace `viola run` binds. A Unix socket
/// lives in a short `/tmp` dir: macOS caps a socket path near 104 bytes.
fn test_endpoint(label: &str) -> (tempfile::TempDir, String) {
    let base = format!("viola-test-chan-{}-{label}", std::process::id());
    let dir = if cfg!(windows) {
        tempfile::tempdir()
    } else {
        tempfile::Builder::new().prefix("vt").tempdir_in("/tmp")
    }
    .expect("socket dir");
    let endpoint = if cfg!(windows) {
        format!(r"\\.\pipe\{base}")
    } else {
        dir.path()
            .join(format!("{base}.sock"))
            .to_string_lossy()
            .into_owned()
    };
    (dir, endpoint)
}

fn raw(endpoint: &str) -> BufReader<Stream> {
    let name = endpoint
        .to_fs_name::<GenericFilePath>()
        .expect("endpoint name");
    BufReader::new(Stream::connect(name).expect("connect"))
}

fn exchange(stream: &mut BufReader<Stream>, bytes: &[u8]) -> Value {
    stream.get_mut().write_all(bytes).expect("write");
    let mut line = String::new();
    stream.read_line(&mut line).expect("reply");
    serde_json::from_str(&line).expect("one JSON reply")
}

/// A frame past the bound, then the reply. The server answers once it has read `MAX_FRAME` bytes
/// and closes, so the frame's tail may meet the close — measured on macOS as EPIPE (run 36313377307)
/// and as ENOTCONN (run 36482322449), whichever state the peer's close reached first; the reply
/// must still be there.
fn send_oversize(stream: &mut BufReader<Stream>) -> Value {
    match stream.get_mut().write_all(&padded(MAX_FRAME + 1)) {
        Err(e)
            if matches!(
                e.kind(),
                ErrorKind::BrokenPipe | ErrorKind::ConnectionReset | ErrorKind::NotConnected
            ) => {}
        written => written.expect("write"),
    }
    let mut line = String::new();
    stream.read_line(&mut line).expect("reply");
    serde_json::from_str(&line).expect("one JSON reply")
}

fn params(pairs: Value) -> Map<String, Value> {
    pairs.as_object().cloned().expect("object")
}

/// A request line of exactly `len` bytes, its `\n` included.
fn padded(len: usize) -> Vec<u8> {
    let head = r#"{"jsonrpc":"2.0","id":9,"method":"send","params":{"v":1,"pad":""#;
    let tail = "\"}}\n";
    let mut line = head.as_bytes().to_vec();
    line.resize(len - tail.len(), b'x');
    line.extend_from_slice(tail.as_bytes());
    line
}

#[rstest]
fn channel_endpoint_answers_protocol_faults() {
    let (_dir, endpoint) = test_endpoint("faults");
    let _serving = Server::bind(&endpoint)
        .expect("bound")
        .serve(Arc::new(NoMethods));
    let mut client = Client::connect(&endpoint, "cli").expect("connect");
    let unknown = client
        .request("last", params(json!({"later_field": true})))
        .expect("reply");
    assert_eq!(
        unknown["error"]["code"], -32601,
        "an extra params field is no fault"
    );
    assert_eq!(unknown["id"], 1);

    let mut stream = raw(&endpoint);
    let newer = exchange(
        &mut stream,
        br#"{"jsonrpc":"2.0","id":4,"method":"send","params":{"v":2,"sender":"9.9.9"}}
"#,
    );
    assert_eq!(
        newer.to_string(),
        r#"{"jsonrpc":"2.0","id":4,"error":{"code":-32602,"message":"unsupported protocol version","data":{"supported":1,"wrapper":"0.1.0"}}}"#
    );
    let not_json = exchange(&mut stream, b"{\"jsonrpc\":\n");
    assert_eq!(not_json["error"]["code"], -32700);
    assert!(not_json["id"].is_null());
    let over = send_oversize(&mut stream);
    assert_eq!(over["error"]["code"], -32600);
}

/// Two binds of one endpoint: exactly one wins while it lives; the name is free once it drops.
#[rstest]
fn channel_endpoint_second_bind_is_taken_while_the_first_lives() {
    let (_dir, endpoint) = test_endpoint("twice");
    let first = Server::bind(&endpoint).expect("first");
    assert!(matches!(
        Server::bind(&endpoint),
        Err(ChannelError::BindTaken)
    ));
    drop(first);
    Server::bind(&endpoint).expect("free once the first dropped");
}

/// A stopped wrapper counts as gone only once its endpoint refuses a client: the stop helper's wait
/// holds while a bound endpoint still answers, and ends once it is dropped.
#[rstest]
fn stop_wait_holds_while_the_endpoint_answers() {
    let (_dir, endpoint) = test_endpoint("gone");
    let server = Server::bind(&endpoint).expect("bound");
    let (tx, rx) = mpsc::channel();
    let polled = endpoint.clone();
    let waiter = std::thread::Builder::new()
        .name("stop-wait-witness".to_owned())
        .spawn(move || {
            wait_endpoint_gone(&polled, "witness", |reachable| {
                let _ = tx.send(reachable);
            });
        })
        .expect("waiter");
    let seen = loop {
        match rx.recv_timeout(WITHIN) {
            Ok(true) => break true,
            Ok(false) => continue,
            Err(_) => break false,
        }
    };
    assert!(seen, "the wait never read the live endpoint as reachable");
    assert!(
        !waiter.is_finished(),
        "the wait returned while the endpoint still answered"
    );
    drop(server);
    waiter
        .join()
        .expect("the wait ends once the endpoint is gone");
    assert_eq!(rx.try_iter().last(), Some(false));
}

/// The listener's DACL read back from the pipe: protected, the user and SYSTEM only (security-plan
/// IPC access control, Windows). Windows reports the `GA` it was given as `FA`.
#[cfg(windows)]
#[rstest]
fn channel_endpoint_pipe_dacl_is_protected_user_and_system() {
    let (_dir, endpoint) = test_endpoint("dacl");
    let _serving = Server::bind(&endpoint)
        .expect("bound")
        .serve(Arc::new(NoMethods));
    let expected = win::canonical_sddl(&format!("D:P(A;;FA;;;{})(A;;FA;;;SY)", win::user_sid()));
    assert_eq!(win::dacl_of(&endpoint), expected);
}

fn role_lines(home: &Path) -> Vec<Value> {
    support::ndjson::read_lines(&home.join("diagnostics").join("run-builder.ndjson"))
}

fn of_event<'a>(lines: &'a [Value], event: &str) -> Vec<&'a Value> {
    lines.iter().filter(|l| l["event"] == event).collect()
}

/// A real wrapper's endpoint: every reply has its `channel-response`, joined to its request by
/// `corr` and the client's `conn`, or the server's `srv-<n>` for a frame without one; an oversize
/// frame is `parse-rejected`. Every line passes the diag-line schema.
#[rstest]
fn channel_wrapper_logs_each_call_with_corr_and_conn(booted_wrapper: Wrapper) {
    let snapshot = snapshot_data(&booted_wrapper.instance_dir()).expect("snapshot");
    let endpoint = snapshot["endpoint"].as_str().expect("endpoint").to_owned();

    let mut client = Client::connect(&endpoint, "cli").expect("connect");
    let conn = client.conn().to_owned();
    let reply = client.request("link", Map::new()).expect("reply");
    assert_eq!(reply["error"]["code"], -32601);

    let mut bare = raw(&endpoint);
    let no_conn = exchange(
        &mut bare,
        br#"{"jsonrpc":"2.0","id":7,"method":"pause","params":{"v":1,"sender":"0.1.0"}}
"#,
    );
    assert_eq!(no_conn["id"], 7);
    let mut big = raw(&endpoint);
    let over = send_oversize(&mut big);
    assert_eq!(over["error"]["code"], -32600);

    let lines = role_lines(booted_wrapper.home());
    let joined = |event: &str| {
        of_event(&lines, event)
            .into_iter()
            .filter(|l| l["conn"] == conn.as_str() && l["corr"] == 1)
            .count()
    };
    assert_eq!(joined("channel-request"), 1, "{lines:?}");
    assert_eq!(joined("channel-response"), 1, "{lines:?}");
    let answered = of_event(&lines, "channel-response");
    let srv = answered
        .iter()
        .find(|l| l["corr"] == 7)
        .expect("the conn-less call's response");
    let srv_conn = srv["srv_conn"].as_str().expect("srv_conn");
    let n = srv_conn.strip_prefix("srv-").expect("srv-<n>");
    assert!(n.parse::<u64>().is_ok(), "{srv_conn}");
    assert!(srv.get("conn").is_none());
    assert_eq!(srv["method"], "pause");
    assert_eq!(srv["error_code"], -32601);
    let rejected = of_event(&lines, "parse-rejected");
    assert!(
        rejected
            .iter()
            .any(|l| l["parser"] == "channel-frame" && l["detail"] == "oversize"),
        "{lines:?}"
    );
    assert!(
        answered
            .iter()
            .any(|l| l["error_code"] == -32600 && l.get("corr").is_none())
    );

    let schema = load_schema(&support::home::workspace_path("schemas/diag-line.v1.json"));
    let validator = jsonschema::validator_for(&schema).expect("schema");
    for line in &lines {
        assert!(validator.is_valid(line), "{line}");
        let text = line.to_string();
        assert!(
            !text.contains(&endpoint.replace('\\', "\\\\")),
            "an endpoint reached a line"
        );
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        let mode = |p: &Path| fs::metadata(p).expect("meta").permissions().mode() & 0o777;
        let socket = Path::new(&endpoint);
        assert_eq!(mode(socket.parent().expect("socket dir")), 0o700);
        assert_eq!(mode(socket), 0o600);
    }
    assert_eq!(booted_wrapper.stop().code(), Some(0));
}

/// At `diagnostics_level: "debug"` a request's content never reaches a role file (obs-plan §8;
/// verification-matrix v1-20): not in a confirmed `send`'s text (typed, hooked back and read back),
/// not in a peer's `conn` or `sender`, not in a frame that is not even JSON.
#[rstest]
fn channel_debug_level_keeps_request_content_out_of_the_role_files(stamped_home: StampedHome) {
    fs::create_dir_all(stamped_home.home.path()).expect("home");
    fs::write(
        stamped_home.home.path().join("config.json"),
        r#"{"v":1,"diagnostics_level":"debug"}"#,
    )
    .expect("config");
    let fixtures = support::home::workspace_path("fixtures/claude");
    let wrapper = Wrapper::boot(
        stamped_home,
        "builder",
        None,
        &["--fixtures", fixtures.to_str().expect("utf-8 path")],
    );
    let snapshot = snapshot_data(&wrapper.instance_dir()).expect("snapshot");
    let endpoint = snapshot["endpoint"].as_str().expect("endpoint").to_owned();

    let mut client = Client::connect(&endpoint, "cli").expect("connect");
    let reply = client
        .request("send", params(json!({"text": CANARY})))
        .expect("reply");
    assert!(reply["result"]["ok"]["cursor"].is_u64(), "{reply}");
    let mut stream = raw(&endpoint);
    let spoofed = format!(
        r#"{{"jsonrpc":"2.0","id":2,"method":"{CANARY}","params":{{"v":1,"conn":"{CANARY}","sender":"{CANARY}","from":"{CANARY}"}}}}"#
    ) + "\n";
    assert_eq!(
        exchange(&mut stream, spoofed.as_bytes())["error"]["code"],
        -32601
    );
    let garbage = format!("{CANARY} not json\n");
    assert_eq!(
        exchange(&mut stream, garbage.as_bytes())["error"]["code"],
        -32700
    );

    let diagnostics = wrapper.home().join("diagnostics");
    let mut scanned = 0;
    for entry in fs::read_dir(&diagnostics).expect("diagnostics") {
        let path = entry.expect("entry").path();
        let text = fs::read_to_string(&path).expect("role file");
        assert!(
            !text.contains(CANARY),
            "{} carries request content",
            path.display()
        );
        scanned += 1;
    }
    assert!(scanned >= 1);
    let lines = role_lines(wrapper.home());
    // The hooks' own `hook.event` notifications (SessionStart, the read-back prompt) aside.
    let requests: Vec<&Value> = of_event(&lines, "channel-request")
        .into_iter()
        .filter(|l| l["method"] != "hook.event")
        .collect();
    assert_eq!(requests.len(), 2, "{lines:?}");
    assert_eq!(wrapper.stop().code(), Some(0));
}
