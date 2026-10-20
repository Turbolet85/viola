//! `viola-mcp-tap`: the test-only tap that records one live handshake / `tools/list` exchange
//! between a real `claude` MCP client and `viola mcp`, and turns the capture into a committed
//! `Mcp.<variant>.json` fixture (test-plan §7 Fixture hygiene).
//!
//! - `record <capture file> <viola exe> <args…>`: spawns `<viola exe> <args…>` directly (no shell,
//!   no `PATH` lookup), copies stdin to the child and the child's stdout back, line for line, each
//!   line read through `take(MAX_FRAME)`, and appends each line tagged with its direction to the
//!   capture file (0600), one write per line. It exits with the child.
//! - `fixture <capture file> <out file>`: the client's frames verbatim and in order up to and
//!   including its first `tools/list` request (on protocol `2026-07-28` claude 2.1.302 sends
//!   `server/discover`, then `tools/list`), and the result each request was answered with, keyed by its
//!   method, scrubbed (home → `~`,
//!   the user word → `<user>`); a document still holding a home, an absolute path, the user word
//!   or an email is refused by file name and closed code, never by its content, and nothing is
//!   written.

use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::Path;
use std::process::{Command, ExitCode, Stdio};
use std::sync::{Arc, Mutex, PoisonError};

use serde_json::{Map, Value, json};
use viola_agent_claude::ledger;
use viola_core::MAX_FRAME;

const USAGE: &str =
    "usage: viola-mcp-tap record <capture> <viola exe> <args...> | fixture <capture> <out>";

/// The capture's two directions.
const CLIENT: &str = "client";
const SERVER: &str = "server";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("record") if args.len() >= 3 => record(&args[1], &args[2], &args[3..]),
        Some("fixture") if args.len() == 3 => fixture(Path::new(&args[1]), Path::new(&args[2])),
        _ => {
            eprintln!("{USAGE}");
            ExitCode::from(2)
        }
    }
}

fn open_capture(path: &str) -> io::Result<File> {
    let mut options = OpenOptions::new();
    options.create(true).append(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt as _;
        options.mode(0o600);
    }
    options.open(path)
}

/// One line, bounded: `None` at EOF, on a read error or past `MAX_FRAME` bytes without its LF.
fn next_line(reader: &mut impl BufRead) -> Option<Vec<u8>> {
    let mut line = Vec::new();
    let read = reader.by_ref().take(MAX_FRAME).read_until(b'\n', &mut line);
    match read {
        Ok(0) | Err(_) => None,
        Ok(_) if line.last() != Some(&b'\n') && line.len() as u64 >= MAX_FRAME => None,
        Ok(_) => Some(line),
    }
}

/// One capture line: `{"dir": …, "line": …}`, the line as text without its terminator.
fn file_line(capture: &Mutex<File>, dir: &str, line: &[u8]) {
    let text = String::from_utf8_lossy(line);
    let mut entry = json!({"dir": dir, "line": text.trim_end_matches(['\r', '\n'])}).to_string();
    entry.push('\n');
    let mut file = capture.lock().unwrap_or_else(PoisonError::into_inner);
    let _ = file.write_all(entry.as_bytes());
}

fn record(capture: &str, viola: &str, args: &[String]) -> ExitCode {
    let Ok(file) = open_capture(capture) else {
        eprintln!("viola-mcp-tap: the capture file could not be opened");
        return ExitCode::from(1);
    };
    let capture = Arc::new(Mutex::new(file));
    let spawned = Command::new(viola)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn();
    let Ok(mut child) = spawned else {
        eprintln!("viola-mcp-tap: the server could not be started");
        return ExitCode::from(1);
    };
    let (Some(mut to_child), Some(from_child)) = (child.stdin.take(), child.stdout.take()) else {
        let _ = child.kill();
        let _ = child.wait();
        return ExitCode::from(1);
    };
    let back = {
        let capture = Arc::clone(&capture);
        std::thread::spawn(move || {
            let mut reader = BufReader::new(from_child);
            let mut out = io::stdout().lock();
            while let Some(line) = next_line(&mut reader) {
                file_line(&capture, SERVER, &line);
                if out.write_all(&line).and_then(|()| out.flush()).is_err() {
                    break;
                }
            }
        })
    };
    let mut reader = BufReader::new(io::stdin().lock());
    while let Some(line) = next_line(&mut reader) {
        file_line(&capture, CLIENT, &line);
        if to_child
            .write_all(&line)
            .and_then(|()| to_child.flush())
            .is_err()
        {
            break;
        }
    }
    drop(to_child);
    let status = child.wait();
    let _ = back.join();
    match status.ok().and_then(|s| s.code()) {
        Some(code) => ExitCode::from(u8::try_from(code).unwrap_or(1)),
        None => ExitCode::from(1),
    }
}

/// The capture's frames by direction, each line parsed as one JSON object; a line that is not one
/// is left out.
fn frames(text: &str) -> Vec<(String, Map<String, Value>)> {
    text.lines()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
        .filter_map(|entry| {
            let dir = entry["dir"].as_str()?.to_owned();
            let frame = serde_json::from_str::<Value>(entry["line"].as_str()?).ok()?;
            Some((dir, frame.as_object()?.clone()))
        })
        .collect()
}

/// The class document: the client's frames in capture order up to and including its first
/// `tools/list` request, and the result each of its requests was answered with, keyed by method.
/// `None` when the capture holds no `tools/list` request or a request went unanswered.
fn exchange(frames: &[(String, Map<String, Value>)]) -> Option<Value> {
    let method =
        |f: &Map<String, Value>| f.get("method").and_then(Value::as_str).map(str::to_owned);
    let mut client = Vec::new();
    let mut results = Map::new();
    for (_, frame) in frames.iter().filter(|(dir, _)| dir == CLIENT) {
        let name = method(frame)?;
        if let Some(id) = frame.get("id") {
            let result = frames
                .iter()
                .find(|(dir, f)| dir == SERVER && f.get("id") == Some(id))
                .and_then(|(_, f)| f.get("result").cloned())?;
            results.insert(name.clone(), result);
        }
        client.push(Value::Object(frame.clone()));
        if name == "tools/list" {
            return Some(json!({"v": 1, "client": client, "results": results}));
        }
    }
    None
}

fn texts<'a>(value: &'a Value, out: &mut Vec<&'a str>) {
    match value {
        Value::String(s) => out.push(s),
        Value::Array(items) => items.iter().for_each(|v| texts(v, out)),
        Value::Object(map) => map.iter().for_each(|(k, v)| {
            out.push(k);
            texts(v, out);
        }),
        _ => {}
    }
}

/// The first closed code a scrubbed document still earns: each string and key checked as one row.
fn fault(doc: &Value, home: &str, user: &str) -> Option<&'static str> {
    let mut all = Vec::new();
    texts(doc, &mut all);
    all.into_iter().find_map(|text| {
        let row = [text.to_owned()];
        ledger::screen_fault(&row, &row, home, user).map(|f| f.why.code())
    })
}

fn fixture(capture: &Path, out: &Path) -> ExitCode {
    let name = out
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let mut text = String::new();
    let read = File::open(capture).and_then(|f| f.take(MAX_FRAME).read_to_string(&mut text));
    if read.is_err() {
        eprintln!("viola-mcp-tap: the capture could not be read");
        return ExitCode::from(1);
    }
    let Some(doc) = exchange(&frames(&text)) else {
        eprintln!("viola-mcp-tap: {name} no-exchange");
        return ExitCode::from(1);
    };
    let user_home = std::env::home_dir().unwrap_or_default();
    let home = user_home.to_string_lossy().into_owned();
    let user = user_home
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let scrubbed = ledger::scrub(&doc, &home, &user, viola_agent_claude::CASE_INSENSITIVE);
    if let Some(code) = fault(&scrubbed, &home, &user) {
        eprintln!("viola-mcp-tap: {name} {code}");
        return ExitCode::from(1);
    }
    let mut body = serde_json::to_string_pretty(&scrubbed).unwrap_or_default();
    body.push('\n');
    if std::fs::write(out, body).is_err() {
        eprintln!("viola-mcp-tap: {name} could not be written");
        return ExitCode::from(1);
    }
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;

    fn capture_of(lines: &[(&str, Value)]) -> String {
        lines
            .iter()
            .map(|(dir, frame)| json!({"dir": dir, "line": frame.to_string()}).to_string() + "\n")
            .collect()
    }

    fn planted() -> String {
        capture_of(&[
            (
                CLIENT,
                json!({"jsonrpc":"2.0","id":"d-1","method":"server/discover","params":{}}),
            ),
            (
                SERVER,
                json!({"jsonrpc":"2.0","id":"d-1","result":{"capabilities":{"tools":{}},"ttlMs":0}}),
            ),
            (
                CLIENT,
                json!({"jsonrpc":"2.0","method":"notifications/message"}),
            ),
            (
                CLIENT,
                json!({"jsonrpc":"2.0","id":0,"method":"tools/list"}),
            ),
            (
                SERVER,
                json!({"jsonrpc":"2.0","id":0,"result":{"tools":[{"name":"send"}],"ttlMs":0}}),
            ),
            (
                CLIENT,
                json!({"jsonrpc":"2.0","id":1,"method":"tools/call"}),
            ),
        ])
    }

    #[test]
    fn exchange_takes_the_client_frames_through_tools_list_and_each_result_by_method() {
        let doc = exchange(&frames(&planted())).expect("an exchange");
        let methods: Vec<&str> = doc["client"]
            .as_array()
            .expect("client")
            .iter()
            .filter_map(|f| f["method"].as_str())
            .collect();
        assert_eq!(
            methods,
            ["server/discover", "notifications/message", "tools/list"]
        );
        assert_eq!(doc["results"]["tools/list"]["ttlMs"], 0);
        assert!(doc["results"]["server/discover"]["capabilities"]["tools"].is_object());
        assert_eq!(doc["results"].as_object().map(Map::len), Some(2));
        assert_eq!(doc["v"], 1);
    }

    #[test]
    fn exchange_of_a_capture_without_tools_list_is_none() {
        let partial: String = planted()
            .lines()
            .take(3)
            .map(|l| format!("{l}\n"))
            .collect();
        assert_eq!(exchange(&frames(&partial)), None);
    }

    #[test]
    fn fault_names_each_closed_code_and_a_clean_document_none() {
        for (label, text, want) in [
            ("clean", "~/work", None),
            ("home", "/srv/h/plantuser/x", Some("home-path")),
            ("absolute", "/home/someone/x", Some("absolute-path")),
            ("user", "made by plantuser", Some("username")),
            ("email", "a.b@example.com", Some("email")),
        ] {
            let doc = json!({"clientInfo": {"name": text}});
            assert_eq!(
                fault(&doc, "/srv/h/plantuser", "plantuser"),
                want,
                "{label}"
            );
        }
    }
}
