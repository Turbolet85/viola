//! The budget gate's refusal of a send.

use super::*;

impl Ctx<'_> {
    /// `budget-paused` with the crossed window's word as its detail: before `send-issued`, so no
    /// cursor.
    pub(super) fn budget_paused(&self, crossed: &Crossed) -> Result<Value, ProtocolError> {
        self.refuse_with(
            None,
            RefusalReason::BudgetPaused,
            Some(crossed.window.as_str()),
            Some(crossed),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_support::*;

    use super::*;
    use std::sync::Arc;

    use rstest::rstest;

    /// A window over its threshold refuses the send with the window's word: nothing is typed, no
    /// cursor is given, and the one wrapper line carries the crossed window's figures. The gate's
    /// change is recorded once, ahead of the first refusal.
    #[rstest]
    #[case::five_hour(91.0, 10.0, "five-hour", 91.0)]
    #[case::five_hour_at_its_threshold(90.0, 10.0, "five-hour", 90.0)]
    #[case::seven_day(10.0, 86.0, "seven-day", 86.0)]
    #[case::both_name_five_hour(95.0, 99.0, "five-hour", 95.0)]
    fn send_budget_paused_is_refused_with_the_window_word_and_no_cursor(
        #[case] five_hour: f64,
        #[case] seven_day: f64,
        #[case] window: &str,
        #[case] crossed: f64,
    ) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (dir, slot, pastes) = budgeted(tmp.path(), five_hour, seven_day);
        let params = json!({"v": 1, "text": "hello", "from": "overseer"});
        let (reply, lines) = capture_lines(|| send(&slot, &name(), &dir, &call(&params, 7)));
        assert_eq!(
            reply.expect("answered"),
            json!({"refusal": "budget-paused", "detail": window})
        );
        assert!(pastes.all().is_empty(), "nothing typed");
        assert!(slot.flight().send.is_none(), "nothing reserved");
        assert_eq!(kinds(&dir), ["budget-gate", "send-refused"]);
        assert_eq!(
            events(&dir)[0]["data"],
            json!({"paused": true, "window": window})
        );
        assert_eq!(
            events(&dir)[1]["data"],
            json!({"refusal": "budget-paused", "detail": window})
        );
        assert_refused_without_a_cursor(&dir, Some(window));

        let mut line = lines
            .iter()
            .find(|l| l["event"] == "send-refused")
            .expect("a send-refused line")
            .clone();
        assert!(line["corr"].is_u64(), "{line}");
        for read in ["corr", "process", "instance"] {
            line.as_object_mut().expect("object").remove(read);
        }
        assert_eq!(
            line,
            json!({"event": "send-refused", "message": "send-refused", "rpc_id": 7,
                "conn": "cli-1-2-3", "from": "overseer", "from_trust": "self-reported",
                "side": "wrapper", "wheel": "driver", "refusal": "budget-paused",
                "detail": window, "window": window, "used_percentage": crossed,
                "resets_at": RESET, "paused": true})
        );
        assert!(!lines.iter().any(|l| l["event"] == "send-issued"));

        let again = send(&slot, &name(), &dir, &call(&params, 8)).expect("answered");
        assert_eq!(again, json!({"refusal": "budget-paused", "detail": window}));
        assert_eq!(kinds(&dir), ["budget-gate", "send-refused", "send-refused"]);
    }

    /// A refusal that is not the gate's carries none of the gate's four fields.
    #[test]
    fn send_budget_paused_fields_are_on_no_other_refusal() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (dir, slot, _pastes) = budgeted(tmp.path(), 10.0, 10.0);
        slot.wheel.turn_started();
        let params = json!({"v": 1, "text": "hello"});
        let (reply, lines) = capture_lines(|| send(&slot, &name(), &dir, &call(&params, 1)));
        assert_eq!(
            reply.expect("answered"),
            json!({"refusal": "not-delivered", "detail": "turn-running"})
        );
        let line = lines
            .iter()
            .find(|l| l["event"] == "send-refused")
            .expect("a send-refused line");
        for absent in ["window", "used_percentage", "resets_at", "paused"] {
            assert!(line.get(absent).is_none(), "{absent} in {line}");
        }
    }

    /// The gate stands after the wheel: under a human wheel the refusal is `human-typing`, and the
    /// gate is not read, so it records nothing.
    #[test]
    fn send_budget_paused_comes_after_human_typing() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (dir, slot, pastes) = budgeted(tmp.path(), 91.0, 10.0);
        slot.wheel.human_input();
        let params = json!({"v": 1, "text": "hello"});
        let reply = send(&slot, &name(), &dir, &call(&params, 1)).expect("answered");
        assert_eq!(reply, json!({"refusal": "human-typing", "detail": null}));
        assert_eq!(kinds(&dir), ["send-refused"]);
        assert!(pastes.all().is_empty(), "nothing typed");
    }

    /// The text's own refusals stand ahead of the gate too, and leave it unread.
    #[rstest]
    #[case::a_refused_character("\u{1b}", "control-character")]
    #[case::an_empty_typed_text("\r\n", "empty-text")]
    fn send_budget_paused_comes_after_the_text_refusals(#[case] text: &str, #[case] detail: &str) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (dir, slot, _pastes) = budgeted(tmp.path(), 91.0, 10.0);
        let params = json!({"v": 1, "text": text});
        let reply = send(&slot, &name(), &dir, &call(&params, 1)).expect("answered");
        assert_eq!(reply, json!({"refusal": "not-delivered", "detail": detail}));
        assert_eq!(kinds(&dir), ["send-refused"]);
    }

    /// The gate stands ahead of the running turn, whether a hook line or another send holds it,
    /// and leaves the turn running.
    #[rstest]
    #[case::a_turn_the_hooks_started(false)]
    #[case::another_send_in_flight(true)]
    fn send_budget_paused_comes_before_turn_running(#[case] in_flight: bool) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (dir, slot, pastes) = budgeted(tmp.path(), 91.0, 10.0);
        if in_flight {
            occupy(&slot);
        } else {
            slot.wheel.turn_started();
        }
        let params = json!({"v": 1, "text": "hello"});
        let reply = send(&slot, &name(), &dir, &call(&params, 1)).expect("answered");
        assert_eq!(
            reply,
            json!({"refusal": "budget-paused", "detail": "five-hour"})
        );
        assert!(pastes.all().is_empty(), "nothing typed");
        assert_eq!(
            slot.wheel.turn_running(),
            !in_flight,
            "the turn is as it was"
        );
        assert_eq!(
            slot.flight().send.is_some(),
            in_flight,
            "the slot is as it was"
        );
    }

    /// Under both thresholds the send goes on down the order: typed, then never submitted by the
    /// mute child. The gate, still open, records nothing.
    #[test]
    fn send_budget_paused_does_not_refuse_under_the_thresholds() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (dir, slot, pastes) = budgeted(tmp.path(), 89.0, 84.0);
        let params = json!({"v": 1, "text": "hello"});
        let reply = send(&slot, &name(), &dir, &call(&params, 1)).expect("answered");
        assert_eq!(
            reply,
            json!({"refusal": "not-delivered", "detail": "no-prompt-submitted"})
        );
        assert_eq!(pastes.all(), ["hello"]);
        assert_eq!(kinds(&dir), ["send-issued", "send-refused"]);
    }

    /// The gate is read once, at arrival: a reading that crosses a threshold while the readiness
    /// gate waits does not refuse the send that is already past the rung. The next send is.
    #[test]
    fn send_budget_paused_is_not_read_again_after_the_gate_wait() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (home, dir, budget) = budget_rig(tmp.path());
        feed_budget(&home, 10.0, 10.0);
        let base = Instant::now();
        let clock = WaitClock {
            now: Arc::new(Mutex::new(base)),
            at: base + Duration::from_secs(3),
            during: Mutex::new(Some(Box::new(move || feed_budget(&home, 91.0, 10.0)))),
        };
        let slot = SendSlot::new(clock, true, Arc::default());
        slot.budget_with(budget);
        let (pastes, _pasted) = Pastes::new();
        let gate = verified_gate(
            base + Duration::from_secs(3),
            &["❯ ", "  ⏸ manual mode on · ← for agents"],
        );
        slot.attach(pastes.paste_fn(), gate);
        let params = json!({"v": 1, "text": "hello"});
        let reply = send(&slot, &name(), &dir, &call(&params, 1)).expect("answered");
        assert_eq!(
            reply,
            json!({"refusal": "not-delivered", "detail": "no-prompt-submitted"})
        );
        assert_eq!(pastes.all(), ["hello"], "typed after the wait");
        assert_eq!(kinds(&dir), ["send-issued", "send-refused"]);

        let next = send(&slot, &name(), &dir, &call(&params, 2)).expect("answered");
        assert_eq!(
            next,
            json!({"refusal": "budget-paused", "detail": "five-hour"})
        );
        assert_eq!(pastes.all(), ["hello"], "nothing more typed");
    }

    /// A gate change that cannot be recorded answers `-32603`, and nothing is typed.
    #[test]
    fn send_budget_paused_whose_record_cannot_land_is_internal() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (dir, slot, pastes) = budgeted(tmp.path(), 91.0, 10.0);
        std::fs::remove_dir_all(&dir).expect("the instance dir gone");
        let params = json!({"v": 1, "text": "hello"});
        assert_eq!(
            send(&slot, &name(), &dir, &call(&params, 1)),
            Err(ProtocolError::Internal)
        );
        assert!(pastes.all().is_empty(), "nothing typed");
    }
}
