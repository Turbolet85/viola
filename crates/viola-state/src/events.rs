//! `instances/<name>/events.ndjson`: one compact JSON object + `\n` per `write`, appended under an
//! exclusive lock on the `events.ndjson.lock` sibling, never truncated or rewritten (architecture
//! §Standard Contracts, ndjson event line).

use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Read as _, Seek as _, SeekFrom, Write as _};
use std::path::Path;

use chrono::{DateTime, Utc};
use serde::{Serialize, Serializer};
use serde_json::{Map, Value};
use viola_core::obs::ObsEvent;
use viola_core::{EventKind, MAX_FRAME, ViolaName, obs_event};

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
    let tail = tail(instance_dir)?;
    write_line(instance_dir, &tail, &bytes)
}

/// The `events.ndjson` length read under the lock, 0 when the log does not exist yet: the byte
/// offset the next line starts at, or one before it when the log ends in a torn line.
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
    let tail = tail(instance_dir)?;
    let at = tail.line_start();
    let bytes = line_bytes(&build(at))?;
    write_line(instance_dir, &tail, &bytes)?;
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
    let tail = tail(instance_dir)?;
    write_line(instance_dir, &tail, &bytes)?;
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

/// What one read stepped over, under the names every surface reports (architecture §Standard
/// Contracts, GUI list envelope): lines of a kind this build does not know, known-kind lines that
/// carry a key outside the contract, and lines that are not one whole JSON object within the cap.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Skipped {
    pub unknown_kinds: u64,
    pub unknown_fields: u64,
    pub torn_lines: u64,
}

/// The top-level keys of an event line.
const LINE_KEYS: [&str; 6] = ["v", "ts", "instance", "kind", "source", "data"];

/// The complete lines of a known kind in `<instance_dir>/events.ndjson` that start at or after
/// `after`, in order. A line `after` falls inside is skipped (architecture §Standard Contracts,
/// `wait`: the line STARTS at or after the cursor). A last line without its `\n` is counted as
/// torn, neither returned nor rewritten, and an absent log reads as no lines.
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

/// The kind an event line names, when this build knows it.
fn known_kind(line: &Map<String, Value>) -> Option<EventKind> {
    line.get("kind")?.as_str().and_then(EventKind::from_name)
}

/// Whether a line of `kind` carries a top-level key outside the event line's six, or a `data` key
/// outside the kind's contract. A `data` that is not an object holds no key to count.
fn has_unknown_field(line: &Map<String, Value>, kind: EventKind) -> bool {
    let outside_line = line.keys().any(|key| !LINE_KEYS.contains(&key.as_str()));
    let outside_data = line
        .get("data")
        .and_then(Value::as_object)
        .is_some_and(|data| {
            data.keys()
                .any(|key| !kind.data_keys().contains(&key.as_str()))
        });
    outside_line || outside_data
}

/// [`read_from`]'s lines, read one at a time; an I/O error ends them.
pub struct LoggedLines {
    reader: Option<BufReader<File>>,
    pos: u64,
    cap: u64,
    skipped: Skipped,
}

impl LoggedLines {
    /// What the read has stepped over so far: whole once the lines are exhausted.
    pub fn skipped(&self) -> Skipped {
        self.skipped
    }

    fn next_line(&mut self) -> Result<Option<LoggedLine>, StateError> {
        while let Some(reader) = self.reader.as_mut() {
            let mut line = Vec::new();
            std::io::Read::take(&mut *reader, self.cap).read_until(b'\n', &mut line)?;
            let len = u64::try_from(line.len()).unwrap_or(u64::MAX);
            if line.last() != Some(&b'\n') {
                if line.is_empty() {
                    break;
                }
                // The read above takes at most `cap` bytes, so a line with no `\n` is at the cap
                // (over-long) or under it (the log ends inside it): torn either way, and only an
                // over-long line whose `\n` follows has lines after it.
                self.skipped.torn_lines += 1;
                if len != self.cap {
                    break;
                }
                let Some(rest) = skip_line(reader)? else {
                    break;
                };
                self.pos += len + rest;
                continue;
            }
            let start = self.pos;
            self.pos += len;
            let Ok(Value::Object(object)) = serde_json::from_slice::<Value>(&line) else {
                self.skipped.torn_lines += 1;
                continue;
            };
            let Some(kind) = known_kind(&object) else {
                self.skipped.unknown_kinds += 1;
                continue;
            };
            if has_unknown_field(&object, kind) {
                self.skipped.unknown_fields += 1;
            }
            return Ok(Some(LoggedLine {
                start,
                end: self.pos,
                value: Value::Object(object),
            }));
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

/// The log's end as an append finds it, read under the lock: its length, and whether its last
/// line was cut short (a log that is not empty and does not end in `\n`).
struct Tail {
    len: u64,
    torn: bool,
}

impl Tail {
    /// Where the appended line starts: straight after a whole log, one byte further after a torn
    /// one, whose fragment the append first ends with a `\n`.
    fn line_start(&self) -> u64 {
        self.len + u64::from(self.torn)
    }
}

fn tail(instance_dir: &Path) -> Result<Tail, StateError> {
    let len = current_len(instance_dir)?;
    let Some(last) = len.checked_sub(1) else {
        return Ok(Tail { len, torn: false });
    };
    let mut file = File::open(instance_dir.join(EVENTS))?;
    file.seek(SeekFrom::Start(last))?;
    let mut byte = [0u8; 1];
    file.read_exact(&mut byte)?;
    Ok(Tail {
        len,
        torn: byte[0] != b'\n',
    })
}

/// One `write_all` per line. After a torn tail that write starts with the `\n` that ends the
/// fragment, so the line begins on a fresh one and no earlier byte moves; the heal is then logged.
fn write_line(instance_dir: &Path, tail: &Tail, bytes: &[u8]) -> Result<(), StateError> {
    let mut file = open_private_append(&instance_dir.join(EVENTS))?;
    if !tail.torn {
        file.write_all(bytes)?;
        return Ok(());
    }
    file.write_all(&[b"\n", bytes].concat())?;
    obs_event!(
        WARN,
        ObsEvent::StateRecovered,
        detail = "torn-line-healed",
        file = EVENTS,
        offset = tail.len,
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_capture::capture;
    use chrono::TimeZone as _;
    use rstest::rstest;
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

    fn wheel() -> EventLine {
        EventLine::new(
            &name(),
            EventKind::Wheel,
            Source::Wrapper,
            json!({"holder": "driver", "cause": "start"}),
            at(),
        )
    }

    /// `wheel()` as it is written.
    const WHEEL_LINE: &str = concat!(
        r#"{"v":1,"ts":"2026-09-27T01:02:03.007Z","instance":"builder","kind":"wheel","#,
        r#""source":"wrapper","data":{"holder":"driver","cause":"start"}}"#,
        "\n"
    );

    fn session_end() -> EventLine {
        EventLine::new(
            &name(),
            EventKind::SessionEnd,
            Source::Hook,
            json!({}),
            at(),
        )
    }

    /// `session_end()` as it is written.
    const SESSION_END_LINE: &str = concat!(
        r#"{"v":1,"ts":"2026-09-27T01:02:03.007Z","instance":"builder","kind":"session-end","#,
        r#""source":"hook","data":{}}"#,
        "\n"
    );

    /// One whole line of 26 bytes, then a line cut short: 48 bytes, the last one not `\n`.
    const TORN: &[u8] = b"{\"v\":1,\"kind\":\"activity\"}\n{\"v\":1,\"kind\":\"turn-en";

    /// The one line a heal of `TORN` writes.
    fn healed_at_48() -> Value {
        json!({
            "event": "state-recovered",
            "detail": "torn-line-healed",
            "file": "events.ndjson",
            "offset": 48,
            "message": "state-recovered",
            "level": "WARN",
        })
    }

    fn log_bytes(dir: &Path) -> Vec<u8> {
        fs::read(dir.join("events.ndjson")).expect("read")
    }

    #[test]
    fn events_torn_tail_append_starts_on_a_fresh_line_and_keeps_the_prior_bytes() {
        assert_eq!(TORN.len(), 48);
        let tmp = seeded(TORN);
        append_event(tmp.path(), &wheel()).expect("append");
        assert_eq!(
            log_bytes(tmp.path()),
            [TORN, b"\n", WHEEL_LINE.as_bytes()].concat()
        );
        assert!(tmp.path().join("events.ndjson.lock").is_file());
    }

    #[test]
    fn events_torn_tail_heal_writes_one_state_recovered_line_and_the_next_append_none() {
        let tmp = seeded(TORN);
        let (healed, lines) = capture(|| append_event(tmp.path(), &wheel()));
        healed.expect("append");
        assert_eq!(lines, [healed_at_48()]);
        let (second, lines) = capture(|| append_event(tmp.path(), &wheel()));
        second.expect("append");
        assert_eq!(lines, Vec::<Value>::new());
    }

    #[rstest]
    #[case::absent(None)]
    #[case::empty(Some(&b""[..]))]
    #[case::whole(Some(&b"{\"v\":1,\"kind\":\"activity\"}\n"[..]))]
    fn events_torn_tail_a_log_with_no_fragment_takes_the_line_alone_and_no_record(
        #[case] prior: Option<&[u8]>,
    ) {
        let tmp = tempfile::tempdir().expect("tempdir");
        if let Some(prior) = prior {
            fs::write(tmp.path().join("events.ndjson"), prior).expect("seed");
        }
        let (appended, lines) = capture(|| append_event(tmp.path(), &wheel()));
        appended.expect("append");
        assert_eq!(lines, Vec::<Value>::new());
        assert_eq!(
            log_bytes(tmp.path()),
            [prior.unwrap_or_default(), WHEEL_LINE.as_bytes()].concat()
        );
    }

    #[test]
    fn events_torn_tail_append_at_returns_and_hands_its_builder_the_fresh_line_start() {
        let tmp = seeded(TORN);
        let mut handed = None;
        let at = append_event_at(tmp.path(), |l| {
            handed = Some(l);
            EventLine::new(
                &name(),
                EventKind::SendIssued,
                Source::Wrapper,
                json!({"cursor": l}),
                at(),
            )
        })
        .expect("append");
        assert_eq!(at, 49);
        assert_eq!(handed, Some(49));
        let bytes = log_bytes(tmp.path());
        assert_eq!(&bytes[..49], [TORN, b"\n"].concat());
        let record: Value = serde_json::from_slice(&bytes[49..]).expect("one line");
        assert_eq!(record["kind"], json!("send-issued"));
        assert_eq!(record["data"], json!({"cursor": 49}));
    }

    #[test]
    fn events_torn_tail_try_append_heals_while_the_lock_is_free_and_writes_nothing_while_held() {
        let tmp = seeded(TORN);
        let held = open_private_lock(&tmp.path().join("events.ndjson.lock")).expect("lock file");
        held.lock().expect("held");
        let (busy, lines) = capture(|| try_append_event(tmp.path(), &session_end()));
        assert!(!busy.expect("busy"));
        assert_eq!(lines, Vec::<Value>::new());
        assert_eq!(log_bytes(tmp.path()), TORN);
        drop(held);
        let (free, lines) = capture(|| try_append_event(tmp.path(), &session_end()));
        assert!(free.expect("free"));
        assert_eq!(lines, [healed_at_48()]);
        assert_eq!(
            log_bytes(tmp.path()),
            [TORN, b"\n", SESSION_END_LINE.as_bytes()].concat()
        );
    }

    #[test]
    fn events_torn_tail_reader_started_at_the_old_end_returns_the_healed_line() {
        let tmp = seeded(TORN);
        append_event(tmp.path(), &wheel()).expect("append");
        let (got, _) = read(tmp.path(), 48, LINE_CAP);
        let end = 49 + u64::try_from(WHEEL_LINE.len()).expect("len");
        assert_eq!(got, [(49, end, "wheel".to_owned())]);
    }

    #[test]
    fn events_torn_tail_counts_one_torn_line_before_the_heal_and_one_after() {
        let one_torn = Skipped {
            unknown_kinds: 0,
            unknown_fields: 0,
            torn_lines: 1,
        };
        let tmp = seeded(TORN);
        let (got, skipped) = read(tmp.path(), 0, LINE_CAP);
        assert_eq!(kinds_of(&got), ["activity"]);
        assert_eq!(skipped, one_torn);
        append_event(tmp.path(), &wheel()).expect("append");
        let (got, skipped) = read(tmp.path(), 0, LINE_CAP);
        assert_eq!(kinds_of(&got), ["activity", "wheel"]);
        assert_eq!(skipped, one_torn);
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
        let line = session_end();
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
        assert!(try_append_event(&tmp.path().join("missing"), &session_end()).is_err());
    }

    #[test]
    fn events_end_offset_is_zero_before_the_log_exists() {
        let tmp = tempfile::tempdir().expect("tempdir");
        assert_eq!(end_offset(tmp.path()).expect("offset"), 0);
    }

    #[test]
    fn events_append_at_returns_the_prior_length_and_the_line_carries_it() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let prior = b"{\"v\":1,\"kind\":\"activity\"}\n".to_vec();
        fs::write(tmp.path().join("events.ndjson"), &prior).expect("seed");
        assert_eq!(end_offset(tmp.path()).expect("offset"), 26);
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
        assert_eq!(at, 26);
        let bytes = fs::read(tmp.path().join("events.ndjson")).expect("read");
        let record: Value = serde_json::from_slice(&bytes[26..]).expect("one line");
        assert_eq!(record["kind"], json!("send-issued"));
        assert_eq!(record["data"]["cursor"], json!(26));
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

    /// Three lines of 17, 16 and 20 bytes.
    const LOG: &[u8] = b"{\"kind\":\"wheel\"}\n{\"kind\":\"plan\"}\n{\"kind\":\"activity\"}\n";

    const NONE_SKIPPED: Skipped = Skipped {
        unknown_kinds: 0,
        unknown_fields: 0,
        torn_lines: 0,
    };

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
                (0, 17, "wheel".to_owned()),
                (17, 33, "plan".to_owned()),
                (33, 53, "activity".to_owned())
            ]
        );
        assert_eq!(skipped, NONE_SKIPPED);
    }

    #[test]
    fn events_read_from_a_line_start_begins_at_that_line() {
        let tmp = seeded(LOG);
        let (got, _) = read(tmp.path(), 17, LINE_CAP);
        assert_eq!(got[0], (17, 33, "plan".to_owned()));
        assert_eq!(kinds_of(&got), ["plan", "activity"]);
    }

    #[test]
    fn events_read_from_inside_a_line_skips_that_line() {
        let tmp = seeded(LOG);
        for after in [1, 16] {
            let (got, _) = read(tmp.path(), after, LINE_CAP);
            assert_eq!(got[0], (17, 33, "plan".to_owned()), "after {after}");
        }
        let (got, _) = read(tmp.path(), 18, LINE_CAP);
        assert_eq!(got, [(33, 53, "activity".to_owned())]);
    }

    #[test]
    fn events_read_at_or_past_the_end_is_empty() {
        let tmp = seeded(LOG);
        for after in [53, 54, 1000] {
            assert!(read(tmp.path(), after, LINE_CAP).0.is_empty(), "{after}");
        }
    }

    #[test]
    fn events_read_of_an_absent_log_is_empty() {
        let tmp = tempfile::tempdir().expect("tempdir");
        assert_eq!(read(tmp.path(), 0, LINE_CAP), (Vec::new(), NONE_SKIPPED));
        assert_eq!(read(tmp.path(), 5, LINE_CAP), (Vec::new(), NONE_SKIPPED));
    }

    #[test]
    fn events_three_counts_an_unterminated_last_line_is_torn_unreturned_and_unwritten() {
        let torn = [LOG, b"{\"kind\":\"plan\""].concat();
        let tmp = seeded(&torn);
        let (got, skipped) = read(tmp.path(), 0, LINE_CAP);
        assert_eq!(kinds_of(&got), ["wheel", "plan", "activity"]);
        assert_eq!(
            skipped,
            Skipped {
                unknown_kinds: 0,
                unknown_fields: 0,
                torn_lines: 1
            }
        );
        assert!(read(tmp.path(), 54, LINE_CAP).0.is_empty());
        let after = fs::read(tmp.path().join("events.ndjson")).expect("read");
        assert_eq!(after, torn);
    }

    #[test]
    fn events_three_counts_a_terminated_line_that_is_not_one_object_is_torn() {
        let tmp = seeded(b"{\"kind\":\"wheel\"}\nnot json\n[1]\n\n{\"kind\":\"plan\"}\n");
        let (got, skipped) = read(tmp.path(), 0, LINE_CAP);
        assert_eq!(
            got,
            [(0, 17, "wheel".to_owned()), (31, 47, "plan".to_owned())]
        );
        assert_eq!(
            skipped,
            Skipped {
                unknown_kinds: 0,
                unknown_fields: 0,
                torn_lines: 3
            }
        );
    }

    /// A line of exactly the cap, its `\n` included, is read; one byte more is torn, whether its
    /// `\n` follows or the log ends inside it, and a line after it keeps its true offsets.
    #[rstest]
    #[case::terminated(
        b"{\"kind\":\"plan\"}\n{\"kind\":\"wheel\"}\n{\"kind\":\"plan\"}\n",
        &[(0, 16), (33, 49)]
    )]
    #[case::last_of_the_read(b"{\"kind\":\"plan\"}\n{\"kind\":\"wheel\",\"data\":{\"holder\"", &[(0, 16)])]
    #[case::last_of_the_read_at_the_cap(b"{\"kind\":\"plan\"}\n{\"kind\":\"wheel\"}", &[(0, 16)])]
    fn events_three_counts_a_line_past_the_cap_is_torn(
        #[case] log: &[u8],
        #[case] read_lines: &[(u64, u64)],
    ) {
        let tmp = seeded(log);
        let (got, skipped) = read(tmp.path(), 0, 16);
        let expected: Vec<(u64, u64, String)> = read_lines
            .iter()
            .map(|(start, end)| (*start, *end, "plan".to_owned()))
            .collect();
        assert_eq!(got, expected);
        assert_eq!(
            skipped,
            Skipped {
                unknown_kinds: 0,
                unknown_fields: 0,
                torn_lines: 1
            }
        );
    }

    #[test]
    fn events_three_counts_an_absent_non_string_or_unlisted_kind_is_unknown_and_unreturned() {
        let tmp = seeded(
            b"{\"kind\":\"wheel\"}\n{}\n{\"kind\":7}\n{\"kind\":\"link\"}\n{\"kind\":\"later-kind\"}\n{\"kind\":\"plan\"}\n",
        );
        let (got, skipped) = read(tmp.path(), 0, LINE_CAP);
        assert_eq!(
            got,
            [(0, 17, "wheel".to_owned()), (69, 85, "plan".to_owned())]
        );
        assert_eq!(
            skipped,
            Skipped {
                unknown_kinds: 4,
                unknown_fields: 0,
                torn_lines: 0
            }
        );
    }

    /// A key outside the six top-level ones, a `data` key outside the kind's row, and both at once
    /// each count once for their line; a `data` that is not an object, and a line holding every
    /// contract key, count nothing. Every line is returned.
    #[test]
    fn events_three_counts_an_unlisted_key_counts_once_for_its_line_and_the_line_is_returned() {
        let log = [
            r#"{"kind":"wheel","later":1}"#,
            r#"{"kind":"wheel","data":{"holder":"human","later":1}}"#,
            r#"{"kind":"wheel","later":1,"data":{"later":2,"more":3}}"#,
            r#"{"kind":"wheel","data":7}"#,
            r#"{"v":1,"ts":"t","instance":"builder","kind":"wheel","source":"wrapper","data":{"holder":"driver","cause":"start"}}"#,
        ]
        .map(|line| format!("{line}\n"))
        .concat();
        let tmp = seeded(log.as_bytes());
        let (got, skipped) = read(tmp.path(), 0, LINE_CAP);
        assert_eq!(kinds_of(&got), ["wheel"; 5]);
        assert_eq!(
            skipped,
            Skipped {
                unknown_kinds: 0,
                unknown_fields: 3,
                torn_lines: 0
            }
        );
    }

    #[test]
    fn events_three_counts_every_kind_written_with_its_own_keys_reads_zero() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let rows = [
            (
                EventKind::SessionStart,
                json!({"cause": "startup", "agent_session_id": "s-1"}),
            ),
            (
                EventKind::PromptSubmitted,
                json!({"text": "hello", "origin": "human"}),
            ),
            (
                EventKind::TurnEnded,
                json!({"last_assistant_message": null}),
            ),
            (EventKind::SessionEnd, json!({})),
            (EventKind::Activity, json!({"tool": "Bash"})),
            (
                EventKind::Wheel,
                json!({"holder": "driver", "cause": "start"}),
            ),
            (
                EventKind::BudgetGate,
                json!({"paused": true, "window": "five-hour", "override_until": "2026-09-27T02:00:00.000Z"}),
            ),
            (
                EventKind::SendIssued,
                json!({"cursor": 0, "from": "overseer"}),
            ),
            (
                EventKind::SendConfirmed,
                json!({"cursor": 0, "confirmed": false}),
            ),
            (
                EventKind::SendRefused,
                json!({"refusal": "not-delivered", "detail": "turn-running", "cursor": 0}),
            ),
            (
                EventKind::Question,
                json!({"dialog_id": 1, "questions": []}),
            ),
            (
                EventKind::Permission,
                json!({"dialog_id": 2, "tool": "Bash", "input": {"command": "ls"}}),
            ),
            (EventKind::Plan, json!({"dialog_id": 3, "plan": "text"})),
        ];
        for (kind, data) in rows {
            let line = EventLine::new(&name(), kind, Source::Wrapper, data, at());
            append_event(tmp.path(), &line).expect("append");
        }
        let (got, skipped) = read(tmp.path(), 0, LINE_CAP);
        assert_eq!(
            kinds_of(&got),
            [
                "session-start",
                "prompt-submitted",
                "turn-ended",
                "session-end",
                "activity",
                "wheel",
                "budget-gate",
                "send-issued",
                "send-confirmed",
                "send-refused",
                "question",
                "permission",
                "plan",
            ]
        );
        assert_eq!(skipped, NONE_SKIPPED);
    }

    #[test]
    fn events_read_line_cap_is_max_frame_and_its_newline() {
        assert_eq!(LINE_CAP, 16_777_217);
        let tmp = seeded(LOG);
        let mut lines = read_from(tmp.path(), 17).expect("read");
        assert_eq!(lines.next().expect("one").expect("line").start, 17);
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

    /// A path holding a NUL byte fails the stat and the open with a kind other than `NotFound`,
    /// before any filesystem call: only an absent log reads as empty.
    const NUL_DIR: &str = "instance\0dir";

    #[test]
    fn events_current_len_of_a_log_that_cannot_be_statted_is_an_error() {
        assert!(current_len(Path::new(NUL_DIR)).is_err());
    }

    #[test]
    fn events_read_of_a_log_that_cannot_be_opened_is_an_error() {
        assert!(read_from(Path::new(NUL_DIR), 0).is_err());
    }
}
