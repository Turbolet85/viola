//! Chaos (test-plan §6 Chaos suite, forward compatibility): a log a newer build appended to reads
//! back whole. A line of a kind this build does not know, and a known-kind line carrying a key
//! outside its contract, are each counted once on `viola list --json`'s `skipped`, and neither
//! fails the read. The wrapper is stopped before the lines are appended, so no append of its own
//! meets them.

#[allow(dead_code)]
mod support;

use std::fs::OpenOptions;
use std::io::Write as _;
use std::path::Path;

use serde_json::{Value, json};
use support::cli::viola;
use support::home::{StampedHome, TestHome, Wrapper};

/// One complete line, appended as one write.
fn append(log: &Path, line: &Value) {
    let mut bytes = serde_json::to_vec(line).expect("a line");
    bytes.push(b'\n');
    OpenOptions::new()
        .append(true)
        .open(log)
        .expect("the log")
        .write_all(&bytes)
        .expect("one write");
}

fn skipped(home: &Path) -> Value {
    let ran = viola(home, &["list", "--json"], None, None);
    assert_eq!(ran.code, Some(0), "stderr: {}", ran.stderr);
    let doc: Value = serde_json::from_str(&ran.stdout).expect("one JSON document");
    doc["ok"]["skipped"].clone()
}

#[test]
fn chaos_lines_of_a_newer_build_are_counted_and_the_board_still_reads() {
    let wrapper = Wrapper::boot(
        StampedHome::unstamped(TestHome::new()),
        "builder",
        None,
        &[],
    );
    let log = wrapper.instance_dir().join("events.ndjson");
    let (stopped, stamped) = wrapper.stop_keep();
    assert_eq!(stopped.code(), Some(0));
    let home = stamped.home.path();
    assert_eq!(
        skipped(home),
        json!({"unknown_kinds": 0, "unknown_fields": 0, "torn_lines": 0})
    );

    let ts = "2025-03-01T00:00:00.000Z";
    append(
        &log,
        &json!({"v": 1, "ts": ts, "instance": "builder", "kind": "later-kind",
            "source": "wrapper", "data": {}}),
    );
    append(
        &log,
        &json!({"v": 1, "ts": ts, "instance": "builder", "kind": "wheel", "source": "wrapper",
            "data": {"holder": "driver", "cause": "start"}, "later": 1}),
    );
    assert_eq!(
        skipped(home),
        json!({"unknown_kinds": 1, "unknown_fields": 1, "torn_lines": 0})
    );
}
