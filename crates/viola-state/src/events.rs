//! `instances/<name>/events.ndjson`: one compact JSON object + `\n` per `write`, appended under an
//! exclusive lock on the `events.ndjson.lock` sibling, never truncated or rewritten (architecture
//! §Standard Contracts, ndjson event line).

use std::io::Write as _;
use std::path::Path;

use chrono::{DateTime, Utc};
use serde::{Serialize, Serializer};
use serde_json::Value;
use viola_core::{EventKind, ViolaName};

use crate::StateError;
use crate::fs::{open_private_append, open_private_lock};

pub const EVENTS: &str = "events.ndjson";
const EVENTS_LOCK: &str = "events.ndjson.lock";

/// Who appended the line: the wrapper, or a `hook` process (through the wrapper's channel, or
/// directly for a SessionEnd the channel could not take).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Source {
    Wrapper,
    Hook,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct EventLine {
    pub v: u32,
    pub ts: String,
    pub instance: String,
    #[serde(serialize_with = "kind_str")]
    pub kind: EventKind,
    pub source: Source,
    pub data: Value,
}

fn kind_str<S: Serializer>(kind: &EventKind, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(kind.as_str())
}

impl EventLine {
    pub fn new(
        instance: &ViolaName,
        kind: EventKind,
        source: Source,
        data: Value,
        at: DateTime<Utc>,
    ) -> Self {
        Self {
            v: 1,
            ts: crate::timestamp(at),
            instance: instance.as_ref().to_owned(),
            kind,
            source,
            data,
        }
    }
}

/// Appends `line` to `<instance_dir>/events.ndjson` with a single `write_all`.
pub fn append_event(instance_dir: &Path, line: &EventLine) -> Result<(), StateError> {
    let bytes = line_bytes(line)?;
    let lock = open_private_lock(&instance_dir.join(EVENTS_LOCK))?;
    lock.lock()?;
    write_line(instance_dir, &bytes)
}

/// `append_event` without waiting: `Ok(false)`, and nothing written, while another writer holds
/// the lock (the SessionEnd hook's direct append, architecture [Hook Transport]).
pub fn try_append_event(instance_dir: &Path, line: &EventLine) -> Result<bool, StateError> {
    let bytes = line_bytes(line)?;
    let lock = open_private_lock(&instance_dir.join(EVENTS_LOCK))?;
    if lock.try_lock().is_err() {
        return Ok(false);
    }
    write_line(instance_dir, &bytes)?;
    Ok(true)
}

fn line_bytes(line: &EventLine) -> Result<Vec<u8>, StateError> {
    let mut bytes = serde_json::to_vec(line)?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn write_line(instance_dir: &Path, bytes: &[u8]) -> Result<(), StateError> {
    let mut file = open_private_append(&instance_dir.join(EVENTS))?;
    file.write_all(bytes)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone as _;
    use serde_json::json;
    use std::fs;

    fn name() -> ViolaName {
        ViolaName::try_new("builder".to_owned()).expect("valid")
    }

    fn at() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 9, 27, 1, 2, 3)
            .single()
            .expect("valid date")
            + chrono::Duration::milliseconds(7)
    }

    #[test]
    fn events_append_keeps_prior_bytes_and_writes_one_line() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let prior = b"{\"v\":1,\"kind\":\"earlier\"}\npartial-without-newline".to_vec();
        fs::write(tmp.path().join("events.ndjson"), &prior).expect("seed");
        let line = EventLine::new(
            &name(),
            EventKind::Wheel,
            Source::Wrapper,
            json!({"holder": "driver", "cause": "start"}),
            at(),
        );
        append_event(tmp.path(), &line).expect("append");

        let bytes = fs::read(tmp.path().join("events.ndjson")).expect("read");
        assert!(bytes.starts_with(&prior));
        let added = &bytes[prior.len()..];
        assert!(added.ends_with(b"\n"));
        assert_eq!(added.iter().filter(|b| **b == b'\n').count(), 1);
        let record: Value = serde_json::from_slice(added).expect("one JSON object");
        assert_eq!(
            record,
            json!({
                "v": 1,
                "ts": "2026-09-27T01:02:03.007Z",
                "instance": "builder",
                "kind": "wheel",
                "source": "wrapper",
                "data": {"holder": "driver", "cause": "start"},
            })
        );
        assert!(tmp.path().join("events.ndjson.lock").is_file());
    }

    #[test]
    fn events_append_twice_adds_two_lines_in_order() {
        let tmp = tempfile::tempdir().expect("tempdir");
        for (kind, data) in [
            (EventKind::Wheel, json!({"n": 1})),
            (EventKind::BudgetGate, json!({"paused": false})),
        ] {
            let line = EventLine::new(&name(), kind, Source::Wrapper, data, at());
            append_event(tmp.path(), &line).expect("append");
        }
        let text = fs::read_to_string(tmp.path().join("events.ndjson")).expect("read");
        let kinds: Vec<Value> = text
            .lines()
            .map(|l| serde_json::from_str::<Value>(l).expect("line")["kind"].clone())
            .collect();
        assert_eq!(kinds, [json!("wheel"), json!("budget-gate")]);
    }

    #[test]
    fn events_append_from_a_hook_reads_source_hook() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let line = EventLine::new(
            &name(),
            EventKind::SessionStart,
            Source::Hook,
            json!({"cause": "startup", "agent_session_id": null}),
            at(),
        );
        append_event(tmp.path(), &line).expect("append");
        let text = fs::read_to_string(tmp.path().join("events.ndjson")).expect("read");
        assert!(
            text.contains(r#""kind":"session-start","source":"hook""#),
            "{text}"
        );
    }

    #[test]
    fn events_try_append_writes_only_while_the_lock_is_free() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let line = EventLine::new(
            &name(),
            EventKind::SessionEnd,
            Source::Hook,
            json!({}),
            at(),
        );
        let held = open_private_lock(&tmp.path().join("events.ndjson.lock")).expect("lock file");
        held.lock().expect("held");
        assert!(!try_append_event(tmp.path(), &line).expect("busy"));
        assert!(!tmp.path().join("events.ndjson").exists());
        drop(held);
        assert!(try_append_event(tmp.path(), &line).expect("free"));
        let text = fs::read_to_string(tmp.path().join("events.ndjson")).expect("read");
        assert_eq!(text.lines().count(), 1);
        assert!(text.contains(r#""kind":"session-end","source":"hook","data":{}"#));
    }

    #[test]
    fn events_try_append_into_a_missing_dir_fails() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let line = EventLine::new(
            &name(),
            EventKind::SessionEnd,
            Source::Hook,
            json!({}),
            at(),
        );
        assert!(try_append_event(&tmp.path().join("missing"), &line).is_err());
    }

    #[test]
    fn events_append_into_a_missing_dir_fails() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let line = EventLine::new(&name(), EventKind::Wheel, Source::Wrapper, json!({}), at());
        assert!(append_event(&tmp.path().join("missing"), &line).is_err());
    }
}
