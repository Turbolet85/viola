//! The in-flight send's prompt: matched on the typed text and relabelled `driver`, and every
//! other prompt appended as the hook filed it.

#[cfg(test)]
mod tests {
    use super::super::test_support::*;
    use super::super::*;
    use std::sync::Arc;

    use rstest::rstest;

    use viola_state::snapshot::Wheel;

    #[test]
    fn send_match_is_ok_with_the_prompt_ts_and_the_cursor() {
        confirmed_with("human");
    }

    #[test]
    fn send_harness_filed_prompt_with_the_in_flight_text_is_relabelled_driver() {
        confirmed_with("harness");
    }

    /// A text ending in CR and LF characters is typed without them, and the prompt the hook
    /// reports for the typed text is the send's own: confirmed, the wheel left with the driver.
    #[rstest]
    #[case::one("\n")]
    #[case::two("\n\n")]
    #[case::one_cr("\r")]
    #[case::one_crlf("\r\n")]
    #[case::two_crs("\r\r")]
    #[case::crlf_twice("\r\n\r\n")]
    #[case::an_lf_before_a_cr("\n\r")]
    fn send_trailing_newlines_are_not_typed_and_the_send_is_confirmed(#[case] tail: &str) {
        confirmed_as(&format!("{CANARY}{tail}"), CANARY, "human");
    }

    /// A CR or a CR LF inside a text is typed as one LF, and the prompt the hook reports for the
    /// typed text is the send's own: one paste of the typed text, the prompt relabelled `driver`,
    /// the send confirmed, the wheel left with the driver.
    #[rstest]
    #[case::one_crlf_inside(
        "canary-chain-value-5c1e\r\nsecond line",
        "canary-chain-value-5c1e\nsecond line"
    )]
    #[case::one_lone_cr_inside(
        "canary-chain-value-5c1e\rsecond line",
        "canary-chain-value-5c1e\nsecond line"
    )]
    #[case::two_crs_in_a_row(
        "canary-chain-value-5c1e\r\rsecond line",
        "canary-chain-value-5c1e\n\nsecond line"
    )]
    #[case::an_lf_before_a_cr(
        "canary-chain-value-5c1e\n\rsecond line",
        "canary-chain-value-5c1e\n\nsecond line"
    )]
    #[case::crlf_on_every_line_with_a_crlf_ending(
        "canary-chain-value-5c1e\r\nsecond line\r\nthird line\r\n",
        "canary-chain-value-5c1e\nsecond line\nthird line"
    )]
    fn send_text_whose_inner_cr_is_one_lf_is_typed_and_confirmed(
        #[case] sent: &str,
        #[case] typed: &str,
    ) {
        confirmed_as(sent, typed, "human");
    }

    /// The claim takes no longer text on the submitted side. While a text with an inner CR is in
    /// flight, typed with an LF there, a prompt that still holds the CR is not the typed text: it
    /// is appended as the hook filed it, the human it names takes the wheel, and the send waits on
    /// to its window.
    #[rstest]
    #[case::a_crlf_inside("canary-chain-value-5c1e\r\nsecond line")]
    #[case::a_lone_cr_inside("canary-chain-value-5c1e\rsecond line")]
    fn send_a_prompt_that_keeps_the_cr_while_its_inner_cr_is_one_lf_claims_nothing(
        #[case] sent: &str,
    ) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let base = Instant::now();
        let (clock, jumping) = SwitchClock::at(base);
        let slot = SendSlot::new(clock, false, Arc::default());
        let (pastes, pasted) = Pastes::new();
        slot.attach(pastes.paste_fn(), quiet_gate(base));
        let params = json!({"v": 1, "text": sent});
        let (filed, waiting, reply) = std::thread::scope(|s| {
            let sending = s.spawn(|| send(&slot, &name(), tmp.path(), &call(&params, 1)));
            pasted.recv().expect("pasted");
            let appended =
                append_hook_event(&slot, &feed(), tmp.path(), prompt(sent, "human"), None);
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
        assert_eq!(pastes.all(), ["canary-chain-value-5c1e\nsecond line"]);
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

    /// The claim takes no longer text. While a text that ended in a newline is in flight, a prompt
    /// that still carries the newline is not the typed text: it is appended as the hook filed it,
    /// the human it names takes the wheel, and the send waits on to its window.
    #[rstest]
    #[case::lf("\n")]
    #[case::cr("\r")]
    #[case::crlf("\r\n")]
    fn send_a_prompt_that_keeps_the_trailing_newline_claims_nothing(#[case] tail: &str) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let base = Instant::now();
        let (clock, jumping) = SwitchClock::at(base);
        let slot = SendSlot::new(clock, false, Arc::default());
        let (pastes, pasted) = Pastes::new();
        slot.attach(pastes.paste_fn(), quiet_gate(base));
        let sent = format!("{CANARY}{tail}");
        let params = json!({"v": 1, "text": sent});
        let (filed, waiting, reply) = std::thread::scope(|s| {
            let sending = s.spawn(|| send(&slot, &name(), tmp.path(), &call(&params, 1)));
            pasted.recv().expect("pasted");
            let appended =
                append_hook_event(&slot, &feed(), tmp.path(), prompt(&sent, "human"), None);
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

    /// A prompt the CLI submitted with the typed text's trailing run of whitespace removed is the
    /// send's own: relabelled `driver` with its submitted text, confirmed, the wheel left with
    /// the driver.
    #[rstest]
    #[case::a_space(" ")]
    #[case::a_tab("\t")]
    #[case::three_spaces("   ")]
    #[case::space_tab_space(" \t ")]
    fn send_a_prompt_the_cli_trimmed_at_the_end_is_relabelled_driver_and_confirmed(
        #[case] tail: &str,
    ) {
        let sent = format!("run the skill{tail}");
        let (reply, events, pastes, holder) = sent_then_submitted(&sent, "run the skill", "human");
        assert_eq!(pastes, [sent.as_str()]);
        let kinds: Vec<&str> = events
            .iter()
            .map(|e| e["kind"].as_str().expect("kind"))
            .collect();
        assert_eq!(kinds, ["send-issued", "prompt-submitted", "send-confirmed"]);
        assert_eq!(
            events[1]["data"],
            json!({"text": "run the skill", "origin": "driver"})
        );
        assert_eq!(
            reply,
            json!({"ok": {"submitted_at": events[1]["ts"], "cursor": 0}})
        );
        assert_eq!(holder, Wheel::Driver);
    }

    /// The tolerance is the trailing side alone: a prompt that lost the typed text's leading
    /// whitespace is not the send's, the human it names takes the wheel, and the send expires.
    #[test]
    fn send_a_prompt_trimmed_at_the_start_claims_nothing() {
        let (reply, events, pastes, holder) = sent_then_submitted(" run", "run", "human");
        assert_eq!(pastes, [" run"]);
        assert_eq!(events[1]["data"], json!({"text": "run", "origin": "human"}));
        assert_eq!(
            reply,
            json!({"refusal": "not-delivered", "detail": "no-prompt-submitted"})
        );
        assert_eq!(holder, Wheel::Human);
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
            None,
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
        append_hook_event(
            &slot,
            &feed(),
            tmp.path(),
            prompt("injected", "harness"),
            None,
        )
        .expect("hook");
        assert_eq!(slot.wheel.holder(), Wheel::Driver);
        let missing = tmp.path().join("missing");
        assert!(
            append_hook_event(&slot, &feed(), &missing, prompt("lost", "human"), None).is_err()
        );
        assert_eq!(slot.wheel.holder(), Wheel::Human);
        assert_eq!(slot.wheel.human_typing(), Some(None));
    }

    /// A keyless human-filed prompt after a turn end that counted a scheduled wake, on a verified
    /// CLI, is the wake's fire: appended `harness`, the wheel left with the driver. A key since the
    /// last human-filed prompt, no wake counted, a null count or an unverified CLI: `human`, as
    /// typed.
    #[rstest]
    #[case::a_fired_wake(false, Some(1), true, "harness", Wheel::Driver)]
    #[case::a_key_before_it(true, Some(1), true, "human", Wheel::Human)]
    #[case::no_wake_counted(false, Some(0), true, "human", Wheel::Human)]
    #[case::a_null_count(false, None, true, "human", Wheel::Human)]
    #[case::an_unverified_cli(false, Some(1), false, "human", Wheel::Human)]
    fn send_a_keyless_human_prompt_after_a_counted_wake_is_told_by_the_rule(
        #[case] typed: bool,
        #[case] wakes: Option<u64>,
        #[case] verified: bool,
        #[case] origin: &str,
        #[case] holder: Wheel,
    ) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = SendSlot::new(FixedClock(Instant::now()), verified, Arc::default());
        let ended = json!({
            "last_assistant_message": null,
            "background_tasks": null,
            "scheduled_wakes": wakes,
        });
        append_hook_event(
            &slot,
            &feed(),
            tmp.path(),
            hook_line(EventKind::TurnEnded, ended),
            None,
        )
        .expect("turn end");
        if typed {
            slot.wheel.human_key();
            assert_eq!(
                slot.wheel.release(&call(&json!({"v": 1}), 1)),
                Ok(json!({"ok": {"wheel": "driver"}}))
            );
        }
        append_hook_event(
            &slot,
            &feed(),
            tmp.path(),
            prompt("Check the build.", "human"),
            None,
        )
        .expect("prompt");
        assert_eq!(events(tmp.path())[1]["data"]["origin"], origin);
        assert_eq!(slot.wheel.holder(), holder);
        assert!(
            !slot.wheel.take_typed(),
            "the human-filed prompt took the mark"
        );
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
            None,
        )
        .expect("hook");
        assert_eq!(events(tmp.path())[0]["data"]["origin"], "harness");
    }
}
