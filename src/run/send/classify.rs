//! The local-command decision: what confirms a send.

use super::*;

pub(super) enum Match {
    Waiting,
    /// A matching prompt is being appended; the outcome waits for it whatever the window.
    Claimed,
    Confirmed(String),
    /// The outcome is decided; the slot stays held until the send has recorded it.
    Closed,
}

/// What confirms a send (architecture [Delivery Confirmation]).
#[derive(Clone, Copy)]
pub(super) enum Confirms {
    /// Its `prompt-submitted`, matched on the text.
    Prompt,
    /// A listed local command's post-condition, measured on this CLI version.
    Post(PostCondition),
    /// Nothing: a listed local command with no post-condition, or none measured on this version.
    Nothing,
}

/// The compiled list read by the exact typed text: no trim, no case folding, never a leading
/// slash.
pub(super) fn classify(text: &str, cli_verified: bool) -> Confirms {
    match LOCAL_COMMANDS.iter().find(|(command, _)| *command == text) {
        None => Confirms::Prompt,
        Some((_, Some(post))) if cli_verified => Confirms::Post(*post),
        Some(_) => Confirms::Nothing,
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_support::*;

    use super::*;
    use std::sync::Arc;

    use rstest::rstest;

    /// A listed command nothing can confirm is typed and answered at once. The clock never moves,
    /// so a wait on the window would hang the case.
    #[rstest]
    #[case::no_post_condition_on_a_verified_version("/remote-control", true)]
    #[case::no_post_condition_on_an_unverified_version("/remote-control", false)]
    #[case::post_condition_not_measured_on_this_version("/clear", false)]
    #[case::compact_on_an_unverified_version("/compact", false)]
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
    #[case::compact_and_more("/compactx")]
    #[case::compact_after_a_leading_space(" /compact")]
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

    /// The list is read by the typed text, so `/clear` and a newline ending is the listed command.
    /// On a version where `/clear` is `unconfirmable`, it is typed as `/clear` and answered at
    /// once.
    #[rstest]
    #[case::lf("\n")]
    #[case::cr("\r")]
    #[case::crlf("\r\n")]
    fn send_local_command_decision_reads_the_text_without_its_trailing_newline(#[case] tail: &str) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let base = Instant::now();
        let slot = SendSlot::new(JumpClock(Mutex::new(base)), false, Arc::default());
        let (pastes, _pasted) = Pastes::new();
        slot.attach(pastes.paste_fn(), quiet_gate(base));
        let params = json!({"v": 1, "text": format!("/clear{tail}")});
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

    /// A listed command with no post-condition, followed by a newline ending, on a verified
    /// version: typed as the command and answered `unconfirmable` at once.
    #[rstest]
    #[case::lf("/remote-control", "\n")]
    #[case::cr("/remote-control", "\r")]
    #[case::crlf("/remote-control", "\r\n")]
    fn send_local_command_without_a_post_condition_and_a_trailing_newline_is_unconfirmable(
        #[case] command: &str,
        #[case] tail: &str,
    ) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let base = Instant::now();
        let slot = SendSlot::new(JumpClock(Mutex::new(base)), true, Arc::default());
        let (pastes, _pasted) = Pastes::new();
        slot.attach(pastes.paste_fn(), quiet_gate(base));
        let params = json!({"v": 1, "text": format!("{command}{tail}")});
        let reply = send(&slot, &name(), tmp.path(), &call(&params, 1)).expect("answered");
        assert_eq!(pastes.all(), [command]);
        assert_eq!(
            reply,
            json!({"ok": {"confirmed": false, "detail": "unconfirmable", "cursor": 0}})
        );
        assert_eq!(kinds(tmp.path()), ["send-issued", "send-confirmed"]);
    }

    /// `/clear` and a newline ending (LF, CR or CRLF) on a verified version is `/clear`: typed as
    /// the command, then confirmed by its post-condition as
    /// `send_local_command_clear_is_confirmed_by_a_new_session` is. The clock jumps once the new
    /// session is on disk, so a send it did not confirm expires.
    #[rstest]
    #[case::lf("\n")]
    #[case::cr("\r")]
    #[case::crlf("\r\n")]
    fn send_local_command_clear_with_a_trailing_newline_is_confirmed_by_a_new_session(
        #[case] tail: &str,
    ) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let base = Instant::now();
        let (clock, jumping) = SwitchClock::at(base);
        let slot = SendSlot::new(clock, true, Arc::default());
        let (pastes, pasted) = Pastes::new();
        slot.attach(pastes.paste_fn(), quiet_gate(base));
        let params = json!({"v": 1, "text": format!("/clear{tail}")});
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

    /// The tap's remembered id before a `/compact` send: a `session-start` of cause `startup`
    /// with `id`, appended; the cursor after it.
    fn booted(slot: &SendSlot, dir: &Path, id: Value) -> u64 {
        let boot = json!({"cause": "startup", "agent_session_id": id});
        append_hook_event(slot, &feed(), dir, hook_line(EventKind::SessionStart, boot))
            .expect("hook");
        std::fs::metadata(dir.join("events.ndjson"))
            .expect("events")
            .len()
    }

    /// `/compact` on a verified version waits for its post-condition: a `session-start` with cause
    /// `compact` and the id the tap remembered before it confirms it, that line's `ts` is the
    /// send's, and no `prompt-submitted` is involved.
    #[test]
    fn send_local_command_compact_is_confirmed_by_its_same_session() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let base = Instant::now();
        let (clock, jumping) = SwitchClock::at(base);
        let slot = SendSlot::new(clock, true, Arc::default());
        let (pastes, pasted) = Pastes::new();
        slot.attach(pastes.paste_fn(), quiet_gate(base));
        let cursor = booted(&slot, tmp.path(), json!("session-one"));
        let params = json!({"v": 1, "text": "/compact"});
        let compacted = json!({"cause": "compact", "agent_session_id": "session-one"});
        let reply = std::thread::scope(|s| {
            let sending = s.spawn(|| send(&slot, &name(), tmp.path(), &call(&params, 1)));
            pasted.recv().expect("pasted");
            let line = hook_line(EventKind::SessionStart, compacted.clone());
            append_hook_event(&slot, &feed(), tmp.path(), line).expect("hook");
            jumping.store(true, std::sync::atomic::Ordering::SeqCst);
            sending.join().expect("send thread")
        })
        .expect("answered");
        let events = events(tmp.path());
        assert_eq!(
            kinds(tmp.path()),
            [
                "session-start",
                "send-issued",
                "session-start",
                "send-confirmed"
            ]
        );
        assert_eq!(events[2]["data"], compacted);
        assert_eq!(events[3]["data"], json!({"cursor": cursor}));
        assert_eq!(
            reply,
            json!({"ok": {"submitted_at": events[2]["ts"], "cursor": cursor}})
        );
        assert_eq!(pastes.all(), ["/compact"]);
        assert!(slot.flight().send.is_none(), "the slot is free again");
    }

    /// Only a `compact` whose id is a string equal to the one the tap remembered before it is the
    /// post-condition: another cause, another id, no id, or nothing remembered (no `session-start`
    /// yet, or one that carried no id) claims nothing. The line is appended unchanged while the
    /// send waits, and the send then expires.
    #[rstest]
    #[case::clear_with_the_remembered_id(Some(json!("session-one")), "clear", json!("session-one"))]
    #[case::compact_with_another_id(Some(json!("session-one")), "compact", json!("session-two"))]
    #[case::compact_with_a_null_id(Some(json!("session-one")), "compact", Value::Null)]
    #[case::compact_with_nothing_remembered(None, "compact", json!("session-one"))]
    #[case::compact_after_a_start_with_no_id(Some(Value::Null), "compact", json!("session-one"))]
    fn send_local_command_compact_is_not_confirmed_by_another_session_start(
        #[case] boot: Option<Value>,
        #[case] cause: &str,
        #[case] id: Value,
    ) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let base = Instant::now();
        let (clock, jumping) = SwitchClock::at(base);
        let slot = SendSlot::new(clock, true, Arc::default());
        let (pastes, pasted) = Pastes::new();
        slot.attach(pastes.paste_fn(), quiet_gate(base));
        let booting = boot.is_some();
        let cursor = boot.map_or(0, |boot| booted(&slot, tmp.path(), boot));
        let params = json!({"v": 1, "text": "/compact"});
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
        let mut expected = vec!["send-issued", "session-start", "send-refused"];
        if booting {
            expected.insert(0, "session-start");
        }
        assert_eq!(kinds(tmp.path()), expected);
        let events = events(tmp.path());
        let at = usize::from(booting);
        assert_eq!(events[at + 1]["data"], other);
        assert_eq!(
            events[at + 2]["data"],
            json!({"refusal": "not-delivered", "detail": "no-prompt-submitted", "cursor": cursor})
        );
        assert_eq!(pastes.all(), ["/compact"]);
        assert!(slot.flight().send.is_none(), "the slot is free again");
    }

    /// Each reading is the next instant of the list, the last one repeated; it counts its readings.
    struct ListClock {
        at: Mutex<Vec<Instant>>,
        read: std::sync::atomic::AtomicUsize,
    }

    impl Clock for ListClock {
        fn now(&self) -> Instant {
            self.read.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            let mut at = self.at.lock().expect("clock");
            if at.len() > 1 { at.remove(0) } else { at[0] }
        }
    }

    /// A `/compact` send's window is its own 40 s bound: at 39.999 s past its opening the send
    /// still waits, at 40 s it closes unclaimed; a full send expires as `not-delivered` /
    /// `no-prompt-submitted` with the cursor.
    #[test]
    fn send_local_command_compact_window_expiry_is_no_prompt_submitted_at_its_own_bound() {
        let opened = Instant::now();
        let clock = ListClock {
            at: Mutex::new(vec![
                opened + Duration::from_millis(39_999),
                opened + Duration::from_secs(40),
            ]),
            read: std::sync::atomic::AtomicUsize::new(0),
        };
        let slot = SendSlot::new(clock, true, Arc::default());
        slot.flight().send = Some(InFlight {
            text: "/compact".to_owned(),
            confirms: classify("/compact", true),
            state: Match::Waiting,
        });
        assert_eq!(slot.confirm(opened), None);
        assert!(matches!(
            slot.flight().send.as_ref().map(|f| &f.state),
            Some(Match::Closed)
        ));
        drop(Reserved(&slot));

        let tmp = tempfile::tempdir().expect("tempdir");
        let base = Instant::now();
        let slot = SendSlot::new(JumpClock(Mutex::new(base)), true, Arc::default());
        let (pastes, _pasted) = Pastes::new();
        slot.attach(pastes.paste_fn(), quiet_gate(base));
        let params = json!({"v": 1, "text": "/compact"});
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
        assert_eq!(pastes.all(), ["/compact"]);
        assert!(slot.flight().send.is_none(), "the slot is free again");
    }

    /// `/compact` and a newline ending (LF, CR or CRLF) on a verified version is `/compact`: typed
    /// as the command, then confirmed by its same session as
    /// `send_local_command_compact_is_confirmed_by_its_same_session` is.
    #[rstest]
    #[case::lf("\n")]
    #[case::cr("\r")]
    #[case::crlf("\r\n")]
    fn send_local_command_compact_with_a_trailing_newline_is_confirmed_by_its_same_session(
        #[case] tail: &str,
    ) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let base = Instant::now();
        let (clock, jumping) = SwitchClock::at(base);
        let slot = SendSlot::new(clock, true, Arc::default());
        let (pastes, pasted) = Pastes::new();
        slot.attach(pastes.paste_fn(), quiet_gate(base));
        let cursor = booted(&slot, tmp.path(), json!("session-one"));
        let params = json!({"v": 1, "text": format!("/compact{tail}")});
        let compacted = json!({"cause": "compact", "agent_session_id": "session-one"});
        let reply = std::thread::scope(|s| {
            let sending = s.spawn(|| send(&slot, &name(), tmp.path(), &call(&params, 1)));
            pasted.recv().expect("pasted");
            let line = hook_line(EventKind::SessionStart, compacted.clone());
            append_hook_event(&slot, &feed(), tmp.path(), line).expect("hook");
            jumping.store(true, std::sync::atomic::Ordering::SeqCst);
            sending.join().expect("send thread")
        })
        .expect("answered");
        let events = events(tmp.path());
        assert_eq!(pastes.all(), ["/compact"]);
        assert_eq!(
            kinds(tmp.path()),
            [
                "session-start",
                "send-issued",
                "session-start",
                "send-confirmed"
            ]
        );
        assert_eq!(
            reply,
            json!({"ok": {"submitted_at": events[2]["ts"], "cursor": cursor}})
        );
    }
}
