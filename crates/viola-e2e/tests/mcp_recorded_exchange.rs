//! The recorded live MCP exchange replayed (test-plan §1 Surfaces under test, MCP stdio; §7): every
//! `fixtures/claude/*/Mcp.*.json` holds the frames a real `claude` client sent `viola mcp` and the
//! `tools/list` result that client accepted. Its client frames are written verbatim to a `viola
//! mcp` child the test owns, and viola's `tools/list` result must hold every key path of the
//! recorded one with the same JSON type, and the same tool names in the same order: the class an
//! rmcp client accepts and the CLI refuses (`ttlMs`, `cacheScope`) is under the gate. The recorded
//! set is walked at run time, red on an empty walk.

use std::io::{BufRead as _, BufReader, Read as _, Write as _};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::time::{Duration, Instant};

use serde_json::Value;
use viola_core::MAX_FRAME;
use viola_e2e::harness::Workspace;
use viola_e2e::harness::boot::session_path;
use viola_e2e::harness::session::{HarnessSession, SessionOptions, harness_session};

/// `<root>/*/Mcp.*.json`, sorted.
fn recorded(root: &Path) -> Vec<PathBuf> {
    let mut found: Vec<PathBuf> = std::fs::read_dir(root)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|version| version.path().is_dir())
        .flat_map(|version| {
            std::fs::read_dir(version.path())
                .into_iter()
                .flatten()
                .flatten()
        })
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with("Mcp.") && n.ends_with(".json"))
        })
        .collect();
    found.sort();
    found
}

fn type_word(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Number(n) if n.is_u64() || n.is_i64() => "integer",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

/// Every key path of `recorded` that `served` lacks or holds with another JSON type; array
/// elements are read by index.
fn missing(recorded: &Value, served: &Value, path: &str, out: &mut Vec<String>) {
    if type_word(recorded) != type_word(served) {
        out.push(path.to_owned());
        return;
    }
    match (recorded, served) {
        (Value::Object(r), Value::Object(s)) => {
            for (key, value) in r {
                let at = if path.is_empty() {
                    key.clone()
                } else {
                    format!("{path}.{key}")
                };
                match s.get(key) {
                    Some(served) => missing(value, served, &at, out),
                    None => out.push(at),
                }
            }
        }
        (Value::Array(r), Value::Array(s)) => {
            for (i, value) in r.iter().enumerate() {
                let at = format!("{path}[{i}]");
                match s.get(i) {
                    Some(served) => missing(value, served, &at, out),
                    None => out.push(at),
                }
            }
        }
        _ => {}
    }
}

fn shape_gaps(recorded: &Value, served: &Value) -> Vec<String> {
    let mut out = Vec::new();
    missing(recorded, served, "", &mut out);
    out
}

fn tool_names(result: &Value) -> Vec<String> {
    result["tools"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|tool| tool["name"].as_str().map(str::to_owned))
        .collect()
}

/// One `viola mcp --home <home>` child over the session's bin dir, `VIOLA_*` removed: the server
/// has no name of its own.
struct Served {
    child: Child,
    stdin: Option<ChildStdin>,
    stdout: BufReader<ChildStdout>,
}

impl Served {
    fn spawn(s: &HarnessSession) -> Self {
        let viola = s
            .bin_dir()
            .join(format!("viola{}", std::env::consts::EXE_SUFFIX));
        let bin = s.workspace().session_dir(s.session()).join("bin");
        let mut child = Command::new(viola)
            .arg("--home")
            .arg(s.home())
            .arg("mcp")
            .env("PATH", session_path(&bin))
            .env_remove("VIOLA_NAME")
            .env_remove("VIOLA_DIR")
            .env_remove("VIOLA_BIN")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("viola mcp");
        let stdin = child.stdin.take();
        let stdout = BufReader::new(child.stdout.take().expect("stdout"));
        Self {
            child,
            stdin,
            stdout,
        }
    }

    /// The frame written verbatim, then, for a request, the response carrying its id.
    fn send(&mut self, frame: &Value) -> Option<Value> {
        let stdin = self.stdin.as_mut().expect("open");
        let mut line = frame.to_string();
        line.push('\n');
        stdin.write_all(line.as_bytes()).expect("a frame");
        stdin.flush().expect("flush");
        let id = frame.get("id")?;
        loop {
            let mut read = Vec::new();
            let n = (&mut self.stdout)
                .take(MAX_FRAME)
                .read_until(b'\n', &mut read)
                .expect("a read");
            assert!(n > 0, "viola mcp closed before answering {id}");
            let response: Value = serde_json::from_slice(&read).expect("one JSON-RPC frame");
            if response.get("id") == Some(id) {
                return Some(response);
            }
        }
    }

    /// stdin closed, then the server's own exit, awaited below the kill line.
    fn close(mut self) -> Option<i32> {
        drop(self.stdin.take());
        let until = Instant::now() + Duration::from_secs(5);
        loop {
            if let Some(status) = self.child.try_wait().expect("try_wait") {
                return status.code();
            }
            if Instant::now() >= until {
                let _ = self.child.kill();
                let _ = self.child.wait();
                return None;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
    }
}

#[test]
fn mcp_recorded_exchange_replays_the_client_frames_and_keeps_the_accepted_result_shape() {
    let walked = recorded(&Workspace::from_build().root.join("fixtures/claude"));
    assert!(!walked.is_empty(), "no recorded MCP exchange walked");
    let session = harness_session(
        "mcp_recorded_exchange",
        "replay",
        SessionOptions {
            instances: vec!["builder".to_owned()],
            unstamped: true,
            ..SessionOptions::default()
        },
    );
    for path in walked {
        let at = path.display().to_string();
        let doc: Value =
            serde_json::from_slice(&std::fs::read(&path).expect("fixture")).expect("JSON");
        let recorded = &doc["results"]["tools/list"];
        let mut served = Served::spawn(&session);
        let mut listed = None;
        for frame in doc["client"].as_array().expect("client frames") {
            let answer = served.send(frame);
            if frame["method"] == "tools/list" {
                listed = answer;
            }
        }
        assert_eq!(served.close(), Some(0), "{at}: viola mcp's exit");
        let listed = &listed.expect("the tools/list response")["result"];
        assert_eq!(shape_gaps(recorded, listed), Vec::<String>::new(), "{at}");
        assert_eq!(tool_names(listed), tool_names(recorded), "{at}");
        assert_eq!(
            tool_names(listed),
            ["send", "wait", "last", "answer", "list"],
            "{at}"
        );
        for (label, key) in [
            ("ttlMs removed", "ttlMs"),
            ("cacheScope removed", "cacheScope"),
        ] {
            let mut planted = listed.clone();
            planted.as_object_mut().expect("a result").remove(key);
            assert_eq!(shape_gaps(recorded, &planted), [key], "{at}: {label}");
        }
    }
    session.cleanup();
}
