//! The test-only `viola-mcp-tap` spawned as it runs (security-plan · MCP fixtures through the
//! test-only tap; test-plan §7 Fixture hygiene): a wrong argument count is refused with its usage,
//! `fixture` writes the class document of a clean capture and refuses a dirty one by file name and
//! closed code alone, and `record` carries a `viola mcp` child's exchange both ways, line for line,
//! into a 0600 capture. Every child is owned and killed at a stated deadline.

use std::io::{BufRead as _, BufReader, Write as _};
use std::os::unix::fs::PermissionsExt as _;
use std::path::Path;
use std::process::{Child, Command, Output, Stdio};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use serde_json::{Value, json};
use viola_e2e::harness::{Workspace, bin_dir_from_exe};

const TAP: &str = env!("CARGO_BIN_EXE_viola-mcp-tap");
const USAGE: &str =
    "usage: viola-mcp-tap record <capture> <viola exe> <args...> | fixture <capture> <out>\n";
const WITHIN: Duration = Duration::from_secs(10);

/// The child's output once it exits, killed and failing the case past `WITHIN`.
fn finished(mut child: Child) -> Output {
    let until = Instant::now() + WITHIN;
    loop {
        if child.try_wait().expect("try_wait").is_some() {
            return child.wait_with_output().expect("output");
        }
        if Instant::now() >= until {
            let _ = child.kill();
            let _ = child.wait();
            panic!("the tap did not exit within {WITHIN:?}");
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn tap(args: &[&str], home: &Path) -> Output {
    let child = Command::new(TAP)
        .args(args)
        .env("HOME", home)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the tap");
    finished(child)
}

fn capture_of(lines: &[(&str, Value)]) -> String {
    lines
        .iter()
        .map(|(dir, frame)| json!({"dir": dir, "line": frame.to_string()}).to_string() + "\n")
        .collect()
}

/// A discover, a notification and a `tools/list`, each request answered.
fn exchange(name: &str) -> String {
    capture_of(&[
        (
            "client",
            json!({"jsonrpc":"2.0","id":"d-1","method":"server/discover","params":{"name":name}}),
        ),
        (
            "server",
            json!({"jsonrpc":"2.0","id":"d-1","result":{"capabilities":{"tools":{}}}}),
        ),
        (
            "client",
            json!({"jsonrpc":"2.0","method":"notifications/initialized"}),
        ),
        (
            "client",
            json!({"jsonrpc":"2.0","id":0,"method":"tools/list"}),
        ),
        (
            "server",
            json!({"jsonrpc":"2.0","id":0,"result":{"tools":[{"name":"send"}]}}),
        ),
    ])
}

#[test]
fn mcp_tap_refuses_a_wrong_argument_count_with_its_usage() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let cases: [(&str, &[&str]); 6] = [
        ("no verb", &[]),
        ("record alone", &["record"]),
        ("record without a server", &["record", "c"]),
        ("fixture without an out file", &["fixture", "c"]),
        ("fixture with one more", &["fixture", "c", "o", "x"]),
        ("another verb", &["replay", "c", "o"]),
    ];
    for (label, args) in cases {
        let out = tap(args, tmp.path());
        assert_eq!(out.status.code(), Some(2), "{label}");
        assert_eq!(String::from_utf8_lossy(&out.stderr), USAGE, "{label}");
        assert!(out.stdout.is_empty(), "{label}");
    }
}

#[test]
fn mcp_tap_fixture_writes_the_class_document_of_a_clean_capture() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let capture = tmp.path().join("capture");
    std::fs::write(&capture, exchange("clean")).expect("capture");
    let fixture = tmp.path().join("Mcp.clean-1.json");
    let out = tap(
        &[
            "fixture",
            capture.to_str().expect("utf-8"),
            fixture.to_str().expect("utf-8"),
        ],
        &tmp.path().join("plantuser"),
    );
    assert_eq!(out.status.code(), Some(0));
    assert!(out.stderr.is_empty() && out.stdout.is_empty());
    let doc: Value =
        serde_json::from_slice(&std::fs::read(&fixture).expect("the fixture")).expect("JSON");
    let methods: Vec<&str> = doc["client"]
        .as_array()
        .expect("client")
        .iter()
        .filter_map(|f| f["method"].as_str())
        .collect();
    assert_eq!(
        methods,
        ["server/discover", "notifications/initialized", "tools/list"]
    );
    assert_eq!(doc["results"]["tools/list"]["tools"][0]["name"], "send");
    assert_eq!(doc["v"], 1);
}

#[test]
fn mcp_tap_fixture_refuses_a_dirty_capture_by_file_name_and_code_alone() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let capture = tmp.path().join("capture");
    std::fs::write(&capture, exchange("a.b@example.com")).expect("capture");
    let fixture = tmp.path().join("Mcp.dirty-1.json");
    let out = tap(
        &[
            "fixture",
            capture.to_str().expect("utf-8"),
            fixture.to_str().expect("utf-8"),
        ],
        &tmp.path().join("plantuser"),
    );
    assert_eq!(out.status.code(), Some(1));
    assert_eq!(
        String::from_utf8_lossy(&out.stderr),
        "viola-mcp-tap: Mcp.dirty-1.json email\n"
    );
    assert!(out.stdout.is_empty());
    assert!(!fixture.exists(), "nothing is written");
}

#[test]
fn mcp_tap_record_carries_a_viola_mcp_exchange_both_ways_into_its_capture() {
    let ws = Workspace::from_build();
    ws.ensure_e2e_home().expect("the e2e home base");
    let tmp = tempfile::tempdir_in(ws.e2e_home()).expect("tempdir");
    let home = tmp.path().join("vhome");
    viola_state::fs::create_private_dir(&home).expect("a private home");
    let capture = tmp.path().join("capture");
    let viola = bin_dir_from_exe(&std::env::current_exe().expect("test exe")).join("viola");
    let mut child = Command::new(TAP)
        .arg("record")
        .arg(&capture)
        .arg(&viola)
        .arg("--home")
        .arg(&home)
        .arg("mcp")
        .env_remove("VIOLA_NAME")
        .env_remove("VIOLA_DIR")
        .env_remove("VIOLA_BIN")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("the tap");
    let client = [
        json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"t","version":"0"}}}),
        json!({"jsonrpc":"2.0","method":"notifications/initialized"}),
        json!({"jsonrpc":"2.0","id":2,"method":"tools/list"}),
    ];
    let stdout = child.stdout.take().expect("stdout");
    let (lines, answered) = mpsc::channel();
    std::thread::spawn(move || {
        for line in BufReader::new(stdout).lines().map_while(Result::ok) {
            if lines.send(line).is_err() {
                break;
            }
        }
    });
    let mut stdin = child.stdin.take().expect("stdin");
    for frame in &client {
        stdin
            .write_all(format!("{frame}\n").as_bytes())
            .expect("a client frame");
    }
    stdin.flush().expect("flush");
    let until = Instant::now() + WITHIN;
    let mut ids = Vec::new();
    while ids.len() < 2 {
        let left = until.saturating_duration_since(Instant::now());
        let Ok(line) = answered.recv_timeout(left) else {
            let _ = child.kill();
            let _ = child.wait();
            panic!("the tap passed back {ids:?} within {WITHIN:?}");
        };
        let frame: Value = serde_json::from_str(&line).expect("a server frame");
        ids.push(frame["id"].clone());
    }
    assert_eq!(ids, [json!(1), json!(2)]);
    drop(stdin);
    let out = finished(child);
    assert_eq!(out.status.code(), Some(0));

    let entries: Vec<Value> = std::fs::read_to_string(&capture)
        .expect("the capture")
        .lines()
        .map(|l| serde_json::from_str(l).expect("one entry per line"))
        .collect();
    let sent: Vec<Value> = entries
        .iter()
        .filter(|e| e["dir"] == "client")
        .map(|e| serde_json::from_str(e["line"].as_str().expect("text")).expect("a frame"))
        .collect();
    assert_eq!(sent, client);
    let served: Vec<Value> = entries
        .iter()
        .filter(|e| e["dir"] == "server")
        .map(|e| serde_json::from_str::<Value>(e["line"].as_str().expect("text")).expect("frame"))
        .map(|f| f["id"].clone())
        .collect();
    assert_eq!(served, [json!(1), json!(2)]);
    let mode = std::fs::metadata(&capture)
        .expect("meta")
        .permissions()
        .mode();
    assert_eq!(mode & 0o777, 0o600);
}
