//! The wake arms: one wrapped start that asks its own session for a scheduled wake through
//! `viola send`, reads `turn-ended`'s `scheduled_wakes` (R3B), or stands until the wake fires and
//! reads the fired turn's capture line and hook-latency window (R3). Under a seeded kept stamp
//! (`R3W`) R3 also reads viola's own `wheel` lines past send 1's prompt and makes a second send
//! after the fired turn. Every reading is a closed word, a count or `unmeasured`; no key is typed.

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use serde_json::Value;

use super::isolate::{Scratch, now_ms};
use super::{
    Plan, SEND_BOUND, SUBMIT_BOUND, Session, Summary, await_kind, bounded, first_frame, new_lines,
    procs, record_reading, seeds_stamp, send_word,
};

/// One wake-arm turn's bound: a short Haiku turn.
const WAKE_TURN_BOUND: Duration = Duration::from_secs(120);
/// The wrapped instance a wake arm starts.
pub const WAKE_NAME: &str = "rig-r3";
const PLAIN_TEXT: &str = "Reply with the single word ok.";

/// The two request forms a wake arm sends.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wake {
    /// `/loop <N>m <prompt>`: a fixed-interval loop.
    Interval,
    /// `/loop <prompt>` asking the session to schedule its own next wake.
    Dynamic,
}

impl Wake {
    pub fn parse(word: &str) -> Option<Self> {
        match word {
            "interval" => Some(Self::Interval),
            "dynamic" => Some(Self::Dynamic),
            _ => None,
        }
    }

    pub const fn word(self) -> &'static str {
        match self {
            Self::Interval => "interval",
            Self::Dynamic => "dynamic",
        }
    }

    /// The request text: the shortest delay (1 minute / 60 s) when `short`, else 30 minutes.
    fn text(self, short: bool) -> String {
        match (self, short) {
            (Self::Interval, true) => "/loop 1m Reply with the single word ok.".to_owned(),
            (Self::Interval, false) => "/loop 30m Reply with the single word ok.".to_owned(),
            (Self::Dynamic, short) => format!(
                "/loop Reply with the single word ok, then schedule the next wake in {} seconds.",
                if short { 60 } else { 1800 }
            ),
        }
    }
}

/// A wrapped wake start's channel: its `viola send`, its events log and its capture file.
struct Wrapped<'a> {
    viola: &'a Path,
    vhome: &'a Path,
    cwd: &'a Path,
    env: &'a [(OsString, OsString)],
    texts: PathBuf,
    events: PathBuf,
    offset: u64,
    capture: PathBuf,
    sent: usize,
    /// The events offset just past send 1's own `prompt-submitted` line, once read.
    prompt_past: Option<u64>,
}

impl Wrapped<'_> {
    /// Sends `text` and waits its turn's end whatever the reply: the reply word and the turn-ended
    /// data, `None` at the bound.
    fn send(&mut self, text: &str, session: &mut Session, summary: &mut Summary) -> Option<Value> {
        self.sent += 1;
        let key = format!("send-{}", self.sent);
        let file = self.texts.join(format!("{}.txt", self.sent));
        let reply =
            if viola_state::fs::replace_private(&file, text.as_bytes(), viola_state::fs::FILE_MODE)
                .is_ok()
            {
                let mut command = Command::new(self.viola);
                command
                    .arg("--home")
                    .arg(self.vhome)
                    .args(["send", WAKE_NAME, "--json", "--file"])
                    .arg(&file)
                    .current_dir(self.cwd)
                    .env_clear()
                    .envs(self.env.iter().map(|(k, v)| (k, v)))
                    .stdin(Stdio::null());
                send_word(bounded(command, SEND_BOUND).as_ref())
            } else {
                "unwritten".to_owned()
            };
        summary.put(&format!("{key}-reply"), reply);
        // The send's own prompt line is passed first, so a later one is never taken for it.
        let prompt = await_kind(
            &self.events,
            &mut self.offset,
            "prompt-submitted",
            SUBMIT_BOUND,
            session,
        );
        if self.sent == 1 && prompt.is_some() {
            self.prompt_past = Some(self.offset);
        }
        let ended = await_kind(
            &self.events,
            &mut self.offset,
            "turn-ended",
            WAKE_TURN_BOUND,
            session,
        );
        session.keep(&key);
        if ended.is_none() {
            summary.put(&format!("{key}-turn"), "unended");
        }
        ended
    }

    fn capture_lines(&self) -> Vec<Value> {
        new_lines(&self.capture, 0)
            .into_iter()
            .map(|(line, _)| line)
            .collect()
    }
}

/// A turn end's `scheduled_wakes` as a count word, `null` when absent or not a count.
fn wakes_word(ended: Option<&Value>) -> String {
    ended
        .and_then(|d| d["scheduled_wakes"].as_u64())
        .map_or_else(|| "null".to_owned(), |n| n.to_string())
}

fn wakes_at_least_one(ended: Option<&Value>) -> bool {
    ended
        .and_then(|d| d["scheduled_wakes"].as_u64())
        .is_some_and(|n| n >= 1)
}

/// A capture line's `source` word: one of its closed words, or `unread`.
fn source_of(line: &Value) -> &'static str {
    match line["source"].as_str() {
        Some("schedule_wakeup") => "schedule_wakeup",
        Some("loop_wakeup") => "loop_wakeup",
        Some("other") => "other",
        Some("absent") => "absent",
        _ => "unread",
    }
}

/// A capture line's listed key names, comma-joined: `none` when empty, `unread` when it read none.
fn keys_of(line: &Value) -> String {
    let Some(keys) = line["keys"].as_array() else {
        return "unread".to_owned();
    };
    let names: Vec<&str> = keys
        .iter()
        .filter_map(Value::as_str)
        .filter(|k| {
            (1..=40).contains(&k.len()) && k.bytes().all(|b| b.is_ascii_lowercase() || b == b'_')
        })
        .collect();
    if names.is_empty() {
        "none".to_owned()
    } else {
        names.join(",")
    }
}

/// An events line's `ts` (`YYYY-MM-DDTHH:MM:SS.mmmZ`, UTC) in Unix milliseconds.
pub fn ts_ms(ts: &str) -> Option<i128> {
    let b = ts.as_bytes();
    if b.len() != 24
        || b[4] != b'-'
        || b[7] != b'-'
        || b[10] != b'T'
        || b[19] != b'.'
        || b[23] != b'Z'
    {
        return None;
    }
    let num = |from: usize, to: usize| -> Option<i128> { ts.get(from..to)?.parse().ok() };
    let (y, m, d) = (num(0, 4)?, num(5, 7)?, num(8, 10)?);
    let (hh, mm, ss, ms) = (num(11, 13)?, num(14, 16)?, num(17, 19)?, num(20, 23)?);
    // Days from the civil date (Howard Hinnant's algorithm).
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = era * 146_097 + doe - 719_468;
    Some((((days * 24 + hh) * 60 + mm) * 60 + ss) * 1000 + ms)
}

/// Shared start of both arms: the first frame, the `session-start` wait, the private texts dir.
fn open<'a>(
    plan: &'a Plan,
    scratch: &'a Scratch,
    env: &'a [(OsString, OsString)],
    session: &mut Session,
    summary: &mut Summary,
) -> Option<Wrapped<'a>> {
    let first = first_frame(plan, session);
    record_reading(summary, &first);
    summary.put("keys", "none");
    if first.trust_shown() || !first.is_box_up() {
        summary.put("refused", "shape");
        return None;
    }
    let (Some(vhome), Some(viola)) = (&scratch.vhome, &plan.viola) else {
        return None;
    };
    let mut wrapped = Wrapped {
        viola,
        vhome,
        cwd: &scratch.cwd,
        env,
        texts: scratch.root.join("texts"),
        events: vhome
            .join("instances")
            .join(WAKE_NAME)
            .join("events.ndjson"),
        offset: 0,
        capture: scratch.root.join("capture.ndjson"),
        sent: 0,
        prompt_past: None,
    };
    // A transport-only wrapper may forward no hook: the start goes on and records what it read.
    let started = await_kind(
        &wrapped.events,
        &mut wrapped.offset,
        "session-start",
        plan.settle,
        session,
    );
    if let Some(data) = &started {
        session.learn_id(data);
    }
    summary.put(
        "session-start",
        if started.is_some() { "seen" } else { "absent" },
    );
    if viola_state::fs::create_private_dir(&wrapped.texts).is_err() {
        summary.put("refused", "texts");
        return None;
    }
    Some(wrapped)
}

/// Send 1's capture lines: the count at its turn's end (or bound), written as the mark.
fn mark_send_one(wrapped: &Wrapped<'_>, root: &Path, summary: &mut Summary) -> usize {
    let lines = wrapped.capture_lines();
    let mark = lines.len();
    let _ = viola_state::fs::replace_private(
        &root.join("send-1.mark"),
        mark.to_string().as_bytes(),
        viola_state::fs::FILE_MODE,
    );
    summary.put("send-1-capture", mark.to_string());
    let sources: Vec<&str> = lines.iter().map(source_of).collect();
    summary.put(
        "send-1-source",
        if sources.is_empty() {
            "none".to_owned()
        } else {
            sources.join(",")
        },
    );
    mark
}

/// R3b: «a real wake reads 1» on `turn-ended`'s `scheduled_wakes`.
pub(super) fn arm_r3b(
    plan: &Plan,
    scratch: &Scratch,
    env: &[(OsString, OsString)],
    session: &mut Session,
    summary: &mut Summary,
) {
    let Some(mut wrapped) = open(plan, scratch, env, session, summary) else {
        return;
    };
    let first = wrapped.send(&Wake::Interval.text(false), session, summary);
    mark_send_one(&wrapped, &scratch.root, summary);
    summary.put("send-1-wakes", wakes_word(first.as_ref()));
    if procs::stop_requested().is_some() {
        return;
    }
    let interval = wakes_at_least_one(first.as_ref());
    let second_text = if interval {
        PLAIN_TEXT.to_owned()
    } else {
        Wake::Dynamic.text(false)
    };
    let second = wrapped.send(&second_text, session, summary);
    summary.put("send-2-wakes", wakes_word(second.as_ref()));
    summary.put(
        "wake-request",
        if interval {
            "interval"
        } else if wakes_at_least_one(second.as_ref()) {
            "dynamic"
        } else {
            "none"
        },
    );
}

/// Which sign of the fired wake stood first.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Sign {
    Event,
    Capture,
}

/// R3a and R3c: the fired wake's capture line and its hook-latency window.
pub(super) fn arm_r3(
    plan: &Plan,
    scratch: &Scratch,
    env: &[(OsString, OsString)],
    session: &mut Session,
    summary: &mut Summary,
) {
    let Some(mut wrapped) = open(plan, scratch, env, session, summary) else {
        return;
    };
    summary.put("wake-form", plan.wake.word());
    let first = wrapped.send(&plan.wake.text(true), session, summary);
    let mark = mark_send_one(&wrapped, &scratch.root, summary);
    summary.put("send-1-wakes", wakes_word(first.as_ref()));
    let idle = session.reading();
    let mut left_idle_ms: Option<u128> = None;
    let mut fired_line: Option<Value> = None;
    let mut sign: Option<Sign> = None;
    let until = Instant::now() + plan.stand;
    while sign.is_none() && Instant::now() < until && procs::stop_requested().is_none() {
        session.pump(Duration::from_millis(100));
        if left_idle_ms.is_none() && session.reading() != idle {
            left_idle_ms = Some(now_ms());
            // The frame a `screen-ms` reading measures from.
            session.keep("left-idle");
        }
        let event = new_lines(&wrapped.events, wrapped.offset)
            .into_iter()
            .find(|(l, _)| l["kind"] == "prompt-submitted");
        if let Some((line, past)) = event {
            wrapped.offset = past;
            fired_line = Some(line);
            sign = Some(Sign::Event);
        }
        // The capture sign: a line past send 1's mark.
        if wrapped.capture_lines().len() > mark {
            sign = match sign {
                Some(Sign::Event) => Some(first_of(fired_line.as_ref(), &wrapped, mark)),
                _ => Some(Sign::Capture),
            };
        }
    }
    let fired = sign.is_some();
    if fired {
        if fired_line.is_none() {
            // The capture came alone: the wrapper's line, if any, is read during the fired turn.
            if let Some(data) = await_kind(
                &wrapped.events,
                &mut wrapped.offset,
                "prompt-submitted",
                SUBMIT_BOUND,
                session,
            ) {
                fired_line = Some(
                    serde_json::json!({"data": data, "ts": last_ts(&wrapped, "prompt-submitted")}),
                );
            }
        }
        if await_kind(
            &wrapped.events,
            &mut wrapped.offset,
            "turn-ended",
            WAKE_TURN_BOUND,
            session,
        )
        .is_none()
        {
            summary.put("fired-turn", "unended");
        }
        session.keep("fired");
    }
    let lines = wrapped.capture_lines();
    summary.put("wake", if fired { "fired" } else { "unfired" });
    summary.put(
        "wake-sign",
        match sign {
            Some(Sign::Event) => "event",
            Some(Sign::Capture) => "capture",
            None => "none",
        },
    );
    summary.put(
        "wake-origin",
        match fired_line
            .as_ref()
            .and_then(|l| l["data"]["origin"].as_str())
        {
            Some("human") => "human",
            Some("harness") => "harness",
            Some(_) => "unknown",
            None => "absent",
        },
    );
    summary.put("capture-lines", lines.len().to_string());
    let fired_capture = lines.get(mark);
    let shown = fired_capture.or_else(|| lines.last());
    summary.put("capture-source", shown.map_or("unread", source_of));
    summary.put(
        "capture-keys",
        shown.map_or_else(|| "unread".to_owned(), keys_of),
    );
    let fired_ts = fired_line
        .as_ref()
        .and_then(|l| l["ts"].as_str())
        .and_then(ts_ms);
    let capture_ms = fired_capture.and_then(|l| l["ms"].as_u64()).map(i128::from);
    summary.put(
        "latency-ms",
        match (fired_ts, capture_ms) {
            (Some(t), Some(c)) => (t - c).to_string(),
            _ => "unmeasured".to_owned(),
        },
    );
    summary.put(
        "screen-ms",
        match (fired_ts, left_idle_ms.and_then(|m| i128::try_from(m).ok())) {
            (Some(t), Some(s)) => (t - s).to_string(),
            _ => "unmeasured".to_owned(),
        },
    );
    if seeds_stamp(&plan.label) {
        summary.put(
            "wheel-after-fire",
            wheel_after_fire(&wrapped.events, wrapped.prompt_past),
        );
        if fired && procs::stop_requested().is_none() {
            session.settle(super::screen::Reading::is_box_up, plan.settle);
            wrapped.send(PLAIN_TEXT, session, summary);
        }
    }
}

/// viola's own `wheel` lines in the events log past `from` (send 1's prompt): `kept` for none,
/// `human` when one hands the wheel to the human, `other` for any other, `unread` for no offset or
/// no log.
fn wheel_after_fire(events: &Path, from: Option<u64>) -> &'static str {
    let Some(from) = from else {
        return "unread";
    };
    if !std::fs::symlink_metadata(events).is_ok_and(|m| m.is_file()) {
        return "unread";
    }
    let wheels: Vec<Value> = new_lines(events, from)
        .into_iter()
        .map(|(line, _)| line)
        .filter(|line| line["kind"] == "wheel")
        .collect();
    if wheels.is_empty() {
        "kept"
    } else if wheels.iter().any(|w| w["data"]["holder"] == "human") {
        "human"
    } else {
        "other"
    }
}

/// Both signs seen in one poll: the earlier by their own clocks, the capture on a tie or a missing
/// clock.
fn first_of(event: Option<&Value>, wrapped: &Wrapped<'_>, mark: usize) -> Sign {
    let event_ms = event.and_then(|l| l["ts"].as_str()).and_then(ts_ms);
    let capture_ms = wrapped
        .capture_lines()
        .get(mark)
        .and_then(|l| l["ms"].as_u64())
        .map(i128::from);
    match (event_ms, capture_ms) {
        (Some(e), Some(c)) if e < c => Sign::Event,
        _ => Sign::Capture,
    }
}

/// The `ts` of the last line of `kind` before the events offset.
fn last_ts(wrapped: &Wrapped<'_>, kind: &str) -> Value {
    new_lines(&wrapped.events, 0)
        .into_iter()
        .take_while(|(_, past)| *past <= wrapped.offset)
        .filter(|(l, _)| l["kind"] == kind)
        .last()
        .map_or(Value::Null, |(l, _)| l["ts"].clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn the_request_texts_are_the_compiled_forms() {
        assert_eq!(
            Wake::Interval.text(true),
            "/loop 1m Reply with the single word ok."
        );
        assert_eq!(
            Wake::Interval.text(false),
            "/loop 30m Reply with the single word ok."
        );
        assert!(Wake::Dynamic.text(true).ends_with("in 60 seconds."));
        assert!(Wake::Dynamic.text(false).ends_with("in 1800 seconds."));
        assert_eq!(Wake::parse("dynamic"), Some(Wake::Dynamic));
        assert_eq!(Wake::parse("cron"), None);
    }

    #[test]
    fn an_events_ts_reads_as_unix_milliseconds() {
        assert_eq!(ts_ms("1970-01-01T00:00:00.000Z"), Some(0));
        assert_eq!(ts_ms("2026-11-17T16:26:19.123Z"), Some(1_794_932_779_123));
        assert_eq!(ts_ms("2026-11-17T16:26:19Z"), None);
    }

    #[test]
    fn a_turn_end_reads_its_wake_count_or_null() {
        assert_eq!(wakes_word(Some(&json!({"scheduled_wakes": 1}))), "1");
        assert_eq!(wakes_word(Some(&json!({"scheduled_wakes": null}))), "null");
        assert_eq!(wakes_word(None), "null");
        assert!(!wakes_at_least_one(Some(&json!({"scheduled_wakes": 0}))));
        assert!(wakes_at_least_one(Some(&json!({"scheduled_wakes": 2}))));
    }

    /// The four words over a fed events file, each fed line's literal copied from what was fed.
    #[test]
    fn wheel_after_fire_reads_kept_human_other_or_unread() {
        let dir = tempfile::tempdir().expect("tempdir");
        let events = dir.path().join("events.ndjson");
        assert_eq!(wheel_after_fire(&events, Some(0)), "unread", "no log");
        let prompt = "{\"kind\":\"prompt-submitted\",\"data\":{\"origin\":\"driver\"}}\n";
        let start = "{\"kind\":\"wheel\",\"data\":{\"holder\":\"driver\",\"cause\":\"start\"}}\n";
        std::fs::write(&events, format!("{start}{prompt}")).expect("events");
        let past = u64::try_from(start.len() + prompt.len()).expect("len");
        assert_eq!(wheel_after_fire(&events, None), "unread", "no offset");
        assert_eq!(wheel_after_fire(&events, Some(past)), "kept");
        let fired = "{\"kind\":\"prompt-submitted\",\"data\":{\"origin\":\"harness\"}}\n";
        let ended = "{\"kind\":\"turn-ended\",\"data\":{\"scheduled_wakes\":1}}\n";
        std::fs::write(&events, format!("{start}{prompt}{fired}{ended}")).expect("events");
        assert_eq!(
            wheel_after_fire(&events, Some(past)),
            "kept",
            "no wheel line"
        );
        assert_eq!(
            wheel_after_fire(&events, Some(0)),
            "other",
            "the start record"
        );
        let taken =
            "{\"kind\":\"wheel\",\"data\":{\"holder\":\"human\",\"cause\":\"human-input\"}}\n";
        std::fs::write(&events, format!("{start}{prompt}{taken}{fired}")).expect("events");
        assert_eq!(wheel_after_fire(&events, Some(past)), "human");
    }

    #[test]
    fn a_capture_line_reads_only_closed_words_and_listed_keys() {
        let line = json!({"keys": ["prompt", "source", "Bad"], "source": "loop_wakeup"});
        assert_eq!(source_of(&line), "loop_wakeup");
        assert_eq!(keys_of(&line), "prompt,source");
        assert_eq!(source_of(&json!({"source": "/a/path"})), "unread");
        assert_eq!(keys_of(&json!({"unread": true})), "unread");
        assert_eq!(keys_of(&json!({"keys": []})), "none");
    }
}
