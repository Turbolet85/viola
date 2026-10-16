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

/// `<scratch>/no-claude/`, empty: a `claude` looked up there is not found.
pub fn empty_bin(scratch: &Path) -> PathBuf {
    let dir = scratch.join("no-claude");
    fs::create_dir_all(&dir).expect("the empty bin dir");
    dir
}

/// `<scratch>/<name>/` holding the fake agent as `claude` (a hard link, else a copy) and beside it
/// its `agents-mode` file and `agents-rows.json` when given: what its `agents --json` answers, and
/// where its own interactive starts register their sessions.
pub fn claude_bin(scratch: &Path, name: &str, mode: Option<&str>, rows: Option<&[u8]>) -> PathBuf {
    let dir = scratch.join(name);
    fs::create_dir_all(&dir).expect("the bin dir");
    let claude = claude_in(&dir);
    if !claude.is_file() && fs::hard_link(FAKE, &claude).is_err() {
        fs::copy(FAKE, &claude).expect("the claude copy");
    }
    if let Some(mode) = mode {
        fs::write(dir.join("agents-mode"), mode).expect("the agents mode");
    }
    if let Some(rows) = rows {
        fs::write(dir.join("agents-rows.json"), rows).expect("the agents rows");
    }
    dir
}

/// The `claude` executable inside a bin dir.
pub fn claude_in(dir: &Path) -> PathBuf {
    dir.join(format!("claude{}", std::env::consts::EXE_SUFFIX))
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

/// Waits for the receipt's `statusline` line: the override's status line as the fake agent ran it
/// at launch.
pub fn wait_statusline(path: &Path) -> Value {
    let lines = wait_for(path, "the statusline receipt", |l| {
        !of_kind(l, "statusline").is_empty()
    });
    of_kind(&lines, "statusline")[0].clone()
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
