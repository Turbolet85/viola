//! The classified snapshot read and the log replay on a real directory (test-plan §4 viola-state;
//! architecture §Standard Contracts, Snapshot envelope). The broken snapshots are the writer's own
//! file, shortened or with its `v` rewritten; the replay reads and writes nothing.

use std::fs::{self, OpenOptions};
use std::path::Path;
use std::time::SystemTime;

use chrono::Utc;
use serde_json::{Value, json};
use viola_core::{EventKind, ViolaName};
use viola_state::events::{EventLine, Skipped, Source, append_event};
use viola_state::replay::{
    Recovered, ReplayCause, Replayed, SessionChain, SessionLink, read_snapshot_or_replay,
    session_chain,
};
use viola_state::snapshot::{
    InstanceSnapshot, SnapshotRead, Wheel, read_snapshot_classified, write_snapshot,
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
        budget_paused: false,
        links: Vec::new(),
        child_pid: Some(42),
        pending_dialog: None,
        cwd: Some("work/project".to_owned()),
        statusline_command: Some("printf status".to_owned()),
    }
}

/// The recorded directory a recovery yields: a snapshot's own, and none from a replay.
fn cwd_of(recovered: &Recovered) -> Option<&str> {
    match recovered {
        Recovered::Snapshot(snapshot) => snapshot.cwd.as_deref(),
        Recovered::Absent | Recovered::Replayed { .. } => None,
    }
}

/// The recorded statusline command a recovery yields: a snapshot's own, and none from a replay.
fn statusline_command_of(recovered: &Recovered) -> Option<&str> {
    match recovered {
        Recovered::Snapshot(snapshot) => snapshot.statusline_command.as_deref(),
        Recovered::Absent | Recovered::Replayed { .. } => None,
    }
}

/// A snapshot, and a log holding a wheel taken by the human, a budget pause and two session
/// starts, all laid down by the crate's writers.
fn written() -> tempfile::TempDir {
    let tmp = tempfile::tempdir().expect("tempdir");
    write_snapshot(tmp.path(), &snapshot()).expect("snapshot");
    let name = ViolaName::try_new("builder".to_owned()).expect("valid");
    let lines: [(EventKind, Value); 5] = [
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
        (EventKind::BudgetGate, json!({"paused": true})),
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
        budget_paused: Some(true),
        budget_override_until: None,
        agent_session_id: Some("s-second".to_owned()),
        links: Vec::new(),
        dialog_pending: false,
        skipped: Skipped {
            unknown_kinds: 0,
            unknown_fields: 0,
            torn_lines: 0,
        },
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
            state: replayed(),
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
            state: replayed(),
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
    assert_eq!(statusline_command_of(&present), Some("printf status"));
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
    assert_eq!(statusline_command_of(&replayed), None);
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
