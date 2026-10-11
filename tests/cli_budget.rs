//! Critical Path 6 over the CLI and the wrapper channel (test-plan §6 Path 6; architecture
//! [Budget Governor]), and Scenario E5's budget leg: a usage reading over a threshold closes the
//! wrapper's gate for automated `send` (exit 11, the crossed window as detail), the human's
//! `viola release --budget` lifts it for that instance until the window's reset, and the log
//! replay reads back what the wrapper wrote. Every reading reaches `budget.json` through
//! `viola hook statusline`, and every case starts from a stamped home.

#[allow(dead_code)]
mod support;

use std::path::Path;
use std::time::Instant;

use rstest::rstest;
use serde_json::{Value, json};
use support::cli::{Ran, viola};
use support::events::{events, wait_events};
use support::fake::{self, of_kind};
use support::home::{StampedHome, Wrapper, snapshot_data, stamped_home, workspace_path};
use support::hygiene::load_schema;
use support::statusline::{
    CANARY, FIVE_HOUR_RESET_STORED, PASSED_RESET, PASSED_RESET_STORED, feed, payload, usage,
};
use support::watch::{WITHIN, Watch};
use viola_state::events::Skipped;
use viola_state::replay::replay;

/// A gated `PostToolUse`, then a gated `Stop`: the turn a confirmed send started, ended on cue.
const PATH3: &str = "fixtures/fake-scripts/path3.json";
/// Gated dialog hooks: the first step raises a question, held for a driver's answer.
const PATH4: &str = "fixtures/fake-scripts/path4.json";

const REFUSED_FIVE_HOUR: &str =
    "{\"v\":1,\"refusal\":\"budget-paused\",\"detail\":\"five-hour\"}\n";
const REFUSED_SEVEN_DAY: &str =
    "{\"v\":1,\"refusal\":\"budget-paused\",\"detail\":\"seven-day\"}\n";
const HINT: &str =
    "hint: budget gate is closed; it lifts at the window reset, or when the human lifts it";

/// A stamped wrapper over the committed hook fixtures (and `script`), its SessionStart landed.
fn boot(stamped: StampedHome, script: Option<&str>) -> Wrapper {
    support::events::boot(stamped, script, &[])
}

/// `viola send builder --json` with a text that carries the tests' canary.
fn send(home: &Path) -> Ran {
    let text = format!("{CANARY} over the budget");
    viola(home, &["send", "builder", "--json"], Some(&text), None)
}

fn release(home: &Path, flags: &[&str], from: Option<&str>) -> Ran {
    let mut args = vec!["release", "builder"];
    args.extend_from_slice(flags);
    viola(home, &args, Some(""), from)
}

/// The `data` of every `budget-gate` record, each a wrapper's own line.
fn gate_records(instance_dir: &Path) -> Vec<Value> {
    events(instance_dir)
        .into_iter()
        .filter(|l| l["kind"] == "budget-gate")
        .map(|l| {
            assert_eq!(l["source"], "wrapper");
            l["data"].clone()
        })
        .collect()
}

fn budget_json(home: &Path) -> Value {
    let bytes = std::fs::read(home.join("budget.json")).expect("budget.json");
    serde_json::from_slice(&bytes).expect("budget.json is JSON")
}

fn diagnostics(home: &Path, file: &str) -> Vec<Value> {
    support::ndjson::read_lines(&home.join("diagnostics").join(file))
}

/// No home-level line carries the canary, and every role line passes the diag-line schema.
fn assert_logs_clean(home: &Path) {
    let schema = load_schema(&workspace_path("schemas/diag-line.v1.json"));
    let validator = jsonschema::validator_for(&schema).expect("valid schema");
    for entry in std::fs::read_dir(home.join("diagnostics")).expect("diagnostics") {
        let path = entry.expect("entry").path();
        let text = std::fs::read_to_string(&path).expect("role file");
        assert!(!text.contains(CANARY), "{path:?} holds content");
        let file = path
            .file_name()
            .expect("name")
            .to_string_lossy()
            .into_owned();
        for (n, line) in diagnostics(home, &file).iter().enumerate() {
            assert!(validator.is_valid(line), "{file} line {n} fails the schema");
        }
    }
}

fn open() -> Value {
    json!({"paused": false})
}

fn five_hour() -> Value {
    json!({"paused": true, "window": "five-hour"})
}

fn seven_day() -> Value {
    json!({"paused": true, "window": "seven-day"})
}

/// test-plan §6 Path 6 steps 2 and 3: a five-hour reading of 91 refuses the send on every surface
/// the path lists, the outer terminal shows none of viola's own words, and a human key still
/// reaches the child.
#[rstest]
fn path6_a_reading_over_the_threshold_refuses_the_send(stamped_home: StampedHome) {
    let mut wrapper = boot(stamped_home, None);
    let home = wrapper.home().to_path_buf();
    let dir = wrapper.instance_dir();
    feed(&dir, usage(91, 10));
    let budget = budget_json(&home);
    assert_eq!(budget["five_hour"]["used_percentage"].as_f64(), Some(91.0));
    assert_eq!(budget["five_hour"]["resets_at"], FIVE_HOUR_RESET_STORED);
    assert_eq!(
        gate_records(&dir),
        [open()],
        "a reading alone moves no gate"
    );

    let refused = send(&home);
    assert_eq!(refused.code, Some(11), "stderr: {}", refused.stderr);
    assert_eq!(refused.stdout, REFUSED_FIVE_HOUR);
    assert!(refused.stderr.is_empty(), "--json carries no hint");

    assert_eq!(gate_records(&dir), [open(), five_hour()]);
    let lines = events(&dir);
    let refusals: Vec<&Value> = lines
        .iter()
        .filter(|e| e["kind"] == "send-refused")
        .collect();
    assert_eq!(refusals.len(), 1, "{refusals:?}");
    assert_eq!(
        refusals[0]["data"],
        json!({"refusal": "budget-paused", "detail": "five-hour"}),
        "no cursor"
    );
    assert!(!lines.iter().any(|e| e["kind"] == "send-issued"));
    assert_eq!(
        snapshot_data(&dir).expect("snapshot")["budget_paused"],
        true
    );

    let run = diagnostics(&home, "run-builder.ndjson");
    let refused: Vec<&Value> = run
        .iter()
        .filter(|l| l["event"] == "send-refused")
        .collect();
    assert_eq!(refused.len(), 1, "{refused:?}");
    let line = refused[0];
    assert_eq!(line["side"], "wrapper");
    assert_eq!(line["wheel"], "driver");
    assert_eq!(line["refusal"], "budget-paused");
    assert_eq!(line["detail"], "five-hour");
    assert_eq!(line["window"], "five-hour");
    assert!(line["used_percentage"].is_number(), "{line}");
    assert_eq!(line["used_percentage"].as_f64(), Some(91.0));
    assert_eq!(line["resets_at"], FIVE_HOUR_RESET_STORED);
    assert_eq!(line["paused"], true);
    assert!(line["corr"].is_u64() && line["rpc_id"].is_u64(), "{line}");
    assert!(line["conn"].is_string(), "{line}");
    assert!(!run.iter().any(|l| l["event"] == "send-issued"));
    let exits: Vec<Value> = diagnostics(&home, "cli-builder.ndjson")
        .into_iter()
        .filter(|l| l["event"] == "process-exit")
        .collect();
    assert_eq!(exits.len(), 1, "{exits:?}");
    assert_eq!(exits[0]["exit_code"], 11);

    let shown = String::from_utf8_lossy(&wrapper.shown()).into_owned();
    for word in ["unable", "hint:", "budget-paused"] {
        assert!(!shown.contains(word), "the outer terminal shows {word:?}");
    }
    wrapper.send(b"k");
    fake::wait_for(&wrapper.receipt(), "the human key's receipt", |l| {
        of_kind(l, "key").iter().any(|k| k["hex"] == "6b")
    });
    assert_logs_clean(&home);
    wrapper.stop();
}

/// Path 6 step 4, in one session: 86 in the seven-day window names it, exactly 90 in the five-hour
/// one pauses, and 89 beside 84 pauses nothing.
#[rstest]
fn path6_the_boundaries_and_the_window_word(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home, None);
    let home = wrapper.home().to_path_buf();
    let dir = wrapper.instance_dir();

    feed(&dir, usage(10, 86));
    let week = send(&home);
    assert_eq!(week.code, Some(11), "stderr: {}", week.stderr);
    assert_eq!(week.stdout, REFUSED_SEVEN_DAY);

    feed(&dir, usage(90, 10));
    let hours = send(&home);
    assert_eq!(hours.code, Some(11), "stderr: {}", hours.stderr);
    assert_eq!(hours.stdout, REFUSED_FIVE_HOUR);

    feed(&dir, usage(89, 84));
    let sent = send(&home);
    assert_eq!(sent.code, Some(0), "stderr: {}", sent.stderr);

    assert_eq!(
        gate_records(&dir),
        [open(), seven_day(), five_hour(), open()]
    );
    assert_eq!(
        snapshot_data(&dir).expect("snapshot")["budget_paused"],
        false
    );
    wrapper.stop();
}

/// Path 6 step 4's malformed `resets_at`, and a window that has already reset: a window is judged
/// only with a reset still ahead, so 91 pauses nothing here.
#[rstest]
#[case::a_reset_that_is_no_instant(json!("soon"), "unknown")]
#[case::a_reset_that_has_passed(json!(PASSED_RESET), PASSED_RESET_STORED)]
fn path6_an_unknown_or_passed_reset_does_not_pause(
    stamped_home: StampedHome,
    #[case] resets_at: Value,
    #[case] stored: &str,
) {
    let wrapper = boot(stamped_home, None);
    let home = wrapper.home().to_path_buf();
    let dir = wrapper.instance_dir();
    let rate_limits = json!({"five_hour": {"used_percentage": 91, "resets_at": resets_at}});
    feed(&dir, payload(Some(rate_limits)));
    let budget = budget_json(&home);
    assert_eq!(budget["five_hour"]["used_percentage"].as_f64(), Some(91.0));
    assert_eq!(budget["five_hour"]["resets_at"], stored);

    let sent = send(&home);
    assert_eq!(sent.code, Some(0), "stderr: {}", sent.stderr);
    assert_eq!(gate_records(&dir), [open()]);
    assert_eq!(
        snapshot_data(&dir).expect("snapshot")["budget_paused"],
        false
    );
    wrapper.stop();
}

/// Path 6 step 5: a plain `release` lifts no pause; `release --budget` lifts it until the crossed
/// window's reset and leaves the wheel alone; the next `send` is taken; with no pause standing a
/// second one records nothing.
#[rstest]
fn path6_release_budget_lifts_the_pause(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home, None);
    let home = wrapper.home().to_path_buf();
    let dir = wrapper.instance_dir();
    feed(&dir, usage(91, 10));
    assert_eq!(send(&home).code, Some(11));

    let plain = release(&home, &["--json"], None);
    assert_eq!(plain.code, Some(0), "stderr: {}", plain.stderr);
    assert_eq!(
        plain.stdout,
        "{\"v\":1,\"ok\":{\"wheel\":\"driver\",\"budget_paused\":true}}\n"
    );
    let still = send(&home);
    assert_eq!(still.code, Some(11), "a plain release lifts no pause");
    assert_eq!(still.stdout, REFUSED_FIVE_HOUR);

    let lifted = release(&home, &["--budget", "--json"], None);
    assert_eq!(lifted.code, Some(0), "stderr: {}", lifted.stderr);
    assert!(lifted.stderr.is_empty());
    assert_eq!(
        lifted.stdout,
        format!(
            "{{\"v\":1,\"ok\":{{\"wheel\":\"driver\",\"budget_paused\":false,\
             \"override_until\":\"{FIVE_HOUR_RESET_STORED}\"}}}}\n"
        )
    );
    let lines = events(&dir);
    let record = lines
        .iter()
        .rfind(|e| e["kind"] == "budget-gate")
        .expect("the override's record");
    assert_eq!(
        record["data"],
        json!({"paused": false, "override_until": FIVE_HOUR_RESET_STORED})
    );
    let instant = |value: &Value| {
        chrono::DateTime::parse_from_rfc3339(value.as_str().expect("an instant")).expect("RFC 3339")
    };
    assert!(instant(&record["data"]["override_until"]) > instant(&record["ts"]));
    let snapshot = snapshot_data(&dir).expect("snapshot");
    assert_eq!(snapshot["budget_paused"], false);
    assert_eq!(
        snapshot["budget_override_until"],
        record["data"]["override_until"]
    );
    assert_eq!(snapshot["wheel"], "driver");
    assert_eq!(
        lines.iter().filter(|e| e["kind"] == "wheel").count(),
        1,
        "the wheel's start record alone"
    );

    let sent = send(&home);
    assert_eq!(sent.code, Some(0), "stderr: {}", sent.stderr);

    let records = gate_records(&dir);
    assert_eq!(records.len(), 3);
    let again = release(&home, &["--budget", "--json"], None);
    assert_eq!(again.code, Some(0), "stderr: {}", again.stderr);
    assert_eq!(
        again.stdout,
        "{\"v\":1,\"ok\":{\"wheel\":\"driver\",\"budget_paused\":false}}\n"
    );
    let open = release(&home, &["--budget"], None);
    assert_eq!(open.code, Some(0), "stderr: {}", open.stderr);
    assert_eq!(open.stdout, "builder  budget gate open  wheel driver\n");
    assert_eq!(gate_records(&dir), records, "no record without a pause");
    assert_logs_clean(&home);
    wrapper.stop();
}

/// Path 6 step 8: in human mode a driver reads the struck `unable` line and then exactly one hint,
/// the last stderr line, which never names `release`. A driver's own `release --budget` is
/// refused and lifts nothing; the human's lifts the pause and says until when.
#[rstest]
fn path6_the_refusal_hint_never_names_release(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home, None);
    let home = wrapper.home().to_path_buf();
    let dir = wrapper.instance_dir();
    feed(&dir, usage(91, 10));

    let text = format!("{CANARY} from a driver");
    let refused = viola(&home, &["send", "builder"], Some(&text), Some("overseer"));
    assert_eq!(refused.code, Some(11));
    assert!(refused.stdout.is_empty());
    assert_eq!(
        refused.stderr,
        format!("[/ ] unable         builder  budget-paused  five-hour\n{HINT}\n")
    );
    let last = refused.stderr.lines().last().expect("a last line");
    assert!(
        last.starts_with("hint: ") && !last.contains("release"),
        "{last}"
    );
    assert!(!refused.stderr.contains('\x1b'));

    let driver = release(&home, &["--budget", "--json"], Some("overseer"));
    assert_eq!(driver.code, Some(20));
    assert_eq!(
        driver.stdout,
        "{\"v\":1,\"error\":\"wrapper-fault\",\"detail\":{\"code\":-32602,\
         \"message\":\"invalid params\",\"data\":{\"reason\":\"release-from-driver\"}}}\n"
    );
    let snapshot = snapshot_data(&dir).expect("snapshot");
    assert_eq!(snapshot["budget_paused"], true);
    assert!(
        snapshot.get("budget_override_until").is_none(),
        "{snapshot}"
    );
    assert_eq!(
        send(&home).code,
        Some(11),
        "a driver's release lifts nothing"
    );

    let lifted = release(&home, &["--budget"], None);
    assert_eq!(lifted.code, Some(0), "stderr: {}", lifted.stderr);
    assert!(lifted.stderr.is_empty());
    assert_eq!(
        lifted.stdout,
        "builder  budget gate lifted until 2099-01-01 00:00:00Z  wheel driver\n"
    );
    let from_driver = diagnostics(&home, "run-builder.ndjson")
        .into_iter()
        .filter(|l| l["event"] == "release-from-driver")
        .count();
    assert_eq!(from_driver, 1);
    assert_logs_clean(&home);
    wrapper.stop();
}

/// The snapshot once `pred` holds over its `data`; past `WITHIN` it panics naming `what`.
fn wait_snapshot(instance_dir: &Path, what: &str, pred: impl Fn(&Value) -> bool) -> Value {
    let watch = Watch::start("snapshot");
    let deadline = Instant::now() + WITHIN;
    loop {
        if let Some(data) = snapshot_data(instance_dir).filter(&pred) {
            return data;
        }
        watch.note("not yet");
        watch.deadline_check(deadline, &format!("timed out waiting for {what}"));
        std::thread::yield_now();
    }
}

/// The third evaluation point: a reading that crosses the threshold while a turn runs closes the
/// gate when that turn ends, its record right behind the `turn-ended` line, with no `send` asked.
#[rstest]
fn path6_a_turn_end_closes_the_gate(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home, Some(PATH3));
    let home = wrapper.home().to_path_buf();
    let dir = wrapper.instance_dir();
    let sent = send(&home);
    assert_eq!(sent.code, Some(0), "stderr: {}", sent.stderr);
    feed(&dir, usage(91, 10));
    assert_eq!(gate_records(&dir), [open()], "no point was reached yet");

    wrapper.release();
    wrapper.release();
    let lines = wait_events(&dir, "the gate's record behind the turn's end", |l| {
        l.iter()
            .any(|e| e["kind"] == "budget-gate" && e["data"]["paused"] == true)
    });
    let ended = lines
        .iter()
        .position(|e| e["kind"] == "turn-ended")
        .expect("the scripted Stop's turn-ended");
    assert_eq!(lines[ended + 1]["kind"], "budget-gate");
    assert_eq!(lines[ended + 1]["source"], "wrapper");
    assert_eq!(lines[ended + 1]["data"], five_hour());
    assert_eq!(gate_records(&dir), [open(), five_hour()]);
    assert!(!lines.iter().any(|e| e["kind"] == "send-refused"));
    wait_snapshot(&dir, "the snapshot's pause", |data| {
        data["budget_paused"] == true
    });
    wrapper.stop();
}

/// An override does not outlive its wrapper: a second start over the same home reads
/// `budget.json` alone, so its second record is the pause again, with no override.
#[rstest]
fn path6_a_restart_pauses_again(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home, None);
    let home = wrapper.home().to_path_buf();
    let dir = wrapper.instance_dir();
    feed(&dir, usage(91, 10));
    assert_eq!(send(&home).code, Some(11));
    let lifted = release(&home, &["--budget", "--json"], None);
    assert_eq!(lifted.code, Some(0), "stderr: {}", lifted.stderr);
    assert_eq!(
        snapshot_data(&dir).expect("snapshot")["budget_override_until"],
        FIVE_HOUR_RESET_STORED
    );
    let (stopped, stamped) = wrapper.stop_keep();
    assert_eq!(stopped.code(), Some(0));
    let first_life = events(&dir).len();

    let second = boot(stamped, None);
    let lines = events(&dir);
    assert_eq!(lines[first_life]["kind"], "wheel");
    assert_eq!(lines[first_life]["data"]["cause"], "start");
    assert_eq!(lines[first_life + 1]["kind"], "budget-gate");
    assert_eq!(lines[first_life + 1]["data"], five_hour());
    let snapshot = snapshot_data(&dir).expect("snapshot");
    assert_eq!(snapshot["budget_paused"], true);
    assert!(
        snapshot.get("budget_override_until").is_none(),
        "{snapshot}"
    );
    let refused = send(&home);
    assert_eq!(refused.code, Some(11), "stderr: {}", refused.stderr);
    assert_eq!(refused.stdout, REFUSED_FIVE_HOUR);
    second.stop();
}

/// test-plan §6 Path 6, the verbs the gate leaves alone: under one closed gate, which refuses an
/// automated `send` (the control), `wait`, `last`, `answer` and `pause` each give the exit code
/// and the document they give under an open one. `pause` comes last: under a paused wheel a `send`
/// is refused `human-typing` before the gate is read.
#[rstest]
fn path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home, Some(PATH4));
    let home = wrapper.home().to_path_buf();
    let dir = wrapper.instance_dir();
    wrapper.release();
    let lines = wait_events(&dir, "the question's line", |l| {
        l.iter().any(|e| e["kind"] == "question")
    });
    let raised = lines
        .iter()
        .find(|e| e["kind"] == "question")
        .expect("the question");
    let dialog_id = raised["data"]["dialog_id"]
        .as_u64()
        .expect("a dialog_id")
        .to_string();
    let read = |verb: &str| viola(&home, &[verb, "builder", "--json"], None, None);

    let (wait_open, last_open) = (read("wait"), read("last"));
    assert_eq!(wait_open.code, Some(0), "stderr: {}", wait_open.stderr);
    let woken: Value = serde_json::from_str(&wait_open.stdout).expect("one JSON document");
    assert_eq!(woken["ok"]["event"]["kind"], "question");
    assert_eq!(last_open.code, Some(0), "stderr: {}", last_open.stderr);
    assert_eq!(
        last_open.stdout,
        "{\"v\":1,\"ok\":{\"last_assistant_message\":null,\"ts\":null}}\n"
    );

    feed(&dir, usage(91, 10));
    let refused = send(&home);
    assert_eq!(refused.code, Some(11), "stderr: {}", refused.stderr);
    assert_eq!(refused.stdout, REFUSED_FIVE_HOUR);
    assert_eq!(gate_records(&dir), [open(), five_hour()]);

    let (wait_closed, last_closed) = (read("wait"), read("last"));
    assert_eq!(
        (wait_closed.code, wait_closed.stdout.as_str()),
        (wait_open.code, wait_open.stdout.as_str())
    );
    assert_eq!(
        (last_closed.code, last_closed.stdout.as_str()),
        (last_open.code, last_open.stdout.as_str())
    );
    let response = json!({"answers": {"Which shape?": format!("{CANARY} a triangle")}}).to_string();
    let args = ["answer", "builder", dialog_id.as_str(), "--json"];
    let answered = viola(&home, &args, Some(&response), None);
    assert_eq!(answered.code, Some(0), "stderr: {}", answered.stderr);
    assert_eq!(answered.stdout, "{\"v\":1,\"ok\":{}}\n");
    let paused = read("pause");
    assert_eq!(paused.code, Some(0), "stderr: {}", paused.stderr);
    assert_eq!(paused.stdout, "{\"v\":1,\"ok\":{\"wheel\":\"human\"}}\n");

    let lines = events(&dir);
    let refusals: Vec<&Value> = lines
        .iter()
        .filter(|e| e["kind"] == "send-refused")
        .collect();
    assert_eq!(refusals.len(), 1, "{refusals:?}");
    assert_eq!(refusals[0]["data"]["refusal"], "budget-paused");
    for ran in [&wait_closed, &last_closed, &answered, &paused] {
        assert!(!ran.stdout.contains("budget-paused"), "{}", ran.stdout);
        assert!(ran.stderr.is_empty(), "stderr: {}", ran.stderr);
    }
    assert_eq!(gate_records(&dir), [open(), five_hour()]);
    assert_logs_clean(&home);
    wrapper.stop();
}

const NONE_SKIPPED: Skipped = Skipped {
    unknown_kinds: 0,
    unknown_fields: 0,
    torn_lines: 0,
};

/// test-plan §6 Scenario E5, the budget leg: the replay over the real wrapper's log reads the
/// pause, then the override with the end the wrapper wrote, and steps over nothing.
#[rstest]
fn e5_the_replay_reads_what_the_wrapper_wrote(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home, None);
    let home = wrapper.home().to_path_buf();
    let dir = wrapper.instance_dir();
    feed(&dir, usage(91, 10));
    assert_eq!(send(&home).code, Some(11));
    let paused = replay(&dir).expect("the replay");
    assert_eq!(paused.budget_paused, Some(true));
    assert_eq!(paused.budget_override_until, None);
    assert_eq!(paused.skipped, NONE_SKIPPED);

    let lifted = release(&home, &["--budget", "--json"], None);
    assert_eq!(lifted.code, Some(0), "stderr: {}", lifted.stderr);
    let overridden = replay(&dir).expect("the replay");
    assert_eq!(overridden.budget_paused, Some(false));
    let snapshot = snapshot_data(&dir).expect("snapshot");
    assert_eq!(
        json!(overridden.budget_override_until),
        snapshot["budget_override_until"]
    );
    assert_eq!(
        overridden.budget_override_until.as_deref(),
        Some(FIVE_HOUR_RESET_STORED)
    );
    assert_eq!(overridden.skipped, NONE_SKIPPED);
    wrapper.stop();
}
