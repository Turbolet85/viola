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

/// The compiled list read by command name (`local_command_of`): exactly a name, or a name, one
/// space and its arguments; no trim, no case folding, never a leading slash.
pub(super) fn classify(text: &str, cli_verified: bool) -> Confirms {
    match local_command_of(text) {
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
            append_hook_event(&slot, &feed(), tmp.path(), line, None).expect("hook");
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
        append_hook_event(&slot, &feed(), tmp.path(), boot, None).expect("hook");
        let cursor = std::fs::metadata(tmp.path().join("events.ndjson"))
            .expect("events")
            .len();
        let params = json!({"v": 1, "text": "/clear"});
        let other = json!({"cause": cause, "agent_session_id": id});
        let reply = std::thread::scope(|s| {
            let sending = s.spawn(|| send(&slot, &name(), tmp.path(), &call(&params, 1)));
            pasted.recv().expect("pasted");
            let line = hook_line(EventKind::SessionStart, other.clone());
            append_hook_event(&slot, &feed(), tmp.path(), line, None).expect("hook");
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
            append_hook_event(&slot, &feed(), tmp.path(), line, None).expect("hook");
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
        append_hook_event(
            slot,
            &feed(),
            dir,
            hook_line(EventKind::SessionStart, boot),
            None,
        )
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
            append_hook_event(&slot, &feed(), tmp.path(), line, None).expect("hook");
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
            append_hook_event(&slot, &feed(), tmp.path(), line, None).expect("hook");
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
            echo: None,
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
            append_hook_event(&slot, &feed(), tmp.path(), line, None).expect("hook");
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

    use viola_agent_claude::ledger::{ECHO_TAG, PROBE_CONTEXT_COMMAND, PROBE_ECHO_COMMAND};

    /// One transcript line holding the echo tag.
    fn tag_line() -> String {
        format!("{{\"content\":\"{ECHO_TAG}ok</local-command-stdout>\"}}\n")
    }

    fn append(path: &Path, text: &str) {
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)
            .expect("transcript");
        std::io::Write::write_all(&mut file, text.as_bytes()).expect("appended");
    }

    /// What one echo send saw: its reply, the log's kinds, the pastes and the `send-confirmed` or
    /// `send-refused` data.
    struct Echoed {
        reply: Value,
        kinds: Vec<String>,
        pastes: Vec<String>,
        last: Value,
        cursor: u64,
    }

    /// A send of `text` to a slot whose tap took a `session-start` carrying a transcript (when
    /// `remembered`) holding `before` already; once the text is typed, `after` is appended to it and
    /// the clock starts jumping, so a send nothing confirmed expires and never parks the case.
    fn echoed(text: &str, verified: bool, remembered: bool, before: &str, after: &str) -> Echoed {
        echoed_in(text, verified, remembered, Some(before), after)
    }

    /// As [`echoed`], the transcript declared but created only by the `after` append when
    /// `before` is `None`: a send made before the CLI wrote its transcript.
    fn echoed_in(
        text: &str,
        verified: bool,
        remembered: bool,
        before: Option<&str>,
        after: &str,
    ) -> Echoed {
        let tmp = tempfile::tempdir().expect("tempdir");
        let transcript = tmp.path().join("transcript.jsonl");
        if let Some(before) = before {
            std::fs::write(&transcript, before).expect("transcript");
        }
        let base = Instant::now();
        let (clock, jumping) = SwitchClock::at(base);
        let slot = SendSlot::new(clock, verified, Arc::default());
        let (pastes, pasted) = Pastes::new();
        slot.attach(pastes.paste_fn(), quiet_gate(base));
        let boot = json!({"cause": "startup", "agent_session_id": "session-one"});
        let line = hook_line(EventKind::SessionStart, boot);
        let remembered = remembered.then(|| transcript.clone());
        append_hook_event(&slot, &feed(), tmp.path(), line, remembered).expect("hook");
        let cursor = std::fs::metadata(tmp.path().join("events.ndjson"))
            .expect("events")
            .len();
        let params = json!({"v": 1, "text": text});
        let reply = std::thread::scope(|s| {
            let sending = s.spawn(|| send(&slot, &name(), tmp.path(), &call(&params, 1)));
            let typed = pasted.recv_timeout(Duration::from_secs(5));
            if typed.is_ok() {
                append(&transcript, after);
            }
            jumping.store(true, std::sync::atomic::Ordering::SeqCst);
            typed.expect("pasted within the bound");
            sending.join().expect("send thread")
        })
        .expect("answered");
        assert!(slot.flight().send.is_none(), "the slot is free again");
        let events = events(tmp.path());
        Echoed {
            reply,
            kinds: kinds(tmp.path()),
            pastes: pastes.all(),
            last: events.last().expect("a record")["data"].clone(),
            cursor,
        }
    }

    /// A listed echo command on a verified CLI is confirmed by one more stdout tag in the remembered
    /// transcript past its end before the paste: `send-confirmed` with the cursor, no prompt.
    #[test]
    fn send_local_command_echo_is_confirmed_by_a_new_stdout_tag() {
        let got = echoed("/mcp", true, true, "{\"earlier\":1}\n", &tag_line());
        assert_eq!(got.pastes, ["/mcp"]);
        assert_eq!(
            got.kinds,
            ["session-start", "send-issued", "send-confirmed"]
        );
        assert_eq!(got.last, json!({"cursor": got.cursor}));
        assert_eq!(got.reply["ok"]["cursor"], got.cursor);
        let at = got.reply["ok"]["submitted_at"].as_str().expect("a time");
        assert!(at.ends_with('Z') && at.len() == 24, "{at}");
        assert_eq!(
            got.reply["ok"].as_object().map(serde_json::Map::len),
            Some(2)
        );
    }

    /// The command is read by its name, its arguments typed with it.
    #[rstest]
    #[case::disable("/mcp disable viola-probe-absent")]
    #[case::enable("/mcp enable a-server")]
    fn send_local_command_echo_with_arguments_is_classified_by_its_name(#[case] text: &str) {
        let got = echoed(text, true, true, "", &tag_line());
        assert_eq!(got.pastes, [text]);
        assert_eq!(
            got.kinds,
            ["session-start", "send-issued", "send-confirmed"]
        );
        assert_eq!(got.last, json!({"cursor": got.cursor}));
    }

    /// A tag already in the transcript before the send is not its echo: nothing new confirms it,
    /// and the window expires.
    #[test]
    fn send_local_command_echo_tag_before_the_send_is_not_its_echo() {
        let got = echoed(
            PROBE_ECHO_COMMAND,
            true,
            true,
            &tag_line(),
            "{\"later\":1}\n",
        );
        assert_eq!(
            got.reply,
            json!({"refusal": "not-delivered", "detail": "no-prompt-submitted"})
        );
        assert_eq!(got.kinds, ["session-start", "send-issued", "send-refused"]);
    }

    #[test]
    fn send_local_command_echo_without_a_tag_is_no_prompt_submitted_at_the_window() {
        let got = echoed(PROBE_ECHO_COMMAND, true, true, "", "{\"other\":1}\n");
        assert_eq!(
            got.reply,
            json!({"refusal": "not-delivered", "detail": "no-prompt-submitted"})
        );
        assert_eq!(
            got.last,
            json!({"refusal": "not-delivered", "detail": "no-prompt-submitted", "cursor": got.cursor})
        );
        assert_eq!(got.pastes, [PROBE_ECHO_COMMAND]);
    }

    /// With no transcript remembered there is nothing to read, so nothing is presumed: typed and
    /// answered `unconfirmable` at once, whatever the file later holds.
    #[test]
    fn send_local_command_echo_without_a_transcript_is_unconfirmable() {
        let got = echoed(PROBE_ECHO_COMMAND, true, false, "", &tag_line());
        assert_eq!(
            got.reply,
            json!({"ok": {"confirmed": false, "detail": "unconfirmable", "cursor": got.cursor}})
        );
        assert_eq!(
            got.kinds,
            ["session-start", "send-issued", "send-confirmed"]
        );
        assert_eq!(got.last, json!({"cursor": got.cursor, "confirmed": false}));
    }

    #[test]
    fn send_local_command_echo_on_an_unverified_cli_is_unconfirmable() {
        let got = echoed(PROBE_ECHO_COMMAND, false, true, "", &tag_line());
        assert_eq!(
            got.reply,
            json!({"ok": {"confirmed": false, "detail": "unconfirmable", "cursor": got.cursor}})
        );
        assert_eq!(got.pastes, [PROBE_ECHO_COMMAND]);
    }

    /// `/context` is a listed local command confirmed by one more stdout tag on a verified CLI.
    #[test]
    fn send_local_command_context_is_confirmed_by_a_new_stdout_tag() {
        let got = echoed(PROBE_CONTEXT_COMMAND, true, true, "", &tag_line());
        assert_eq!(got.pastes, [PROBE_CONTEXT_COMMAND]);
        assert_eq!(
            got.kinds,
            ["session-start", "send-issued", "send-confirmed"]
        );
        assert_eq!(got.last, json!({"cursor": got.cursor}));
        assert!(got.reply["ok"]["submitted_at"].is_string());
    }

    #[test]
    fn send_local_command_context_on_an_unverified_cli_is_unconfirmable() {
        let got = echoed(PROBE_CONTEXT_COMMAND, false, true, "", &tag_line());
        assert_eq!(
            got.reply,
            json!({"ok": {"confirmed": false, "detail": "unconfirmable", "cursor": got.cursor}})
        );
        assert_eq!(got.pastes, [PROBE_CONTEXT_COMMAND]);
    }

    /// A `/context` sent before its declared transcript stands on disk has no pre-paste end to
    /// count from, so it is `unconfirmable` at once, even when the CLI then writes the file with
    /// its tag.
    #[test]
    fn send_local_command_context_before_its_transcript_exists_is_unconfirmable() {
        let got = echoed_in(PROBE_CONTEXT_COMMAND, true, true, None, &tag_line());
        assert_eq!(
            got.reply,
            json!({"ok": {"confirmed": false, "detail": "unconfirmable", "cursor": got.cursor}})
        );
        assert_eq!(
            got.kinds,
            ["session-start", "send-issued", "send-confirmed"]
        );
        assert_eq!(got.last, json!({"cursor": got.cursor, "confirmed": false}));
        assert_eq!(got.pastes, [PROBE_CONTEXT_COMMAND]);
    }

    /// A later `session-start` with no transcript clears the remembered one.
    #[test]
    fn send_local_command_echo_after_a_start_without_a_transcript_is_unconfirmable() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let transcript = tmp.path().join("transcript.jsonl");
        std::fs::write(&transcript, "").expect("transcript");
        let base = Instant::now();
        let slot = SendSlot::new(FixedClock(base), true, Arc::default());
        let (pastes, _pasted) = Pastes::new();
        slot.attach(pastes.paste_fn(), quiet_gate(base));
        for kept in [Some(transcript), None] {
            let boot = json!({"cause": "clear", "agent_session_id": "session-one"});
            let line = hook_line(EventKind::SessionStart, boot);
            append_hook_event(&slot, &feed(), tmp.path(), line, kept).expect("hook");
        }
        let params = json!({"v": 1, "text": PROBE_ECHO_COMMAND});
        let reply = send(&slot, &name(), tmp.path(), &call(&params, 1)).expect("answered");
        assert_eq!(reply["ok"]["detail"], "unconfirmable");
    }
}
