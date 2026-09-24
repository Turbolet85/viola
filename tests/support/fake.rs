//! Driving the fake agent: its control and receipt files, test-written plugin folders and
//! fixtures. Waits are bounded deadline loops over file state, never sleeps.

use std::fs::{self, OpenOptions};
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use serde_json::{Value, json};

pub const FAKE: &str = env!("CARGO_BIN_EXE_viola-fake-agent");
const WAIT_WITHIN: Duration = Duration::from_secs(10);

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

pub fn receipt(path: &Path) -> Vec<Value> {
    fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .map(|l| serde_json::from_str(l).expect("one JSON object per receipt line"))
        .collect()
}

pub fn of_kind<'a>(lines: &'a [Value], kind: &str) -> Vec<&'a Value> {
    lines.iter().filter(|l| l["kind"] == kind).collect()
}

/// Waits until `pred` holds over the receipt; panics at the deadline.
pub fn wait_for(path: &Path, what: &str, pred: impl Fn(&[Value]) -> bool) -> Vec<Value> {
    let deadline = Instant::now() + WAIT_WITHIN;
    loop {
        let lines = receipt(path);
        if pred(&lines) {
            return lines;
        }
        assert!(Instant::now() < deadline, "timed out waiting for {what}");
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
