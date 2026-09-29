//! `viola run` as a human's terminal hosts it (a11y-plan §3 Keyboard test harness, test-plan §6 tui
//! row): keys reach the child as typed, Ctrl-C ends it, a host resize is forwarded, the child's cwd
//! is the wrapper's, and viola adds no bytes of its own. On Windows ConPTY re-renders the stream,
//! so the oracle there is the absence of viola's own literals; on Unix the stream equals an
//! unwrapped run's.

#[allow(dead_code)]
mod support;

use std::ffi::OsString;
use std::fs::OpenOptions;
use std::path::Path;
use std::time::{Duration, Instant};

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
    support::ndjson::read_lines(&home.join("diagnostics").join("run-builder.ndjson"))
}

fn unix_us() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_micros())
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

/// Below nextest's `mutants` kill (5 s × 2): a test killed before its own assertion loses its evidence.
const RESIZE_WITHIN: Duration = Duration::from_secs(8);

/// Resizes the host terminal once the child has started and waits for the child to see the new
/// size. Every observation is appended as it happens to `viola-resize-<pid>.ndjson` in the temp
/// dir, outside the test home and any scratch copy, so a run killed mid-wait still leaves it;
/// the file is removed on success. Returns how long the new size took to reach the child.
fn resize_reaches_the_child(tmp: &TestHome, env: &[(&str, &str)]) -> Duration {
    let receipt = tmp.scratch().join("resize.receipt.ndjson");
    let trail = std::env::temp_dir().join(format!("viola-resize-{}.ndjson", std::process::id()));
    let note = |line: Value| {
        use std::io::Write as _;
        if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(&trail) {
            let _ = writeln!(f, "{line}");
        }
    };
    let mut pty = OuterPty::spawn_sized(
        Path::new(VIOLA),
        &run_args(tmp.path(), Some(&receipt), &[]),
        env,
        Size { cols: 80, rows: 24 },
    );
    fake::wait_for(&receipt, "start", |l| !of_kind(l, "start").is_empty());
    note(json!({"t_us": unix_us(), "what": "resize-sent"}));
    let sent = Instant::now();
    pty.resize(Size {
        cols: 100,
        rows: 30,
    });
    let target = json!({"v": 1, "kind": "size", "cols": 100, "rows": 30});
    let deadline = sent + RESIZE_WITHIN;
    let mut observed: Vec<Value> = Vec::new();
    // Each key makes the child re-read its size; the wrapper forwards the host size on its own poll.
    while !of_kind(&fake::receipt(&receipt), "size").contains(&&target) {
        for size in of_kind(&fake::receipt(&receipt), "size") {
            if !observed.contains(size) {
                observed.push(size.clone());
                note(json!({"t_us": unix_us(), "what": "size-observed", "receipt": size}));
            }
        }
        if Instant::now() >= deadline {
            let trail_text = std::fs::read_to_string(&trail).unwrap_or_default();
            panic!(
                "the resize never reached the child\ntrail ({}):\n{trail_text}",
                trail.display()
            );
        }
        let seen = keys(&fake::receipt(&receipt)).len();
        pty.write(b"z");
        fake::wait_for(&receipt, "the key", |l| keys(l).len() > seen);
    }
    let took = sent.elapsed();
    pty.write(b"\x03");
    assert_eq!(pty.wait_exit(EXIT_WITHIN), 0);
    let _ = std::fs::remove_file(&trail);
    took
}

#[rstest]
fn tui_host_resize_reaches_the_child(#[from(home)] tmp: TestHome) {
    resize_reaches_the_child(&tmp, &[]);
}

/// The window between the spawn sizing and the pump's first look, forced open: `viola run` (built
/// with `fake-agent`) holds the pump back 1 s, and the resize is sent inside that hold. The new size
/// arrives only after the hold and the pump's first period, which proves the resize landed inside it.
#[rstest]
fn tui_host_resize_in_the_pump_start_window_reaches_the_child(#[from(home)] tmp: TestHome) {
    let took = resize_reaches_the_child(&tmp, &[("FAKE_AGENT_PUMP_DELAY_MS", "1000")]);
    assert!(
        took >= Duration::from_millis(900) && took < Duration::from_secs(3),
        "the resize took {took:?}: it did not land inside the 1 s hold"
    );
}

/// Types one pasted prompt once the child is raw, waits for its prompt receipt and `hooks` hook
/// receipts, then ends the child; returns the outer-PTY stream.
fn prompt_and_finish(mut pty: OuterPty, receipt: &Path, hooks: usize) -> Vec<u8> {
    fake::wait_for(receipt, "start", |l| !of_kind(l, "start").is_empty());
    pty.write(b"\x1b[200~hello\x1b[201~\r");
    fake::wait_for(receipt, "the prompt and its hooks", |l| {
        !of_kind(l, "prompt").is_empty() && of_kind(l, "hook").len() >= hooks
    });
    pty.write(b"\x03");
    assert_eq!(pty.wait_exit(EXIT_WITHIN), 0);
    pty.finish()
}

/// a11y-plan §4 P4 tui case (1) with the plugin's hooks registered and firing: SessionStart at the
/// child's start and UserPromptSubmit on the prompt run `viola hook`, and the outer stream still
/// carries no viola-originated byte.
#[rstest]
fn tui_hooks_firing_add_no_viola_bytes(#[from(home)] wrapped: TestHome) {
    let fixtures = wrapped.scratch().join("fixtures");
    for event in ["SessionStart", "UserPromptSubmit"] {
        fake::write_fixture(
            &fixtures,
            fake::RECORDED_CLI_VERSION,
            event,
            "default",
            &json!({"hook_event_name": event, "session_id": "s-4", "source": "startup",
                    "prompt": "canary-chain-value-5c1e"}),
        );
    }
    let fixtures = fixtures.to_str().expect("utf-8 path").to_owned();
    let receipt = wrapped.scratch().join("hooks.receipt.ndjson");
    let pty = OuterPty::spawn(
        Path::new(VIOLA),
        &run_args(wrapped.path(), Some(&receipt), &["--fixtures", &fixtures]),
        &[],
    );
    let stream = prompt_and_finish(pty, &receipt, 2);
    let lines = fake::receipt(&receipt);
    let fired: Vec<(&Value, &Value, &Value)> = of_kind(&lines, "hook")
        .iter()
        .map(|h| (&h["event"], &h["ran"], &h["exit_code"]))
        .collect();
    assert_eq!(
        fired,
        [
            (&json!("SessionStart"), &json!(true), &json!(0)),
            (&json!("UserPromptSubmit"), &json!(true), &json!(0)),
        ]
    );
    assert_no_viola_bytes(&stream);
    #[cfg(unix)]
    {
        let direct_receipt = wrapped.scratch().join("direct.receipt.ndjson");
        let args: Vec<OsString> = vec![
            "--receipt".into(),
            direct_receipt.clone().into(),
            "--fixtures".into(),
            fixtures.clone().into(),
        ];
        let direct = OuterPty::spawn(Path::new(FAKE), &args, &[]);
        assert_eq!(
            stream,
            prompt_and_finish(direct, &direct_receipt, 0),
            "wrapped and unwrapped streams differ"
        );
    }
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
        holds(&stream, "2.1.283 (Claude Code)"),
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
