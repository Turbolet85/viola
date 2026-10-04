//! The wrapper's `wait` and `last` (architecture [Message Broker / IPC]): nothing polls. Every line
//! the wrapper appends for a hook bumps a generation under one lock and wakes the parked `wait`s,
//! which re-scan `events.ndjson` from their cursor for the first driver-relevant line; a step
//! timeout only re-reads the clock against the deadline. The newest `turn-ended` is held in
//! memory for `last`, rebuilt from the log before the endpoint serves.

use std::path::Path;
use std::sync::{Condvar, Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant};

use serde_json::{Value, json};
use tracing::instrument;
use viola_channel::ProtocolError;
use viola_core::{Clock, EventKind};
use viola_state::StateError;
use viola_state::events::{EventLine, LoggedLine, end_offset, read_from};

use crate::run::send::parse_from;

/// How often a parked `wait` re-reads the clock against its deadline.
const STEP: Duration = Duration::from_millis(20);

/// The newest turn's `last_assistant_message` and `ts`.
#[derive(Debug, Clone, PartialEq)]
struct Turn {
    message: Option<String>,
    ts: String,
}

impl Turn {
    fn of(data: &Value, ts: &str) -> Self {
        Self {
            message: data["last_assistant_message"].as_str().map(str::to_owned),
            ts: ts.to_owned(),
        }
    }
}

#[derive(Default)]
struct Feed {
    generation: u64,
    newest: Option<Turn>,
}

/// What every appended hook line signals, and the newest turn.
pub(crate) struct WaitFeed {
    clock: Box<dyn Clock>,
    feed: Mutex<Feed>,
    appended: Condvar,
}

impl WaitFeed {
    pub(crate) fn new(clock: impl Clock + 'static) -> Self {
        Self {
            clock: Box::new(clock),
            feed: Mutex::new(Feed::default()),
            appended: Condvar::new(),
        }
    }

    fn feed(&self) -> MutexGuard<'_, Feed> {
        self.feed.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// `line` is on disk: a parked `wait` re-scans, and a `turn-ended` becomes the newest turn.
    pub(crate) fn appended(&self, line: &EventLine) {
        let mut feed = self.feed();
        feed.generation = feed.generation.wrapping_add(1);
        if line.kind == EventKind::TurnEnded {
            feed.newest = Some(Turn::of(&line.data, &line.ts));
        }
        self.appended.notify_all();
    }

    /// The newest `turn-ended` the log already holds, read once at start.
    pub(crate) fn rebuild(&self, instance_dir: &Path) -> Result<(), StateError> {
        let mut newest = None;
        for line in read_from(instance_dir, 0)? {
            let line = line?;
            if line.value["kind"] == EventKind::TurnEnded.as_str() {
                let ts = line.value["ts"].as_str().unwrap_or_default();
                newest = Some(Turn::of(&line.value["data"], ts));
            }
        }
        self.feed().newest = newest;
        Ok(())
    }

    /// `params` = `{after?, timeout_ms?, from?}`: the first driver-relevant line starting at or after
    /// `after` (the log's end at the call when absent), at once when it is already logged.
    #[instrument(
        skip_all,
        name = "run.wait_dispatch",
        fields(woken_kind = tracing::field::Empty, timed_out = tracing::field::Empty)
    )]
    pub(crate) fn wait(&self, instance_dir: &Path, params: &Value) -> Result<Value, ProtocolError> {
        let after = count(params, "after")?;
        let timeout_ms = count(params, "timeout_ms")?;
        parse_from(params)?;
        let mut from = match after {
            Some(after) => after,
            None => end_offset(instance_dir).map_err(|_| ProtocolError::Internal)?,
        };
        let deadline = timeout_ms.and_then(|ms| deadline(self.clock.now(), ms));
        let span = tracing::Span::current();
        loop {
            let seen = self.feed().generation;
            if let Some(line) = scan(instance_dir, &mut from)? {
                span.record("woken_kind", line.value["kind"].as_str());
                return Ok(json!({"ok": {"event": line.value, "cursor": line.end}}));
            }
            let mut feed = self.feed();
            while feed.generation == seen {
                if deadline.is_some_and(|d| self.clock.now() >= d) {
                    span.record("timed_out", true);
                    return Ok(json!({"ok": {"timed_out": true}}));
                }
                feed = self
                    .appended
                    .wait_timeout(feed, STEP)
                    .unwrap_or_else(PoisonError::into_inner)
                    .0;
            }
        }
    }

    /// `params` = `{from?}`: the newest turn's message and `ts`, both `null` before any turn.
    pub(crate) fn last(&self, params: &Value) -> Result<Value, ProtocolError> {
        parse_from(params)?;
        let newest = self.feed().newest.clone();
        let (message, ts) = newest.map_or((None, None), |t| (t.message, Some(t.ts)));
        Ok(json!({"ok": {"last_assistant_message": message, "ts": ts}}))
    }
}

/// An absent field, or a non-negative integer that fits `u64`; anything else is `-32602`.
fn count(params: &Value, field: &str) -> Result<Option<u64>, ProtocolError> {
    match params.get(field) {
        None => Ok(None),
        Some(value) => value.as_u64().map(Some).ok_or(ProtocolError::InvalidParams),
    }
}

/// `now + ms`; `None` (no deadline) when that instant cannot be represented.
fn deadline(now: Instant, ms: u64) -> Option<Instant> {
    now.checked_add(Duration::from_millis(ms))
}

/// The first wake line from `*from` on; `*from` moves past every line read without a match, so a
/// next scan starts where this one stopped.
fn scan(instance_dir: &Path, from: &mut u64) -> Result<Option<LoggedLine>, ProtocolError> {
    let lines = read_from(instance_dir, *from).map_err(|_| ProtocolError::Internal)?;
    for line in lines {
        let line = line.map_err(|_| ProtocolError::Internal)?;
        let kind = line.value["kind"].as_str();
        if EventKind::WAIT_WAKE
            .iter()
            .any(|k| Some(k.as_str()) == kind)
        {
            return Ok(Some(line));
        }
        *from = line.end;
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::mpsc;

    use chrono::Utc;
    use rstest::rstest;
    use viola_core::ViolaName;
    use viola_state::events::{Source, append_event};

    /// Every reading is a second past the previous one.
    struct JumpClock(Mutex<Instant>);

    impl JumpClock {
        fn new() -> Self {
            Self(Mutex::new(Instant::now()))
        }
    }

    impl Clock for JumpClock {
        fn now(&self) -> Instant {
            let mut now = self.0.lock().expect("clock");
            *now += Duration::from_secs(1);
            *now
        }
    }

    /// Every reading 100 ms past the previous one, each kept.
    #[derive(Clone)]
    struct StepClock(Arc<Mutex<Vec<Instant>>>);

    impl Clock for StepClock {
        fn now(&self) -> Instant {
            let mut readings = self.0.lock().expect("clock");
            let next = readings
                .last()
                .map_or_else(Instant::now, |l| *l + Duration::from_millis(100));
            readings.push(next);
            next
        }
    }

    /// Never moves; each reading after the first is announced, so a test knows the waiter is
    /// parked on its deadline.
    struct Announcing {
        at: Instant,
        readings: Mutex<u64>,
        parked: mpsc::Sender<()>,
    }

    impl Clock for Announcing {
        fn now(&self) -> Instant {
            let mut n = self.readings.lock().expect("readings");
            *n += 1;
            if *n > 1 {
                let _ = self.parked.send(());
            }
            self.at
        }
    }

    fn announcing() -> (Announcing, mpsc::Receiver<()>) {
        let (parked, rx) = mpsc::channel();
        let clock = Announcing {
            at: Instant::now(),
            readings: Mutex::new(0),
            parked,
        };
        (clock, rx)
    }

    fn name() -> ViolaName {
        ViolaName::try_new("builder".to_owned()).expect("valid")
    }

    fn line(kind: EventKind, data: Value) -> EventLine {
        EventLine::new(&name(), kind, Source::Hook, data, Utc::now())
    }

    fn turn(message: Value) -> EventLine {
        line(
            EventKind::TurnEnded,
            json!({"last_assistant_message": message}),
        )
    }

    fn append(dir: &Path, line: &EventLine) -> u64 {
        append_event(dir, line).expect("append");
        end_offset(dir).expect("end")
    }

    fn seed_raw(dir: &Path, kind: &str) -> u64 {
        let raw = format!("{{\"v\":1,\"kind\":\"{kind}\",\"data\":{{}}}}\n");
        let mut bytes = std::fs::read(dir.join("events.ndjson")).unwrap_or_default();
        bytes.extend_from_slice(raw.as_bytes());
        std::fs::write(dir.join("events.ndjson"), &bytes).expect("seed");
        u64::try_from(bytes.len()).expect("len")
    }

    #[rstest]
    #[case::turn_ended("turn-ended")]
    #[case::question("question")]
    #[case::permission("permission")]
    #[case::plan("plan")]
    #[case::session_end("session-end")]
    fn wait_each_wake_kind_wakes(#[case] kind: &str) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let end = seed_raw(tmp.path(), kind);
        let feed = WaitFeed::new(JumpClock::new());
        let reply = feed
            .wait(tmp.path(), &json!({"v": 1, "after": 0, "timeout_ms": 3000}))
            .expect("answered");
        assert_eq!(reply["ok"]["event"]["kind"], kind);
        assert_eq!(reply["ok"]["cursor"], end);
    }

    #[rstest]
    #[case::activity("activity")]
    #[case::wheel("wheel")]
    #[case::budget_gate("budget-gate")]
    #[case::send_issued("send-issued")]
    #[case::send_confirmed("send-confirmed")]
    #[case::send_refused("send-refused")]
    #[case::session_start("session-start")]
    #[case::prompt_submitted("prompt-submitted")]
    #[case::unknown("later-kind")]
    fn wait_log_only_kinds_never_wake(#[case] kind: &str) {
        let tmp = tempfile::tempdir().expect("tempdir");
        seed_raw(tmp.path(), kind);
        let feed = WaitFeed::new(JumpClock::new());
        let reply = feed
            .wait(tmp.path(), &json!({"v": 1, "after": 0, "timeout_ms": 3000}))
            .expect("answered");
        assert_eq!(reply, json!({"ok": {"timed_out": true}}));
    }

    /// The first wake line at or after `after` comes back at once, the log-only line before it
    /// stepped over, its `cursor` the end of its own line.
    #[test]
    fn wait_a_logged_match_returns_at_once_with_its_line_end() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let start = append(tmp.path(), &line(EventKind::Activity, json!({})));
        let end = append(tmp.path(), &turn(json!("first")));
        append(tmp.path(), &turn(json!("second")));
        let feed = WaitFeed::new(JumpClock::new());
        let reply = feed
            .wait(tmp.path(), &json!({"v": 1, "after": 0, "timeout_ms": 3000}))
            .expect("answered");
        assert_eq!(reply["ok"]["cursor"], end);
        assert_eq!(
            reply["ok"]["event"]["data"]["last_assistant_message"],
            "first"
        );
        let again = feed
            .wait(
                tmp.path(),
                &json!({"v": 1, "after": start, "timeout_ms": 3000}),
            )
            .expect("answered");
        assert_eq!(again, reply);
    }

    #[test]
    fn wait_a_mid_line_after_skips_that_line() {
        let tmp = tempfile::tempdir().expect("tempdir");
        append(tmp.path(), &turn(json!("first")));
        let end = append(tmp.path(), &turn(json!("second")));
        let feed = WaitFeed::new(JumpClock::new());
        let reply = feed
            .wait(tmp.path(), &json!({"v": 1, "after": 1, "timeout_ms": 3000}))
            .expect("answered");
        assert_eq!(
            reply["ok"]["event"]["data"]["last_assistant_message"],
            "second"
        );
        assert_eq!(reply["ok"]["cursor"], end);
    }

    #[test]
    fn wait_without_after_starts_at_the_end_of_the_log() {
        let tmp = tempfile::tempdir().expect("tempdir");
        append(tmp.path(), &turn(json!("already")));
        let feed = WaitFeed::new(JumpClock::new());
        let reply = feed
            .wait(tmp.path(), &json!({"v": 1, "timeout_ms": 3000}))
            .expect("answered");
        assert_eq!(reply, json!({"ok": {"timed_out": true}}));
    }

    /// `timed_out` at the first reading past the deadline, never at one before it.
    #[test]
    fn wait_times_out_only_once_the_clock_passes_timeout_ms() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let clock = StepClock(Arc::default());
        let feed = WaitFeed::new(clock.clone());
        let reply = feed
            .wait(tmp.path(), &json!({"v": 1, "after": 0, "timeout_ms": 1000}))
            .expect("answered");
        assert_eq!(reply, json!({"ok": {"timed_out": true}}));
        let readings = clock.0.lock().expect("readings");
        let elapsed: Vec<Duration> = readings.iter().map(|r| *r - readings[0]).collect();
        let last = elapsed.len() - 1;
        assert!(elapsed[last] >= Duration::from_millis(1000), "{elapsed:?}");
        assert!(
            elapsed[last - 1] < Duration::from_millis(1000),
            "{elapsed:?}"
        );
    }

    /// A parked `wait` wakes on the signal of a line appended after it parked, and only on the
    /// signal: the same line on disk without one never ends the park.
    #[test]
    fn wait_a_line_appended_and_signalled_during_the_park_wakes_it() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (clock, parked) = announcing();
        let feed = WaitFeed::new(clock);
        let (done, result) = mpsc::channel();
        std::thread::scope(|s| {
            s.spawn(|| {
                let params = json!({"v": 1, "after": 0, "timeout_ms": 1_000_000_000});
                let _ = done.send(feed.wait(tmp.path(), &params));
            });
            parked.recv().expect("parked");
            let ended = turn(json!("woken"));
            let end = append(tmp.path(), &ended);
            for _ in 0..3 {
                parked.recv().expect("still parked");
            }
            assert!(result.try_recv().is_err(), "woke without a signal");
            feed.appended(&ended);
            let reply = result
                .recv_timeout(Duration::from_secs(5))
                .expect("woken")
                .expect("answered");
            assert_eq!(reply["ok"]["cursor"], end);
            assert_eq!(
                reply["ok"]["event"]["data"]["last_assistant_message"],
                "woken"
            );
        });
    }

    /// A signal for a log-only line re-scans and parks again.
    #[test]
    fn wait_a_signalled_log_only_line_parks_again() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (clock, parked) = announcing();
        let feed = WaitFeed::new(clock);
        let (done, result) = mpsc::channel();
        std::thread::scope(|s| {
            s.spawn(|| {
                let params = json!({"v": 1, "after": 0, "timeout_ms": 1_000_000_000});
                let _ = done.send(feed.wait(tmp.path(), &params));
            });
            parked.recv().expect("parked");
            let activity = line(EventKind::Activity, json!({}));
            append(tmp.path(), &activity);
            feed.appended(&activity);
            parked.recv().expect("parked again");
            assert!(result.try_recv().is_err(), "an activity line woke it");
            let ended = turn(Value::Null);
            append(tmp.path(), &ended);
            feed.appended(&ended);
            let reply = result
                .recv_timeout(Duration::from_secs(5))
                .expect("woken")
                .expect("answered");
            assert_eq!(reply["ok"]["event"]["kind"], "turn-ended");
        });
    }

    /// The largest `timeout_ms` is either no deadline (where `Instant` cannot hold it) or one
    /// beyond any session: it never wraps into an early one.
    #[test]
    fn wait_deadline_of_the_largest_timeout_never_comes_early() {
        let now = Instant::now();
        let ages = Duration::from_secs(1_000_000_000_000);
        assert!(deadline(now, u64::MAX).is_none_or(|d| d > now + ages));
        assert_eq!(deadline(now, 5), Some(now + Duration::from_millis(5)));
    }

    #[rstest]
    #[case::after_negative(json!({"v": 1, "after": -1}))]
    #[case::after_string(json!({"v": 1, "after": "x"}))]
    #[case::after_fraction(json!({"v": 1, "after": 1.5}))]
    #[case::after_null(json!({"v": 1, "after": null}))]
    #[case::timeout_string(json!({"v": 1, "timeout_ms": "x"}))]
    #[case::timeout_negative(json!({"v": 1, "timeout_ms": -5}))]
    #[case::from_not_a_name(json!({"v": 1, "from": "Bad Name"}))]
    #[case::from_number(json!({"v": 1, "from": 7}))]
    fn wait_params_it_cannot_take_are_invalid_params(#[case] params: Value) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let feed = WaitFeed::new(JumpClock::new());
        assert_eq!(
            feed.wait(tmp.path(), &params),
            Err(ProtocolError::InvalidParams)
        );
    }

    #[test]
    fn wait_an_extra_field_and_a_valid_from_are_no_fault() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let feed = WaitFeed::new(JumpClock::new());
        let params =
            json!({"v": 1, "after": 0, "timeout_ms": 1, "from": "overseer", "later": true});
        assert_eq!(
            feed.wait(tmp.path(), &params),
            Ok(json!({"ok": {"timed_out": true}}))
        );
    }

    #[test]
    fn wait_an_unreadable_log_is_internal() {
        let tmp = tempfile::tempdir().expect("tempdir");
        std::fs::create_dir(tmp.path().join("events.ndjson")).expect("a dir in its place");
        let feed = WaitFeed::new(JumpClock::new());
        assert_eq!(
            feed.wait(tmp.path(), &json!({"v": 1, "after": 0, "timeout_ms": 1})),
            Err(ProtocolError::Internal)
        );
    }

    #[test]
    fn last_before_any_turn_is_null_and_null() {
        let feed = WaitFeed::new(JumpClock::new());
        assert_eq!(
            feed.last(&json!({"v": 1})),
            Ok(json!({"ok": {"last_assistant_message": null, "ts": null}}))
        );
    }

    #[test]
    fn last_after_a_signalled_turn_is_its_message_and_ts() {
        let feed = WaitFeed::new(JumpClock::new());
        let ended = turn(json!("the answer"));
        feed.appended(&line(EventKind::Activity, json!({})));
        feed.appended(&ended);
        assert_eq!(
            feed.last(&json!({"v": 1})),
            Ok(json!({"ok": {"last_assistant_message": "the answer", "ts": ended.ts}}))
        );
        let silent = turn(Value::Null);
        feed.appended(&silent);
        assert_eq!(
            feed.last(&json!({"v": 1})),
            Ok(json!({"ok": {"last_assistant_message": null, "ts": silent.ts}}))
        );
    }

    #[test]
    fn last_after_a_rebuild_is_the_newest_turn_in_the_log() {
        let tmp = tempfile::tempdir().expect("tempdir");
        append(tmp.path(), &turn(json!("older")));
        let newest = turn(json!("newest"));
        append(tmp.path(), &newest);
        append(tmp.path(), &line(EventKind::SessionEnd, json!({})));
        let feed = WaitFeed::new(JumpClock::new());
        feed.rebuild(tmp.path()).expect("rebuilt");
        assert_eq!(
            feed.last(&json!({"v": 1})),
            Ok(json!({"ok": {"last_assistant_message": "newest", "ts": newest.ts}}))
        );
    }

    #[test]
    fn last_after_a_rebuild_of_no_turn_stays_null() {
        let tmp = tempfile::tempdir().expect("tempdir");
        append(tmp.path(), &line(EventKind::Activity, json!({})));
        let feed = WaitFeed::new(JumpClock::new());
        feed.rebuild(tmp.path()).expect("rebuilt");
        assert_eq!(
            feed.last(&json!({"v": 1})).expect("answered")["ok"]["ts"],
            Value::Null
        );
    }

    #[test]
    fn last_rebuild_of_an_unreadable_log_fails() {
        let tmp = tempfile::tempdir().expect("tempdir");
        std::fs::create_dir(tmp.path().join("events.ndjson")).expect("a dir in its place");
        let feed = WaitFeed::new(JumpClock::new());
        assert!(feed.rebuild(tmp.path()).is_err());
    }

    #[rstest]
    #[case::from_number(json!({"v": 1, "from": 7}))]
    #[case::from_not_a_name(json!({"v": 1, "from": "../x"}))]
    fn last_params_it_cannot_take_are_invalid_params(#[case] params: Value) {
        let feed = WaitFeed::new(JumpClock::new());
        assert_eq!(feed.last(&params), Err(ProtocolError::InvalidParams));
    }
}
