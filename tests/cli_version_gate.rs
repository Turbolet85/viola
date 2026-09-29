//! `viola run`'s version gate (architecture §Established Decisions → [CLI Version Compatibility],
//! [Session Liveness]; obs-plan §4 `viola run` start sequence, Edge flows → Capability ledger):
//! the child's `--version` answer and the stamps `viola verify` wrote decide `cli_version` /
//! `cli_verified` in the first snapshot and on the `claude-child` start line; every other outcome
//! degrades silently. Stamps come only from `viola verify` against the fake agent.

#[allow(dead_code)]
mod support;

use std::ffi::OsString;
use std::io::{Read, Write as _};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
use std::time::Instant;

use rstest::rstest;
use serde_json::Value;
use support::fake::{FAKE, RECORDED_CLI_VERSION};
use support::home::{StampedHome, TestHome, VIOLA, Wrapper, home, snapshot_data};
use support::verify::{verify, write_spine_set};
use support::watch::{WITHIN, Watch};

/// Stamps `home` for `version` through `viola verify` against the fake agent; `skip` leaves one
/// spine fixture out, so that row fails.
fn stamp(home: &TestHome, version: &str, skip: Option<&str>) {
    let fixtures = home.scratch().join(format!("fixtures-{version}"));
    write_spine_set(&fixtures, version, skip);
    let ran = verify(home.path(), &fixtures, version, &[], &[], &[]);
    let expected = if skip.is_some() { Some(1) } else { Some(0) };
    assert_eq!(ran.code, expected, "verify: {}", ran.stdout_text());
}

fn role_lines(home: &Path) -> Vec<Value> {
    support::ndjson::read_lines(&home.join("diagnostics").join("run-builder.ndjson"))
}

/// `(event, subject)` of every role line.
fn shape(lines: &[Value]) -> Vec<(String, String)> {
    lines
        .iter()
        .map(|l| {
            (
                l["event"].as_str().unwrap_or("").to_owned(),
                l["subject"].as_str().unwrap_or("").to_owned(),
            )
        })
        .collect()
}

fn child_start(lines: &[Value]) -> &Value {
    lines
        .iter()
        .find(|l| l["event"] == "process-start" && l["subject"] == "claude-child")
        .expect("claude-child start")
}

/// Boots `viola run builder` on the fake agent under an outer PTY and hands back the first
/// snapshot's data and the role lines once it stopped.
fn run_once(home: TestHome) -> (Value, Vec<Value>) {
    let wrapper = Wrapper::boot(StampedHome::unstamped(home), "builder", None, &[]);
    let snapshot = snapshot_data(&wrapper.instance_dir()).expect("snapshot");
    let (stopped, stamped) = wrapper.stop_keep();
    assert_eq!(stopped.code(), Some(0));
    let lines = role_lines(stamped.home.path());
    (snapshot, lines)
}

#[rstest]
fn run_with_a_verified_version_records_cli_verified(#[from(home)] home: TestHome) {
    stamp(&home, RECORDED_CLI_VERSION, None);
    let (snapshot, lines) = run_once(home);
    assert_eq!(snapshot["cli_version"], RECORDED_CLI_VERSION);
    assert_eq!(snapshot["cli_verified"], true);
    let shape = shape(&lines);
    let at = |event: &str, subject: &str| {
        shape
            .iter()
            .position(|(e, s)| e == event && s == subject)
            .unwrap_or_else(|| panic!("no {event} {subject} in {shape:?}"))
    };
    let probe_start = at("process-start", "version-probe");
    let probe_exit = at("process-exit", "version-probe");
    let child = at("process-start", "claude-child");
    assert!(at("process-start", "self") < probe_start);
    assert!(probe_start < probe_exit && probe_exit < child);
    assert_eq!(lines[probe_exit]["child_exit_status"], 0);
    assert!(lines[probe_exit]["duration_ms"].is_u64());
    let start = child_start(&lines);
    assert_eq!(start["cli_version"], RECORDED_CLI_VERSION);
    assert_eq!(start["cli_verified"], true);
    assert!(!lines.iter().any(|l| l["event"] == "parse-rejected"));
}

#[rstest]
#[case::failing_row(Some((RECORDED_CLI_VERSION, Some("Stop"))))]
#[case::other_version(Some(("3.0.0", None)))]
#[case::unstamped(None)]
fn run_without_a_verified_stamp_degrades(
    #[from(home)] home: TestHome,
    #[case] stamped: Option<(&str, Option<&str>)>,
) {
    if let Some((version, skip)) = stamped {
        stamp(&home, version, skip);
    }
    let (snapshot, lines) = run_once(home);
    assert_eq!(snapshot["cli_version"], RECORDED_CLI_VERSION);
    assert_eq!(snapshot["cli_verified"], false);
    let start = child_start(&lines);
    assert_eq!(start["cli_version"], RECORDED_CLI_VERSION);
    assert_eq!(start["cli_verified"], false);
    assert!(!lines.iter().any(|l| l["event"] == "parse-rejected"));
}

/// A stamps path the read cannot use degrades with exactly one `parse-rejected` code, never a
/// path. The stand-in is a directory where the file should be: no stamps byte is hand-written.
#[rstest]
fn run_with_unreadable_stamps_logs_one_rejection(#[from(home)] home: TestHome) {
    std::fs::create_dir_all(home.path().join("ledger").join("stamps.json")).expect("dir");
    let (snapshot, lines) = run_once(home);
    assert_eq!(snapshot["cli_verified"], false);
    let rejected: Vec<&Value> = lines
        .iter()
        .filter(|l| l["event"] == "parse-rejected")
        .collect();
    assert_eq!(rejected.len(), 1);
    let line = rejected[0];
    assert_eq!(line["parser"], "ledger-stamps");
    assert_eq!(line["detail"], "unreadable");
    assert_eq!(line["count"], 1);
    assert_eq!(line["level"], "WARN");
    let mut keys: Vec<&str> = line
        .as_object()
        .expect("object")
        .keys()
        .map(String::as_str)
        .collect();
    keys.sort_unstable();
    assert_eq!(
        keys,
        [
            "count",
            "detail",
            "event",
            "instance",
            "level",
            "message",
            "parser",
            "process",
            "target",
            "timestamp"
        ]
    );
}

fn drain(pipe: Option<impl Read + Send + 'static>) -> mpsc::Receiver<Vec<u8>> {
    let (tx, rx) = mpsc::channel();
    if let Some(mut pipe) = pipe {
        std::thread::spawn(move || {
            let mut bytes = Vec::new();
            let _ = pipe.read_to_end(&mut bytes);
            let _ = tx.send(bytes);
        });
    }
    rx
}

/// Waits below the kill line for `done`, failing the test at the deadline.
fn wait_until(what: &str, mut done: impl FnMut() -> bool) {
    let watch = Watch::start(what);
    let deadline = Instant::now() + WITHIN;
    while !done() {
        watch.note("waiting");
        watch.deadline_check(deadline, &format!("{what} timed out"));
        std::thread::yield_now();
    }
}

fn exited(child: &mut Child) -> bool {
    child.try_wait().expect("try_wait").is_some()
}

/// `viola run builder -- <program> <args>` on pipes; stdin gets Ctrl-C once `ready` holds.
fn run_piped(
    home: &Path,
    program: &str,
    args: &[OsString],
    ready: impl Fn() -> bool,
) -> (Vec<u8>, Vec<u8>) {
    let mut child = Command::new(VIOLA)
        .arg("--home")
        .arg(home)
        .args(["run", "builder", "--", program])
        .args(args)
        .env_remove("VIOLA_NAME")
        .env_remove("VIOLA_DIR")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("viola run");
    let stdout = drain(child.stdout.take());
    let stderr = drain(child.stderr.take());
    wait_until("ready", || ready() || exited(&mut child));
    if let Some(stdin) = child.stdin.as_mut() {
        let _ = stdin.write_all(b"\x03");
    }
    wait_until("exit", || exited(&mut child));
    (
        stdout.recv().unwrap_or_default(),
        stderr.recv().unwrap_or_default(),
    )
}

/// viola's own human and diagnostic literals (the `run_cli` set): none may reach its stdout.
const VIOLA_LITERALS: [&str; 5] = ["unable:", "hint:", "error:", "\"event\":", "\"process\":"];

/// `run` itself writes no byte on either stream, stamped or not. stdout carries only the child's
/// screen as its PTY renders it: nothing on Unix for the silent fake agent, ConPTY's own sequences
/// on Windows, and never one of viola's literals.
#[rstest]
#[case::stamped(true)]
#[case::unstamped(false)]
fn run_writes_nothing_of_its_own_on_any_gate_outcome(
    #[from(home)] home: TestHome,
    #[case] stamped: bool,
) {
    if stamped {
        stamp(&home, RECORDED_CLI_VERSION, None);
    }
    let receipt = home.scratch().join("receipt.ndjson");
    let args = [
        OsString::from("--receipt"),
        receipt.clone().into_os_string(),
    ];
    let (stdout, stderr) = run_piped(home.path(), FAKE, &args, || {
        std::fs::read_to_string(&receipt)
            .unwrap_or_default()
            .contains("\"kind\":\"start\"")
    });
    let text = String::from_utf8_lossy(&stdout);
    for literal in VIOLA_LITERALS {
        assert!(!text.contains(literal), "{literal} on stdout");
    }
    if cfg!(unix) {
        assert!(stdout.is_empty(), "run wrote {} stdout bytes", stdout.len());
    }
    assert!(stderr.is_empty(), "run wrote {} stderr bytes", stderr.len());
    let snapshot = snapshot_data(&home.path().join("instances").join("builder")).expect("snapshot");
    assert_eq!(snapshot["cli_verified"], stamped);
}

/// `viola --version` prints `viola 0.1.0`: no CLI version, so none is recorded.
#[rstest]
fn run_with_a_child_that_is_not_the_cli_records_no_version(#[from(home)] home: TestHome) {
    stamp(&home, RECORDED_CLI_VERSION, None);
    let (_, _) = run_piped(home.path(), VIOLA, &[], || false);
    let snapshot = snapshot_data(&home.path().join("instances").join("builder")).expect("snapshot");
    assert!(snapshot.get("cli_version").is_none(), "{snapshot}");
    assert_eq!(snapshot["cli_verified"], false);
    let lines = role_lines(home.path());
    let start = child_start(&lines);
    assert!(start.get("cli_version").is_none());
    assert_eq!(start["cli_verified"], false);
    let probe_exit = lines
        .iter()
        .find(|l| l["event"] == "process-exit" && l["subject"] == "version-probe")
        .expect("version-probe exit");
    assert_eq!(probe_exit["child_exit_status"], 0);
}
