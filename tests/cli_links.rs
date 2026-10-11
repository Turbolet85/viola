//! Session links over the CLI and the wrapper channel (architecture §Established Decisions,
//! [Links]; test-plan §6 Scenario E5's `link` step): an issued `send` or a taken `answer` from
//! another instance leaves one `link` line `driver -> driven` in the driven instance's own log,
//! directly before the record it rides with; a refused call and a call under the wrapper's own name
//! leave none; the snapshot's `links` mirrors the set, a restarted wrapper restores it from the log,
//! and the log replay reads the same set back. Every case starts from a stamped home.

#[allow(dead_code)]
mod support;

use std::path::Path;

use rstest::rstest;
use serde_json::{Value, json};
use support::cli::{Ran, viola};
use support::events::{events, wait_events};
use support::home::{StampedHome, Wrapper, snapshot_data, stamped_home, workspace_path};
use support::hygiene::load_schema;
use support::statusline::CANARY;
use viola_state::events::Skipped;
use viola_state::replay::replay;

/// A gated `PostToolUse`, then a gated `Stop`: the turn a confirmed send started, ended on cue.
const PATH3: &str = "fixtures/fake-scripts/path3.json";
/// Gated dialog hooks: the first step raises a question.
const PATH4: &str = "fixtures/fake-scripts/path4.json";

/// A stamped wrapper over the committed hook fixtures (and `script`), its SessionStart landed.
fn boot(stamped: StampedHome, script: Option<&str>) -> Wrapper {
    support::events::boot(stamped, script, &[])
}

/// `viola send builder --json` as `from`, with a text that carries the tests' canary.
fn send(home: &Path, from: Option<&str>) -> Ran {
    let text = format!("{CANARY} to the builder");
    viola(home, &["send", "builder", "--json"], Some(&text), from)
}

fn links(lines: &[Value]) -> Vec<&Value> {
    lines.iter().filter(|l| l["kind"] == "link").collect()
}

/// The one `link` line of `lines` and where it stands: `overseer -> builder`, the wrapper's own.
fn the_link(lines: &[Value]) -> (usize, &Value) {
    let at: Vec<usize> = (0..lines.len())
        .filter(|n| lines[*n]["kind"] == "link")
        .collect();
    assert_eq!(at.len(), 1, "{lines:?}");
    let link = &lines[at[0]];
    assert_eq!(link["source"], "wrapper");
    assert_eq!(link["instance"], "builder");
    assert_eq!(
        link["data"],
        json!({"driver": "overseer", "driven": "builder"})
    );
    (at[0], link)
}

/// The snapshot's `links`.
fn snapshot_links(instance_dir: &Path) -> Value {
    snapshot_data(instance_dir).expect("snapshot")["links"].clone()
}

/// The one item `link`'s line makes.
fn linked_since(link: &Value) -> Value {
    json!([{"driver": "overseer", "driven": "builder", "since": link["ts"]}])
}

/// The offset each complete line of the log starts at.
fn line_starts(instance_dir: &Path) -> Vec<usize> {
    let log = std::fs::read(instance_dir.join("events.ndjson")).expect("the log");
    let ends = log.iter().enumerate().filter(|(_, b)| **b == b'\n');
    std::iter::once(0)
        .chain(ends.map(|(at, _)| at + 1))
        .collect()
}

/// No home-level line carries the canary or a link's own fields, and every role line passes the
/// diag-line schema.
fn assert_logs_clean(home: &Path) {
    let schema = load_schema(&workspace_path("schemas/diag-line.v1.json"));
    let validator = jsonschema::validator_for(&schema).expect("valid schema");
    for entry in std::fs::read_dir(home.join("diagnostics")).expect("diagnostics") {
        let path = entry.expect("entry").path();
        let text = std::fs::read_to_string(&path).expect("role file");
        assert!(!text.contains(CANARY), "{path:?} holds content");
        for (n, line) in support::ndjson::read_lines(&path).iter().enumerate() {
            assert!(
                validator.is_valid(line),
                "{path:?} line {n} fails the schema"
            );
            for key in ["driver", "driven", "since"] {
                assert!(line.get(key).is_none(), "{path:?} line {n} holds {key}");
            }
        }
    }
}

/// An issued send from `overseer` links it once: the `link` line stands directly before that
/// send's `send-issued`, whose cursor is still the offset its own line starts at, the snapshot
/// holds the pair since that line's `ts`, and a second issued send adds nothing.
#[rstest]
fn links_a_send_from_another_instance_links_once(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home, Some(PATH3));
    let home = wrapper.home().to_path_buf();
    let dir = wrapper.instance_dir();
    assert_eq!(snapshot_links(&dir), json!([]));

    let sent = send(&home, Some("overseer"));
    assert_eq!(sent.code, Some(0), "stderr: {}", sent.stderr);
    let lines = events(&dir);
    let (at, link) = the_link(&lines);
    let issued = &lines[at + 1];
    assert_eq!(issued["kind"], "send-issued");
    assert_eq!(
        issued["data"],
        json!({"cursor": line_starts(&dir)[at + 1], "from": "overseer"})
    );
    assert_eq!(snapshot_links(&dir), linked_since(link));

    wrapper.release();
    wrapper.release();
    wait_events(&dir, "the scripted Stop's turn-ended", |l| {
        l.iter().any(|e| e["kind"] == "turn-ended")
    });
    let again = send(&home, Some("overseer"));
    assert_eq!(again.code, Some(0), "stderr: {}", again.stderr);
    let lines = events(&dir);
    assert_eq!(
        lines.iter().filter(|e| e["kind"] == "send-issued").count(),
        2
    );
    let (_, still) = the_link(&lines);
    assert_eq!(snapshot_links(&dir), linked_since(still));
    assert_logs_clean(&home);
    wrapper.stop();
}

/// A taken answer from `overseer` links it; an answer the wrapper refuses links nothing.
#[rstest]
fn links_an_answer_from_another_instance_links(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home, Some(PATH4));
    let home = wrapper.home().to_path_buf();
    let dir = wrapper.instance_dir();
    wrapper.release();
    let lines = wait_events(&dir, "the question's line", |l| {
        l.iter().any(|e| e["kind"] == "question")
    });
    let asked = lines
        .iter()
        .position(|e| e["kind"] == "question")
        .expect("the question");
    let dialog_id = lines[asked]["data"]["dialog_id"]
        .as_u64()
        .expect("a dialog_id");
    let response = json!({"answers": {"Which shape?": format!("{CANARY} a triangle")}}).to_string();
    let answer = |id: u64| {
        let id = id.to_string();
        let args = ["answer", "builder", id.as_str(), "--json"];
        viola(&home, &args, Some(&response), Some("overseer"))
    };

    let unknown = answer(dialog_id + 7);
    assert_eq!(unknown.code, Some(13), "stderr: {}", unknown.stderr);
    assert_eq!(
        unknown.stdout,
        "{\"v\":1,\"refusal\":\"not-delivered\",\"detail\":\"unknown-dialog\"}\n"
    );
    assert!(links(&events(&dir)).is_empty(), "a refused answer links");
    assert_eq!(snapshot_links(&dir), json!([]));

    let taken = answer(dialog_id);
    assert_eq!(taken.code, Some(0), "stderr: {}", taken.stderr);
    assert_eq!(taken.stdout, "{\"v\":1,\"ok\":{}}\n");
    let lines = events(&dir);
    let (at, link) = the_link(&lines);
    assert!(at > asked, "the link follows the dialog it answers");
    assert_eq!(snapshot_links(&dir), linked_since(link));
    assert_logs_clean(&home);
    wrapper.stop();
}

/// A send refused under a paused wheel links nothing, and neither does an issued send under the
/// wrapper's own name.
#[rstest]
fn links_a_refused_send_and_an_own_name_send_record_none(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home, None);
    let home = wrapper.home().to_path_buf();
    let dir = wrapper.instance_dir();

    let paused = viola(&home, &["pause", "builder", "--json"], None, None);
    assert_eq!(paused.code, Some(0), "stderr: {}", paused.stderr);
    let refused = send(&home, Some("overseer"));
    assert_eq!(refused.code, Some(10), "stderr: {}", refused.stderr);
    assert_eq!(
        refused.stdout,
        "{\"v\":1,\"refusal\":\"human-typing\",\"detail\":\"manual-pause\"}\n"
    );
    assert!(links(&events(&dir)).is_empty(), "a refused send links");

    let released = viola(&home, &["release", "builder", "--json"], Some(""), None);
    assert_eq!(released.code, Some(0), "stderr: {}", released.stderr);
    let own = send(&home, Some("builder"));
    assert_eq!(own.code, Some(0), "stderr: {}", own.stderr);
    let lines = events(&dir);
    let issued: Vec<&Value> = lines
        .iter()
        .filter(|e| e["kind"] == "send-issued")
        .collect();
    assert_eq!(issued.len(), 1, "{issued:?}");
    assert_eq!(issued[0]["data"]["from"], "builder");
    assert!(links(&lines).is_empty(), "the wrapper's own name links");
    assert_eq!(snapshot_links(&dir), json!([]));
    assert_logs_clean(&home);
    wrapper.stop();
}

/// A wrapper started over a log that holds a link restores it: its snapshot holds the same pair
/// with the same `since`, and a further send from that driver appends no second `link`.
#[rstest]
fn links_survive_a_wrapper_restart(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home, None);
    let home = wrapper.home().to_path_buf();
    let dir = wrapper.instance_dir();
    let sent = send(&home, Some("overseer"));
    assert_eq!(sent.code, Some(0), "stderr: {}", sent.stderr);
    let linked = snapshot_links(&dir);
    assert_eq!(linked, linked_since(the_link(&events(&dir)).1));
    let (stopped, stamped) = wrapper.stop_keep();
    assert_eq!(stopped.code(), Some(0));
    let first_life = events(&dir).len();

    let second = boot(stamped, None);
    let lines = events(&dir);
    assert_eq!(lines[first_life]["kind"], "wheel");
    assert_eq!(lines[first_life]["data"]["cause"], "start");
    assert_eq!(lines[first_life + 1]["kind"], "budget-gate");
    assert_eq!(snapshot_links(&dir), linked);

    let again = send(&home, Some("overseer"));
    assert_eq!(again.code, Some(0), "stderr: {}", again.stderr);
    let lines = events(&dir);
    assert_eq!(
        lines.iter().filter(|e| e["kind"] == "send-issued").count(),
        2
    );
    the_link(&lines);
    assert_eq!(snapshot_links(&dir), linked);
    assert_logs_clean(&home);
    second.stop();
}

/// test-plan §6 Scenario E5, the `link` step: the replay over the real wrapper's log returns the
/// link the wrapper recorded, equal to the snapshot's, and steps over nothing.
#[rstest]
fn e5_the_replay_reads_the_link_the_wrapper_wrote(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home, None);
    let home = wrapper.home().to_path_buf();
    let dir = wrapper.instance_dir();
    assert_eq!(replay(&dir).expect("the replay").links, Vec::<Value>::new());

    let sent = send(&home, Some("overseer"));
    assert_eq!(sent.code, Some(0), "stderr: {}", sent.stderr);
    let replayed = replay(&dir).expect("the replay");
    assert_eq!(replayed.links.len(), 1);
    assert_eq!(json!(replayed.links), snapshot_links(&dir));
    assert_eq!(
        json!(replayed.links),
        linked_since(the_link(&events(&dir)).1)
    );
    assert_eq!(
        replayed.skipped,
        Skipped {
            unknown_kinds: 0,
            unknown_fields: 0,
            torn_lines: 0,
        }
    );
    assert_logs_clean(&home);
    wrapper.stop();
}
