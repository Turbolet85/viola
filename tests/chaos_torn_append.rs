//! Chaos (test-plan §6 Chaos suite, torn append): a log whose last line was cut short costs the
//! next start nothing. Its first record begins on a fresh line, every earlier byte stays where it
//! was, and the process that healed says so once in its role file. The wrapper is stopped and the
//! log shortened in place of a kill: the file on disk is the same.

#[allow(dead_code)]
mod support;

use std::fs::{self, OpenOptions};
use std::path::Path;
use std::time::Instant;

use serde_json::{Value, json};
use support::home::{StampedHome, TestHome, Wrapper, workspace_path};
use support::hygiene::load_schema;
use support::watch::{WITHIN, Watch};
use viola_state::events::{LoggedLine, Skipped, read_from};

const NONE_SKIPPED: Skipped = Skipped {
    unknown_kinds: 0,
    unknown_fields: 0,
    torn_lines: 0,
};

const ONE_TORN: Skipped = Skipped {
    unknown_kinds: 0,
    unknown_fields: 0,
    torn_lines: 1,
};

/// The product reader from `after` on: its lines, and what it stepped over.
fn read(instance_dir: &Path, after: u64) -> (Vec<LoggedLine>, Skipped) {
    let mut lines = read_from(instance_dir, after).expect("the log opens");
    let read = lines.by_ref().map(|l| l.expect("a line")).collect();
    (read, lines.skipped())
}

fn recovered(home: &Path) -> Vec<Value> {
    support::ndjson::read_lines(&home.join("diagnostics").join("run-builder.ndjson"))
        .into_iter()
        .filter(|line| line["event"] == "state-recovered")
        .collect()
}

#[test]
fn chaos_torn_tail_left_by_a_stopped_wrapper_is_healed_by_the_next_start() {
    let wrapper = Wrapper::boot(
        StampedHome::unstamped(TestHome::new()),
        "builder",
        None,
        &[],
    );
    let dir = wrapper.instance_dir();
    let log = dir.join("events.ndjson");
    let (stopped, stamped) = wrapper.stop_keep();
    assert_eq!(stopped.code(), Some(0));
    let (whole, skipped) = read(&dir, 0);
    assert_eq!(skipped, NONE_SKIPPED);

    let cut = whole.last().expect("the first start's records").end - 5;
    OpenOptions::new()
        .write(true)
        .open(&log)
        .expect("the log")
        .set_len(cut)
        .expect("shorten");
    let torn = fs::read(&log).expect("the log");
    assert_eq!(u64::try_from(torn.len()).expect("len"), cut);
    assert_ne!(torn.last(), Some(&b'\n'));
    let (kept, skipped) = read(&dir, 0);
    assert_eq!(kept.len(), whole.len() - 1);
    assert_eq!(skipped, ONE_TORN);

    let again = Wrapper::boot(stamped, "builder", None, &[]);
    let home = again.home().to_path_buf();
    let watch = Watch::start("recovered");
    let deadline = Instant::now() + WITHIN;
    while recovered(&home).is_empty() {
        watch.note("no state-recovered line");
        watch.deadline_check(deadline, "the second start logged no state-recovered line");
        std::thread::yield_now();
    }
    // Stopped before the files are read, so no read meets an append part-way.
    let (stopped, _home) = again.stop_keep();
    assert_eq!(stopped.code(), Some(0));

    let healed = fs::read(&log).expect("the log");
    assert_eq!(&healed[..torn.len()], torn);
    assert_eq!(healed[torn.len()], b'\n');
    let (from_the_cut, _) = read(&dir, cut);
    let first = from_the_cut.first().expect("a line after the heal");
    assert_eq!(first.start, cut + 1);
    assert_eq!(first.value["kind"], "wheel");
    assert_eq!(first.value["source"], "wrapper");
    assert_eq!(
        first.value["data"],
        json!({"holder": "driver", "cause": "start"})
    );
    let (lines, skipped) = read(&dir, 0);
    assert_eq!(lines.len(), kept.len() + from_the_cut.len());
    assert_eq!(skipped, ONE_TORN);

    let lines = recovered(&home);
    assert_eq!(lines.len(), 1, "one heal, one line");
    let line = &lines[0];
    assert_eq!(line["detail"], "torn-line-healed");
    assert_eq!(line["file"], "events.ndjson");
    assert_eq!(line["offset"], cut);
    assert_eq!(line["level"], "WARN");
    assert_eq!(line["process"], "run");
    assert_eq!(line["instance"], "builder");
    assert!(
        line.get("corr").is_none(),
        "state-recovered carries no corr"
    );
    // A test home is removed unless a run keeps it, so the line is held to the schema here too.
    let schema = load_schema(&workspace_path("schemas/diag-line.v1.json"));
    let validator = jsonschema::validator_for(&schema).expect("valid schema");
    assert!(
        validator.is_valid(line),
        "the state-recovered line fails the diag-line schema"
    );
}
