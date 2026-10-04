//! A forced vt100 feed panic, run level (test-plan §6 Chaos; architecture [Screen Model];
//! security-plan §Input Validation, "PTY output bytes (vt100)"): at one column the fake agent's
//! wide character panics vt100 inside the wrapper's feed thread. The panic is contained and
//! witnessed — its `parse-rejected` line and the panic hook's line are both asserted present — the
//! poisoned gate refuses a send `input-not-ready`, and the human's keys still reach the child. The
//! home sits outside `target/e2e-home` (`TestHome::outside_scan`), so the CI scans never count
//! this deliberate panic.

#[allow(dead_code)]
mod support;

use std::io::Write as _;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Instant;

use serde_json::{Value, json};
use support::fake::{self, of_kind};
use support::home::{StampedHome, TestHome, VIOLA, Wrapper, workspace_path};
use support::watch::{WITHIN, Watch};
use viola_pty::Size;

const CANARY: &str = "canary-chain-value-5c1e";

fn role_lines(home: &Path) -> Vec<Value> {
    support::ndjson::read_lines(&home.join("diagnostics").join("run-builder.ndjson"))
}

fn wait_role(home: &Path, what: &str, pred: impl Fn(&[Value]) -> bool) -> Vec<Value> {
    let watch = Watch::start("role");
    let deadline = Instant::now() + WITHIN;
    loop {
        let lines = role_lines(home);
        if pred(&lines) {
            return lines;
        }
        watch.note(&format!("role lines {}", lines.len()));
        watch.deadline_check(deadline, &format!("timed out waiting for {what}"));
        std::thread::yield_now();
    }
}

#[test]
fn chaos_feed_panic_refuses_send_and_keeps_passthrough() {
    let fixtures = workspace_path("fixtures/claude");
    let mut wrapper = Wrapper::boot_sized(
        StampedHome::unstamped(TestHome::outside_scan()),
        "builder",
        None,
        &[
            "--fixtures",
            fixtures.to_str().expect("utf-8 path"),
            "--vt100-panic-bytes",
        ],
        Size { cols: 1, rows: 24 },
    );
    let home = wrapper.home().to_path_buf();
    let rejected = |l: &Value| {
        l["event"] == "parse-rejected" && l["parser"] == "vt100-feed" && l["detail"] == "panicked"
    };
    let role = wait_role(&home, "the contained feed panic", |lines| {
        lines.iter().any(rejected) && lines.iter().any(|l| l["event"] == "panic")
    });
    assert_eq!(role.iter().filter(|l| rejected(l)).count(), 1);
    assert_eq!(
        role.iter().filter(|l| l["event"] == "panic").count(),
        1,
        "the panic hook's line is witnessed, not hidden"
    );

    let mut child = Command::new(VIOLA)
        .arg("--home")
        .arg(&home)
        .args(["send", "builder", "--json"])
        .env_remove("VIOLA_NAME")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("viola send");
    let mut stdin = child.stdin.take().expect("stdin");
    stdin
        .write_all(format!("{CANARY} after the panic").as_bytes())
        .expect("stdin");
    drop(stdin);
    let out = child.wait_with_output().expect("viola send exits");
    assert_eq!(out.status.code(), Some(13));
    assert_eq!(
        serde_json::from_slice::<Value>(&out.stdout).expect("one JSON document"),
        json!({"v": 1, "refusal": "not-delivered", "detail": "input-not-ready"})
    );
    let events = support::ndjson::read_lines(&wrapper.instance_dir().join("events.ndjson"));
    assert!(!events.iter().any(|e| e["kind"] == "send-issued"));
    let refused: Vec<&Value> = events
        .iter()
        .filter(|e| e["kind"] == "send-refused")
        .collect();
    assert_eq!(refused.len(), 1);
    assert_eq!(
        refused[0]["data"],
        json!({"refusal": "not-delivered", "detail": "input-not-ready"})
    );

    wrapper.send(b"k");
    let receipt = fake::wait_for(&wrapper.receipt(), "the typed key", |l| {
        of_kind(l, "key").iter().any(|k| k["hex"] == "6b")
    });
    assert!(of_kind(&receipt, "prompt").is_empty(), "nothing was typed");
    wrapper.stop();
}
