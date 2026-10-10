//! The log replay (architecture §Standard Contracts, Snapshot envelope): when `snapshot.json` is
//! unreadable or a newer build's, the state `events.ndjson` holds is read back from it. Read-only:
//! the instance's wrapper stays the snapshot's only writer.

use std::path::Path;

use serde_json::Value;
use viola_core::obs::ObsEvent;
use viola_core::{EventKind, obs_event};

use crate::StateError;
use crate::events::{Skipped, read_from};
use crate::snapshot::{InstanceSnapshot, SNAPSHOT, SnapshotRead, Wheel, read_snapshot_classified};

/// What one pass over the log recovers. A field no line gave is `None`, never a default. `links`
/// is derived from the link kinds alone, and a pending dialog is never rebuilt from the log.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Replayed {
    pub wheel: Option<Wheel>,
    pub budget_paused: Option<bool>,
    pub budget_override_until: Option<String>,
    pub agent_session_id: Option<String>,
    pub links: Vec<Value>,
    pub dialog_pending: bool,
    /// What the pass stepped over.
    pub skipped: Skipped,
}

/// Why the log was replayed in place of the snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayCause {
    Unreadable,
    Unsupported { v_seen: u64 },
}

/// An instance's state as [`read_snapshot_or_replay`] found it.
#[derive(Debug, Clone, PartialEq)]
pub enum Recovered {
    Snapshot(Box<InstanceSnapshot>),
    /// No snapshot: nothing is replayed.
    Absent,
    Replayed {
        cause: ReplayCause,
        state: Replayed,
    },
}

/// One pass over the whole log: `wheel` and the budget gate from their last line, the session id
/// from the last `session-start` that carries one.
#[tracing::instrument(skip_all, name = "state.replay")]
pub fn replay(instance_dir: &Path) -> Result<Replayed, StateError> {
    let mut replayed = Replayed::default();
    let mut lines = read_from(instance_dir, 0)?;
    for line in lines.by_ref() {
        let line = line?;
        let data = &line.value["data"];
        match line.value["kind"].as_str().and_then(EventKind::from_name) {
            Some(EventKind::Wheel) => replayed.wheel = holder(&data["holder"]),
            Some(EventKind::BudgetGate) => {
                replayed.budget_paused = data["paused"].as_bool();
                replayed.budget_override_until = data["override_until"].as_str().map(str::to_owned);
            }
            Some(EventKind::SessionStart) => {
                if let Some(id) = data["agent_session_id"].as_str() {
                    replayed.agent_session_id = Some(id.to_owned());
                }
            }
            _ => {}
        }
    }
    replayed.skipped = lines.skipped();
    Ok(replayed)
}

/// One logged session: a `session-start` line that carries an id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionLink {
    pub ts: String,
    /// The cause as logged; `unknown` when the line holds none.
    pub cause: String,
    pub agent_session_id: String,
}

/// The logged sessions in log order, and what the pass stepped over.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SessionChain {
    pub links: Vec<SessionLink>,
    pub skipped: Skipped,
}

/// One pass over the whole log: a link per `session-start` line whose id is a string. Read-only,
/// and it logs nothing.
pub fn session_chain(instance_dir: &Path) -> Result<SessionChain, StateError> {
    let mut chain = SessionChain::default();
    let mut lines = read_from(instance_dir, 0)?;
    for line in lines.by_ref() {
        let line = line?;
        if line.value["kind"].as_str() != Some(EventKind::SessionStart.as_str()) {
            continue;
        }
        let data = &line.value["data"];
        if let Some(id) = data["agent_session_id"].as_str() {
            chain.links.push(SessionLink {
                ts: line.value["ts"].as_str().unwrap_or("unknown").to_owned(),
                cause: data["cause"].as_str().unwrap_or("unknown").to_owned(),
                agent_session_id: id.to_owned(),
            });
        }
    }
    chain.skipped = lines.skipped();
    Ok(chain)
}

fn holder(value: &Value) -> Option<Wheel> {
    [Wheel::Driver, Wheel::Human]
        .into_iter()
        .find(|wheel| value.as_str() == Some(wheel.as_str()))
}

/// The snapshot when it reads; the replay, with one `state-recovered` line, when it is unreadable
/// or a newer build's. An absent snapshot is neither.
#[tracing::instrument(skip_all, name = "state.snapshot_recover")]
pub fn read_snapshot_or_replay(instance_dir: &Path) -> Result<Recovered, StateError> {
    let cause = match read_snapshot_classified(instance_dir) {
        SnapshotRead::Present(snapshot) => return Ok(Recovered::Snapshot(snapshot)),
        SnapshotRead::Absent => return Ok(Recovered::Absent),
        SnapshotRead::Unreadable => ReplayCause::Unreadable,
        SnapshotRead::Unsupported { v_seen } => ReplayCause::Unsupported { v_seen },
    };
    let state = replay(instance_dir)?;
    let (detail, v_seen) = match cause {
        ReplayCause::Unreadable => ("snapshot-replayed", None),
        ReplayCause::Unsupported { v_seen } => ("snapshot-unsupported-v", Some(v_seen)),
    };
    obs_event!(
        WARN,
        ObsEvent::StateRecovered,
        detail = detail,
        file = SNAPSHOT,
        v_seen = v_seen,
    );
    Ok(Recovered::Replayed { cause, state })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::{EventLine, Source, append_event};
    use crate::snapshot::write_snapshot;
    use crate::test_capture::capture;
    use chrono::{TimeZone as _, Utc};
    use serde_json::json;
    use std::fs;
    use viola_core::ViolaName;

    const NONE_SKIPPED: Skipped = Skipped {
        unknown_kinds: 0,
        unknown_fields: 0,
        torn_lines: 0,
    };

    fn log(dir: &Path, lines: &[(EventKind, Value)]) {
        let name = ViolaName::try_new("builder".to_owned()).expect("valid");
        let at = Utc
            .with_ymd_and_hms(2026, 9, 27, 1, 2, 3)
            .single()
            .expect("valid date");
        for (kind, data) in lines {
            let line = EventLine::new(&name, *kind, Source::Wrapper, data.clone(), at);
            append_event(dir, &line).expect("append");
        }
    }

    fn snapshot() -> InstanceSnapshot {
        InstanceSnapshot {
            endpoint: None,
            pid: 41,
            started_at: "2026-09-27T01:02:03.000Z".to_owned(),
            pinned_bin: "C:/h/bin/0.1.0-0123456789abcdef/viola.exe".to_owned(),
            cli_verified: false,
            cli_version: None,
            wheel: Wheel::Driver,
            budget_paused: false,
            links: Vec::new(),
            child_pid: None,
            pending_dialog: None,
            cwd: None,
            statusline_command: None,
        }
    }

    /// A start, then a human key, a budget pause with its override and a `/clear`.
    fn full_log(dir: &Path) {
        log(
            dir,
            &[
                (
                    EventKind::Wheel,
                    json!({"holder": "driver", "cause": "start"}),
                ),
                (EventKind::BudgetGate, json!({"paused": false})),
                (
                    EventKind::SessionStart,
                    json!({"cause": "startup", "agent_session_id": "s-1"}),
                ),
                (
                    EventKind::Wheel,
                    json!({"holder": "human", "cause": "human-input"}),
                ),
                (
                    EventKind::BudgetGate,
                    json!({"paused": true, "window": "five-hour", "override_until": "2026-09-27T02:00:00.000Z"}),
                ),
                (
                    EventKind::SessionStart,
                    json!({"cause": "clear", "agent_session_id": "s-2"}),
                ),
                (
                    EventKind::TurnEnded,
                    json!({"last_assistant_message": null}),
                ),
            ],
        );
    }

    fn full_state() -> Replayed {
        Replayed {
            wheel: Some(Wheel::Human),
            budget_paused: Some(true),
            budget_override_until: Some("2026-09-27T02:00:00.000Z".to_owned()),
            agent_session_id: Some("s-2".to_owned()),
            links: Vec::new(),
            dialog_pending: false,
            skipped: NONE_SKIPPED,
        }
    }

    #[test]
    fn replay_recovers_each_field_from_its_last_line() {
        let tmp = tempfile::tempdir().expect("tempdir");
        full_log(tmp.path());
        assert_eq!(replay(tmp.path()).expect("replay"), full_state());
    }

    #[test]
    fn replay_recovers_the_driver_wheel_and_an_open_gate_from_a_start() {
        let tmp = tempfile::tempdir().expect("tempdir");
        log(
            tmp.path(),
            &[
                (
                    EventKind::Wheel,
                    json!({"holder": "driver", "cause": "start"}),
                ),
                (EventKind::BudgetGate, json!({"paused": false})),
            ],
        );
        assert_eq!(
            replay(tmp.path()).expect("replay"),
            Replayed {
                wheel: Some(Wheel::Driver),
                budget_paused: Some(false),
                budget_override_until: None,
                agent_session_id: None,
                links: Vec::new(),
                dialog_pending: false,
                skipped: NONE_SKIPPED,
            }
        );
    }

    /// The last `wheel` and `budget-gate` lines decide alone: one that holds no usable value
    /// leaves the field absent, and an earlier line's value does not stand in.
    #[test]
    fn replay_recovers_a_field_as_absent_when_its_last_line_holds_no_value() {
        let tmp = tempfile::tempdir().expect("tempdir");
        full_log(tmp.path());
        log(
            tmp.path(),
            &[
                (EventKind::Wheel, json!({"holder": "copilot"})),
                (EventKind::BudgetGate, json!({"paused": "yes"})),
            ],
        );
        assert_eq!(
            replay(tmp.path()).expect("replay"),
            Replayed {
                wheel: None,
                budget_paused: None,
                budget_override_until: None,
                agent_session_id: Some("s-2".to_owned()),
                links: Vec::new(),
                dialog_pending: false,
                skipped: NONE_SKIPPED,
            }
        );
    }

    #[test]
    fn replay_recovers_the_earlier_session_id_when_a_later_start_carries_null() {
        let tmp = tempfile::tempdir().expect("tempdir");
        log(
            tmp.path(),
            &[
                (
                    EventKind::SessionStart,
                    json!({"cause": "startup", "agent_session_id": "s-1"}),
                ),
                (
                    EventKind::SessionStart,
                    json!({"cause": "resume", "agent_session_id": null}),
                ),
            ],
        );
        let replayed = replay(tmp.path()).expect("replay");
        assert_eq!(replayed.agent_session_id.as_deref(), Some("s-1"));
    }

    #[test]
    fn replay_recovers_nothing_from_an_empty_or_an_absent_log() {
        let nothing = Replayed {
            wheel: None,
            budget_paused: None,
            budget_override_until: None,
            agent_session_id: None,
            links: Vec::new(),
            dialog_pending: false,
            skipped: NONE_SKIPPED,
        };
        let tmp = tempfile::tempdir().expect("tempdir");
        assert_eq!(replay(tmp.path()).expect("absent"), nothing);
        fs::write(tmp.path().join("events.ndjson"), b"").expect("seed");
        assert_eq!(replay(tmp.path()).expect("empty"), nothing);
    }

    #[test]
    fn replay_recovers_the_three_counts_of_its_pass() {
        let tmp = tempfile::tempdir().expect("tempdir");
        log(
            tmp.path(),
            &[
                (
                    EventKind::Wheel,
                    json!({"holder": "human", "cause": "human-input", "later": 1}),
                ),
                (EventKind::BudgetGate, json!({"paused": true})),
            ],
        );
        let mut bytes = fs::read(tmp.path().join("events.ndjson")).expect("read");
        bytes.extend_from_slice(b"{\"kind\":\"later-kind\"}\n{\"kind\":\"whe");
        fs::write(tmp.path().join("events.ndjson"), bytes).expect("seed");
        assert_eq!(
            replay(tmp.path()).expect("replay"),
            Replayed {
                wheel: Some(Wheel::Human),
                budget_paused: Some(true),
                budget_override_until: None,
                agent_session_id: None,
                links: Vec::new(),
                dialog_pending: false,
                skipped: Skipped {
                    unknown_kinds: 1,
                    unknown_fields: 1,
                    torn_lines: 1,
                },
            }
        );
    }

    #[test]
    fn replay_recovers_an_unreadable_snapshot_with_one_snapshot_replayed_line() {
        let tmp = tempfile::tempdir().expect("tempdir");
        full_log(tmp.path());
        fs::write(tmp.path().join("snapshot.json"), "{\"v\":1,\"writ").expect("seed");
        let (recovered, lines) = capture(|| read_snapshot_or_replay(tmp.path()));
        assert_eq!(
            recovered.expect("recovered"),
            Recovered::Replayed {
                cause: ReplayCause::Unreadable,
                state: full_state(),
            }
        );
        assert_eq!(
            lines,
            [json!({
                "event": "state-recovered",
                "detail": "snapshot-replayed",
                "file": "snapshot.json",
                "message": "state-recovered",
                "level": "WARN",
            })]
        );
    }

    #[test]
    fn replay_recovers_a_newer_snapshot_with_one_unsupported_v_line() {
        let tmp = tempfile::tempdir().expect("tempdir");
        full_log(tmp.path());
        fs::write(
            tmp.path().join("snapshot.json"),
            r#"{"v":99,"written_at":"x","writer":"9.9.9","data":{"shape":"of a later build"}}"#,
        )
        .expect("seed");
        let (recovered, lines) = capture(|| read_snapshot_or_replay(tmp.path()));
        assert_eq!(
            recovered.expect("recovered"),
            Recovered::Replayed {
                cause: ReplayCause::Unsupported { v_seen: 99 },
                state: full_state(),
            }
        );
        assert_eq!(
            lines,
            [json!({
                "event": "state-recovered",
                "detail": "snapshot-unsupported-v",
                "file": "snapshot.json",
                "v_seen": 99,
                "message": "state-recovered",
                "level": "WARN",
            })]
        );
    }

    #[test]
    fn replay_recovers_nothing_and_writes_no_line_for_a_present_or_an_absent_snapshot() {
        let tmp = tempfile::tempdir().expect("tempdir");
        full_log(tmp.path());
        let (absent, lines) = capture(|| read_snapshot_or_replay(tmp.path()));
        assert_eq!(absent.expect("absent"), Recovered::Absent);
        assert_eq!(lines, Vec::<Value>::new());
        write_snapshot(tmp.path(), &snapshot()).expect("write");
        let (present, lines) = capture(|| read_snapshot_or_replay(tmp.path()));
        assert_eq!(
            present.expect("present"),
            Recovered::Snapshot(Box::new(snapshot()))
        );
        assert_eq!(lines, Vec::<Value>::new());
    }

    /// A log that cannot be read (a directory in its place) fails the replay, and the
    /// read-or-replay with it, which then writes no line.
    #[test]
    fn replay_recovers_an_error_and_no_line_from_a_log_that_cannot_be_read() {
        let tmp = tempfile::tempdir().expect("tempdir");
        fs::create_dir(tmp.path().join("events.ndjson")).expect("a dir in its place");
        assert!(replay(tmp.path()).is_err());
        fs::write(tmp.path().join("snapshot.json"), "not json").expect("seed");
        let (recovered, lines) = capture(|| read_snapshot_or_replay(tmp.path()));
        assert!(recovered.is_err());
        assert_eq!(lines, Vec::<Value>::new());
    }

    fn link(cause: &str, id: &str) -> SessionLink {
        SessionLink {
            ts: "2026-09-27T01:02:03.000Z".to_owned(),
            cause: cause.to_owned(),
            agent_session_id: id.to_owned(),
        }
    }

    #[test]
    fn session_chain_of_an_empty_or_an_absent_log_holds_no_link() {
        let tmp = tempfile::tempdir().expect("tempdir");
        assert_eq!(
            session_chain(tmp.path()).expect("absent"),
            SessionChain::default()
        );
        fs::write(tmp.path().join("events.ndjson"), b"").expect("seed");
        assert_eq!(
            session_chain(tmp.path()).expect("empty"),
            SessionChain {
                links: Vec::new(),
                skipped: NONE_SKIPPED,
            }
        );
    }

    /// A start, a `/clear` and a resume of the first session, with the other kinds between them.
    #[test]
    fn session_chain_holds_every_logged_session_in_log_order_with_its_cause() {
        let tmp = tempfile::tempdir().expect("tempdir");
        full_log(tmp.path());
        log(
            tmp.path(),
            &[(
                EventKind::SessionStart,
                json!({"cause": "resume", "agent_session_id": "s-1"}),
            )],
        );
        let (chain, lines) = capture(|| session_chain(tmp.path()));
        assert_eq!(
            chain.expect("chain"),
            SessionChain {
                links: vec![
                    link("startup", "s-1"),
                    link("clear", "s-2"),
                    link("resume", "s-1"),
                ],
                skipped: NONE_SKIPPED,
            }
        );
        assert_eq!(lines, Vec::<Value>::new());
    }

    #[test]
    fn session_chain_holds_no_link_for_a_session_start_whose_id_is_null() {
        let tmp = tempfile::tempdir().expect("tempdir");
        log(
            tmp.path(),
            &[
                (
                    EventKind::SessionStart,
                    json!({"cause": "startup", "agent_session_id": null}),
                ),
                (
                    EventKind::SessionStart,
                    json!({"cause": "resume", "agent_session_id": "s-1"}),
                ),
                (
                    EventKind::SessionStart,
                    json!({"cause": "clear", "agent_session_id": 7}),
                ),
            ],
        );
        assert_eq!(
            session_chain(tmp.path()).expect("chain").links,
            [link("resume", "s-1")]
        );
    }

    /// A line no writer of this build made: no `ts`, and a `cause` that is no string.
    #[test]
    fn session_chain_reads_a_missing_ts_and_a_cause_that_is_no_string_as_unknown() {
        let tmp = tempfile::tempdir().expect("tempdir");
        fs::write(
            tmp.path().join("events.ndjson"),
            concat!(
                r#"{"v":1,"instance":"builder","kind":"session-start","source":"hook","data":{"agent_session_id":"s-9"}}"#,
                "\n",
                r#"{"v":1,"ts":"t","instance":"builder","kind":"session-start","source":"hook","data":{"cause":3,"agent_session_id":"s-10"}}"#,
                "\n",
            ),
        )
        .expect("seed");
        assert_eq!(
            session_chain(tmp.path()).expect("chain").links,
            [
                SessionLink {
                    ts: "unknown".to_owned(),
                    cause: "unknown".to_owned(),
                    agent_session_id: "s-9".to_owned(),
                },
                SessionLink {
                    ts: "t".to_owned(),
                    cause: "unknown".to_owned(),
                    agent_session_id: "s-10".to_owned(),
                },
            ]
        );
    }

    #[test]
    fn session_chain_counts_a_torn_last_line_and_an_unknown_kind_and_links_neither() {
        let tmp = tempfile::tempdir().expect("tempdir");
        log(
            tmp.path(),
            &[(
                EventKind::SessionStart,
                json!({"cause": "startup", "agent_session_id": "s-1"}),
            )],
        );
        let mut bytes = fs::read(tmp.path().join("events.ndjson")).expect("read");
        bytes.extend_from_slice(
            b"{\"kind\":\"later-kind\",\"data\":{\"agent_session_id\":\"s-7\"}}\n{\"kind\":\"session-start\",\"data\":{\"agent_session_id\":\"s-8\"",
        );
        fs::write(tmp.path().join("events.ndjson"), bytes).expect("seed");
        assert_eq!(
            session_chain(tmp.path()).expect("chain"),
            SessionChain {
                links: vec![link("startup", "s-1")],
                skipped: Skipped {
                    unknown_kinds: 1,
                    unknown_fields: 0,
                    torn_lines: 1,
                },
            }
        );
    }

    #[test]
    fn session_chain_of_a_log_that_cannot_be_read_is_an_error() {
        let tmp = tempfile::tempdir().expect("tempdir");
        fs::create_dir(tmp.path().join("events.ndjson")).expect("a dir in its place");
        assert!(session_chain(tmp.path()).is_err());
    }
}
