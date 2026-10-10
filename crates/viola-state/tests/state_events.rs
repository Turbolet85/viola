//! The event log on a real directory, through the crate's public writer and reader (test-plan §4
//! viola-state; §5 Cross-module patterns → On-disk): every kind round-trips the six-key line, the
//! reader counts what it steps over, and a log whose last line was cut short takes the next append
//! on a fresh line.

use std::fs::{self, OpenOptions};
use std::io::Write as _;
use std::path::Path;

use chrono::Utc;
use serde_json::{Value, json};
use viola_core::{EventKind, ViolaName};
use viola_state::events::{EventLine, LoggedLine, Skipped, Source, append_event, read_from};

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

fn append(dir: &Path, kind: EventKind, data: Value) {
    let name = ViolaName::try_new("builder".to_owned()).expect("valid");
    let line = EventLine::new(&name, kind, Source::Wrapper, data, Utc::now());
    append_event(dir, &line).expect("append");
}

/// The lines a read from `after` returns, and what it stepped over.
fn read(dir: &Path, after: u64) -> (Vec<LoggedLine>, Skipped) {
    let mut lines = read_from(dir, after).expect("the log opens");
    let read = lines.by_ref().map(|l| l.expect("a line")).collect();
    (read, lines.skipped())
}

fn kinds(lines: &[LoggedLine]) -> Vec<&str> {
    lines
        .iter()
        .map(|l| l.value["kind"].as_str().expect("kind"))
        .collect()
}

fn log(dir: &Path) -> Vec<u8> {
    fs::read(dir.join("events.ndjson")).expect("the log")
}

fn offset(n: usize) -> u64 {
    u64::try_from(n).expect("offset")
}

/// Architecture's per-kind `data` list, one literal body per kind holding every key of its row.
fn every_kind() -> [(EventKind, &'static str, Value); 13] {
    [
        (
            EventKind::SessionStart,
            "session-start",
            json!({"cause": "startup", "agent_session_id": "s-1"}),
        ),
        (
            EventKind::PromptSubmitted,
            "prompt-submitted",
            json!({"text": "line one\nline two", "origin": "driver"}),
        ),
        (
            EventKind::TurnEnded,
            "turn-ended",
            json!({"last_assistant_message": "done"}),
        ),
        (EventKind::SessionEnd, "session-end", json!({})),
        (EventKind::Activity, "activity", json!({"tool": "Bash"})),
        (
            EventKind::Wheel,
            "wheel",
            json!({"holder": "human", "cause": "human-input"}),
        ),
        (
            EventKind::BudgetGate,
            "budget-gate",
            json!({"paused": true, "window": "seven-day", "override_until": "2026-09-27T02:00:00.000Z"}),
        ),
        (
            EventKind::SendIssued,
            "send-issued",
            json!({"cursor": 512, "from": "overseer"}),
        ),
        (
            EventKind::SendConfirmed,
            "send-confirmed",
            json!({"cursor": 512, "confirmed": false}),
        ),
        (
            EventKind::SendRefused,
            "send-refused",
            json!({"refusal": "not-delivered", "detail": "turn-running", "cursor": 512}),
        ),
        (
            EventKind::Question,
            "question",
            json!({"dialog_id": 1, "questions": [{"question": "Which?", "options": ["a", "b"], "multi_select": false}]}),
        ),
        (
            EventKind::Permission,
            "permission",
            json!({"dialog_id": 2, "tool": "Bash", "input": {"command": "ls"}}),
        ),
        (
            EventKind::Plan,
            "plan",
            json!({"dialog_id": 3, "plan": "step one"}),
        ),
    ]
}

#[test]
fn state_events_every_kind_round_trips_the_six_key_line() {
    let tmp = tempfile::tempdir().expect("tempdir");
    for (kind, _, data) in every_kind() {
        append(tmp.path(), kind, data);
    }
    let bytes = log(tmp.path());
    let (lines, skipped) = read(tmp.path(), 0);
    assert_eq!(lines.len(), 13);
    for (line, (_, name, data)) in lines.iter().zip(every_kind()) {
        let keys: Vec<&str> = line
            .value
            .as_object()
            .expect("an object")
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(
            keys,
            ["v", "ts", "instance", "kind", "source", "data"],
            "{name}"
        );
        assert_eq!(line.value["v"], 1, "{name}");
        let ts = line.value["ts"].as_str().expect("ts");
        assert!(ts.len() == 24 && ts.ends_with('Z'), "{name}: {ts}");
        assert_eq!(line.value["instance"], "builder", "{name}");
        assert_eq!(line.value["kind"], name);
        assert_eq!(line.value["source"], "wrapper", "{name}");
        assert_eq!(line.value["data"], data, "{name}");
        let (start, end) = (
            usize::try_from(line.start).expect("start"),
            usize::try_from(line.end).expect("end"),
        );
        let written = format!(
            "{{\"v\":1,\"ts\":\"{ts}\",\"instance\":\"builder\",\"kind\":\"{name}\",\"source\":\"wrapper\",\"data\":{data}}}\n"
        );
        assert_eq!(&bytes[start..end], written.as_bytes(), "{name}");
    }
    assert_eq!(lines.last().expect("a last line").end, offset(bytes.len()));
    assert_eq!(skipped, NONE_SKIPPED);
}

/// The writer's own lines, then four a newer or a broken writer could leave: a kind this build
/// does not know, a key beside the six, a `data` key outside its kind's row, and a line that is no
/// object.
#[test]
fn state_events_lines_outside_the_contract_are_counted_and_known_kinds_still_returned() {
    let tmp = tempfile::tempdir().expect("tempdir");
    append(
        tmp.path(),
        EventKind::Wheel,
        json!({"holder": "driver", "cause": "start"}),
    );
    append(tmp.path(), EventKind::BudgetGate, json!({"paused": false}));
    let raw = [
        r#"{"v":1,"ts":"t","instance":"builder","kind":"link","source":"wrapper","data":{"driver":"a","driven":"b"}}"#,
        r#"{"v":1,"ts":"t","instance":"builder","kind":"wheel","source":"wrapper","data":{"holder":"human","cause":"release"},"later":true}"#,
        r#"{"v":1,"ts":"t","instance":"builder","kind":"activity","source":"hook","data":{"tool":"Bash","later":true}}"#,
        r#""a string, not an object""#,
    ]
    .map(|line| format!("{line}\n"))
    .concat();
    OpenOptions::new()
        .append(true)
        .open(tmp.path().join("events.ndjson"))
        .expect("the log")
        .write_all(raw.as_bytes())
        .expect("append");

    let (lines, skipped) = read(tmp.path(), 0);
    assert_eq!(kinds(&lines), ["wheel", "budget-gate", "wheel", "activity"]);
    assert_eq!(
        skipped,
        Skipped {
            unknown_kinds: 1,
            unknown_fields: 2,
            torn_lines: 1,
        }
    );
}

#[test]
fn state_events_a_log_cut_short_takes_the_next_append_on_a_fresh_line() {
    let tmp = tempfile::tempdir().expect("tempdir");
    append(
        tmp.path(),
        EventKind::Wheel,
        json!({"holder": "driver", "cause": "start"}),
    );
    append(tmp.path(), EventKind::BudgetGate, json!({"paused": false}));
    append(
        tmp.path(),
        EventKind::TurnEnded,
        json!({"last_assistant_message": "done"}),
    );
    let (whole, skipped) = read(tmp.path(), 0);
    assert_eq!(kinds(&whole), ["wheel", "budget-gate", "turn-ended"]);
    assert_eq!(skipped, NONE_SKIPPED);
    let second = whole[1].clone();

    let cut = whole[2].end - 5;
    OpenOptions::new()
        .write(true)
        .open(tmp.path().join("events.ndjson"))
        .expect("the log")
        .set_len(cut)
        .expect("shorten");
    let torn = log(tmp.path());
    assert_eq!(offset(torn.len()), cut);
    assert_ne!(torn.last(), Some(&b'\n'));
    let (kept, skipped) = read(tmp.path(), 0);
    assert_eq!(kinds(&kept), ["wheel", "budget-gate"]);
    assert_eq!(skipped, ONE_TORN);

    append(tmp.path(), EventKind::SessionEnd, json!({}));
    let healed = log(tmp.path());
    assert_eq!(&healed[..torn.len()], torn);
    assert_eq!(healed[torn.len()], b'\n');
    let (lines, skipped) = read(tmp.path(), 0);
    assert_eq!(kinds(&lines), ["wheel", "budget-gate", "session-end"]);
    assert_eq!(skipped, ONE_TORN);
    assert_eq!(lines[2].start, cut + 1);
    assert_eq!(lines[2].end, offset(healed.len()));
    assert_eq!(lines[1], second);
    let (from_second, _) = read(tmp.path(), second.start);
    assert_eq!(from_second[0], second);
    let (from_the_cut, _) = read(tmp.path(), cut);
    assert_eq!(from_the_cut, [lines[2].clone()]);
}
