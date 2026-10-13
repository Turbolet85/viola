//! The wrapper's `send` (architecture [Delivery Confirmation]): the text re-validated, the wheel,
//! one send in flight, the readiness gate, `send-issued` at the pre-paste cursor, one bracketed paste + Enter,
//! then confirmation after the fact — the matching `prompt-submitted`, relabelled `driver`, inside
//! the window — or `not-delivered`. The text is validated as received and typed with every CR LF
//! pair and every other CR as one LF and without its trailing CR and LF characters (`typed_text`;
//! the founder's rulings of 2026-10-07T15:21Z, 2026-10-09 and 2026-10-09T16:51Z): the list, the
//! paste, `text_bytes` and the exact match all read that one typed text, which holds no CR,
//! and a typed text with nothing in it is refused `empty-text` before the wheel is read. A text that is
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

use crate::run::budget::{BudgetGate, Crossed, Trigger};
use crate::run::gate::Gate;
use crate::run::links::LinkSet;
use crate::run::wait::WaitFeed;
use crate::run::wheel::WheelSlot;

mod budget;
mod classify;
mod params;
mod relabel;
mod slot;
#[cfg(test)]
mod test_support;

pub(crate) use params::parse_from;
pub(crate) use slot::{SendSlot, append_hook_event};

use classify::{Confirms, Match, classify};
use params::parse;
use slot::{InFlight, Reserved};

/// What a send's lines and records carry besides the outcome.
struct Ctx<'a> {
    name: &'a ViolaName,
    wheel: &'a WheelSlot,
    instance_dir: &'a Path,
    call: &'a Call<'a>,
    from: Option<ViolaName>,
    started: Instant,
}

/// The `send` method, in the documented refusal order. The text's own refusals come first, a
/// refused character and then an empty typed text, before anything of the session is read. The
/// wheel and the running turn are read twice, in that order both times: at arrival, and again
/// once the gate's wait has ended, because that wait can run to `GATE_MAX_WAIT`. A human key or a
/// turn that started during it refuses with nothing typed. The budget gate is evaluated once,
/// between the first two readings, and never after the wait. A send no rung refused, from another
/// instance, links that instance to this one before `send-issued` (architecture [Links]); a
/// refused send links nothing.
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
    let typed = typed_text(text);
    let text: &str = &typed;
    if text.is_empty() {
        return ctx.not_delivered(None, NotDelivered::EmptyText);
    }
    if let Some(detail) = slot.wheel.human_typing() {
        return ctx.refuse(
            None,
            RefusalReason::HumanTyping,
            detail.map(HumanTyping::as_str),
        );
    }
    if let Some(budget) = slot.budget.get() {
        let crossed = budget
            .evaluate(Trigger::Send, Utc::now())
            .map_err(|_| ProtocolError::Internal)?;
        if let Some(crossed) = crossed {
            return ctx.budget_paused(&crossed);
        }
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
    if let (Some(links), Some(from)) = (slot.links.get(), ctx.from.as_ref())
        && from != name
    {
        links.record(from).map_err(|_| ProtocolError::Internal)?;
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

    fn refuse(
        &self,
        cursor: Option<u64>,
        refusal: RefusalReason,
        detail: Option<&'static str>,
    ) -> Result<Value, ProtocolError> {
        self.refuse_with(cursor, refusal, detail, None)
    }

    /// A refusal: before `send-issued` (no cursor; the line's `corr` is the end offset at refusal,
    /// obs-plan D-28) or after it (the send's own cursor). Its line carries the wheel's holder,
    /// and for a budget refusal the crossed window's figures.
    fn refuse_with(
        &self,
        cursor: Option<u64>,
        refusal: RefusalReason,
        detail: Option<&'static str>,
        budget: Option<&Crossed>,
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
            window = budget.map(|crossed| crossed.window.as_str()),
            used_percentage = budget.map(|crossed| crossed.used_percentage),
            resets_at = budget.map(|crossed| crossed.resets_at.as_str()),
            paused = budget.map(|_| true),
        );
        Ok(json!({"refusal": refusal.as_str(), "detail": detail}))
    }
}

#[cfg(test)]
mod tests {
    use super::test_support::*;
    use super::*;
    use std::sync::Arc;

    use rstest::rstest;

    use viola_state::snapshot::Wheel;

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

    /// The `send` order, written out: control-character ahead of everything, then an empty typed
    /// text, then the wheel, then the one in flight, then the gate, then the window.
    #[rstest]
    #[case::control_character_before_empty_text(
        "\u{1b}\r\n",
        Setup::NoChild,
        "not-delivered",
        Some("control-character")
    )]
    #[case::empty_text_before_human_typing(
        "\r\n",
        Setup::HumanInFlight,
        "not-delivered",
        Some("empty-text")
    )]
    #[case::empty_text_before_manual_pause(
        "\n",
        Setup::PausedTurn,
        "not-delivered",
        Some("empty-text")
    )]
    #[case::empty_text_before_turn_running(
        "",
        Setup::InFlightNoChild,
        "not-delivered",
        Some("empty-text")
    )]
    #[case::empty_text_before_a_running_turn(
        "\r\r",
        Setup::TurnNoChild,
        "not-delivered",
        Some("empty-text")
    )]
    #[case::empty_text_before_input_not_ready(
        "\n\r",
        Setup::Poisoned,
        "not-delivered",
        Some("empty-text")
    )]
    #[case::control_character_beside_an_inner_crlf(
        "x\r\ny\u{1b}",
        Setup::NoChild,
        "not-delivered",
        Some("control-character")
    )]
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

    /// A typed text with nothing in it is refused first after `control-character`: before the
    /// wheel, the slot and the child are read. Nothing is reserved, issued or typed, and the
    /// refusal's record holds no cursor. The child here takes every paste, so a text that got
    /// past the rung would be typed.
    #[rstest]
    #[case::the_empty_text("")]
    #[case::one_lf("\n")]
    #[case::one_crlf("\r\n")]
    #[case::two_crs("\r\r")]
    #[case::a_crlf_then_a_cr("\r\n\r")]
    fn send_empty_text_is_refused_first_after_control_character(#[case] text: &str) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (slot, pastes) = mute_child();
        let params = json!({"v": 1, "text": text});
        let reply = send(&slot, &name(), tmp.path(), &call(&params, 1)).expect("answered");
        assert_eq!(
            reply,
            json!({"refusal": "not-delivered", "detail": "empty-text"})
        );
        assert!(pastes.all().is_empty(), "nothing typed");
        assert_eq!(kinds(tmp.path()), ["send-refused"], "no send-issued");
        assert_refused_without_a_cursor(tmp.path(), Some("empty-text"));
        assert_eq!(
            events(tmp.path())[0]["data"],
            json!({"refusal": "not-delivered", "detail": "empty-text"})
        );
        assert!(slot.flight().send.is_none(), "nothing reserved");
        assert_eq!(slot.wheel.holder(), Wheel::Driver);
        assert!(!slot.wheel.turn_running());
    }

    /// The rung stands ahead of the wheel: under a human wheel the refusal is still `empty-text`,
    /// and the wheel stays where the human put it.
    #[test]
    fn send_empty_text_under_a_human_wheel_is_still_empty_text() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (slot, pastes) = mute_child();
        slot.wheel.human_input();
        let params = json!({"v": 1, "text": "\r\n"});
        let reply = send(&slot, &name(), tmp.path(), &call(&params, 1)).expect("answered");
        assert_eq!(
            reply,
            json!({"refusal": "not-delivered", "detail": "empty-text"})
        );
        assert!(pastes.all().is_empty(), "nothing typed");
        assert_eq!(kinds(tmp.path()), ["send-refused"]);
        assert_refused_without_a_cursor(tmp.path(), Some("empty-text"));
        assert_eq!(slot.wheel.holder(), Wheel::Human, "the wheel is not moved");
    }

    /// The rung stands ahead of the running turn too, and leaves the turn running.
    #[test]
    fn send_empty_text_while_a_turn_runs_is_still_empty_text() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (slot, pastes) = mute_child();
        slot.wheel.turn_started();
        let params = json!({"v": 1, "text": "\n"});
        let reply = send(&slot, &name(), tmp.path(), &call(&params, 1)).expect("answered");
        assert_eq!(
            reply,
            json!({"refusal": "not-delivered", "detail": "empty-text"})
        );
        assert!(pastes.all().is_empty(), "nothing typed");
        assert_eq!(kinds(tmp.path()), ["send-refused"]);
        assert_refused_without_a_cursor(tmp.path(), Some("empty-text"));
        assert!(slot.wheel.turn_running(), "the turn still runs");
        assert_eq!(slot.wheel.holder(), Wheel::Driver);
    }

    /// A refused character is refused whatever follows it: the text is validated as received.
    #[rstest]
    #[case::lf("\n")]
    #[case::cr("\r")]
    #[case::crlf("\r\n")]
    fn send_a_refused_character_before_a_trailing_newline_is_control_character(#[case] tail: &str) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (slot, pastes) = mute_child();
        let params = json!({"v": 1, "text": format!("x\u{1b}[201~{tail}")});
        let reply = send(&slot, &name(), tmp.path(), &call(&params, 1)).expect("answered");
        assert_eq!(
            reply,
            json!({"refusal": "not-delivered", "detail": "control-character"})
        );
        assert!(pastes.all().is_empty(), "nothing typed");
        assert_eq!(kinds(tmp.path()), ["send-refused"]);
        assert_refused_without_a_cursor(tmp.path(), Some("control-character"));
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

    /// An issued send from another instance: `link` is the line directly before its
    /// `send-issued`, whose cursor is still the offset its own line starts at.
    #[test]
    fn send_link_an_issued_send_from_another_instance_appends_link_before_send_issued() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (slot, pastes) = linking(tmp.path());
        let params = json!({"v": 1, "text": "hello", "from": "overseer"});
        let reply = send(&slot, &name(), tmp.path(), &call(&params, 1)).expect("answered");
        assert_eq!(
            reply,
            json!({"refusal": "not-delivered", "detail": "no-prompt-submitted"})
        );
        assert_eq!(pastes.all(), ["hello"]);
        assert_eq!(kinds(tmp.path()), ["link", "send-issued", "send-refused"]);
        let events = events(tmp.path());
        assert_eq!(events[0]["source"], "wrapper");
        assert_eq!(events[0]["instance"], "builder");
        assert_eq!(
            events[0]["data"],
            json!({"driver": "overseer", "driven": "builder"})
        );
        let issued_at = line_starts(tmp.path())[1];
        assert!(issued_at > 0);
        assert_eq!(
            events[1]["data"],
            json!({"cursor": issued_at, "from": "overseer"})
        );
        assert_eq!(events[2]["data"]["cursor"], issued_at);
    }

    #[test]
    fn send_link_a_second_issued_send_adds_no_link() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (slot, pastes) = linking(tmp.path());
        let params = json!({"v": 1, "text": "hello", "from": "overseer"});
        for id in [1, 2] {
            send(&slot, &name(), tmp.path(), &call(&params, id)).expect("answered");
        }
        assert_eq!(pastes.all(), ["hello", "hello"]);
        assert_eq!(
            kinds(tmp.path()),
            [
                "link",
                "send-issued",
                "send-refused",
                "send-issued",
                "send-refused"
            ]
        );
    }

    /// A send refused before it is issued links nothing, whichever rung refuses it.
    #[rstest]
    #[case::human_typing(true, "human-typing", None)]
    #[case::turn_running(false, "not-delivered", Some("turn-running"))]
    fn send_link_a_refused_send_adds_none(
        #[case] human_key: bool,
        #[case] refusal: &str,
        #[case] detail: Option<&str>,
    ) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (slot, pastes) = linking(tmp.path());
        slot.wheel.turn_started();
        if human_key {
            slot.wheel.human_input();
        }
        let params = json!({"v": 1, "text": "hello", "from": "overseer"});
        let reply = send(&slot, &name(), tmp.path(), &call(&params, 1)).expect("answered");
        assert_eq!(reply, json!({"refusal": refusal, "detail": detail}));
        assert!(pastes.all().is_empty(), "nothing typed");
        assert_eq!(kinds(tmp.path()), ["send-refused"]);
    }

    /// The link is made to another instance: a `from` that is the wrapper's own name is none.
    #[test]
    fn send_link_a_send_under_the_wrappers_own_name_adds_none() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (slot, pastes) = linking(tmp.path());
        let params = json!({"v": 1, "text": "hello", "from": "builder"});
        send(&slot, &name(), tmp.path(), &call(&params, 1)).expect("answered");
        assert_eq!(pastes.all(), ["hello"]);
        assert_eq!(kinds(tmp.path()), ["send-issued", "send-refused"]);
        assert_eq!(events(tmp.path())[0]["data"]["from"], "builder");
    }

    #[rstest]
    #[case::absent(json!({"v": 1, "text": "hello"}))]
    #[case::null(json!({"v": 1, "text": "hello", "from": null}))]
    fn send_link_a_send_with_no_from_adds_none(#[case] params: Value) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (slot, pastes) = linking(tmp.path());
        send(&slot, &name(), tmp.path(), &call(&params, 1)).expect("answered");
        assert_eq!(pastes.all(), ["hello"]);
        assert_eq!(kinds(tmp.path()), ["send-issued", "send-refused"]);
    }

    /// A link that cannot be recorded answers `-32603`: nothing is issued and nothing typed.
    #[test]
    fn send_link_a_link_that_cannot_be_recorded_is_internal_and_issues_nothing() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (slot, pastes) = linking(&tmp.path().join("missing"));
        let params = json!({"v": 1, "text": "hello", "from": "overseer"});
        assert_eq!(
            send(&slot, &name(), tmp.path(), &call(&params, 1)),
            Err(ProtocolError::Internal)
        );
        assert!(pastes.all().is_empty(), "nothing typed");
        assert!(events(tmp.path()).is_empty(), "nothing issued");
        assert!(slot.flight().send.is_none(), "the slot is free again");
    }
}
