//! The one send in flight, the hook tap that claims its line, and its window.

use super::*;

/// How often the waiting handler re-reads the clock against the window.
const STEP: Duration = Duration::from_millis(20);

pub(crate) type PasteFn = Box<dyn Fn(&str) -> Result<(), PtyError> + Send + Sync>;

pub(super) struct InFlight {
    pub(super) text: String,
    pub(super) confirms: Confirms,
    pub(super) state: Match,
}

/// What the sends and the hook tap share under one lock.
#[derive(Default)]
pub(super) struct Flight {
    pub(super) send: Option<InFlight>,
    /// The `agent_session_id` of the last `session-start` the tap took: `None` before the first,
    /// and when that line carried none.
    session: Option<String>,
}

/// The child's input and gate, set once the pump starts, the budget gate and the link set, set by
/// the start, the one send in flight, the wheel it reads before typing, and the version gate's
/// reading.
pub(crate) struct SendSlot {
    pub(super) clock: Box<dyn Clock>,
    pub(super) cli_verified: bool,
    pub(super) wheel: Arc<WheelSlot>,
    pub(super) io: OnceLock<(PasteFn, Gate)>,
    pub(super) budget: OnceLock<Arc<BudgetGate>>,
    pub(super) links: OnceLock<Arc<LinkSet>>,
    pub(super) snapshots: OnceLock<Arc<Snapshots>>,
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
            budget: OnceLock::new(),
            links: OnceLock::new(),
            snapshots: OnceLock::new(),
            flight: Mutex::default(),
            settled: Condvar::new(),
        }
    }

    /// The first call wins; the pump calls it once, before it starts.
    pub(crate) fn attach(&self, paste: PasteFn, gate: Gate) {
        let _ = self.io.set((paste, gate));
    }

    /// The first call wins; the start calls it once, before the endpoint serves.
    pub(crate) fn budget_with(&self, gate: Arc<BudgetGate>) {
        let _ = self.budget.set(gate);
    }

    /// The first call wins; the start calls it once, before the endpoint serves.
    pub(crate) fn links_with(&self, links: Arc<LinkSet>) {
        let _ = self.links.set(links);
    }

    /// The first call wins; the start calls it once, before the endpoint serves.
    pub(crate) fn snapshots_with(&self, snapshots: Arc<Snapshots>) {
        let _ = self.snapshots.set(snapshots);
    }

    pub(super) fn flight(&self) -> MutexGuard<'_, Flight> {
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
    pub(super) fn confirm(&self, opened: Instant) -> Option<String> {
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
/// post-condition is claimed and settled the same way, its line appended unchanged. A
/// `session-start` whose `agent_session_id` is a string has the snapshot record that id before the
/// line can be read, so whoever reads the line finds the snapshot already written.
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
    let session = (line.kind == EventKind::SessionStart)
        .then(|| line.data["agent_session_id"].as_str())
        .flatten();
    if let (Some(id), Some(snapshots)) = (session, slot.snapshots.get()) {
        snapshots.update(|snapshot| snapshot.agent_session_id = Some(id.to_owned()))?;
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
pub(super) struct Reserved<'a>(pub(super) &'a SendSlot);

impl Drop for Reserved<'_> {
    fn drop(&mut self) {
        self.0.flight().send = None;
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_support::*;

    use super::*;
    use std::sync::Arc;

    use rstest::rstest;

    use viola_state::snapshot::{InstanceSnapshot, Wheel, read_snapshot};

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

    /// The window expires with the send unclaimed: the send is closed then, so its text arriving
    /// later is filed as typed, never claimed, and a second read of the window finds nothing.
    #[test]
    fn send_expired_window_closes_the_send_so_a_late_line_is_not_claimed() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let opened = Instant::now();
        let slot = SendSlot::new(
            FixedClock(opened + CONFIRM_WINDOW_FALLBACK),
            false,
            Arc::default(),
        );
        slot.flight().send = Some(InFlight {
            text: "late".to_owned(),
            confirms: Confirms::Prompt,
            state: Match::Waiting,
        });
        assert_eq!(slot.confirm(opened), None);
        append_hook_event(&slot, &feed(), tmp.path(), prompt("late", "human")).expect("hook");
        assert_eq!(events(tmp.path())[0]["data"]["origin"], "human");
        assert_eq!(slot.confirm(opened), None, "nothing was claimed");
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

    fn snapshot_of(dir: &Path) -> InstanceSnapshot {
        InstanceSnapshot {
            endpoint: None,
            pid: 1,
            started_at: "s".to_owned(),
            pinned_bin: "b".to_owned(),
            cli_verified: false,
            cli_version: None,
            wheel: Wheel::Driver,
            budget_paused: false,
            budget_override_until: None,
            links: Vec::new(),
            child_pid: Some(7),
            pending_dialog: None,
            cwd: dir.to_str().map(str::to_owned),
            statusline_command: None,
            agent_session_id: None,
        }
    }

    /// Each `session-start` whose id is a string rewrites the snapshot's id with its line (a
    /// `startup`, then a `clear`); one whose id is `null` or absent writes nothing.
    #[test]
    fn send_session_start_records_its_string_id_in_the_snapshot() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (slot, _pastes) = mute_child();
        let snapshots = Arc::new(Snapshots::new(tmp.path().to_path_buf()));
        snapshots.init(snapshot_of(tmp.path())).expect("init");
        slot.snapshots_with(Arc::clone(&snapshots));
        let start = |data: Value| {
            append_hook_event(
                &slot,
                &feed(),
                tmp.path(),
                hook_line(EventKind::SessionStart, data),
            )
            .expect("hook");
        };
        let id = |dir: &Path| read_snapshot(dir).expect("snapshot").agent_session_id;
        start(json!({"cause": "startup", "agent_session_id": "s-1"}));
        assert_eq!(id(tmp.path()).as_deref(), Some("s-1"));
        start(json!({"cause": "clear", "agent_session_id": "s-2"}));
        assert_eq!(id(tmp.path()).as_deref(), Some("s-2"));
        let written = std::fs::read(tmp.path().join("snapshot.json")).expect("bytes");
        start(json!({"cause": "resume", "agent_session_id": null}));
        start(json!({"cause": "compact"}));
        append_hook_event(&slot, &feed(), tmp.path(), prompt("typed", "human")).expect("hook");
        let after = std::fs::read(tmp.path().join("snapshot.json")).expect("bytes");
        assert_eq!(after, written, "nothing written");
        assert_eq!(kinds(tmp.path()).len(), 5);
    }

    /// Without the snapshot handle the line is still appended.
    #[test]
    fn send_session_start_without_the_snapshot_handle_is_appended() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (slot, _pastes) = mute_child();
        let line = hook_line(
            EventKind::SessionStart,
            json!({"cause": "startup", "agent_session_id": "s-1"}),
        );
        append_hook_event(&slot, &feed(), tmp.path(), line).expect("hook");
        assert_eq!(kinds(tmp.path()), ["session-start"]);
        assert!(read_snapshot(tmp.path()).is_none());
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

    #[test]
    fn send_confirm_window_is_the_fallback() {
        assert_eq!(window_ms(), 10_000);
    }
}
