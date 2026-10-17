//! The one `events.ndjson` reader and wait for root tests, and the wrapper boot that waits for the
//! session-start record (test-plan §3 `run` step 2). The wait is the root-watch loop on the one
//! bound `WITHIN`; the boot takes its home from the caller and makes none.

use std::path::Path;
use std::time::Instant;

use serde_json::Value;

use super::home::{StampedHome, Wrapper, workspace_path};
use super::ndjson;
use super::watch::{WITHIN, Watch};

/// The complete lines of `<instance_dir>/events.ndjson`.
pub fn events(instance_dir: &Path) -> Vec<Value> {
    ndjson::read_lines(&instance_dir.join("events.ndjson"))
}

/// The complete lines once `pred` holds over them; past `WITHIN` it panics naming `what`.
pub fn wait_events(instance_dir: &Path, what: &str, pred: impl Fn(&[Value]) -> bool) -> Vec<Value> {
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

/// Raises the next gated step and waits for its dialog line: its `dialog_id`.
pub fn raise(wrapper: &Wrapper, kind: &str, nth: usize) -> u64 {
    wrapper.release();
    let of_kind = |lines: &[Value]| -> Vec<Value> {
        lines
            .iter()
            .filter(|l| l["kind"] == kind)
            .cloned()
            .collect()
    };
    let lines = wait_events(&wrapper.instance_dir(), "the dialog line", |l| {
        of_kind(l).len() >= nth
    });
    of_kind(&lines)[nth - 1]["data"]["dialog_id"]
        .as_u64()
        .expect("a dialog_id")
}

/// A `builder` wrapper on `stamped` over the committed hook fixtures (then `extra`, and `script`),
/// its SessionStart record landed.
pub fn boot(stamped: StampedHome, script: Option<&str>, extra: &[&str]) -> Wrapper {
    let fixtures = workspace_path("fixtures/claude");
    let mut args = vec!["--fixtures", fixtures.to_str().expect("utf-8 path")];
    args.extend_from_slice(extra);
    let wrapper = Wrapper::boot(stamped, "builder", script, &args);
    wait_events(&wrapper.instance_dir(), "the session-start record", |l| {
        l.iter().any(|e| e["kind"] == "session-start")
    });
    wrapper
}
