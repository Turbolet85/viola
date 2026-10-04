//! The wrapper's `send` (architecture [Delivery Confirmation]): the text re-validated, one send in
//! flight, the readiness gate, `send-issued` at the pre-paste cursor, one bracketed paste + Enter,
//! then confirmation after the fact — the matching `prompt-submitted`, relabelled `driver`, inside
//! the window — or `not-delivered`. Each outcome is an `events.ndjson` record and a codes-only
//! `send-*` line; the text reaches neither.

use std::path::Path;
use std::sync::{Condvar, Mutex, MutexGuard, OnceLock, PoisonError};
use std::time::{Duration, Instant};

use chrono::Utc;
use serde_json::{Value, json};
use tracing::instrument;
use viola_agent_claude::screen::{CONFIRM_WINDOW_FALLBACK, Readiness};
use viola_channel::{Call, ProtocolError};
use viola_core::obs::ObsEvent;
use viola_core::{Clock, EventKind, NotDelivered, RefusalReason, ViolaName, obs_event};
use viola_pty::PtyError;
use viola_state::StateError;
use viola_state::events::{EventLine, Source, append_event, append_event_at, end_offset};

use crate::run::gate::Gate;
use crate::run::wait::WaitFeed;

/// How often the waiting handler re-reads the clock against the window.
const STEP: Duration = Duration::from_millis(20);

pub(crate) type PasteFn = Box<dyn Fn(&str) -> Result<(), PtyError> + Send + Sync>;

enum Match {
    Waiting,
    /// A matching prompt is being appended; the outcome waits for it whatever the window.
    Claimed,
    Confirmed(String),
    /// The outcome is decided; the slot stays held until the send has recorded it.
    Closed,
}

struct InFlight {
    text: String,
    state: Match,
}

/// The child's input and gate, set once the pump starts, and the one send in flight.
pub(crate) struct SendSlot {
    clock: Box<dyn Clock>,
    io: OnceLock<(PasteFn, Gate)>,
    in_flight: Mutex<Option<InFlight>>,
    settled: Condvar,
}

impl SendSlot {
    pub(crate) fn new(clock: impl Clock + 'static) -> Self {
        Self {
            clock: Box::new(clock),
            io: OnceLock::new(),
            in_flight: Mutex::new(None),
            settled: Condvar::new(),
        }
    }

    /// The first call wins; the pump calls it once, before it starts.
    pub(crate) fn attach(&self, paste: PasteFn, gate: Gate) {
        let _ = self.io.set((paste, gate));
    }

    fn flight(&self) -> MutexGuard<'_, Option<InFlight>> {
        self.in_flight
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    /// Whether a prompt with `text` is the in-flight send's: it is then claimed, and only
    /// [`SendSlot::settle`] resolves it.
    pub(crate) fn claim(&self, text: &str) -> bool {
        let mut flight = self.flight();
        match flight.as_mut() {
            Some(f) if matches!(f.state, Match::Waiting) && f.text == text => {
                f.state = Match::Claimed;
                true
            }
            _ => false,
        }
    }

    /// The claimed prompt's line was appended at `ts`, or (`None`) was not, and the send waits on.
    pub(crate) fn settle(&self, ts: Option<String>) {
        let mut flight = self.flight();
        if let Some(f) = flight
            .as_mut()
            .filter(|f| matches!(f.state, Match::Claimed))
        {
            f.state = ts.map_or(Match::Waiting, Match::Confirmed);
        }
        self.settled.notify_all();
    }

    /// The window: `Some(submitted_at)` on a match, `None` once it expires unclaimed. The send is
    /// closed under the same lock, so a prompt after the expiry is never claimed; the slot itself
    /// is emptied only by the send's `Reserved` guard, once its outcome is recorded.
    #[instrument(skip_all, name = "run.confirm_window", fields(window_ms = window_ms()))]
    fn confirm(&self, opened: Instant) -> Option<String> {
        let deadline = opened + CONFIRM_WINDOW_FALLBACK;
        let mut flight = self.flight();
        loop {
            match flight.as_ref().map(|f| &f.state) {
                Some(Match::Confirmed(ts)) => {
                    let ts = ts.clone();
                    close(&mut flight);
                    return Some(ts);
                }
                Some(Match::Waiting) if self.clock.now() >= deadline => {
                    close(&mut flight);
                    return None;
                }
                None | Some(Match::Closed) => return None,
                Some(Match::Waiting | Match::Claimed) => {}
            }
            flight = self
                .settled
                .wait_timeout(flight, STEP)
                .unwrap_or_else(PoisonError::into_inner)
                .0;
        }
    }
}

/// Appends a hook's event and signals the parked `wait`s once it is on disk. The in-flight send's
/// own prompt, whatever origin the hook filed, is the driver's: the match keys on the normalised
/// text, never on the hook's origin, and the waiting send settles only once the relabelled line is
/// on disk.
pub(crate) fn append_hook_event(
    slot: &SendSlot,
    feed: &WaitFeed,
    instance_dir: &Path,
    mut line: EventLine,
) -> Result<(), StateError> {
    let claimed = line.kind == EventKind::PromptSubmitted
        && line.data["text"]
            .as_str()
            .is_some_and(|text| slot.claim(text));
    if claimed {
        line.data["origin"] = json!("driver");
    }
    let appended = append_event(instance_dir, &line);
    if claimed {
        slot.settle(appended.is_ok().then(|| line.ts.clone()));
    }
    if appended.is_ok() {
        feed.appended(&line);
    }
    appended
}

fn close(flight: &mut Option<InFlight>) {
    if let Some(f) = flight.as_mut() {
        f.state = Match::Closed;
    }
}

fn window_ms() -> u64 {
    u64::try_from(CONFIRM_WINDOW_FALLBACK.as_millis()).unwrap_or(u64::MAX)
}

/// Empties the slot on every path out of a send that reserved it, a panic included.
struct Reserved<'a>(&'a SendSlot);

impl Drop for Reserved<'_> {
    fn drop(&mut self) {
        *self.0.flight() = None;
    }
}

/// What a send's lines and records carry besides the outcome.
struct Ctx<'a> {
    name: &'a ViolaName,
    instance_dir: &'a Path,
    call: &'a Call<'a>,
    from: Option<ViolaName>,
    started: Instant,
}

/// `params` = `{text, from?}`; anything else is `-32602`.
fn parse(params: &Value) -> Result<(&str, Option<ViolaName>), ProtocolError> {
    let text = params["text"]
        .as_str()
        .ok_or(ProtocolError::InvalidParams)?;
    Ok((text, parse_from(params)?))
}

/// `from`: absent, `null` or a valid name, self-reported; anything else is `-32602`.
pub(crate) fn parse_from(params: &Value) -> Result<Option<ViolaName>, ProtocolError> {
    match params.get("from") {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(from)) => ViolaName::try_new(from.clone())
            .map(Some)
            .map_err(|_| ProtocolError::InvalidParams),
        Some(_) => Err(ProtocolError::InvalidParams),
    }
}

/// The `send` method, in the documented refusal order.
pub(crate) fn send(
    slot: &SendSlot,
    name: &ViolaName,
    instance_dir: &Path,
    call: &Call<'_>,
) -> Result<Value, ProtocolError> {
    let (text, from) = parse(call.params)?;
    let ctx = Ctx {
        name,
        instance_dir,
        call,
        from,
        started: slot.clock.now(),
    };
    if let Err(detail) = viola_core::validate_paste_text(text) {
        return ctx.refuse(None, detail);
    }
    {
        let mut flight = slot.flight();
        if flight.is_some() {
            drop(flight);
            return ctx.refuse(None, NotDelivered::TurnRunning);
        }
        *flight = Some(InFlight {
            text: text.to_owned(),
            state: Match::Waiting,
        });
    }
    let _reserved = Reserved(slot);
    let Some((paste, gate)) = slot.io.get() else {
        return ctx.refuse(None, NotDelivered::InputNotReady);
    };
    let (readiness, _) = gate.wait_ready(slot.clock.as_ref(), slot.clock.now());
    if readiness != Readiness::Ready {
        return ctx.refuse(None, NotDelivered::InputNotReady);
    }
    let cursor = ctx.issue(text.len())?;
    if paste(text).is_err() {
        return ctx.refuse(Some(cursor), NotDelivered::InputNotReady);
    }
    match slot.confirm(slot.clock.now()) {
        Some(submitted_at) => ctx.confirm(cursor, &submitted_at, slot.clock.now()),
        None => ctx.refuse(Some(cursor), NotDelivered::NoPromptSubmitted),
    }
}

impl Ctx<'_> {
    fn sender(&self) -> Option<&str> {
        self.from.as_ref().map(AsRef::as_ref)
    }

    fn trust(&self) -> Option<&'static str> {
        self.from.as_ref().map(|_| "self-reported")
    }

    fn record(&self, kind: EventKind, data: Value) -> Result<u64, ProtocolError> {
        append_event_at(self.instance_dir, |_| {
            EventLine::new(self.name, kind, Source::Wrapper, data, Utc::now())
        })
        .map_err(|_| ProtocolError::Internal)
    }

    /// `send-issued` at the end offset it is appended at, which is the send's cursor.
    fn issue(&self, text_bytes: usize) -> Result<u64, ProtocolError> {
        let from = self.sender();
        let cursor = append_event_at(self.instance_dir, |cursor| {
            let mut data = json!({"cursor": cursor});
            if let Some(from) = from {
                data["from"] = json!(from);
            }
            EventLine::new(
                self.name,
                EventKind::SendIssued,
                Source::Wrapper,
                data,
                Utc::now(),
            )
        })
        .map_err(|_| ProtocolError::Internal)?;
        obs_event!(
            INFO,
            ObsEvent::SendIssued,
            corr = cursor,
            rpc_id = self.call.id,
            conn = self.call.conn,
            srv_conn = self.call.srv_conn,
            from = from,
            from_trust = self.trust(),
            text_bytes = u64::try_from(text_bytes).unwrap_or(u64::MAX),
        );
        Ok(cursor)
    }

    fn confirm(
        &self,
        cursor: u64,
        submitted_at: &str,
        now: Instant,
    ) -> Result<Value, ProtocolError> {
        self.record(EventKind::SendConfirmed, json!({"cursor": cursor}))?;
        let duration_ms = u64::try_from(now.saturating_duration_since(self.started).as_millis())
            .unwrap_or(u64::MAX);
        obs_event!(
            INFO,
            ObsEvent::SendConfirmed,
            corr = cursor,
            rpc_id = self.call.id,
            conn = self.call.conn,
            srv_conn = self.call.srv_conn,
            from = self.sender(),
            from_trust = self.trust(),
            confirmed = true,
            duration_ms = duration_ms,
        );
        Ok(json!({"ok": {"submitted_at": submitted_at, "cursor": cursor}}))
    }

    /// A `not-delivered` refusal: before `send-issued` (no cursor; the line's `corr` is the end
    /// offset at refusal, obs-plan D-28) or after it (the send's own cursor).
    fn refuse(&self, cursor: Option<u64>, detail: NotDelivered) -> Result<Value, ProtocolError> {
        let refusal = RefusalReason::NotDelivered;
        let mut data = json!({"refusal": refusal.as_str(), "detail": detail.as_str()});
        if let Some(cursor) = cursor {
            data["cursor"] = json!(cursor);
        }
        let corr = match cursor {
            Some(cursor) => Some(cursor),
            None => end_offset(self.instance_dir).ok(),
        };
        self.record(EventKind::SendRefused, data)?;
        obs_event!(
            INFO,
            ObsEvent::SendRefused,
            corr = corr,
            rpc_id = self.call.id,
            conn = self.call.conn,
            srv_conn = self.call.srv_conn,
            from = self.sender(),
            from_trust = self.trust(),
            side = "wrapper",
            refusal = refusal.as_str(),
            detail = detail.as_str(),
        );
        Ok(json!({"refusal": refusal.as_str(), "detail": detail.as_str()}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::mpsc;

    use rstest::rstest;
    use viola_pty::Size;

    use crate::run::gate;

    const CANARY: &str = "canary-chain-value-5c1e\nsecond line";

    #[derive(Clone, Copy)]
    struct FixedClock(Instant);

    impl Clock for FixedClock {
        fn now(&self) -> Instant {
            self.0
        }
    }

    /// Every reading is a second past the previous one: a window expires in a few steps.
    struct JumpClock(Mutex<Instant>);

    impl Clock for JumpClock {
        fn now(&self) -> Instant {
            let mut now = self.0.lock().expect("clock");
            *now += Duration::from_secs(1);
            *now
        }
    }

    fn name() -> ViolaName {
        ViolaName::try_new("builder".to_owned()).expect("valid")
    }

    /// A gate whose screen went quiet a second before `base`.
    fn quiet_gate(base: Instant) -> Gate {
        let (feeder, gate, thread) =
            gate::start(FixedClock(base - Duration::from_secs(1)), Size::DEFAULT);
        drop(feeder);
        thread.join().expect("feed thread");
        gate
    }

    /// A gate poisoned by a vt100 panic (a wide character at one column).
    fn poisoned_gate(base: Instant) -> Gate {
        let (feeder, gate, thread) = gate::start(FixedClock(base), Size { cols: 1, rows: 24 });
        let mut tee = gate::Tee::new(Vec::new(), feeder);
        std::io::Write::write_all(&mut tee, b"\xe4\xb8\xad").expect("write");
        drop(tee);
        thread.join().expect("feed thread");
        gate
    }

    /// Every paste, in order; each one also announced on `pasted`.
    #[derive(Clone)]
    struct Pastes(Arc<Mutex<Vec<String>>>, mpsc::SyncSender<()>);

    impl Pastes {
        fn new() -> (Self, mpsc::Receiver<()>) {
            let (tx, rx) = mpsc::sync_channel(8);
            (Self(Arc::default(), tx), rx)
        }

        fn paste_fn(&self) -> PasteFn {
            let pastes = self.clone();
            Box::new(move |text| {
                pastes.0.lock().expect("pastes").push(text.to_owned());
                let _ = pastes.1.send(());
                Ok(())
            })
        }

        fn all(&self) -> Vec<String> {
            self.0.lock().expect("pastes").clone()
        }
    }

    fn call(params: &Value, id: u64) -> Call<'_> {
        Call {
            method: "send",
            params,
            id: Some(id),
            conn: Some("cli-1-2-3"),
            srv_conn: None,
        }
    }

    fn events(dir: &Path) -> Vec<Value> {
        std::fs::read_to_string(dir.join("events.ndjson"))
            .unwrap_or_default()
            .lines()
            .map(|l| serde_json::from_str(l).expect("one JSON object per line"))
            .collect()
    }

    fn kinds(dir: &Path) -> Vec<String> {
        events(dir)
            .iter()
            .map(|e| e["kind"].as_str().expect("kind").to_owned())
            .collect()
    }

    fn prompt(text: &str, origin: &str) -> EventLine {
        EventLine::new(
            &name(),
            EventKind::PromptSubmitted,
            Source::Hook,
            json!({"text": text, "origin": origin}),
            Utc::now(),
        )
    }

    fn feed() -> WaitFeed {
        WaitFeed::new(FixedClock(Instant::now()))
    }

    fn occupy(slot: &SendSlot) {
        *slot.flight() = Some(InFlight {
            text: "another send".to_owned(),
            state: Match::Waiting,
        });
    }

    #[derive(Clone, Copy)]
    enum Setup {
        InFlightNoChild,
        NoChild,
        Poisoned,
        Mute,
    }

    /// The `send` order, written out: control-character ahead of everything, then the one in
    /// flight, then the gate, then the window.
    #[rstest]
    #[case::control_character_before_turn_running(
        "x\u{1b}[201~",
        Setup::InFlightNoChild,
        "control-character"
    )]
    #[case::turn_running_before_input_not_ready("hello", Setup::InFlightNoChild, "turn-running")]
    #[case::input_not_ready_without_a_child("hello", Setup::NoChild, "input-not-ready")]
    #[case::input_not_ready_on_a_poisoned_screen("hello", Setup::Poisoned, "input-not-ready")]
    #[case::no_prompt_submitted_when_the_window_expires(
        "hello",
        Setup::Mute,
        "no-prompt-submitted"
    )]
    fn send_refusal_order(#[case] text: &str, #[case] setup: Setup, #[case] detail: &str) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let base = Instant::now();
        let slot = SendSlot::new(JumpClock(Mutex::new(base)));
        let (pastes, _pasted) = Pastes::new();
        match setup {
            Setup::InFlightNoChild => occupy(&slot),
            Setup::NoChild => {}
            Setup::Poisoned => slot.attach(pastes.paste_fn(), poisoned_gate(base)),
            Setup::Mute => slot.attach(pastes.paste_fn(), quiet_gate(base)),
        }
        let params = json!({"v": 1, "text": text});
        let reply = send(&slot, &name(), tmp.path(), &call(&params, 7)).expect("answered");
        assert_eq!(reply, json!({"refusal": "not-delivered", "detail": detail}));
        let events = events(tmp.path());
        let refused = events.last().expect("a send-refused record");
        assert_eq!(refused["kind"], "send-refused");
        assert_eq!(refused["source"], "wrapper");
        assert_eq!(refused["data"]["refusal"], "not-delivered");
        assert_eq!(refused["data"]["detail"], detail);
        if matches!(setup, Setup::Mute) {
            assert_eq!(kinds(tmp.path()), ["send-issued", "send-refused"]);
            assert_eq!(refused["data"]["cursor"], 0);
            assert_eq!(pastes.all(), ["hello"]);
        } else {
            assert_eq!(kinds(tmp.path()), ["send-refused"]);
            assert!(refused["data"].get("cursor").is_none());
            assert!(pastes.all().is_empty(), "nothing typed");
        }
        if matches!(setup, Setup::InFlightNoChild) {
            assert!(slot.flight().is_some(), "the other send keeps its slot");
        } else {
            assert!(slot.flight().is_none(), "the slot is free again");
        }
    }

    /// A send whose prompt the hook reports back: the waiting handler wakes on it.
    fn confirmed_with(origin: &str) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let base = Instant::now();
        let slot = SendSlot::new(FixedClock(base));
        let (pastes, pasted) = Pastes::new();
        slot.attach(pastes.paste_fn(), quiet_gate(base));
        std::fs::write(
            tmp.path().join("events.ndjson"),
            b"{\"kind\":\"earlier\"}\n",
        )
        .expect("seed");
        let params = json!({"v": 1, "text": CANARY, "from": "overseer"});
        let reply = std::thread::scope(|s| {
            let sending = s.spawn(|| send(&slot, &name(), tmp.path(), &call(&params, 3)));
            pasted.recv().expect("pasted");
            append_hook_event(&slot, &feed(), tmp.path(), prompt(CANARY, origin)).expect("hook");
            sending.join().expect("send thread")
        })
        .expect("answered");
        let events = events(tmp.path());
        assert_eq!(
            kinds(tmp.path())[1..],
            ["send-issued", "prompt-submitted", "send-confirmed"]
        );
        assert_eq!(events[1]["data"], json!({"cursor": 19, "from": "overseer"}));
        assert_eq!(
            events[2]["data"],
            json!({"text": CANARY, "origin": "driver"})
        );
        assert_eq!(events[3]["data"], json!({"cursor": 19}));
        assert_eq!(
            reply,
            json!({"ok": {"submitted_at": events[2]["ts"], "cursor": 19}})
        );
        assert_eq!(pastes.all(), [CANARY]);
        assert!(slot.flight().is_none());
    }

    #[test]
    fn send_match_is_ok_with_the_prompt_ts_and_the_cursor() {
        confirmed_with("human");
    }

    #[test]
    fn send_harness_filed_prompt_with_the_in_flight_text_is_relabelled_driver() {
        confirmed_with("harness");
    }

    #[test]
    fn send_a_different_prompt_is_appended_unchanged_and_claims_nothing() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = SendSlot::new(FixedClock(Instant::now()));
        occupy(&slot);
        append_hook_event(
            &slot,
            &feed(),
            tmp.path(),
            prompt("typed by the human", "human"),
        )
        .expect("hook");
        assert_eq!(
            events(tmp.path())[0]["data"],
            json!({"text": "typed by the human", "origin": "human"})
        );
        assert!(matches!(
            slot.flight().as_ref().map(|f| &f.state),
            Some(Match::Waiting)
        ));
    }

    #[test]
    fn send_no_prompt_while_nothing_is_in_flight_is_appended_unchanged() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = SendSlot::new(FixedClock(Instant::now()));
        append_hook_event(
            &slot,
            &feed(),
            tmp.path(),
            prompt("another send", "harness"),
        )
        .expect("hook");
        assert_eq!(events(tmp.path())[0]["data"]["origin"], "harness");
    }

    /// A send whose outcome is decided but not yet recorded still holds the slot: a next send is
    /// `turn-running`, a late prompt with its text is not claimed, and only its own guard frees it.
    #[test]
    fn send_closed_send_keeps_the_slot_until_its_guard_drops() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = SendSlot::new(FixedClock(Instant::now()));
        *slot.flight() = Some(InFlight {
            text: "decided".to_owned(),
            state: Match::Closed,
        });
        let params = json!({"v": 1, "text": "next"});
        let reply = send(&slot, &name(), tmp.path(), &call(&params, 1)).expect("answered");
        assert_eq!(reply["detail"], "turn-running");
        append_hook_event(&slot, &feed(), tmp.path(), prompt("decided", "human")).expect("hook");
        assert_eq!(events(tmp.path())[1]["data"]["origin"], "human");
        assert!(slot.confirm(Instant::now()).is_none());
        assert!(
            slot.flight().is_some(),
            "only the send's own guard frees it"
        );
        drop(Reserved(&slot));
        assert!(slot.flight().is_none());
    }

    #[test]
    fn send_window_expiry_is_no_prompt_submitted_with_the_cursor() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let base = Instant::now();
        let slot = SendSlot::new(JumpClock(Mutex::new(base)));
        let (pastes, _pasted) = Pastes::new();
        slot.attach(pastes.paste_fn(), quiet_gate(base));
        let params = json!({"v": 1, "text": "hello"});
        let reply = send(&slot, &name(), tmp.path(), &call(&params, 1)).expect("answered");
        assert_eq!(
            reply,
            json!({"refusal": "not-delivered", "detail": "no-prompt-submitted"})
        );
        let events = events(tmp.path());
        assert_eq!(events[1]["data"]["cursor"], 0);
        // A prompt after the window is no longer the send's.
        append_hook_event(&slot, &feed(), tmp.path(), prompt("hello", "human")).expect("hook");
        assert_eq!(
            super::tests::events(tmp.path())[2]["data"]["origin"],
            "human"
        );
    }

    #[test]
    fn send_second_while_one_is_in_flight_is_turn_running_and_types_nothing() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let base = Instant::now();
        let slot = SendSlot::new(FixedClock(base));
        let (pastes, pasted) = Pastes::new();
        slot.attach(pastes.paste_fn(), quiet_gate(base));
        let first = json!({"v": 1, "text": "first"});
        let second = json!({"v": 1, "text": "second"});
        let (a, b) = std::thread::scope(|s| {
            let sending = s.spawn(|| send(&slot, &name(), tmp.path(), &call(&first, 1)));
            pasted.recv().expect("pasted");
            let b = send(&slot, &name(), tmp.path(), &call(&second, 2));
            append_hook_event(&slot, &feed(), tmp.path(), prompt("first", "human")).expect("hook");
            (sending.join().expect("send thread"), b)
        });
        assert_eq!(
            b.expect("answered"),
            json!({"refusal": "not-delivered", "detail": "turn-running"})
        );
        assert_eq!(a.expect("answered")["ok"]["cursor"], 0);
        assert_eq!(pastes.all(), ["first"]);
        assert_eq!(
            kinds(tmp.path()),
            [
                "send-issued",
                "send-refused",
                "prompt-submitted",
                "send-confirmed"
            ]
        );
    }

    #[test]
    fn send_a_failed_paste_is_input_not_ready_with_the_cursor() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let base = Instant::now();
        let slot = SendSlot::new(FixedClock(base));
        slot.attach(Box::new(|_| Err(PtyError::NoInput)), quiet_gate(base));
        let params = json!({"v": 1, "text": "hello"});
        let reply = send(&slot, &name(), tmp.path(), &call(&params, 1)).expect("answered");
        assert_eq!(reply["detail"], "input-not-ready");
        assert_eq!(events(tmp.path())[1]["data"]["cursor"], 0);
        assert!(slot.flight().is_none());
    }

    #[rstest]
    #[case::no_text(json!({"v": 1}))]
    #[case::text_not_string(json!({"v": 1, "text": 5}))]
    #[case::from_not_a_name(json!({"v": 1, "text": "x", "from": "../x"}))]
    #[case::from_not_string(json!({"v": 1, "text": "x", "from": 3}))]
    fn send_params_it_cannot_take_are_invalid_params(#[case] params: Value) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = SendSlot::new(FixedClock(Instant::now()));
        assert_eq!(
            send(&slot, &name(), tmp.path(), &call(&params, 1)),
            Err(ProtocolError::InvalidParams)
        );
        assert!(events(tmp.path()).is_empty());
    }

    #[test]
    fn send_from_null_is_no_from() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = SendSlot::new(FixedClock(Instant::now()));
        let params = json!({"v": 1, "text": "x", "from": null});
        let reply = send(&slot, &name(), tmp.path(), &call(&params, 1)).expect("answered");
        assert_eq!(reply["detail"], "input-not-ready");
    }

    #[test]
    fn send_a_record_that_cannot_be_appended_is_internal() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = SendSlot::new(FixedClock(Instant::now()));
        let params = json!({"v": 1, "text": "x"});
        assert_eq!(
            send(
                &slot,
                &name(),
                &tmp.path().join("missing"),
                &call(&params, 1)
            ),
            Err(ProtocolError::Internal)
        );
    }

    /// The appended line reaches the wait feed; a line that never landed does not.
    #[test]
    fn send_append_hook_event_signals_the_wait_feed_after_the_append() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = SendSlot::new(FixedClock(Instant::now()));
        let feed = feed();
        let ended = |message: &str| {
            EventLine::new(
                &name(),
                EventKind::TurnEnded,
                Source::Hook,
                json!({"last_assistant_message": message}),
                Utc::now(),
            )
        };
        let missing = tmp.path().join("missing");
        assert!(append_hook_event(&slot, &feed, &missing, ended("lost")).is_err());
        let none = json!({"ok": {"last_assistant_message": null, "ts": null}});
        assert_eq!(feed.last(&json!({"v": 1})), Ok(none));
        let landed = ended("landed");
        let ts = landed.ts.clone();
        append_hook_event(&slot, &feed, tmp.path(), landed).expect("hook");
        assert_eq!(kinds(tmp.path()), ["turn-ended"]);
        assert_eq!(
            feed.last(&json!({"v": 1})),
            Ok(json!({"ok": {"last_assistant_message": "landed", "ts": ts}}))
        );
    }

    #[test]
    fn send_confirm_window_is_the_fallback() {
        assert_eq!(window_ms(), 10_000);
    }
}
