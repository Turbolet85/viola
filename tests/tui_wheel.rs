//! The wheel as a human's terminal moves it (test-plan §6 Path 5; a11y-plan §3 Keyboard test
//! harness case (3)): `viola run` under an outer PTY, the fake agent as its child. A human editing
//! key takes the wheel and still reaches the child; a harness-injected turn, focus reports, a mouse
//! report and a host resize never take it; `viola release` hands it back. `^Z` is a key like any
//! other and later keys still arrive (CARRY §8). On `windows-2025` the focus and `^Z` cases are the
//! measurements of the inbox ConPTY outer terminal and of viola's own console reader. Waits are on
//! receipts, events and exits only.

#[allow(dead_code)]
mod support;

use std::io::Write as _;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Instant;

use rstest::rstest;
use serde_json::{Value, json};
use support::fake::{self, of_kind};
use support::home::{StampedHome, VIOLA, Wrapper, snapshot_data, stamped_home, workspace_path};
use support::watch::{WITHIN, Watch};
use viola_pty::Size;

const CANARY: &str = "canary-chain-value-5c1e";

/// A stamped wrapper over the committed hook fixtures, its SessionStart record landed.
fn boot(stamped: StampedHome, extra: &[&str]) -> Wrapper {
    let fixtures = workspace_path("fixtures/claude");
    let mut args = vec!["--fixtures", fixtures.to_str().expect("utf-8 path")];
    args.extend_from_slice(extra);
    let wrapper = Wrapper::boot(stamped, "builder", None, &args);
    wait_events(&wrapper.instance_dir(), "the session-start record", |l| {
        l.iter().any(|e| e["kind"] == "session-start")
    });
    wrapper
}

fn events(instance_dir: &Path) -> Vec<Value> {
    support::ndjson::read_lines(&instance_dir.join("events.ndjson"))
}

fn wait_events(instance_dir: &Path, what: &str, pred: impl Fn(&[Value]) -> bool) -> Vec<Value> {
    let watch = Watch::start("events");
    let deadline = Instant::now() + WITHIN;
    loop {
        let lines = events(instance_dir);
        if pred(&lines) {
            return lines;
        }
        watch.note(&format!("events {}", lines.len()));
        watch.deadline_check(deadline, &format!("timed out waiting for {what}"));
        std::thread::yield_now();
    }
}

fn wheel_records(instance_dir: &Path) -> Vec<Value> {
    events(instance_dir)
        .into_iter()
        .filter(|l| l["kind"] == "wheel")
        .map(|l| l["data"].clone())
        .collect()
}

fn start_record() -> Value {
    json!({"holder": "driver", "cause": "start"})
}

/// The fake agent's `key` receipts, as hex.
fn keys(lines: &[Value]) -> Vec<String> {
    of_kind(lines, "key")
        .iter()
        .map(|k| k["hex"].as_str().expect("hex").to_owned())
        .collect()
}

/// Waits for `count` key receipts; the watch report names each key that arrived (the test's own
/// synthetic bytes) and how many `wheel` records the log holds, so a red names what the terminal
/// delivered.
fn wait_keys(receipt: &Path, dir: &Path, count: usize, what: &str) -> Vec<Value> {
    let watch = Watch::start("keys");
    let deadline = Instant::now() + WITHIN;
    loop {
        let lines = fake::receipt(receipt);
        let typed = keys(&lines);
        if typed.len() >= count {
            return lines;
        }
        watch.note(&format!(
            "keys [{}] wheel records {} receipt lines {}",
            typed.join(" "),
            wheel_records(dir).len(),
            lines.len()
        ));
        watch.deadline_check(deadline, &format!("timed out waiting for {what}"));
        std::thread::yield_now();
    }
}

fn hex_of(bytes: &[u8]) -> Vec<String> {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

struct Ran {
    code: Option<i32>,
    stdout: String,
    stderr: String,
}

/// `viola --home <home> <args…>`, `stdin` written and closed, `from` as `VIOLA_NAME`; waited.
fn viola(home: &Path, args: &[&str], stdin: &str, from: Option<&str>) -> Ran {
    let mut command = Command::new(VIOLA);
    command
        .arg("--home")
        .arg(home)
        .args(args)
        .env_remove("VIOLA_NAME")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(from) = from {
        command.env("VIOLA_NAME", from);
    }
    let mut child = command.spawn().expect("viola");
    let mut pipe = child.stdin.take().expect("stdin");
    match pipe.write_all(stdin.as_bytes()) {
        Err(e) if e.kind() != std::io::ErrorKind::BrokenPipe => panic!("stdin: {e}"),
        _ => {}
    }
    drop(pipe);
    let watch = Watch::start("viola");
    let deadline = Instant::now() + WITHIN;
    while child.try_wait().expect("try_wait").is_none() {
        watch.note("running");
        watch.deadline_check(deadline, "viola never exited");
        std::thread::yield_now();
    }
    let out = child.wait_with_output().expect("output");
    Ran {
        code: out.status.code(),
        stdout: String::from_utf8(out.stdout).expect("utf-8 stdout"),
        stderr: String::from_utf8(out.stderr).expect("utf-8 stderr"),
    }
}

/// A driver's `send --json`; exit 0 also proves the wheel was the driver's when it arrived.
fn send_json(home: &Path) -> Ran {
    viola(
        home,
        &["send", "builder", "--json"],
        &format!("{CANARY} from the driver"),
        None,
    )
}

/// test-plan §6 Path 5 steps 1–2 and 4–7: a typed key takes the wheel and still reaches the
/// child, the human's prompt is filed `human`, `release` returns the wheel, `pause` takes it again,
/// and a driver's `release` is refused.
#[rstest]
fn path5_human_takes_the_wheel_and_release_returns_it(stamped_home: StampedHome) {
    let mut wrapper = boot(stamped_home, &[]);
    let home = wrapper.home().to_path_buf();
    let dir = wrapper.instance_dir();
    let receipt = wrapper.receipt();

    wrapper.send(b"h");
    let taken = json!({"holder": "human", "cause": "human-input"});
    wait_events(&dir, "the human-input wheel record", |l| {
        l.iter().any(|e| e["kind"] == "wheel" && e["data"] == taken)
    });
    assert_eq!(wheel_records(&dir), [start_record(), taken.clone()]);
    let refused = send_json(&home);
    assert_eq!(refused.code, Some(10));
    assert_eq!(
        refused.stdout,
        "{\"v\":1,\"refusal\":\"human-typing\",\"detail\":null}\n"
    );

    wrapper.send(b"ello\r");
    let lines = fake::wait_for(&receipt, "the human's prompt", |l| {
        !of_kind(l, "prompt").is_empty()
    });
    assert_eq!(of_kind(&lines, "prompt")[0]["text"], "hello");
    let lines = wait_events(&dir, "the human's prompt-submitted", |l| {
        l.iter().any(|e| e["kind"] == "prompt-submitted")
    });
    let prompt = lines
        .iter()
        .find(|e| e["kind"] == "prompt-submitted")
        .expect("prompt-submitted");
    assert_eq!(prompt["data"]["origin"], "human");
    assert_eq!(prompt["data"]["text"], "hello");

    let released = viola(&home, &["release", "builder", "--json"], "", None);
    assert_eq!(released.code, Some(0), "stderr: {}", released.stderr);
    assert_eq!(
        released.stdout,
        "{\"v\":1,\"ok\":{\"wheel\":\"driver\",\"budget_paused\":false}}\n"
    );
    let sent = send_json(&home);
    assert_eq!(sent.code, Some(0), "stderr: {}", sent.stderr);

    let paused = viola(&home, &["pause", "builder"], "", None);
    assert_eq!(paused.code, Some(0));
    let human = viola(&home, &["send", "builder"], CANARY, None);
    assert_eq!(human.code, Some(10));
    let last = human.stderr.lines().last().expect("a stderr line");
    assert!(last.starts_with("hint: "), "{}", human.stderr);
    assert!(!last.contains("release"), "{last}");

    let driver = viola(
        &home,
        &["release", "builder", "--json"],
        "",
        Some("overseer"),
    );
    assert_eq!(driver.code, Some(20));
    let doc: Value = serde_json::from_str(&driver.stdout).expect("one JSON document");
    assert_eq!(doc["detail"]["code"], -32602);
    assert_eq!(doc["detail"]["data"]["reason"], "release-from-driver");

    let typed = keys(&fake::receipt(&receipt));
    assert_eq!(
        typed[..5],
        hex_of(b"hello"),
        "the typed keys reach the child"
    );
    assert_eq!(
        wheel_records(&dir),
        [
            start_record(),
            taken,
            json!({"holder": "driver", "cause": "release"}),
            json!({"holder": "human", "cause": "manual-pause"}),
        ]
    );
    wrapper.stop();
}

/// test-plan §6 Path 5 step 8: a harness-injected turn is filed `harness` and never takes the wheel.
#[rstest]
fn path5_harness_turns_never_take_the_wheel(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home, &["--inject-harness-turn"]);
    let home = wrapper.home().to_path_buf();
    let dir = wrapper.instance_dir();
    wrapper.release();
    let lines = wait_events(&dir, "the harness prompt-submitted", |l| {
        l.iter().any(|e| e["kind"] == "prompt-submitted")
    });
    let prompt = lines
        .iter()
        .find(|e| e["kind"] == "prompt-submitted")
        .expect("prompt-submitted");
    assert_eq!(prompt["data"]["origin"], "harness");
    let sent = send_json(&home);
    assert_eq!(sent.code, Some(0), "stderr: {}", sent.stderr);
    assert_eq!(wheel_records(&dir), [start_record()]);
    wrapper.stop();
}

/// a11y-plan §4 P4 tui case (3): focus reports, an SGR mouse report and a host resize reach the
/// child unchanged and never take the wheel. The size receipt is awaited before any key (the H2
/// rule). On `windows-2025` this reading is CARRY §9's measurement of the inbox ConPTY outer
/// terminal.
#[rstest]
fn tui_focus_mouse_and_resize_never_take_the_wheel(stamped_home: StampedHome) {
    let mut wrapper = boot(stamped_home, &[]);
    let home = wrapper.home().to_path_buf();
    let dir = wrapper.instance_dir();
    let receipt = wrapper.receipt();
    let reports: &[u8] = b"\x1b[I\x1b[O\x1b[<0;10;5M";
    wrapper.send(b"\x1b[I");
    wrapper.send(b"\x1b[O");
    wrapper.send(b"\x1b[<0;10;5M");
    wrapper.resize(Size {
        cols: 100,
        rows: 30,
    });
    let target = json!({"v": 1, "kind": "size", "cols": 100, "rows": 30});
    fake::wait_for(&receipt, "the resized size receipt", |l| {
        of_kind(l, "size").contains(&&target)
    });
    let lines = wait_keys(&receipt, &dir, reports.len(), "the focus and mouse bytes");
    assert_eq!(keys(&lines), hex_of(reports), "reached the child unchanged");
    // The reports sit in the child's prompt line, so a `send` could never be read back: an
    // `answer` to no pending dialog asks the wheel instead, typing nothing (`human-typing` is
    // checked ahead of `unknown-dialog`).
    let probe = viola(
        &home,
        &["answer", "builder", "999999", "--json"],
        "{\"behavior\": \"allow\"}",
        None,
    );
    assert_eq!(probe.code, Some(13), "stdout: {}", probe.stdout);
    assert_eq!(
        probe.stdout,
        "{\"v\":1,\"refusal\":\"not-delivered\",\"detail\":\"unknown-dialog\"}\n"
    );
    assert_eq!(wheel_records(&dir), [start_record()]);
    assert_eq!(snapshot_data(&dir).expect("snapshot")["wheel"], "driver");
    wrapper.stop();
}

/// CARRY §8: `^Z` reaches the child as a key, and the key after it still does (on Windows a read
/// of `^Z` alone must not end the human's copy). `^Z` is typing, so the wheel is the human's.
#[rstest]
fn tui_ctrl_z_reaches_the_child_and_later_keys_still_do(stamped_home: StampedHome) {
    let mut wrapper = boot(stamped_home, &[]);
    let dir = wrapper.instance_dir();
    let receipt = wrapper.receipt();
    wrapper.send(b"\x1a");
    wait_keys(&receipt, &dir, 1, "the ^Z key");
    wrapper.send(b"k");
    let lines = wait_keys(&receipt, &dir, 2, "the key after ^Z");
    assert_eq!(keys(&lines), ["1a", "6b"]);
    let taken = json!({"holder": "human", "cause": "human-input"});
    wait_events(&dir, "the human-input wheel record", |l| {
        l.iter().any(|e| e["kind"] == "wheel" && e["data"] == taken)
    });
    wrapper.stop();
}
