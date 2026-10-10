//! The wheel as a human's terminal moves it (test-plan §6 Path 5; a11y-plan §3 Keyboard test
//! harness case (3)): `viola run` under an outer PTY, the fake agent as its child. A human editing
//! key takes the wheel and still reaches the child; a harness-injected turn, focus reports, a mouse
//! report and a host resize never take it; `viola release` hands it back. `^Z` is a key like any
//! other and later keys still arrive (CARRY §8). On `windows-2025` the focus and `^Z` cases are the
//! measurements of the inbox ConPTY outer terminal and of viola's own console reader. Waits are on
//! receipts, events and exits only.

#[allow(dead_code)]
mod support;

use std::path::Path;
use std::time::Instant;

use rstest::rstest;
use serde_json::{Value, json};
use support::cli::{Ran, viola};
use support::events::{events, wait_events};
use support::fake::{self, of_kind};
use support::home::{StampedHome, Wrapper, snapshot_data, stamped_home};
use support::watch::{WITHIN, Watch};
use viola_pty::Size;

const CANARY: &str = "canary-chain-value-5c1e";
/// A gated `PostToolUse`, then a gated `Stop`.
const PATH3: &str = "fixtures/fake-scripts/path3.json";

/// A stamped wrapper over the committed hook fixtures, its SessionStart record landed.
fn boot(stamped: StampedHome, script: Option<&str>, extra: &[&str]) -> Wrapper {
    support::events::boot(stamped, script, extra)
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

/// A driver's `send --json`; exit 0 also proves the wheel was the driver's when it arrived.
fn send_json(home: &Path) -> Ran {
    viola(
        home,
        &["send", "builder", "--json"],
        Some(&format!("{CANARY} from the driver")),
        None,
    )
}

/// test-plan §6 Path 5 steps 1–2 and 4–7: a typed key takes the wheel and still reaches the
/// child, the human's prompt is filed `human`, `release` returns the wheel, `pause` takes it again,
/// and a driver's `release` is refused.
#[rstest]
fn path5_human_takes_the_wheel_and_release_returns_it(stamped_home: StampedHome) {
    let mut wrapper = boot(stamped_home, None, &[]);
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

    let released = viola(&home, &["release", "builder", "--json"], Some(""), None);
    assert_eq!(released.code, Some(0), "stderr: {}", released.stderr);
    assert_eq!(
        released.stdout,
        "{\"v\":1,\"ok\":{\"wheel\":\"driver\",\"budget_paused\":false}}\n"
    );
    let sent = send_json(&home);
    assert_eq!(sent.code, Some(0), "stderr: {}", sent.stderr);

    let paused = viola(&home, &["pause", "builder"], Some(""), None);
    assert_eq!(paused.code, Some(0));
    let human = viola(&home, &["send", "builder"], Some(CANARY), None);
    assert_eq!(human.code, Some(10));
    let last = human.stderr.lines().last().expect("a stderr line");
    assert!(last.starts_with("hint: "), "{}", human.stderr);
    assert!(!last.contains("release"), "{last}");

    let driver = viola(
        &home,
        &["release", "builder", "--json"],
        Some(""),
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
/// While it runs, a driver's `send` is refused `turn-running` and nothing is typed; once its scripted
/// Stop ends it, the same `send` is accepted.
#[rstest]
fn path5_harness_turns_never_take_the_wheel(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home, Some(PATH3), &["--inject-harness-turn"]);
    let home = wrapper.home().to_path_buf();
    let dir = wrapper.instance_dir();
    let receipt = wrapper.receipt();
    wrapper.release();
    let lines = wait_events(&dir, "the harness prompt-submitted", |l| {
        l.iter().any(|e| e["kind"] == "prompt-submitted")
    });
    let prompt = lines
        .iter()
        .find(|e| e["kind"] == "prompt-submitted")
        .expect("prompt-submitted");
    assert_eq!(prompt["data"]["origin"], "harness");

    let refused = send_json(&home);
    assert_eq!(refused.code, Some(13), "stderr: {}", refused.stderr);
    assert_eq!(
        refused.stdout,
        "{\"v\":1,\"refusal\":\"not-delivered\",\"detail\":\"turn-running\"}\n"
    );
    // The harness prompt's receipt lands after its hook returns, which can be after its line.
    let typed = fake::wait_for(&receipt, "the harness prompt's receipt", |l| {
        !of_kind(l, "prompt").is_empty()
    });
    assert_eq!(of_kind(&typed, "prompt").len(), 1, "nothing typed");
    let after = events(&dir).split_off(lines.len());
    assert_eq!(after.len(), 1, "{after:?}");
    assert_eq!(after[0]["kind"], "send-refused");
    assert_eq!(
        after[0]["data"],
        json!({"refusal": "not-delivered", "detail": "turn-running"})
    );

    wrapper.release();
    wrapper.release();
    wait_events(&dir, "the scripted Stop's turn-ended", |l| {
        l.iter().any(|e| e["kind"] == "turn-ended")
    });
    let sent = send_json(&home);
    assert_eq!(sent.code, Some(0), "stderr: {}", sent.stderr);
    assert_eq!(wheel_records(&dir), [start_record()]);
    wrapper.stop();
}

/// a11y-plan §4 P4 tui case (3), each step asked on its own. A host resize never takes the wheel
/// (its size receipt is the barrier, and is awaited before any key: the H2 rule). Focus reports,
/// then an SGR mouse report as their read barrier: on Unix both reach the child unchanged and
/// neither takes the wheel. On `windows-2025` the inbox ConPTY outer terminal swallows the focus
/// reports, and under the sideloaded ConPTY's win32-input-mode it hands the wrapper an injected
/// mouse report as typed keys, which take the wheel — the error only ever favours the human (the
/// founder's live ruling, 2026-10-04, on CARRY §9 as measured by CI run ci#37227518624; viola-pty
/// pins the encoding). A mouse report from a real Windows terminal is measured live at `:84`.
#[rstest]
fn tui_focus_mouse_and_resize_never_take_the_wheel(stamped_home: StampedHome) {
    const MOUSE: &[u8] = b"\x1b[<0;10;5M";
    let mut wrapper = boot(stamped_home, None, &[]);
    let home = wrapper.home().to_path_buf();
    let dir = wrapper.instance_dir();
    let receipt = wrapper.receipt();

    wrapper.resize(Size {
        cols: 100,
        rows: 30,
    });
    let target = json!({"v": 1, "kind": "size", "cols": 100, "rows": 30});
    fake::wait_for(&receipt, "the resized size receipt", |l| {
        of_kind(l, "size").contains(&&target)
    });
    assert_driver_holds(&home, &dir, "a host resize");

    wrapper.send(b"\x1b[I");
    wrapper.send(b"\x1b[O");
    wrapper.send(MOUSE);
    let focus: &[u8] = if cfg!(windows) { b"" } else { b"\x1b[I\x1b[O" };
    let expected = [focus, MOUSE].concat();
    let lines = wait_keys(&receipt, &dir, expected.len(), "the focus reports' barrier");
    assert_eq!(keys(&lines), hex_of(&expected), "what reached the child");
    if cfg!(windows) {
        let probe = viola(
            &home,
            &["answer", "builder", "999999", "--json"],
            Some("{\"behavior\": \"allow\"}"),
            None,
        );
        assert_eq!(probe.code, Some(10), "stdout: {}", probe.stdout);
        let taken = json!({"holder": "human", "cause": "human-input"});
        wait_events(&dir, "the injected report's wheel record", |l| {
            l.iter().any(|e| e["kind"] == "wheel" && e["data"] == taken)
        });
        assert_eq!(wheel_records(&dir), [start_record(), taken]);
    } else {
        assert_driver_holds(&home, &dir, "focus and mouse reports");
        assert_eq!(snapshot_data(&dir).expect("snapshot")["wheel"], "driver");
    }
    wrapper.stop();
}

/// The driver still holds the wheel after `step`. The reports sit in the child's prompt line, so a
/// `send` could never be read back: an `answer` to no pending dialog asks the wheel instead, typing
/// nothing (`human-typing` is checked ahead of `unknown-dialog`).
fn assert_driver_holds(home: &Path, dir: &Path, step: &str) {
    let probe = viola(
        home,
        &["answer", "builder", "999999", "--json"],
        Some("{\"behavior\": \"allow\"}"),
        None,
    );
    assert_eq!(
        probe.stdout,
        "{\"v\":1,\"refusal\":\"not-delivered\",\"detail\":\"unknown-dialog\"}\n",
        "{step} took the wheel; wheel records {:?}",
        wheel_records(dir)
    );
    assert_eq!(probe.code, Some(13));
    assert_eq!(wheel_records(dir), [start_record()], "after {step}");
}

/// CARRY §8: `^Z` reaches the child as a key, and the key after it still does (on Windows a read
/// of `^Z` alone must not end the human's copy). `^Z` is typing, so the wheel is the human's.
#[rstest]
fn tui_ctrl_z_reaches_the_child_and_later_keys_still_do(stamped_home: StampedHome) {
    let mut wrapper = boot(stamped_home, None, &[]);
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
