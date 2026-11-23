//! The classified snapshot read and the log replay on a real directory (test-plan §4 viola-state;
//! architecture §Standard Contracts, Snapshot envelope). The broken snapshots are the writer's own
//! file, shortened or with its `v` rewritten; the replay reads and writes nothing.

use std::fs::{self, OpenOptions};
use std::path::Path;
use std::time::SystemTime;

use chrono::{TimeZone as _, Utc};
use serde_json::{Value, json};
use viola_core::{EventKind, ViolaName};
use viola_state::events::{EventLine, Skipped, Source, append_event, read_from};
use viola_state::replay::{
    Recovered, ReplayCause, Replayed, SessionChain, SessionLink, read_snapshot_or_replay, replay,
    session_chain,
};
use viola_state::snapshot::{
    InstanceSnapshot, Launch, Lineage, SnapshotRead, Wheel, read_snapshot_classified,
    write_snapshot,
};

fn snapshot() -> InstanceSnapshot {
    InstanceSnapshot {
        endpoint: Some("viola-test-endpoint".to_owned()),
        pid: 41,
        started_at: "2026-10-10T01:02:03.000Z".to_owned(),
        pinned_bin: "bin/0.1.0-0123456789abcdef/viola".to_owned(),
        cli_verified: true,
        cli_version: Some("2.1.287".to_owned()),
        wheel: Wheel::Driver,
        links: Vec::new(),
        child_pid: Some(42),
        pending_dialog: None,
        cwd: Some("work/project".to_owned()),
        agent_session_id: None,
        lineage: None,
        launch: None,
        window: None,
    }
}

/// The recorded directory a recovery yields: a snapshot's own, and none from a replay.
fn cwd_of(recovered: &Recovered) -> Option<&str> {
    match recovered {
        Recovered::Snapshot(snapshot) => snapshot.cwd.as_deref(),
        Recovered::Absent | Recovered::Replayed { .. } => None,
    }
}

/// A snapshot, and a log holding a wheel taken by the human and two session starts, all laid down
/// by the crate's writers.
fn written() -> tempfile::TempDir {
    let tmp = tempfile::tempdir().expect("tempdir");
    write_snapshot(tmp.path(), &snapshot()).expect("snapshot");
    let name = ViolaName::try_new("builder".to_owned()).expect("valid");
    let lines: [(EventKind, Value); 4] = [
        (
            EventKind::Wheel,
            json!({"holder": "driver", "cause": "start"}),
        ),
        (
            EventKind::SessionStart,
            json!({"cause": "startup", "agent_session_id": "s-first"}),
        ),
        (
            EventKind::Wheel,
            json!({"holder": "human", "cause": "human-input"}),
        ),
        (
            EventKind::SessionStart,
            json!({"cause": "clear", "agent_session_id": "s-second"}),
        ),
    ];
    for (kind, data) in lines {
        let line = EventLine::new(&name, kind, Source::Wrapper, data, Utc::now());
        append_event(tmp.path(), &line).expect("append");
    }
    tmp
}

/// What the log of `written` replays to.
fn replayed() -> Replayed {
    Replayed {
        wheel: Some(Wheel::Human),
        agent_session_id: Some("s-second".to_owned()),
        links: Vec::new(),
        dialog_pending: false,
        skipped: Skipped {
            unknown_kinds: 0,
            unknown_fields: 0,
            torn_lines: 0,
        },
        lineage: None,
        launch: None,
        window: None,
    }
}

/// `snapshot.json`'s bytes and modification time, and the names of every file beside it.
fn on_disk(dir: &Path) -> (Vec<u8>, SystemTime, Vec<String>) {
    let path = dir.join("snapshot.json");
    let modified = fs::metadata(&path)
        .and_then(|m| m.modified())
        .expect("modified");
    let mut names: Vec<String> = fs::read_dir(dir)
        .expect("the dir")
        .map(|entry| {
            entry
                .expect("an entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    names.sort_unstable();
    (fs::read(&path).expect("the snapshot"), modified, names)
}

#[test]
fn state_replay_the_written_snapshot_reads_present_and_is_returned() {
    let tmp = written();
    assert_eq!(
        read_snapshot_classified(tmp.path()),
        SnapshotRead::Present(Box::new(snapshot()))
    );
    assert_eq!(
        read_snapshot_or_replay(tmp.path()).expect("recovered"),
        Recovered::Snapshot(Box::new(snapshot()))
    );
}

#[test]
fn state_replay_a_snapshot_cut_short_is_replayed_from_the_log_and_left_untouched() {
    let tmp = written();
    let path = tmp.path().join("snapshot.json");
    let whole = fs::metadata(&path).expect("the snapshot").len();
    OpenOptions::new()
        .write(true)
        .open(&path)
        .expect("the snapshot")
        .set_len(whole / 2)
        .expect("shorten");
    assert_eq!(
        read_snapshot_classified(tmp.path()),
        SnapshotRead::Unreadable
    );
    let before = on_disk(tmp.path());

    assert_eq!(
        read_snapshot_or_replay(tmp.path()).expect("recovered"),
        Recovered::Replayed {
            cause: ReplayCause::Unreadable,
            state: Box::new(replayed()),
        }
    );
    assert_eq!(on_disk(tmp.path()), before);
    assert_eq!(
        before.2,
        [
            "events.ndjson",
            "events.ndjson.lock",
            "snapshot.json",
            "snapshot.json.lock"
        ]
    );
}

/// Two starts' `lineage` lines laid down by the crate's own writers, the snapshot mirroring the
/// last; cut short, the replay returns that last line's lineage, not the first.
#[test]
fn state_replay_a_snapshot_cut_short_is_replayed_with_the_logged_lineage() {
    let tmp = written();
    let name = ViolaName::try_new("builder".to_owned()).expect("valid");
    for data in [
        json!({"parent": "tester", "role": null}),
        json!({"parent": "overseer", "role": "builder"}),
    ] {
        let line = EventLine::new(&name, EventKind::Lineage, Source::Wrapper, data, Utc::now());
        append_event(tmp.path(), &line).expect("append");
    }
    let last = Lineage {
        parent: Some("overseer".to_owned()),
        role: Some("builder".to_owned()),
    };
    let mirrored = InstanceSnapshot {
        lineage: Some(last.clone()),
        ..snapshot()
    };
    write_snapshot(tmp.path(), &mirrored).expect("snapshot");
    assert_eq!(
        read_snapshot_or_replay(tmp.path()).expect("recovered"),
        Recovered::Snapshot(Box::new(mirrored))
    );
    let path = tmp.path().join("snapshot.json");
    let whole = fs::metadata(&path).expect("the snapshot").len();
    OpenOptions::new()
        .write(true)
        .open(&path)
        .expect("the snapshot")
        .set_len(whole / 2)
        .expect("shorten");

    assert_eq!(
        read_snapshot_or_replay(tmp.path()).expect("recovered"),
        Recovered::Replayed {
            cause: ReplayCause::Unreadable,
            state: Box::new(Replayed {
                lineage: Some(last),
                ..replayed()
            }),
        }
    );
}

/// Two starts' `launch` lines laid down by the crate's own writer, the snapshot mirroring the
/// last; cut short, the replay returns that last line's pair, not the first.
#[test]
fn state_replay_a_snapshot_cut_short_is_replayed_with_the_logged_launch() {
    let tmp = written();
    let name = ViolaName::try_new("builder".to_owned()).expect("valid");
    for data in [
        json!({"model": "haiku", "effort": "low"}),
        json!({"model": "claude-opus-5-5", "effort": "xhigh"}),
    ] {
        let line = EventLine::new(&name, EventKind::Launch, Source::Wrapper, data, Utc::now());
        append_event(tmp.path(), &line).expect("append");
    }
    let last = Launch {
        model: "claude-opus-5-5".to_owned(),
        effort: "xhigh".to_owned(),
    };
    let mirrored = InstanceSnapshot {
        launch: Some(last.clone()),
        window: None,
        ..snapshot()
    };
    write_snapshot(tmp.path(), &mirrored).expect("snapshot");
    assert_eq!(
        read_snapshot_or_replay(tmp.path()).expect("recovered"),
        Recovered::Snapshot(Box::new(mirrored))
    );
    let path = tmp.path().join("snapshot.json");
    let whole = fs::metadata(&path).expect("the snapshot").len();
    OpenOptions::new()
        .write(true)
        .open(&path)
        .expect("the snapshot")
        .set_len(whole / 2)
        .expect("shorten");

    assert_eq!(
        read_snapshot_or_replay(tmp.path()).expect("recovered"),
        Recovered::Replayed {
            cause: ReplayCause::Unreadable,
            state: Box::new(Replayed {
                launch: Some(last),
                window: None,
                ..replayed()
            }),
        }
    );
}

#[test]
fn state_replay_a_snapshot_of_v_99_is_replayed_with_the_v_it_saw_and_left_untouched() {
    let tmp = written();
    let path = tmp.path().join("snapshot.json");
    let text = fs::read_to_string(&path).expect("the snapshot");
    assert!(text.starts_with("{\"v\":1,"), "{text}");
    fs::write(&path, text.replacen("{\"v\":1,", "{\"v\":99,", 1)).expect("rewrite v");
    assert_eq!(
        read_snapshot_classified(tmp.path()),
        SnapshotRead::Unsupported { v_seen: 99 }
    );
    let before = on_disk(tmp.path());

    assert_eq!(
        read_snapshot_or_replay(tmp.path()).expect("recovered"),
        Recovered::Replayed {
            cause: ReplayCause::Unsupported { v_seen: 99 },
            state: Box::new(replayed()),
        }
    );
    assert_eq!(on_disk(tmp.path()), before);
}

/// The recorded directory lives in the snapshot alone: a replay over a snapshot cut short yields
/// none, and writes no file to make one.
#[test]
fn state_replay_a_snapshot_cut_short_yields_no_cwd_and_is_left_untouched() {
    let tmp = written();
    let present = read_snapshot_or_replay(tmp.path()).expect("recovered");
    assert_eq!(cwd_of(&present), Some("work/project"));
    let path = tmp.path().join("snapshot.json");
    let whole = fs::metadata(&path).expect("the snapshot").len();
    OpenOptions::new()
        .write(true)
        .open(&path)
        .expect("the snapshot")
        .set_len(whole / 2)
        .expect("shorten");
    let before = on_disk(tmp.path());

    let replayed = read_snapshot_or_replay(tmp.path()).expect("recovered");
    assert!(
        matches!(replayed, Recovered::Replayed { .. }),
        "{replayed:?}"
    );
    assert_eq!(cwd_of(&replayed), None);
    assert_eq!(on_disk(tmp.path()), before);
}

#[test]
fn state_replay_the_session_chain_of_a_two_session_log_is_both_in_order() {
    let tmp = written();
    let before = on_disk(tmp.path());
    let SessionChain { links, skipped } = session_chain(tmp.path()).expect("chain");
    let read: Vec<(&str, &str)> = links
        .iter()
        .map(
            |SessionLink {
                 cause,
                 agent_session_id,
                 ..
             }| (cause.as_str(), agent_session_id.as_str()),
        )
        .collect();
    assert_eq!(read, [("startup", "s-first"), ("clear", "s-second")]);
    assert!(
        links
            .iter()
            .all(|link| link.ts.len() == 24 && link.ts.ends_with('Z')),
        "{links:?}"
    );
    assert_eq!(skipped, replayed().skipped);
    assert_eq!(on_disk(tmp.path()), before);
}

const NONE_SKIPPED: Skipped = Skipped {
    unknown_kinds: 0,
    unknown_fields: 0,
    torn_lines: 0,
};

/// One `link` or `unlink` line of `overseer -> builder`, appended at `second` past a fixed minute;
/// its `ts` as the writer spells it.
fn append_pair(dir: &Path, kind: EventKind, second: u32) -> String {
    let name = ViolaName::try_new("builder".to_owned()).expect("valid");
    let at = Utc
        .with_ymd_and_hms(2026, 10, 10, 1, 2, second)
        .single()
        .expect("valid date");
    let data = json!({"driver": "overseer", "driven": "builder"});
    let line = EventLine::new(&name, kind, Source::Wrapper, data, at);
    append_event(dir, &line).expect("append");
    line.ts
}

/// The link set a pass over `dir`'s log derives, after reading that it stepped over nothing.
fn replayed_links(dir: &Path) -> Vec<Value> {
    let replayed = replay(dir).expect("replay");
    assert_eq!(replayed.skipped, NONE_SKIPPED);
    replayed.links
}

/// A pair's `since` is the `ts` of its first `link` line after any `unlink` of it: a repeated
/// `link` leaves it, and a `link` after an `unlink` starts it again.
#[test]
fn state_replay_links_hold_the_first_since_until_an_unlink_and_a_later_one_after_it() {
    let tmp = tempfile::tempdir().expect("tempdir");
    assert_eq!(replayed_links(tmp.path()), Vec::<Value>::new());
    let first = append_pair(tmp.path(), EventKind::Link, 3);
    assert_eq!(first, "2026-10-10T01:02:03.000Z");
    let linked = [json!({"driver": "overseer", "driven": "builder", "since": first})];
    assert_eq!(replayed_links(tmp.path()), linked);
    append_pair(tmp.path(), EventKind::Link, 4);
    assert_eq!(replayed_links(tmp.path()), linked);
    append_pair(tmp.path(), EventKind::Unlink, 5);
    assert_eq!(replayed_links(tmp.path()), Vec::<Value>::new());
    let again = append_pair(tmp.path(), EventKind::Link, 6);
    assert_eq!(again, "2026-10-10T01:02:06.000Z");
    assert_eq!(
        replayed_links(tmp.path()),
        [json!({"driver": "overseer", "driven": "builder", "since": again})]
    );
}

/// The link set rides beside the other recovered fields, and a replay over a snapshot cut short
/// carries it.
#[test]
fn state_replay_a_snapshot_cut_short_is_replayed_with_the_logged_link() {
    let tmp = written();
    let since = append_pair(tmp.path(), EventKind::Link, 3);
    let path = tmp.path().join("snapshot.json");
    let whole = fs::metadata(&path).expect("the snapshot").len();
    OpenOptions::new()
        .write(true)
        .open(&path)
        .expect("the snapshot")
        .set_len(whole / 2)
        .expect("shorten");
    let before = on_disk(tmp.path());
    assert_eq!(
        read_snapshot_or_replay(tmp.path()).expect("recovered"),
        Recovered::Replayed {
            cause: ReplayCause::Unreadable,
            state: Box::new(Replayed {
                links: vec![json!({"driver": "overseer", "driven": "builder", "since": since})],
                ..replayed()
            }),
        }
    );
    assert_eq!(on_disk(tmp.path()), before);
}

/// The 0.1.0 shapes a budget-era wrapper wrote, kept as fixtures outside the product tree, and
/// the words that era used which no build writes now.
const BUDGET_ERA_SNAPSHOT: &str = include_str!("../../../fixtures/viola-0.1.0/snapshot.json");
const BUDGET_ERA_LOG: &str = include_str!("../../../fixtures/viola-0.1.0/events.ndjson");
const RETIRED_WORDS: &str = include_str!("../../../fixtures/viola-0.1.0/retired-words.json");

fn retired_words() -> Value {
    serde_json::from_str(RETIRED_WORDS).expect("the retired words")
}

/// A snapshot a 0.1.0 wrapper wrote, its budget fields standing, reads as a present snapshot: the
/// fields this build does not know are skipped and every other field reads as written.
#[test]
fn snapshot_of_a_budget_era_writer_reads_whole_skipping_its_budget_fields() {
    let written: Value = serde_json::from_str(BUDGET_ERA_SNAPSHOT).expect("the fixture");
    for key in retired_words()["snapshot_keys"]
        .as_array()
        .expect("the keys")
    {
        let key = key.as_str().expect("a key");
        assert!(written["data"].get(key).is_some(), "{key}");
    }
    let tmp = tempfile::tempdir().expect("tempdir");
    fs::write(tmp.path().join("snapshot.json"), BUDGET_ERA_SNAPSHOT).expect("seed");
    assert_eq!(
        read_snapshot_classified(tmp.path()),
        SnapshotRead::Present(Box::new(InstanceSnapshot {
            endpoint: None,
            pid: 7,
            started_at: "2026-11-01T09:59:59.000Z".to_owned(),
            pinned_bin: "bin/0.1.0-0123456789abcdef/viola".to_owned(),
            cli_verified: true,
            cli_version: Some("2.1.302".to_owned()),
            wheel: Wheel::Human,
            links: vec![
                json!({"driver": "overseer", "driven": "builder", "since": "2026-11-01T10:00:01.000Z"})
            ],
            child_pid: None,
            pending_dialog: None,
            cwd: None,
            agent_session_id: Some("0d6f1c52-3b7e-4a86-9a52-2f8b1c3d4e5f".to_owned()),
            lineage: None,
            launch: None,
            window: None,
        }))
    );
}

/// A 0.1.0 log under a snapshot that cannot be read replays to its wheel and its link: the gate
/// line is an unknown kind, counted once and never returned, and nothing of it is recovered.
#[test]
fn replay_over_a_budget_era_log_skips_its_gate_lines() {
    let gate = retired_words()["event_kind"]
        .as_str()
        .expect("the kind")
        .to_owned();
    assert!(BUDGET_ERA_LOG.contains(&format!("\"kind\":\"{gate}\"")));
    let tmp = tempfile::tempdir().expect("tempdir");
    fs::write(tmp.path().join("events.ndjson"), BUDGET_ERA_LOG).expect("seed the log");
    fs::write(tmp.path().join("snapshot.json"), "{\"v\":1,\"writ").expect("seed the snapshot");

    assert_eq!(
        read_snapshot_or_replay(tmp.path()).expect("recovered"),
        Recovered::Replayed {
            cause: ReplayCause::Unreadable,
            state: Box::new(Replayed {
                wheel: Some(Wheel::Human),
                agent_session_id: None,
                links: vec![
                    json!({"driver": "overseer", "driven": "builder", "since": "2026-11-01T10:00:02.000Z"})
                ],
                dialog_pending: false,
                skipped: Skipped {
                    unknown_kinds: 1,
                    unknown_fields: 0,
                    torn_lines: 0,
                },
                lineage: None,
                launch: None,
                window: None,
            }),
        }
    );

    let mut lines = read_from(tmp.path(), 0).expect("the log");
    let kinds: Vec<String> = lines
        .by_ref()
        .map(|line| {
            line.expect("a line").value["kind"]
                .as_str()
                .expect("a kind")
                .to_owned()
        })
        .collect();
    assert_eq!(kinds, ["wheel", "link", "wheel"]);
    assert_eq!(
        lines.skipped(),
        Skipped {
            unknown_kinds: 1,
            unknown_fields: 0,
            torn_lines: 0,
        }
    );
}
