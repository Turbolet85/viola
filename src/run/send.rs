//! The wrapper's `send` (architecture [Delivery Confirmation]): the text re-validated, the wheel,
//! one send in flight, the readiness gate, `send-issued` at the pre-paste cursor, one bracketed paste + Enter,
//! then confirmation after the fact — the matching `prompt-submitted`, relabelled `driver`, inside
//! the window — or `not-delivered`. The text is validated as received and typed without its
//! trailing newlines (`typed_text`; the founder's ruling of 2026-10-07T15:21Z): the list,
//! the paste, `text_bytes` and the exact match all read that one typed text. A text that is
//! exactly a compiled local command fires no prompt: one with a post-condition measured on this
//! CLI version is confirmed by it inside the same window, any other is `unconfirmable` at once.
//! Each outcome is an `events.ndjson` record and a codes-only `send-*` line; the text reaches
//! neither.

use std::path::Path;
use std::sync::{Arc, Condvar, Mutex, MutexGuard, OnceLock, PoisonError};
use std::time::{Duration, Instant};

use chrono::Utc;
use serde_json::{Value, json};
use tracing::instrument;
use viola_agent_claude::hook::typed_text;
use viola_agent_claude::ledger::{LOCAL_COMMANDS, PostCondition};
use viola_agent_claude::screen::{CONFIRM_WINDOW_FALLBACK, Readiness};
use viola_channel::{Call, ProtocolError};
use viola_core::obs::ObsEvent;
use viola_core::{
    Clock, EventKind, HumanTyping, NotDelivered, RefusalReason, ViolaName, obs_event,
};
use viola_pty::PtyError;
use viola_state::StateError;
use viola_state::events::{EventLine, Source, append_event, append_event_at, end_offset};

use crate::run::gate::Gate;
use crate::run::wait::WaitFeed;
use crate::run::wheel::WheelSlot;

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

/// What confirms a send (architecture [Delivery Confirmation]).
#[derive(Clone, Copy)]
enum Confirms {
    /// Its `prompt-submitted`, matched on the text.
    Prompt,
    /// A listed local command's post-condition, measured on this CLI version.
    Post(PostCondition),
    /// Nothing: a listed local command with no post-condition, or none measured on this version.
    Nothing,
}

/// The compiled list read by the exact typed text: no trim, no case folding, never a leading
/// slash.
fn classify(text: &str, cli_verified: bool) -> Confirms {
    match LOCAL_COMMANDS.iter().find(|(command, _)| *command == text) {
        None => Confirms::Prompt,
        Some((_, Some(post))) if cli_verified => Confirms::Post(*post),
        Some(_) => Confirms::Nothing,
    }
}

struct InFlight {
    text: String,
    confirms: Confirms,
    state: Match,
}

/// What the sends and the hook tap share under one lock.
#[derive(Default)]
struct Flight {
    send: Option<InFlight>,
    /// The `agent_session_id` of the last `session-start` the tap took: `None` before the first,
    /// and when that line carried none.
    session: Option<String>,
}

/// The child's input and gate, set once the pump starts, the one send in flight, the wheel it
/// reads before typing, and the version gate's reading.
pub(crate) struct SendSlot {
    clock: Box<dyn Clock>,
    cli_verified: bool,
    wheel: Arc<WheelSlot>,
    io: OnceLock<(PasteFn, Gate)>,
    flight: Mutex<Flight>,
    settled: Condvar,
}

impl SendSlot {
    pub(crate) fn new(
        clock: impl Clock + 'static,
        cli_verified: bool,
        wheel: Arc<WheelSlot>,
    ) -> Self {
        Self {
            clock: Box::new(clock),
            cli_verified,
            wheel,
            io: OnceLock::new(),
            flight: Mutex::default(),
            settled: Condvar::new(),
        }
    }

    /// The first call wins; the pump calls it once, before it starts.
    pub(crate) fn attach(&self, paste: PasteFn, gate: Gate) {
        let _ = self.io.set((paste, gate));
    }

    fn flight(&self) -> MutexGuard<'_, Flight> {
        self.flight.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Whether a prompt with `text` is the in-flight send's: it is then claimed, and only
    /// [`SendSlot::settle`] resolves it. A send that waits for a post-condition, or for nothing,
    /// is never a prompt's.
    pub(crate) fn claim(&self, text: &str) -> bool {
        let mut flight = self.flight();
        match flight.send.as_mut() {
            Some(f)
                if matches!((f.confirms, &f.state), (Confirms::Prompt, Match::Waiting))
                    && f.text == text =>
            {
                f.state = Match::Claimed;
                true
            }
            _ => false,
        }
    }

    /// A `session-start` about to be appended, its `data` read as fields that may be absent or
    /// `null`. Its id is the remembered one from here on, whatever its cause. The line is the
    /// in-flight send's when that send waits for a new session, the cause is `clear`, and the id
    /// is a string that differs from the one remembered before: it is then claimed, as by
    /// [`SendSlot::claim`].
    fn session_started(&self, data: &Value) -> bool {
        let id = data["agent_session_id"].as_str();
        let mut flight = self.flight();
        let new = id.is_some_and(|id| flight.session.as_deref() != Some(id));
        flight.session = id.map(str::to_owned);
        match flight.send.as_mut() {
            Some(f)
                if new
                    && data["cause"] == "clear"
                    && matches!(
                        (f.confirms, &f.state),
                        (Confirms::Post(PostCondition::NewSession), Match::Waiting)
                    ) =>
            {
                f.state = Match::Claimed;
                true
            }
            _ => false,
        }
    }

    /// The claimed line was appended at `ts`, or (`None`) was not, and the send waits on.
    pub(crate) fn settle(&self, ts: Option<String>) {
        let mut flight = self.flight();
        if let Some(f) = flight
            .send
            .as_mut()
            .filter(|f| matches!(f.state, Match::Claimed))
        {
            f.state = ts.map_or(Match::Waiting, Match::Confirmed);
        }
        self.settled.notify_all();
    }

    /// The window: `Some(submitted_at)` on a match, `None` once it expires unclaimed. The send is
    /// closed under the same lock, so a line after the expiry is never claimed; the slot itself
    /// is emptied only by the send's `Reserved` guard, once its outcome is recorded.
    #[instrument(skip_all, name = "run.confirm_window", fields(window_ms = window_ms()))]
    fn confirm(&self, opened: Instant) -> Option<String> {
        let deadline = opened + CONFIRM_WINDOW_FALLBACK;
        let mut flight = self.flight();
        loop {
            match flight.send.as_ref().map(|f| &f.state) {
                Some(Match::Confirmed(ts)) => {
                    let ts = ts.clone();
                    close(&mut flight.send);
                    return Some(ts);
                }
                Some(Match::Waiting) if self.clock.now() >= deadline => {
                    close(&mut flight.send);
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

/// Appends a hook's event through the wait feed, which signals the parked `wait`s. The in-flight send's
/// own prompt, whatever origin the hook filed, is the driver's: the match keys on the normalised
/// text, never on the hook's origin, and the waiting send settles only once the relabelled line is
/// on disk. Any other prompt the hook filed `human` was typed by the human, who then holds the wheel;
/// a `harness` prompt never moves it. A prompt of any origin starts a turn; `turn-ended`,
/// `session-start` and `session-end` end it. A `session-start` that is the in-flight send's
/// post-condition is claimed and settled the same way, its line appended unchanged.
pub(crate) fn append_hook_event(
    slot: &SendSlot,
    feed: &WaitFeed,
    instance_dir: &Path,
    mut line: EventLine,
) -> Result<(), StateError> {
    let prompt = line.kind == EventKind::PromptSubmitted
        && line.data["text"]
            .as_str()
            .is_some_and(|text| slot.claim(text));
    if prompt {
        line.data["origin"] = json!("driver");
    }
    let claimed =
        prompt || (line.kind == EventKind::SessionStart && slot.session_started(&line.data));
    // Before the line can be read, as the wheel below: a driver that read `turn-ended` is never
    // refused by that turn. The turn is marked ahead of the human's wheel move, so a `release`
    // between the two still clears it.
    match line.kind {
        EventKind::PromptSubmitted => slot.wheel.turn_started(),
        EventKind::TurnEnded | EventKind::SessionStart | EventKind::SessionEnd => {
            slot.wheel.turn_ended();
        }
        _ => {}
    }
    // Moved before the line can be read: whoever sees the human's prompt on disk already sees the
    // human's wheel, so a `release` made after it is never undone by it.
    if line.kind == EventKind::PromptSubmitted && line.data["origin"] == "human" {
        slot.wheel.human_input();
    }
    let appended = feed.appending(&line, || append_event(instance_dir, &line));
    if claimed {
        slot.settle(appended.is_ok().then(|| line.ts.clone()));
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
        self.0.flight().send = None;
    }
}

/// What a send's lines and records carry besides the outcome.
struct Ctx<'a> {
    name: &'a ViolaName,
    wheel: &'a WheelSlot,
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

/// The `send` method, in the documented refusal order. The wheel and the running turn are read
/// twice, in that order both times: at arrival, and again once the gate's wait has ended, because
/// that wait can run to `GATE_MAX_WAIT`. A human key or a turn that started during it refuses with
/// nothing typed.
pub(crate) fn send(
    slot: &SendSlot,
    name: &ViolaName,
    instance_dir: &Path,
    call: &Call<'_>,
) -> Result<Value, ProtocolError> {
    let (text, from) = parse(call.params)?;
    let ctx = Ctx {
        name,
        wheel: &slot.wheel,
        instance_dir,
        call,
        from,
        started: slot.clock.now(),
    };
    if let Err(detail) = viola_core::validate_paste_text(text) {
        return ctx.not_delivered(None, detail);
    }
    let text = typed_text(text);
    if let Some(detail) = slot.wheel.human_typing() {
        return ctx.refuse(
            None,
            RefusalReason::HumanTyping,
            detail.map(HumanTyping::as_str),
        );
    }
    let confirms = classify(text, slot.cli_verified);
    {
        let mut flight = slot.flight();
        if flight.send.is_some() || slot.wheel.turn_running() {
            drop(flight);
            return ctx.not_delivered(None, NotDelivered::TurnRunning);
        }
        flight.send = Some(InFlight {
            text: text.to_owned(),
            confirms,
            state: Match::Waiting,
        });
    }
    let _reserved = Reserved(slot);
    let Some((paste, gate)) = slot.io.get() else {
        return ctx.not_delivered(None, NotDelivered::InputNotReady);
    };
    let (readiness, _) = gate.wait_ready(slot.clock.as_ref(), slot.clock.now());
    if readiness != Readiness::Ready {
        return ctx.not_delivered(None, NotDelivered::InputNotReady);
    }
    if let Some(detail) = slot.wheel.human_typing() {
        return ctx.refuse(
            None,
            RefusalReason::HumanTyping,
            detail.map(HumanTyping::as_str),
        );
    }
    if slot.wheel.turn_running() {
        return ctx.not_delivered(None, NotDelivered::TurnRunning);
    }
    let cursor = ctx.issue(text.len())?;
    if paste(text).is_err() {
        return ctx.not_delivered(Some(cursor), NotDelivered::InputNotReady);
    }
    if matches!(confirms, Confirms::Nothing) {
        return ctx.unconfirmable(cursor, slot.clock.now());
    }
    match slot.confirm(slot.clock.now()) {
        Some(submitted_at) => ctx.confirm(cursor, &submitted_at, slot.clock.now()),
        None => ctx.not_delivered(Some(cursor), NotDelivered::NoPromptSubmitted),
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
        self.log_confirmed(cursor, true, now);
        Ok(json!({"ok": {"submitted_at": submitted_at, "cursor": cursor}}))
    }

    /// A listed local command nothing can confirm: typed, then recorded and answered at once, with
    /// no window. An `ok`, never a refusal.
    fn unconfirmable(&self, cursor: u64, now: Instant) -> Result<Value, ProtocolError> {
        let data = json!({"cursor": cursor, "confirmed": false});
        self.record(EventKind::SendConfirmed, data)?;
        self.log_confirmed(cursor, false, now);
        Ok(json!({"ok": {"confirmed": false, "detail": "unconfirmable", "cursor": cursor}}))
    }

    fn log_confirmed(&self, cursor: u64, confirmed: bool, now: Instant) {
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
            confirmed = confirmed,
            duration_ms = duration_ms,
        );
    }

    fn not_delivered(
        &self,
        cursor: Option<u64>,
        detail: NotDelivered,
    ) -> Result<Value, ProtocolError> {
        self.refuse(cursor, RefusalReason::NotDelivered, Some(detail.as_str()))
    }

    /// A refusal: before `send-issued` (no cursor; the line's `corr` is the end offset at refusal,
    /// obs-plan D-28) or after it (the send's own cursor). Its line carries the wheel's holder.
    fn refuse(
        &self,
        cursor: Option<u64>,
        refusal: RefusalReason,
        detail: Option<&'static str>,
    ) -> Result<Value, ProtocolError> {
        let mut data = json!({"refusal": refusal.as_str(), "detail": detail});
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
            wheel = self.wheel.holder().as_str(),
            refusal = refusal.as_str(),
            detail = detail,
        );
        Ok(json!({"refusal": refusal.as_str(), "detail": detail}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::mpsc;

    use rstest::rstest;
    use viola_agent_claude::screen::SIGNATURES;
    use viola_pty::Size;
    use viola_state::snapshot::Wheel;

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

    /// Fixed while `jumping` is false, a [`JumpClock`] while it is true: a send that must be
    /// confirmed waits for its prompt, and one that must be refused can never park on its window.
    struct SwitchClock {
        now: Mutex<Instant>,
        jumping: Arc<std::sync::atomic::AtomicBool>,
    }

    impl Clock for SwitchClock {
        fn now(&self) -> Instant {
            let mut now = self.now.lock().expect("clock");
            if self.jumping.load(std::sync::atomic::Ordering::SeqCst) {
                *now += Duration::from_secs(1);
            }
            *now
        }
    }

    impl SwitchClock {
        /// The clock standing still at `base`, and its switch.
        fn at(base: Instant) -> (Self, Arc<std::sync::atomic::AtomicBool>) {
            let jumping = Arc::new(std::sync::atomic::AtomicBool::new(false));
            let clock = Self {
                now: Mutex::new(base),
                jumping: Arc::clone(&jumping),
            };
            (clock, jumping)
        }
    }

    fn name() -> ViolaName {
        ViolaName::try_new("builder".to_owned()).expect("valid")
    }

    /// A gate whose screen went quiet a second before `base`.
    fn quiet_gate(base: Instant) -> Gate {
        let (feeder, gate, thread) = gate::start(
            FixedClock(base - Duration::from_secs(1)),
            Size::DEFAULT,
            None,
        );
        drop(feeder);
        thread.join().expect("feed thread");
        gate
    }

    /// A verified gate whose screen was fed `rows` at `fed_at`.
    fn verified_gate(fed_at: Instant, rows: &[&str]) -> Gate {
        let (feeder, gate, thread) =
            gate::start(FixedClock(fed_at), Size::DEFAULT, Some(&SIGNATURES));
        let mut tee = gate::Tee::new(Vec::new(), feeder);
        std::io::Write::write_all(&mut tee, rows.join("\r\n").as_bytes()).expect("write");
        drop(tee);
        thread.join().expect("feed thread");
        gate
    }

    /// A [`JumpClock`] the test keeps a handle on. The first reading at or past `at` first runs
    /// `during`: what happens while the gate waits.
    struct WaitClock {
        now: Arc<Mutex<Instant>>,
        at: Instant,
        during: Mutex<Option<Box<dyn FnOnce() + Send>>>,
    }

    impl Clock for WaitClock {
        fn now(&self) -> Instant {
            let mut now = self.now.lock().expect("clock");
            *now += Duration::from_secs(1);
            if *now >= self.at
                && let Some(during) = self.during.lock().expect("during").take()
            {
                during();
            }
            *now
        }
    }

    /// A gate poisoned by a vt100 panic (a wide character at one column).
    fn poisoned_gate(base: Instant) -> Gate {
        let (feeder, gate, thread) =
            gate::start(FixedClock(base), Size { cols: 1, rows: 24 }, None);
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
        slot.flight().send = Some(InFlight {
            text: "another send".to_owned(),
            confirms: Confirms::Prompt,
            state: Match::Waiting,
        });
    }

    #[derive(Clone, Copy)]
    enum Setup {
        InFlightNoChild,
        /// A human key took the wheel, and another send is in flight.
        HumanInFlight,
        /// `viola pause` took the wheel, and another send is in flight.
        PausedInFlight,
        /// A turn is running, with no send in flight and no child.
        TurnNoChild,
        /// A human key took the wheel during a running turn.
        HumanTurn,
        /// `viola pause` took the wheel during a running turn.
        PausedTurn,
        NoChild,
        Poisoned,
        Mute,
    }

    /// A wheel `viola pause` took.
    fn paused(wheel: &WheelSlot) {
        let none = json!({"v": 1});
        wheel.pause(&call(&none, 1)).expect("paused");
    }

    /// The `send` order, written out: control-character ahead of everything, then the wheel, then
    /// the one in flight, then the gate, then the window.
    #[rstest]
    #[case::control_character_before_human_typing(
        "x\u{1b}[201~",
        Setup::PausedInFlight,
        "not-delivered",
        Some("control-character")
    )]
    #[case::control_character_before_turn_running(
        "x\u{1b}[201~",
        Setup::InFlightNoChild,
        "not-delivered",
        Some("control-character")
    )]
    #[case::human_typing_before_turn_running("hello", Setup::HumanInFlight, "human-typing", None)]
    #[case::manual_pause_before_turn_running(
        "hello",
        Setup::PausedInFlight,
        "human-typing",
        Some("manual-pause")
    )]
    #[case::turn_running_before_input_not_ready(
        "hello",
        Setup::InFlightNoChild,
        "not-delivered",
        Some("turn-running")
    )]
    #[case::control_character_before_a_running_turn(
        "x\u{1b}[201~",
        Setup::TurnNoChild,
        "not-delivered",
        Some("control-character")
    )]
    #[case::human_typing_before_a_running_turn("hello", Setup::HumanTurn, "human-typing", None)]
    #[case::manual_pause_before_a_running_turn(
        "hello",
        Setup::PausedTurn,
        "human-typing",
        Some("manual-pause")
    )]
    #[case::a_running_turn_before_input_not_ready(
        "hello",
        Setup::TurnNoChild,
        "not-delivered",
        Some("turn-running")
    )]
    #[case::input_not_ready_without_a_child(
        "hello",
        Setup::NoChild,
        "not-delivered",
        Some("input-not-ready")
    )]
    #[case::input_not_ready_on_a_poisoned_screen(
        "hello",
        Setup::Poisoned,
        "not-delivered",
        Some("input-not-ready")
    )]
    #[case::no_prompt_submitted_when_the_window_expires(
        "hello",
        Setup::Mute,
        "not-delivered",
        Some("no-prompt-submitted")
    )]
    #[case::local_command_under_the_human_wheel("/clear", Setup::HumanTurn, "human-typing", None)]
    #[case::local_command_during_a_running_turn(
        "/clear",
        Setup::TurnNoChild,
        "not-delivered",
        Some("turn-running")
    )]
    #[case::local_command_on_a_poisoned_screen(
        "/clear",
        Setup::Poisoned,
        "not-delivered",
        Some("input-not-ready")
    )]
    fn send_refusal_order(
        #[case] text: &str,
        #[case] setup: Setup,
        #[case] refusal: &str,
        #[case] detail: Option<&str>,
    ) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let base = Instant::now();
        let slot = SendSlot::new(JumpClock(Mutex::new(base)), true, Arc::default());
        let (pastes, _pasted) = Pastes::new();
        match setup {
            Setup::InFlightNoChild => occupy(&slot),
            Setup::HumanInFlight => {
                slot.wheel.human_input();
                occupy(&slot);
            }
            Setup::PausedInFlight => {
                paused(&slot.wheel);
                occupy(&slot);
            }
            Setup::TurnNoChild => slot.wheel.turn_started(),
            Setup::HumanTurn => {
                slot.wheel.turn_started();
                slot.wheel.human_input();
            }
            Setup::PausedTurn => {
                slot.wheel.turn_started();
                paused(&slot.wheel);
            }
            Setup::NoChild => {}
            Setup::Poisoned => slot.attach(pastes.paste_fn(), poisoned_gate(base)),
            Setup::Mute => slot.attach(pastes.paste_fn(), quiet_gate(base)),
        }
        let params = json!({"v": 1, "text": text});
        let reply = send(&slot, &name(), tmp.path(), &call(&params, 7)).expect("answered");
        assert_eq!(reply, json!({"refusal": refusal, "detail": detail}));
        let events = events(tmp.path());
        let refused = events.last().expect("a send-refused record");
        assert_eq!(refused["kind"], "send-refused");
        assert_eq!(refused["source"], "wrapper");
        assert_eq!(refused["data"]["refusal"], refusal);
        assert_eq!(refused["data"]["detail"], json!(detail));
        if matches!(setup, Setup::Mute) {
            assert_eq!(kinds(tmp.path()), ["send-issued", "send-refused"]);
            assert_eq!(refused["data"]["cursor"], 0);
            assert_eq!(pastes.all(), ["hello"]);
        } else {
            assert_eq!(kinds(tmp.path()), ["send-refused"]);
            assert!(refused["data"].get("cursor").is_none());
            assert!(pastes.all().is_empty(), "nothing typed");
        }
        if matches!(
            setup,
            Setup::InFlightNoChild | Setup::HumanInFlight | Setup::PausedInFlight
        ) {
            assert!(
                slot.flight().send.is_some(),
                "the other send keeps its slot"
            );
        } else {
            assert!(slot.flight().send.is_none(), "the slot is free again");
        }
    }

    /// A send whose prompt the hook reports back: the waiting handler wakes on it.
    fn confirmed_with(origin: &str) {
        confirmed_as(CANARY, CANARY, origin);
    }

    /// A send of `sent` that types `typed`, the text the hook then reports under `origin`. The
    /// clock stands still until that prompt is on disk and jumps after it: a send its prompt did
    /// not confirm expires, and never parks the case.
    fn confirmed_as(sent: &str, typed: &str, origin: &str) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let base = Instant::now();
        let (clock, jumping) = SwitchClock::at(base);
        let slot = SendSlot::new(clock, false, Arc::default());
        let (pastes, pasted) = Pastes::new();
        slot.attach(pastes.paste_fn(), quiet_gate(base));
        std::fs::write(
            tmp.path().join("events.ndjson"),
            b"{\"kind\":\"earlier\"}\n",
        )
        .expect("seed");
        let params = json!({"v": 1, "text": sent, "from": "overseer"});
        let reply = std::thread::scope(|s| {
            let sending = s.spawn(|| send(&slot, &name(), tmp.path(), &call(&params, 3)));
            pasted.recv().expect("pasted");
            append_hook_event(&slot, &feed(), tmp.path(), prompt(typed, origin)).expect("hook");
            jumping.store(true, std::sync::atomic::Ordering::SeqCst);
            sending.join().expect("send thread")
        })
        .expect("answered");
        let events = events(tmp.path());
        assert_eq!(pastes.all(), [typed]);
        assert_eq!(
            kinds(tmp.path())[1..],
            ["send-issued", "prompt-submitted", "send-confirmed"]
        );
        assert_eq!(events[1]["data"], json!({"cursor": 19, "from": "overseer"}));
        assert_eq!(
            events[2]["data"],
            json!({"text": typed, "origin": "driver"})
        );
        assert_eq!(events[3]["data"], json!({"cursor": 19}));
        assert_eq!(
            reply,
            json!({"ok": {"submitted_at": events[2]["ts"], "cursor": 19}})
        );
        assert!(slot.flight().send.is_none());
        assert_eq!(slot.wheel.holder(), Wheel::Driver, "the send's own prompt");
    }

    #[test]
    fn send_match_is_ok_with_the_prompt_ts_and_the_cursor() {
        confirmed_with("human");
    }

    #[test]
    fn send_harness_filed_prompt_with_the_in_flight_text_is_relabelled_driver() {
        confirmed_with("harness");
    }

    /// A text ending in newlines is typed without them, and the prompt the hook reports for the
    /// typed text is the send's own: confirmed, the wheel left with the driver.
    #[rstest]
    #[case::one("\n")]
    #[case::two("\n\n")]
    fn send_trailing_newlines_are_not_typed_and_the_send_is_confirmed(#[case] tail: &str) {
        confirmed_as(&format!("{CANARY}{tail}"), CANARY, "human");
    }

    /// A refused character is refused whatever follows it: the text is validated as received.
    #[test]
    fn send_a_refused_character_before_a_trailing_newline_is_control_character() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (slot, pastes) = mute_child();
        let params = json!({"v": 1, "text": "x\u{1b}[201~\n"});
        let reply = send(&slot, &name(), tmp.path(), &call(&params, 1)).expect("answered");
        assert_eq!(
            reply,
            json!({"refusal": "not-delivered", "detail": "control-character"})
        );
        assert!(pastes.all().is_empty(), "nothing typed");
        assert_eq!(kinds(tmp.path()), ["send-refused"]);
        assert_refused_without_a_cursor(tmp.path(), Some("control-character"));
    }

    /// The claim has no tolerance. While a text that ended in a newline is in flight, a prompt
    /// that still carries the newline is not the typed text: it is appended as the hook filed it,
    /// the human it names takes the wheel, and the send waits on to its window.
    #[test]
    fn send_a_prompt_that_keeps_the_trailing_newline_claims_nothing() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let base = Instant::now();
        let (clock, jumping) = SwitchClock::at(base);
        let slot = SendSlot::new(clock, false, Arc::default());
        let (pastes, pasted) = Pastes::new();
        slot.attach(pastes.paste_fn(), quiet_gate(base));
        let sent = format!("{CANARY}\n");
        let params = json!({"v": 1, "text": sent});
        let (filed, waiting, reply) = std::thread::scope(|s| {
            let sending = s.spawn(|| send(&slot, &name(), tmp.path(), &call(&params, 1)));
            pasted.recv().expect("pasted");
            let appended = append_hook_event(&slot, &feed(), tmp.path(), prompt(&sent, "human"));
            let filed = events(tmp.path()).last().cloned();
            let waiting = matches!(
                slot.flight().send.as_ref().map(|f| &f.state),
                Some(Match::Waiting)
            );
            jumping.store(true, std::sync::atomic::Ordering::SeqCst);
            let reply = sending.join().expect("send thread");
            appended.expect("hook");
            (filed, waiting, reply)
        });
        assert_eq!(pastes.all(), [CANARY]);
        assert_eq!(
            filed.expect("the prompt's line")["data"],
            json!({"text": sent, "origin": "human"})
        );
        assert!(waiting, "the send still waits for its own prompt");
        assert_eq!(
            reply.expect("answered"),
            json!({"refusal": "not-delivered", "detail": "no-prompt-submitted"})
        );
        assert_eq!(
            kinds(tmp.path()),
            ["send-issued", "prompt-submitted", "send-refused"]
        );
        assert_eq!(slot.wheel.holder(), Wheel::Human);
    }

    #[test]
    fn send_a_different_prompt_is_appended_unchanged_and_claims_nothing() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = SendSlot::new(FixedClock(Instant::now()), false, Arc::default());
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
            slot.flight().send.as_ref().map(|f| &f.state),
            Some(Match::Waiting)
        ));
    }

    /// An unsent prompt the hook filed `human` was typed by the human: the wheel is theirs, moved
    /// before its line is appended (a line that could not land still moved it). A `harness`
    /// prompt moves nothing.
    #[test]
    fn send_an_unsent_human_prompt_takes_the_wheel_before_its_line_and_a_harness_one_does_not() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = SendSlot::new(FixedClock(Instant::now()), false, Arc::default());
        append_hook_event(&slot, &feed(), tmp.path(), prompt("injected", "harness")).expect("hook");
        assert_eq!(slot.wheel.holder(), Wheel::Driver);
        let missing = tmp.path().join("missing");
        assert!(append_hook_event(&slot, &feed(), &missing, prompt("lost", "human")).is_err());
        assert_eq!(slot.wheel.holder(), Wheel::Human);
        assert_eq!(slot.wheel.human_typing(), Some(None));
    }

    #[test]
    fn send_no_prompt_while_nothing_is_in_flight_is_appended_unchanged() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = SendSlot::new(FixedClock(Instant::now()), false, Arc::default());
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
        let slot = SendSlot::new(FixedClock(Instant::now()), false, Arc::default());
        slot.flight().send = Some(InFlight {
            text: "decided".to_owned(),
            confirms: Confirms::Prompt,
            state: Match::Closed,
        });
        let params = json!({"v": 1, "text": "next"});
        let reply = send(&slot, &name(), tmp.path(), &call(&params, 1)).expect("answered");
        assert_eq!(reply["detail"], "turn-running");
        append_hook_event(&slot, &feed(), tmp.path(), prompt("decided", "human")).expect("hook");
        assert_eq!(events(tmp.path())[1]["data"]["origin"], "human");
        assert!(slot.confirm(Instant::now()).is_none());
        assert!(
            slot.flight().send.is_some(),
            "only the send's own guard frees it"
        );
        drop(Reserved(&slot));
        assert!(slot.flight().send.is_none());
    }

    #[test]
    fn send_window_expiry_is_no_prompt_submitted_with_the_cursor() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let base = Instant::now();
        let slot = SendSlot::new(JumpClock(Mutex::new(base)), false, Arc::default());
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
        let slot = SendSlot::new(FixedClock(base), false, Arc::default());
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
        let slot = SendSlot::new(FixedClock(base), false, Arc::default());
        slot.attach(Box::new(|_| Err(PtyError::NoInput)), quiet_gate(base));
        let params = json!({"v": 1, "text": "hello"});
        let reply = send(&slot, &name(), tmp.path(), &call(&params, 1)).expect("answered");
        assert_eq!(reply["detail"], "input-not-ready");
        assert_eq!(events(tmp.path())[1]["data"]["cursor"], 0);
        assert!(slot.flight().send.is_none());
    }

    #[rstest]
    #[case::no_text(json!({"v": 1}))]
    #[case::text_not_string(json!({"v": 1, "text": 5}))]
    #[case::from_not_a_name(json!({"v": 1, "text": "x", "from": "../x"}))]
    #[case::from_not_string(json!({"v": 1, "text": "x", "from": 3}))]
    fn send_params_it_cannot_take_are_invalid_params(#[case] params: Value) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = SendSlot::new(FixedClock(Instant::now()), false, Arc::default());
        assert_eq!(
            send(&slot, &name(), tmp.path(), &call(&params, 1)),
            Err(ProtocolError::InvalidParams)
        );
        assert!(events(tmp.path()).is_empty());
    }

    #[test]
    fn send_from_null_is_no_from() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = SendSlot::new(FixedClock(Instant::now()), false, Arc::default());
        let params = json!({"v": 1, "text": "x", "from": null});
        let reply = send(&slot, &name(), tmp.path(), &call(&params, 1)).expect("answered");
        assert_eq!(reply["detail"], "input-not-ready");
    }

    #[test]
    fn send_a_record_that_cannot_be_appended_is_internal() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = SendSlot::new(FixedClock(Instant::now()), false, Arc::default());
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
        let slot = SendSlot::new(FixedClock(Instant::now()), false, Arc::default());
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

    const HARNESS_TURN: &str = "<task-notification>synthetic harness turn</task-notification>";

    fn hook_line(kind: EventKind, data: Value) -> EventLine {
        EventLine::new(&name(), kind, Source::Hook, data, Utc::now())
    }

    /// A slot whose child takes every paste and never submits: a send past the turn's rung is
    /// typed, then expires `no-prompt-submitted`.
    fn mute_child() -> (SendSlot, Pastes) {
        let base = Instant::now();
        let slot = SendSlot::new(JumpClock(Mutex::new(base)), false, Arc::default());
        let (pastes, _pasted) = Pastes::new();
        slot.attach(pastes.paste_fn(), quiet_gate(base));
        (slot, pastes)
    }

    /// Refused before `send-issued`: the last record is a `send-refused` with no cursor.
    fn assert_refused_without_a_cursor(dir: &Path, detail: Option<&str>) {
        let events = events(dir);
        let refused = events.last().expect("a send-refused record");
        assert_eq!(refused["kind"], "send-refused");
        assert_eq!(refused["data"]["detail"], json!(detail));
        assert!(refused["data"].get("cursor").is_none());
    }

    /// A prompt of any origin starts a turn. An unsent human prompt also takes the wheel, whose
    /// rung comes first.
    #[rstest]
    #[case::harness(HARNESS_TURN, "harness", "not-delivered", Some("turn-running"))]
    #[case::human("typed by the human", "human", "human-typing", None)]
    fn send_a_prompt_of_any_origin_starts_a_turn(
        #[case] text: &str,
        #[case] origin: &str,
        #[case] refusal: &str,
        #[case] detail: Option<&str>,
    ) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (slot, pastes) = mute_child();
        append_hook_event(&slot, &feed(), tmp.path(), prompt(text, origin)).expect("hook");
        assert!(slot.wheel.turn_running());
        let params = json!({"v": 1, "text": "hello"});
        let reply = send(&slot, &name(), tmp.path(), &call(&params, 1)).expect("answered");
        assert_eq!(reply, json!({"refusal": refusal, "detail": detail}));
        assert!(pastes.all().is_empty(), "nothing typed");
        assert_refused_without_a_cursor(tmp.path(), detail);
        assert_eq!(kinds(tmp.path()), ["prompt-submitted", "send-refused"]);
    }

    /// `turn-ended`, `session-start` and `session-end` end a turn; `activity` does not.
    #[rstest]
    #[case::turn_ended(EventKind::TurnEnded, json!({"last_assistant_message": "done"}), true)]
    #[case::session_start(EventKind::SessionStart, json!({"cause": "clear"}), true)]
    #[case::session_end(EventKind::SessionEnd, json!({}), true)]
    #[case::activity(EventKind::Activity, json!({}), false)]
    fn send_a_turn_end_lets_the_next_send_past_the_rung(
        #[case] kind: EventKind,
        #[case] data: Value,
        #[case] ends: bool,
    ) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (slot, pastes) = mute_child();
        append_hook_event(&slot, &feed(), tmp.path(), prompt(HARNESS_TURN, "harness"))
            .expect("hook");
        append_hook_event(&slot, &feed(), tmp.path(), hook_line(kind, data)).expect("hook");
        let params = json!({"v": 1, "text": "hello"});
        let reply = send(&slot, &name(), tmp.path(), &call(&params, 1)).expect("answered");
        if ends {
            assert_eq!(
                reply,
                json!({"refusal": "not-delivered", "detail": "no-prompt-submitted"})
            );
            assert_eq!(pastes.all(), ["hello"]);
        } else {
            assert_eq!(
                reply,
                json!({"refusal": "not-delivered", "detail": "turn-running"})
            );
            assert!(pastes.all().is_empty(), "nothing typed");
            assert_refused_without_a_cursor(tmp.path(), Some("turn-running"));
        }
    }

    /// A send confirmed by the hook's report of its prompt.
    fn confirmed(slot: &SendSlot, dir: &Path, pasted: &mpsc::Receiver<()>, text: &str) -> Value {
        let params = json!({"v": 1, "text": text});
        std::thread::scope(|s| {
            let sending = s.spawn(|| send(slot, &name(), dir, &call(&params, 1)));
            pasted.recv().expect("pasted");
            append_hook_event(slot, &feed(), dir, prompt(text, "human")).expect("hook");
            sending.join().expect("send thread")
        })
        .expect("answered")
    }

    /// The driver's own prompt starts a turn too: the next send is `turn-running` until its
    /// `turn-ended`.
    #[test]
    fn send_after_a_confirmed_send_is_turn_running_until_turn_ended() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let base = Instant::now();
        let jumping = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let clock = SwitchClock {
            now: Mutex::new(base),
            jumping: Arc::clone(&jumping),
        };
        let slot = SendSlot::new(clock, false, Arc::default());
        let (pastes, pasted) = Pastes::new();
        slot.attach(pastes.paste_fn(), quiet_gate(base));
        assert_eq!(
            confirmed(&slot, tmp.path(), &pasted, "first")["ok"]["cursor"],
            0
        );
        let params = json!({"v": 1, "text": "second"});
        jumping.store(true, std::sync::atomic::Ordering::SeqCst);
        let reply = send(&slot, &name(), tmp.path(), &call(&params, 2)).expect("answered");
        jumping.store(false, std::sync::atomic::Ordering::SeqCst);
        assert_eq!(
            reply,
            json!({"refusal": "not-delivered", "detail": "turn-running"})
        );
        assert_refused_without_a_cursor(tmp.path(), Some("turn-running"));
        assert_eq!(pastes.all(), ["first"]);
        let ended = hook_line(
            EventKind::TurnEnded,
            json!({"last_assistant_message": "done"}),
        );
        append_hook_event(&slot, &feed(), tmp.path(), ended).expect("hook");
        assert!(confirmed(&slot, tmp.path(), &pasted, "second")["ok"]["cursor"].is_u64());
        assert_eq!(pastes.all(), ["first", "second"]);
        assert_eq!(
            kinds(tmp.path()),
            [
                "send-issued",
                "prompt-submitted",
                "send-confirmed",
                "send-refused",
                "turn-ended",
                "send-issued",
                "prompt-submitted",
                "send-confirmed"
            ]
        );
        assert_eq!(slot.wheel.holder(), Wheel::Driver);
    }

    /// A listed command nothing can confirm is typed and answered at once. The clock never moves,
    /// so a wait on the window would hang the case.
    #[rstest]
    #[case::no_post_condition_on_a_verified_version("/remote-control", true)]
    #[case::no_post_condition_on_an_unverified_version("/remote-control", false)]
    #[case::post_condition_not_measured_on_this_version("/clear", false)]
    fn send_local_command_without_a_measured_post_condition_is_unconfirmable(
        #[case] text: &str,
        #[case] verified: bool,
    ) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let base = Instant::now();
        let slot = SendSlot::new(FixedClock(base), verified, Arc::default());
        let (pastes, _pasted) = Pastes::new();
        slot.attach(pastes.paste_fn(), quiet_gate(base));
        let params = json!({"v": 1, "text": text});
        let reply = send(&slot, &name(), tmp.path(), &call(&params, 1)).expect("answered");
        assert_eq!(
            reply,
            json!({"ok": {"confirmed": false, "detail": "unconfirmable", "cursor": 0}})
        );
        assert_eq!(kinds(tmp.path()), ["send-issued", "send-confirmed"]);
        assert_eq!(
            events(tmp.path())[1]["data"],
            json!({"cursor": 0, "confirmed": false})
        );
        assert_eq!(pastes.all(), [text]);
        assert!(slot.flight().send.is_none(), "the slot is free again");
    }

    /// `/clear` on a verified version waits for its post-condition: a `session-start` with cause
    /// `clear` and an id the tap had not seen confirms it, and that line's `ts` is the send's.
    #[test]
    fn send_local_command_clear_is_confirmed_by_a_new_session() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let base = Instant::now();
        let slot = SendSlot::new(FixedClock(base), true, Arc::default());
        let (pastes, pasted) = Pastes::new();
        slot.attach(pastes.paste_fn(), quiet_gate(base));
        let params = json!({"v": 1, "text": "/clear"});
        let started = json!({"cause": "clear", "agent_session_id": "session-two"});
        let reply = std::thread::scope(|s| {
            let sending = s.spawn(|| send(&slot, &name(), tmp.path(), &call(&params, 1)));
            pasted.recv().expect("pasted");
            let line = hook_line(EventKind::SessionStart, started.clone());
            append_hook_event(&slot, &feed(), tmp.path(), line).expect("hook");
            sending.join().expect("send thread")
        })
        .expect("answered");
        let events = events(tmp.path());
        assert_eq!(
            kinds(tmp.path()),
            ["send-issued", "session-start", "send-confirmed"]
        );
        assert_eq!(events[1]["data"], started);
        assert_eq!(events[2]["data"], json!({"cursor": 0}));
        assert_eq!(
            reply,
            json!({"ok": {"submitted_at": events[1]["ts"], "cursor": 0}})
        );
        assert_eq!(pastes.all(), ["/clear"]);
        assert!(slot.flight().send.is_none(), "the slot is free again");
    }

    /// Only a `clear` whose id is a string the tap had not seen is the post-condition. Any other
    /// `session-start` is appended unchanged while the send waits, and the send then expires.
    #[rstest]
    #[case::clear_with_the_remembered_id("clear", json!("session-one"))]
    #[case::startup_with_a_new_id("startup", json!("session-two"))]
    #[case::clear_with_a_null_id("clear", Value::Null)]
    fn send_local_command_clear_is_not_confirmed_by_another_session_start(
        #[case] cause: &str,
        #[case] id: Value,
    ) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let base = Instant::now();
        let jumping = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let clock = SwitchClock {
            now: Mutex::new(base),
            jumping: Arc::clone(&jumping),
        };
        let slot = SendSlot::new(clock, true, Arc::default());
        let (pastes, pasted) = Pastes::new();
        slot.attach(pastes.paste_fn(), quiet_gate(base));
        let boot = json!({"cause": "startup", "agent_session_id": "session-one"});
        let boot = hook_line(EventKind::SessionStart, boot);
        append_hook_event(&slot, &feed(), tmp.path(), boot).expect("hook");
        let cursor = std::fs::metadata(tmp.path().join("events.ndjson"))
            .expect("events")
            .len();
        let params = json!({"v": 1, "text": "/clear"});
        let other = json!({"cause": cause, "agent_session_id": id});
        let reply = std::thread::scope(|s| {
            let sending = s.spawn(|| send(&slot, &name(), tmp.path(), &call(&params, 1)));
            pasted.recv().expect("pasted");
            let line = hook_line(EventKind::SessionStart, other.clone());
            append_hook_event(&slot, &feed(), tmp.path(), line).expect("hook");
            jumping.store(true, std::sync::atomic::Ordering::SeqCst);
            sending.join().expect("send thread")
        })
        .expect("answered");
        assert_eq!(
            reply,
            json!({"refusal": "not-delivered", "detail": "no-prompt-submitted"})
        );
        let events = events(tmp.path());
        assert_eq!(
            kinds(tmp.path()),
            [
                "session-start",
                "send-issued",
                "session-start",
                "send-refused"
            ]
        );
        assert_eq!(events[2]["data"], other);
        assert_eq!(
            events[3]["data"],
            json!({"refusal": "not-delivered", "detail": "no-prompt-submitted", "cursor": cursor})
        );
        assert_eq!(pastes.all(), ["/clear"]);
        assert!(slot.flight().send.is_none(), "the slot is free again");
    }

    #[test]
    fn send_local_command_window_expiry_is_no_prompt_submitted_with_the_cursor() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let base = Instant::now();
        let slot = SendSlot::new(JumpClock(Mutex::new(base)), true, Arc::default());
        let (pastes, _pasted) = Pastes::new();
        slot.attach(pastes.paste_fn(), quiet_gate(base));
        let params = json!({"v": 1, "text": "/clear"});
        let reply = send(&slot, &name(), tmp.path(), &call(&params, 1)).expect("answered");
        assert_eq!(
            reply,
            json!({"refusal": "not-delivered", "detail": "no-prompt-submitted"})
        );
        assert_eq!(kinds(tmp.path()), ["send-issued", "send-refused"]);
        assert_eq!(
            events(tmp.path())[1]["data"],
            json!({"refusal": "not-delivered", "detail": "no-prompt-submitted", "cursor": 0})
        );
        assert_eq!(pastes.all(), ["/clear"]);
        assert!(slot.flight().send.is_none(), "the slot is free again");
    }

    /// The decision is the compiled list read by the exact text. On a version where the listed
    /// `/clear` is `unconfirmable` at once, each of these is an ordinary send: typed, and with no
    /// prompt expired at the window.
    #[rstest]
    #[case::a_trailing_space("/clear ")]
    #[case::a_leading_space(" /clear")]
    #[case::another_case("/CLEAR")]
    #[case::a_slash_text_off_the_list("/andromeda-arch")]
    fn send_local_command_decision_is_exact_text_never_a_leading_slash(#[case] text: &str) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let base = Instant::now();
        let slot = SendSlot::new(JumpClock(Mutex::new(base)), false, Arc::default());
        let (pastes, _pasted) = Pastes::new();
        slot.attach(pastes.paste_fn(), quiet_gate(base));
        let params = json!({"v": 1, "text": text});
        let reply = send(&slot, &name(), tmp.path(), &call(&params, 1)).expect("answered");
        assert_eq!(
            reply,
            json!({"refusal": "not-delivered", "detail": "no-prompt-submitted"})
        );
        assert_eq!(kinds(tmp.path()), ["send-issued", "send-refused"]);
        assert_eq!(
            events(tmp.path())[1]["data"],
            json!({"refusal": "not-delivered", "detail": "no-prompt-submitted", "cursor": 0})
        );
        assert_eq!(pastes.all(), [text]);
        assert!(slot.flight().send.is_none(), "the slot is free again");
    }

    /// The list is read by the typed text, so `/clear` and a newline is the listed command. On a
    /// version where `/clear` is `unconfirmable`, it is typed as `/clear` and answered at once.
    #[test]
    fn send_local_command_decision_reads_the_text_without_its_trailing_newline() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let base = Instant::now();
        let slot = SendSlot::new(JumpClock(Mutex::new(base)), false, Arc::default());
        let (pastes, _pasted) = Pastes::new();
        slot.attach(pastes.paste_fn(), quiet_gate(base));
        let params = json!({"v": 1, "text": "/clear\n"});
        let reply = send(&slot, &name(), tmp.path(), &call(&params, 1)).expect("answered");
        assert_eq!(pastes.all(), ["/clear"]);
        assert_eq!(
            reply,
            json!({"ok": {"confirmed": false, "detail": "unconfirmable", "cursor": 0}})
        );
        assert_eq!(kinds(tmp.path()), ["send-issued", "send-confirmed"]);
        assert_eq!(
            events(tmp.path())[1]["data"],
            json!({"cursor": 0, "confirmed": false})
        );
    }

    /// The listed command with no post-condition, followed by a newline, on a verified version:
    /// typed as the command and answered `unconfirmable` at once.
    #[test]
    fn send_local_command_without_a_post_condition_and_a_trailing_newline_is_unconfirmable() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let base = Instant::now();
        let slot = SendSlot::new(JumpClock(Mutex::new(base)), true, Arc::default());
        let (pastes, _pasted) = Pastes::new();
        slot.attach(pastes.paste_fn(), quiet_gate(base));
        let params = json!({"v": 1, "text": "/remote-control\n"});
        let reply = send(&slot, &name(), tmp.path(), &call(&params, 1)).expect("answered");
        assert_eq!(pastes.all(), ["/remote-control"]);
        assert_eq!(
            reply,
            json!({"ok": {"confirmed": false, "detail": "unconfirmable", "cursor": 0}})
        );
        assert_eq!(kinds(tmp.path()), ["send-issued", "send-confirmed"]);
    }

    /// `/clear` and a newline on a verified version is `/clear`: typed as the command, then
    /// confirmed by its post-condition as `send_local_command_clear_is_confirmed_by_a_new_session`
    /// is. The clock jumps once the new session is on disk, so a send it did not confirm expires.
    #[test]
    fn send_local_command_clear_with_a_trailing_newline_is_confirmed_by_a_new_session() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let base = Instant::now();
        let (clock, jumping) = SwitchClock::at(base);
        let slot = SendSlot::new(clock, true, Arc::default());
        let (pastes, pasted) = Pastes::new();
        slot.attach(pastes.paste_fn(), quiet_gate(base));
        let params = json!({"v": 1, "text": "/clear\n"});
        let started = json!({"cause": "clear", "agent_session_id": "session-two"});
        let reply = std::thread::scope(|s| {
            let sending = s.spawn(|| send(&slot, &name(), tmp.path(), &call(&params, 1)));
            pasted.recv().expect("pasted");
            let line = hook_line(EventKind::SessionStart, started.clone());
            append_hook_event(&slot, &feed(), tmp.path(), line).expect("hook");
            jumping.store(true, std::sync::atomic::Ordering::SeqCst);
            sending.join().expect("send thread")
        })
        .expect("answered");
        let events = events(tmp.path());
        assert_eq!(pastes.all(), ["/clear"]);
        assert_eq!(
            kinds(tmp.path()),
            ["send-issued", "session-start", "send-confirmed"]
        );
        assert_eq!(
            reply,
            json!({"ok": {"submitted_at": events[1]["ts"], "cursor": 0}})
        );
    }

    /// A send to a verified wrapper whose input box goes quiet 3.3 s past `base`. The send reads
    /// its clock at 1 s (its start) and 2 s (the gate's start), then the gate reads it at 3 s,
    /// still waiting, where `during` runs, and at 4 s, ready. The reply, the instance dir and the
    /// pastes.
    fn sent_after_the_gate_wait(
        during: impl FnOnce(&WheelSlot) + Send + 'static,
    ) -> (Value, tempfile::TempDir, Pastes) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let base = Instant::now();
        let wheel: Arc<WheelSlot> = Arc::default();
        let moved = Arc::clone(&wheel);
        let now = Arc::new(Mutex::new(base));
        let clock = WaitClock {
            now: Arc::clone(&now),
            at: base + Duration::from_secs(3),
            during: Mutex::new(Some(Box::new(move || during(&moved)))),
        };
        let slot = SendSlot::new(clock, true, wheel);
        let (pastes, _pasted) = Pastes::new();
        let gate = verified_gate(
            base + Duration::from_secs(3),
            &["❯ ", "  ⏸ manual mode on · ← for agents"],
        );
        slot.attach(pastes.paste_fn(), gate);
        let params = json!({"v": 1, "text": "hello"});
        let reply = send(&slot, &name(), tmp.path(), &call(&params, 1)).expect("answered");
        assert_eq!(
            *now.lock().expect("clock"),
            base + Duration::from_secs(4),
            "one waiting step, then ready"
        );
        assert!(slot.flight().send.is_none(), "the slot is free again");
        (reply, tmp, pastes)
    }

    #[test]
    fn send_a_human_key_after_the_gate_wait_is_human_typing() {
        let (reply, tmp, pastes) = sent_after_the_gate_wait(WheelSlot::human_input);
        assert_eq!(reply, json!({"refusal": "human-typing", "detail": null}));
        assert_eq!(kinds(tmp.path()), ["send-refused"]);
        assert_refused_without_a_cursor(tmp.path(), None);
        assert!(pastes.all().is_empty(), "nothing typed");
    }

    #[test]
    fn send_a_turn_started_after_the_gate_wait_is_turn_running() {
        let (reply, tmp, pastes) = sent_after_the_gate_wait(WheelSlot::turn_started);
        assert_eq!(
            reply,
            json!({"refusal": "not-delivered", "detail": "turn-running"})
        );
        assert_eq!(kinds(tmp.path()), ["send-refused"]);
        assert_refused_without_a_cursor(tmp.path(), Some("turn-running"));
        assert!(pastes.all().is_empty(), "nothing typed");
    }

    #[test]
    fn send_after_the_gate_wait_human_typing_comes_before_turn_running() {
        let (reply, tmp, pastes) = sent_after_the_gate_wait(|wheel| {
            wheel.turn_started();
            wheel.human_input();
        });
        assert_eq!(reply, json!({"refusal": "human-typing", "detail": null}));
        assert_eq!(kinds(tmp.path()), ["send-refused"]);
        assert!(pastes.all().is_empty(), "nothing typed");
    }

    /// A verified screen that stays quiet with no input box: the gate starts at the clock's 2 s
    /// and refuses at 11 s, its first one-second step at or past 8.5 s of waiting.
    #[test]
    fn send_on_a_verified_screen_without_a_literal_waits_for_the_input_box_to_the_bound() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let base = Instant::now();
        let now = Arc::new(Mutex::new(base));
        let clock = WaitClock {
            now: Arc::clone(&now),
            at: base,
            during: Mutex::new(None),
        };
        let slot = SendSlot::new(clock, true, Arc::default());
        let (pastes, _pasted) = Pastes::new();
        let gate = verified_gate(base - Duration::from_secs(1), &["paste again to expand"]);
        slot.attach(pastes.paste_fn(), gate);
        let params = json!({"v": 1, "text": "hello"});
        let reply = send(&slot, &name(), tmp.path(), &call(&params, 1)).expect("answered");
        assert_eq!(
            reply,
            json!({"refusal": "not-delivered", "detail": "input-not-ready"})
        );
        assert_eq!(*now.lock().expect("clock"), base + Duration::from_secs(11));
        assert_eq!(kinds(tmp.path()), ["send-refused"]);
        assert_refused_without_a_cursor(tmp.path(), Some("input-not-ready"));
        assert!(pastes.all().is_empty(), "nothing typed");
        assert!(slot.flight().send.is_none(), "the slot is free again");
    }

    #[test]
    fn send_confirm_window_is_the_fallback() {
        assert_eq!(window_ms(), 10_000);
    }
}
