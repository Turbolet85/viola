//! `viola run` as a human's terminal hosts it (a11y-plan §3 Keyboard test harness, test-plan §6 tui
//! row): keys reach the child as typed, Ctrl-C ends it, a host resize is forwarded, the child's cwd
//! is the wrapper's, and viola adds no bytes of its own. On Windows ConPTY re-renders the stream,
//! so the oracle there is the absence of viola's own literals; on Unix the stream equals an
//! unwrapped run's.

#[allow(dead_code)]
mod support;

use std::ffi::OsString;
use std::path::Path;
use std::time::Instant;

use rstest::rstest;
use serde_json::{Value, json};
use support::fake::{self, FAKE, of_kind};
use support::home::{TestHome, VIOLA, home};
use support::outer_pty::{EXIT_WITHIN, OuterPty};
use viola_pty::Size;

/// viola's own human and diagnostic literals: none may reach the terminal.
const VIOLA_LITERALS: [&str; 5] = ["unable:", "hint:", "error:", "\"event\":", "\"process\":"];

fn holds(haystack: &[u8], needle: &str) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle.as_bytes())
}

fn run_args(home: &Path, receipt: Option<&Path>, extra: &[&str]) -> Vec<OsString> {
    let mut args: Vec<OsString> = vec!["--home".into(), home.into()];
    args.extend(["run", "builder", "--", FAKE].map(OsString::from));
    if let Some(receipt) = receipt {
        args.push("--receipt".into());
        args.push(receipt.into());
    }
    args.extend(extra.iter().map(OsString::from));
    args
}

fn role_lines(home: &Path) -> Vec<Value> {
    std::fs::read_to_string(home.join("diagnostics").join("run-builder.ndjson"))
        .expect("role file")
        .lines()
        .map(|l| serde_json::from_str(l).expect("line"))
        .collect()
}

fn keys(lines: &[Value]) -> Vec<&str> {
    of_kind(lines, "key")
        .iter()
        .map(|k| k["hex"].as_str().expect("hex"))
        .collect()
}

fn assert_no_viola_bytes(stream: &[u8]) {
    for literal in VIOLA_LITERALS {
        assert!(!holds(stream, literal), "viola wrote {literal:?}");
    }
}

#[rstest]
fn tui_keys_reach_the_child_as_typed_and_ctrl_c_ends_it(#[from(home)] tmp: TestHome) {
    let receipt = tmp.scratch().join("keys.receipt.ndjson");
    let mut pty = OuterPty::spawn(
        Path::new(VIOLA),
        &run_args(tmp.path(), Some(&receipt), &[]),
        &[],
    );
    fake::wait_for(&receipt, "start", |l| !of_kind(l, "start").is_empty());
    pty.write(b"ab\r");
    let lines = fake::wait_for(&receipt, "three keys", |l| keys(l).len() >= 3);
    assert_eq!(keys(&lines), ["61", "62", "0d"]);
    let cwd = std::env::current_dir().expect("cwd");
    assert_eq!(
        of_kind(&lines, "cwd")[0]["cwd"],
        json!(cwd.to_string_lossy())
    );
    pty.write(b"\x03");
    assert_eq!(pty.wait_exit(EXIT_WITHIN), 0);
    let lines = fake::receipt(&receipt);
    assert_eq!(keys(&lines), ["61", "62", "0d"], "a byte was added");
    let role = role_lines(tmp.path());
    let exit = role
        .iter()
        .find(|l| l["event"] == "process-exit" && l["subject"] == "claude-child")
        .expect("child exit");
    assert_eq!(exit["exit_source"], "handle-wait");
    assert_eq!(exit["child_exit_status"], 0);
    assert_no_viola_bytes(&pty.finish());
}

#[rstest]
fn tui_host_resize_reaches_the_child(#[from(home)] tmp: TestHome) {
    let receipt = tmp.scratch().join("resize.receipt.ndjson");
    let mut pty = OuterPty::spawn_sized(
        Path::new(VIOLA),
        &run_args(tmp.path(), Some(&receipt), &[]),
        &[],
        Size { cols: 80, rows: 24 },
    );
    fake::wait_for(&receipt, "start", |l| !of_kind(l, "start").is_empty());
    pty.resize(Size {
        cols: 100,
        rows: 30,
    });
    let target = json!({"v": 1, "kind": "size", "cols": 100, "rows": 30});
    let deadline = Instant::now() + EXIT_WITHIN;
    // Each key makes the child re-read its size; the wrapper forwards the host size on its own poll.
    while !of_kind(&fake::receipt(&receipt), "size").contains(&&target) {
        assert!(
            Instant::now() < deadline,
            "the resize never reached the child"
        );
        let seen = keys(&fake::receipt(&receipt)).len();
        pty.write(b"z");
        fake::wait_for(&receipt, "the key", |l| keys(l).len() > seen);
    }
    pty.write(b"\x03");
    assert_eq!(pty.wait_exit(EXIT_WITHIN), 0);
}

#[rstest]
fn tui_child_output_passes_through_without_viola_bytes(#[from(home)] wrapped: TestHome) {
    let mut pty = OuterPty::spawn(
        Path::new(VIOLA),
        &run_args(wrapped.path(), None, &["--version"]),
        &[],
    );
    assert_eq!(pty.wait_exit(EXIT_WITHIN), 0);
    let stream = pty.finish();
    assert!(
        holds(&stream, "2.1.0 (Claude Code)"),
        "the child's output never reached the terminal"
    );
    assert_no_viola_bytes(&stream);
    #[cfg(unix)]
    {
        let mut direct = OuterPty::spawn(Path::new(FAKE), &["--version".into()], &[]);
        assert_eq!(direct.wait_exit(EXIT_WITHIN), 0);
        assert_eq!(
            stream,
            direct.finish(),
            "wrapped and unwrapped streams differ"
        );
    }
}
