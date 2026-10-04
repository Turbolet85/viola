//! Driving the fake agent: its control and receipt files, test-written plugin folders and
//! fixtures. Waits are bounded deadline loops over file state, never sleeps.

use std::fs::{self, OpenOptions};
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use serde_json::{Value, json};

use super::watch::{WITHIN, Watch};

pub const FAKE: &str = env!("CARGO_BIN_EXE_viola-fake-agent");
/// The CLI version the committed `fixtures/claude/` set was recorded at, and the fake agent's
/// default answer: a test literal, never the fake agent's own constant.
pub const RECORDED_CLI_VERSION: &str = "2.1.287";

pub fn control_path(home: &Path, name: &str) -> PathBuf {
    home.join("fake").join(format!("{name}.control"))
}

pub fn receipt_path(home: &Path, name: &str) -> PathBuf {
    home.join("fake").join(format!("{name}.receipt.ndjson"))
}

/// Releases one gated step: one more complete line in the control file.
pub fn release(control: &Path) {
    if let Some(dir) = control.parent() {
        fs::create_dir_all(dir).expect("control dir");
    }
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(control)
        .expect("control");
    f.write_all(b"go\n").expect("append");
}

/// The receipt's complete lines; a line the agent is still appending is read once it lands.
pub fn receipt(path: &Path) -> Vec<Value> {
    super::ndjson::read_lines(path)
}

pub fn of_kind<'a>(lines: &'a [Value], kind: &str) -> Vec<&'a Value> {
    lines.iter().filter(|l| l["kind"] == kind).collect()
}

/// The receipt's kinds in first-seen order: codes only, never a line's content.
fn kinds(lines: &[Value]) -> String {
    let mut seen: Vec<&str> = Vec::new();
    for kind in lines.iter().filter_map(|l| l["kind"].as_str()) {
        if !seen.contains(&kind) {
            seen.push(kind);
        }
    }
    seen.join(",")
}

/// Waits until `pred` holds over the receipt; panics at the deadline with the watch report.
pub fn wait_for(path: &Path, what: &str, pred: impl Fn(&[Value]) -> bool) -> Vec<Value> {
    let watch = Watch::start("receipt");
    let deadline = Instant::now() + WITHIN;
    loop {
        let lines = receipt(path);
        if pred(&lines) {
            return lines;
        }
        watch.note(&format!(
            "receipt lines {} kinds {}",
            lines.len(),
            kinds(&lines)
        ));
        watch.deadline_check(deadline, &format!("timed out waiting for {what}"));
        std::thread::yield_now();
    }
}

/// Holds for a bounded window and asserts `pred` never becomes true (an ordering check).
pub fn stays_false(path: &Path, window: Duration, pred: impl Fn(&[Value]) -> bool) {
    let until = Instant::now() + window;
    while Instant::now() < until {
        assert!(!pred(&receipt(path)), "released too early");
        std::thread::yield_now();
    }
}

/// `<dir>/hooks/hooks.json` registering one exec-form command for `event`.
pub fn write_plugin(dir: &Path, event: &str, command: &str, args: &[&str]) {
    let hooks = dir.join("hooks");
    fs::create_dir_all(&hooks).expect("hooks dir");
    let doc = json!({
        "hooks": {
            event: [{"hooks": [{"type": "command", "command": command, "args": args}]}]
        }
    });
    fs::write(hooks.join("hooks.json"), doc.to_string()).expect("hooks.json");
}

/// `<fixtures>/<cli-version>/<event>.<variant>.json` — synthetic payloads only.
pub fn write_fixture(fixtures: &Path, version: &str, event: &str, variant: &str, body: &Value) {
    let dir = fixtures.join(version);
    fs::create_dir_all(&dir).expect("fixture dir");
    fs::write(
        dir.join(format!("{event}.{variant}.json")),
        body.to_string(),
    )
    .expect("fixture");
}

pub fn unhex(hex: &str) -> Vec<u8> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).expect("hex"))
        .collect()
}
