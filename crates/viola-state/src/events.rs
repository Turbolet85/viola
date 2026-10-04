//! `instances/<name>/events.ndjson`: one compact JSON object + `\n` per `write`, appended under an
//! exclusive lock on the `events.ndjson.lock` sibling, never truncated or rewritten (architecture
//! §Standard Contracts, ndjson event line).

use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Read as _, Seek as _, SeekFrom, Write as _};
use std::path::Path;

use chrono::{DateTime, Utc};
use serde::{Serialize, Serializer};
use serde_json::Value;
use viola_core::{EventKind, MAX_FRAME, ViolaName};

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

/// The `events.ndjson` length read under the lock: the byte offset the next line starts at, 0 when
/// the log does not exist yet.
pub fn end_offset(instance_dir: &Path) -> Result<u64, StateError> {
    let lock = open_private_lock(&instance_dir.join(EVENTS_LOCK))?;
    lock.lock()?;
    current_len(instance_dir)
}

/// Appends the line `build` makes from the offset it will start at, under one lock hold, and
/// returns that offset (a `send`'s `cursor`).
pub fn append_event_at(
    instance_dir: &Path,
    build: impl FnOnce(u64) -> EventLine,
) -> Result<u64, StateError> {
    let lock = open_private_lock(&instance_dir.join(EVENTS_LOCK))?;
    lock.lock()?;
    let at = current_len(instance_dir)?;
    let bytes = line_bytes(&build(at))?;
    write_line(instance_dir, &bytes)?;
    Ok(at)
}

fn current_len(instance_dir: &Path) -> Result<u64, StateError> {
    match std::fs::metadata(instance_dir.join(EVENTS)) {
        Ok(meta) => Ok(meta.len()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(0),
        Err(e) => Err(e.into()),
    }
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

/// The bound on one line, its `\n` included.
const LINE_CAP: u64 = MAX_FRAME + 1;

/// One complete `events.ndjson` line: the offset it starts at, the offset after its `\n` (the
/// `cursor` a `wait` hands back), and the object it holds.
#[derive(Debug, Clone, PartialEq)]
pub struct LoggedLine {
    pub start: u64,
    pub end: u64,
    pub value: Value,
}

/// What a read stepped over: lines past the cap, and lines that are not one JSON object.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Skipped {
    pub oversize: u64,
    pub malformed: u64,
}

/// The complete lines of `<instance_dir>/events.ndjson` that start at or after `after`, in order. A
/// line `after` falls inside is skipped (architecture §Standard Contracts, `wait`: the line STARTS at
/// or after the cursor). A last line without its `\n` is neither returned nor rewritten, and an
/// absent log reads as no lines.
pub fn read_from(instance_dir: &Path, after: u64) -> Result<LoggedLines, StateError> {
    read_within(instance_dir, after, LINE_CAP)
}

fn read_within(instance_dir: &Path, after: u64, cap: u64) -> Result<LoggedLines, StateError> {
    let mut lines = LoggedLines {
        reader: None,
        pos: after,
        cap,
        skipped: Skipped::default(),
    };
    let file = match File::open(instance_dir.join(EVENTS)) {
        Ok(file) => file,
        Err(e) if e.kind() == ErrorKind::NotFound => return Ok(lines),
        Err(e) => return Err(e.into()),
    };
    let mut reader = BufReader::new(file);
    if let Some(before) = after.checked_sub(1) {
        reader.seek(SeekFrom::Start(before))?;
        let mut prev = [0u8; 1];
        if reader.read(&mut prev)? == 0 {
            return Ok(lines);
        }
        if prev[0] != b'\n' {
            match skip_line(&mut reader)? {
                Some(skipped) => lines.pos += skipped,
                None => return Ok(lines),
            }
        }
    }
    lines.reader = Some(reader);
    Ok(lines)
}

/// Consumes up to and including the next `\n`: how many bytes, or `None` at the end of the file.
fn skip_line(reader: &mut impl BufRead) -> Result<Option<u64>, StateError> {
    let mut consumed = 0u64;
    loop {
        let buf = reader.fill_buf()?;
        if buf.is_empty() {
            return Ok(None);
        }
        let (n, found) = match buf.iter().position(|b| *b == b'\n') {
            Some(i) => (i + 1, true),
            None => (buf.len(), false),
        };
        reader.consume(n);
        consumed += u64::try_from(n).unwrap_or(u64::MAX);
        if found {
            return Ok(Some(consumed));
        }
    }
}

/// [`read_from`]'s lines, read one at a time; an I/O error ends them.
pub struct LoggedLines {
    reader: Option<BufReader<File>>,
    pos: u64,
    cap: u64,
    skipped: Skipped,
}

impl LoggedLines {
    pub fn skipped(&self) -> Skipped {
        self.skipped
    }

    fn next_line(&mut self) -> Result<Option<LoggedLine>, StateError> {
        while let Some(reader) = self.reader.as_mut() {
            let mut line = Vec::new();
            std::io::Read::take(&mut *reader, self.cap).read_until(b'\n', &mut line)?;
            let len = u64::try_from(line.len()).unwrap_or(u64::MAX);
            if line.last() != Some(&b'\n') {
                if len < self.cap {
                    break;
                }
                let Some(rest) = skip_line(reader)? else {
                    break;
                };
                self.pos += len + rest;
                self.skipped.oversize += 1;
                continue;
            }
            let start = self.pos;
            self.pos += len;
            match serde_json::from_slice::<Value>(&line) {
                Ok(value @ Value::Object(_)) => {
                    return Ok(Some(LoggedLine {
                        start,
                        end: self.pos,
                        value,
                    }));
                }
                _ => self.skipped.malformed += 1,
            }
        }
        self.reader = None;
        Ok(None)
    }
}

impl Iterator for LoggedLines {
    type Item = Result<LoggedLine, StateError>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next_line();
        if next.is_err() {
            self.reader = None;
        }
        next.transpose()
    }
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
    fn events_end_offset_is_zero_before_the_log_exists() {
        let tmp = tempfile::tempdir().expect("tempdir");
        assert_eq!(end_offset(tmp.path()).expect("offset"), 0);
    }

    #[test]
    fn events_append_at_returns_the_prior_length_and_the_line_carries_it() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let prior = b"{\"v\":1,\"kind\":\"earlier\"}\n".to_vec();
        fs::write(tmp.path().join("events.ndjson"), &prior).expect("seed");
        assert_eq!(end_offset(tmp.path()).expect("offset"), 25);
        let at = append_event_at(tmp.path(), |l| {
            EventLine::new(
                &name(),
                EventKind::SendIssued,
                Source::Wrapper,
                json!({"cursor": l}),
                at(),
            )
        })
        .expect("append");
        assert_eq!(at, 25);
        let bytes = fs::read(tmp.path().join("events.ndjson")).expect("read");
        let record: Value = serde_json::from_slice(&bytes[25..]).expect("one line");
        assert_eq!(record["kind"], json!("send-issued"));
        assert_eq!(record["data"]["cursor"], json!(25));
        assert_eq!(
            end_offset(tmp.path()).expect("offset"),
            u64::try_from(bytes.len()).expect("len")
        );
    }

    #[test]
    fn events_append_at_twice_returns_increasing_offsets() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let build = |l: u64| {
            EventLine::new(
                &name(),
                EventKind::SendIssued,
                Source::Wrapper,
                json!({"cursor": l}),
                at(),
            )
        };
        let first = append_event_at(tmp.path(), build).expect("first");
        let second = append_event_at(tmp.path(), build).expect("second");
        assert_eq!(first, 0);
        assert!(second > first);
        let text = fs::read_to_string(tmp.path().join("events.ndjson")).expect("read");
        let first_line = text.split_inclusive('\n').next().expect("first line");
        assert_eq!(
            second,
            u64::try_from(first_line.len()).expect("len"),
            "{text}"
        );
    }

    #[test]
    fn events_append_at_into_a_missing_dir_fails() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let missing = tmp.path().join("missing");
        assert!(end_offset(&missing).is_err());
        let result = append_event_at(&missing, |_| {
            EventLine::new(&name(), EventKind::Wheel, Source::Wrapper, json!({}), at())
        });
        assert!(result.is_err());
    }

    const LOG: &[u8] = b"{\"kind\":\"a\"}\n{\"kind\":\"b\"}\n{\"kind\":\"c\"}\n";

    fn seeded(bytes: &[u8]) -> tempfile::TempDir {
        let tmp = tempfile::tempdir().expect("tempdir");
        fs::write(tmp.path().join("events.ndjson"), bytes).expect("seed");
        tmp
    }

    /// `(start, end, kind)` per line, and the skip counts.
    fn read(dir: &Path, after: u64, cap: u64) -> (Vec<(u64, u64, String)>, Skipped) {
        let mut lines = read_within(dir, after, cap).expect("read");
        let got = lines
            .by_ref()
            .map(|l| {
                let l = l.expect("line");
                let kind = l.value["kind"].as_str().expect("kind").to_owned();
                (l.start, l.end, kind)
            })
            .collect();
        (got, lines.skipped())
    }

    fn kinds_of(got: &[(u64, u64, String)]) -> Vec<&str> {
        got.iter().map(|(_, _, k)| k.as_str()).collect()
    }

    #[test]
    fn events_read_returns_each_line_with_its_start_and_end() {
        let tmp = seeded(LOG);
        let (got, skipped) = read(tmp.path(), 0, LINE_CAP);
        assert_eq!(
            got,
            [
                (0, 13, "a".to_owned()),
                (13, 26, "b".to_owned()),
                (26, 39, "c".to_owned())
            ]
        );
        assert_eq!(skipped, Skipped::default());
    }

    #[test]
    fn events_read_from_a_line_start_begins_at_that_line() {
        let tmp = seeded(LOG);
        let (got, _) = read(tmp.path(), 13, LINE_CAP);
        assert_eq!(got[0], (13, 26, "b".to_owned()));
        assert_eq!(kinds_of(&got), ["b", "c"]);
    }

    #[test]
    fn events_read_from_inside_a_line_skips_that_line() {
        let tmp = seeded(LOG);
        for after in [1, 12] {
            let (got, _) = read(tmp.path(), after, LINE_CAP);
            assert_eq!(got[0], (13, 26, "b".to_owned()), "after {after}");
        }
        let (got, _) = read(tmp.path(), 14, LINE_CAP);
        assert_eq!(got, [(26, 39, "c".to_owned())]);
    }

    #[test]
    fn events_read_at_or_past_the_end_is_empty() {
        let tmp = seeded(LOG);
        for after in [39, 40, 1000] {
            assert!(read(tmp.path(), after, LINE_CAP).0.is_empty(), "{after}");
        }
    }

    #[test]
    fn events_read_of_an_absent_log_is_empty() {
        let tmp = tempfile::tempdir().expect("tempdir");
        assert!(read(tmp.path(), 0, LINE_CAP).0.is_empty());
        assert!(read(tmp.path(), 5, LINE_CAP).0.is_empty());
    }

    #[test]
    fn events_read_leaves_a_torn_last_line_unread_and_unwritten() {
        let torn = [LOG, b"{\"kind\":\"d\""].concat();
        let tmp = seeded(&torn);
        let (got, skipped) = read(tmp.path(), 0, LINE_CAP);
        assert_eq!(kinds_of(&got), ["a", "b", "c"]);
        assert_eq!(skipped, Skipped::default());
        assert!(read(tmp.path(), 40, LINE_CAP).0.is_empty());
        let after = fs::read(tmp.path().join("events.ndjson")).expect("read");
        assert_eq!(after, torn);
    }

    #[test]
    fn events_read_skips_and_counts_a_line_that_is_not_one_object() {
        let tmp = seeded(b"{\"kind\":\"a\"}\nnot json\n[1]\n\n{\"kind\":\"b\"}\n");
        let (got, skipped) = read(tmp.path(), 0, LINE_CAP);
        assert_eq!(got, [(0, 13, "a".to_owned()), (27, 40, "b".to_owned())]);
        assert_eq!(
            skipped,
            Skipped {
                oversize: 0,
                malformed: 3
            }
        );
    }

    /// A line of exactly the cap, its `\n` included, is read; one byte more is skipped and counted,
    /// and the next line keeps its true offsets.
    #[test]
    fn events_read_skips_and_counts_a_line_past_the_cap() {
        let fits = b"{\"kind\":\"a\"}\n";
        let over = b"{\"kind\":\"xx\"}\n";
        let tmp = seeded(&[&fits[..], over, fits].concat());
        let (got, skipped) = read(tmp.path(), 0, 13);
        assert_eq!(got, [(0, 13, "a".to_owned()), (27, 40, "a".to_owned())]);
        assert_eq!(
            skipped,
            Skipped {
                oversize: 1,
                malformed: 0
            }
        );
    }

    #[test]
    fn events_read_an_oversize_torn_last_line_is_not_counted() {
        let tmp = seeded(b"{\"kind\":\"a\"}\n{\"kind\":\"xxxxxxxx");
        let (got, skipped) = read(tmp.path(), 0, 13);
        assert_eq!(kinds_of(&got), ["a"]);
        assert_eq!(skipped, Skipped::default());
    }

    #[test]
    fn events_read_line_cap_is_max_frame_and_its_newline() {
        assert_eq!(LINE_CAP, 16_777_217);
        let tmp = seeded(LOG);
        let mut lines = read_from(tmp.path(), 13).expect("read");
        assert_eq!(lines.next().expect("one").expect("line").start, 13);
    }

    /// A log that cannot be read fails at the open (Windows) or at the first line (Unix), and
    /// then ends.
    #[test]
    fn events_read_of_an_unreadable_log_is_an_error() {
        let tmp = tempfile::tempdir().expect("tempdir");
        fs::create_dir(tmp.path().join("events.ndjson")).expect("a dir in its place");
        match read_from(tmp.path(), 0) {
            Err(_) => {}
            Ok(mut lines) => {
                assert!(lines.next().is_some_and(|l| l.is_err()));
                assert!(lines.next().is_none());
            }
        }
    }

    #[test]
    fn events_append_into_a_missing_dir_fails() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let line = EventLine::new(&name(), EventKind::Wheel, Source::Wrapper, json!({}), at());
        assert!(append_event(&tmp.path().join("missing"), &line).is_err());
    }
}
