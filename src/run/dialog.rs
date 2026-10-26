//! The wrapper's dialog slot (architecture §Standard Contracts `hook.dialog` / `answer`; [Message
//! Broker / IPC]): every raised dialog is logged once, with a `dialog_id` that never repeats across
//! restarts, before any reply. At most one is held pending for a driver's answer, until the answer
//! arrives, the hold `config.json` sets (`dialog_hold_s`, read once at start) passes on the
//! injected clock, or the human takes the wheel; any other is left to the human (`null` at once),
//! and so is every dialog raised while the human holds the wheel. A hold that runs out is left to
//! the human too, with one `dialog-expired` record that wakes `wait`. A PermissionRequest that
//! repeats its PreToolUse's `tool_input` is the same dialog: it is answered from that dialog's
//! outcome and logs nothing. viola carries the driver's answer; it never picks one.

use std::fs::File;
use std::io::Read as _;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, OnceLock, PoisonError};
use std::time::{Duration, Instant};

use chrono::Utc;
use serde_json::{Value, json};
use tracing::instrument;
use viola_agent_claude::dialog::{DialogKind, DialogTool, Response, Verdict};
use viola_agent_claude::hook::HookEvent;
use viola_channel::{Call, ProtocolError};
use viola_core::obs::ObsEvent;
use viola_core::{
    Clock, DIALOG_HOLD_DEFAULT, DialogHold, EventKind, HumanTyping, MAX_FRAME, NotDelivered,
    RefusalReason, ViolaName, obs_event,
};
use viola_state::events::{EventLine, LoggedLine, Source, append_event, append_event_at};
use viola_state::snapshot::{PendingDialog, Wheel};

use crate::obs::ConfigRejection;
use crate::run::links::LinkSet;
use crate::run::send::parse_from;
use crate::run::snapshot::Snapshots;
use crate::run::wait::WaitFeed;
use crate::run::wheel::WheelSlot;

/// How often a held dialog re-reads the clock against its deadline.
const STEP: Duration = Duration::from_millis(20);

/// The dialog held for a driver's answer.
struct Pending {
    dialog_id: u64,
    kind: DialogKind,
    raised: Instant,
    answer: Option<Response>,
    /// The human took the wheel while it was held: it ends unanswered at once.
    handed_back: bool,
}

/// What became of a dialog a PreToolUse raised, kept for the PermissionRequest that repeats it.
enum Outcome {
    Open,
    Answered(Response),
    Unanswered,
}

struct Arm {
    tool: DialogTool,
    dialog_id: u64,
    input: Value,
    outcome: Outcome,
}

#[derive(Default)]
struct State {
    next_id: u64,
    pending: Option<Pending>,
    arms: Vec<Arm>,
}

/// One instance's dialogs.
pub(crate) struct DialogSlot {
    clock: Box<dyn Clock>,
    hold: Duration,
    cli_verified: bool,
    name: ViolaName,
    instance_dir: PathBuf,
    feed: Arc<WaitFeed>,
    wheel: Arc<WheelSlot>,
    snapshots: Arc<Snapshots>,
    links: OnceLock<Arc<LinkSet>>,
    state: Mutex<State>,
    answered: Condvar,
}

/// A `hook.dialog`'s params, re-validated: the hook's checks are advisory.
struct Raise {
    kind: DialogKind,
    data: serde_json::Map<String, Value>,
    hook: HookEvent,
    tool: Option<DialogTool>,
    input: Value,
    continuation: bool,
}

impl Raise {
    /// `{kind, data, hook_event, tool?, input?, continuation?}`; anything else is `-32602`. A
    /// continuation names its tool, and a tool's kind is its own.
    fn parse(params: &Value) -> Result<Self, ProtocolError> {
        let kind = params["kind"]
            .as_str()
            .and_then(DialogKind::from_name)
            .ok_or(ProtocolError::InvalidParams)?;
        let data = params["data"]
            .as_object()
            .cloned()
            .ok_or(ProtocolError::InvalidParams)?;
        let hook = params["hook_event"]
            .as_str()
            .and_then(HookEvent::from_arg)
            .filter(|h| h.is_dialog())
            .ok_or(ProtocolError::InvalidParams)?;
        let tool = match params.get("tool") {
            None | Some(Value::Null) => None,
            Some(name) => Some(
                name.as_str()
                    .and_then(DialogTool::from_name)
                    .ok_or(ProtocolError::InvalidParams)?,
            ),
        };
        let continuation = match params.get("continuation") {
            None => false,
            Some(flag) => flag.as_bool().ok_or(ProtocolError::InvalidParams)?,
        };
        let consistent = tool.is_none_or(|t| t.kind() == kind) && (!continuation || tool.is_some());
        if !consistent {
            return Err(ProtocolError::InvalidParams);
        }
        Ok(Self {
            kind,
            data,
            hook,
            tool,
            input: params.get("input").cloned().unwrap_or(Value::Null),
            continuation,
        })
    }
}

fn reply(dialog_id: u64, response: Option<&Response>) -> Value {
    json!({"ok": {"dialog_id": dialog_id, "response": response.map(Response::to_value)}})
}

fn refusal(reason: RefusalReason, detail: Option<&'static str>) -> Value {
    json!({"refusal": reason.as_str(), "detail": detail})
}

fn millis(d: Duration) -> u64 {
    u64::try_from(d.as_millis()).unwrap_or(u64::MAX)
}

const HOLD_KEY: &str = "dialog_hold_s";

/// The dialog hold from `<home>/config.json`'s `dialog_hold_s`, and one rejection when its value is
/// not a whole number of seconds from 60 to 3600: the hold is then the default, so no value makes
/// it zero or unbounded.
pub(crate) fn read_hold(home: &Path) -> (DialogHold, Vec<ConfigRejection>) {
    let mut bytes = Vec::new();
    // An absent, unreadable or malformed document is `read_diagnostics_level`'s to report, once.
    match File::open(home.join("config.json")) {
        Ok(file) => match file.take(MAX_FRAME).read_to_end(&mut bytes) {
            Ok(_) => parse_hold(&bytes),
            Err(_) => (DialogHold::default(), Vec::new()),
        },
        Err(_) => (DialogHold::default(), Vec::new()),
    }
}

fn parse_hold(bytes: &[u8]) -> (DialogHold, Vec<ConfigRejection>) {
    let Ok(Value::Object(map)) = serde_json::from_slice::<Value>(bytes) else {
        return (DialogHold::default(), Vec::new());
    };
    if map.get("v").and_then(Value::as_u64) != Some(1) {
        return (DialogHold::default(), Vec::new());
    }
    match map.get(HOLD_KEY).map(whole_hold) {
        None => (DialogHold::default(), Vec::new()),
        Some(Some(hold)) => (hold, Vec::new()),
        Some(None) => (DialogHold::default(), vec![ConfigRejection::Malformed]),
    }
}

fn whole_hold(value: &Value) -> Option<DialogHold> {
    let whole = u16::try_from(value.as_u64()?).ok()?;
    DialogHold::try_new(whole).ok()
}

impl DialogSlot {
    /// A slot holding each dialog for `DIALOG_HOLD_DEFAULT`; the start sets the configured hold
    /// with [`DialogSlot::hold_for`].
    pub(crate) fn new(
        clock: impl Clock + 'static,
        cli_verified: bool,
        name: ViolaName,
        instance_dir: PathBuf,
        feed: Arc<WaitFeed>,
        wheel: Arc<WheelSlot>,
        snapshots: Arc<Snapshots>,
    ) -> Self {
        Self {
            clock: Box::new(clock),
            hold: DIALOG_HOLD_DEFAULT,
            cli_verified,
            name,
            instance_dir,
            feed,
            wheel,
            snapshots,
            links: OnceLock::new(),
            state: Mutex::new(State {
                next_id: 1,
                ..State::default()
            }),
            answered: Condvar::new(),
        }
    }

    /// The hold each dialog is held for, read once at the start.
    pub(crate) fn hold_for(mut self, hold: Duration) -> Self {
        self.hold = hold;
        self
    }

    /// The first call wins; the start calls it once, before the endpoint serves.
    pub(crate) fn links_with(&self, links: Arc<LinkSet>) {
        let _ = self.links.set(links);
    }

    fn state(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Ids continue past the highest `dialog_id` the log already holds.
    pub(crate) fn restore(&self, highest: Option<u64>) {
        self.state().next_id = highest.map_or(1, |h| h.saturating_add(1));
    }

    /// `hook.dialog`.
    pub(crate) fn hook_dialog(&self, params: &Value) -> Result<Value, ProtocolError> {
        let raise = Raise::parse(params)?;
        let mut state = self.state();
        if raise.continuation
            && let Some(answer) = continued(&mut state, &raise)
        {
            return Ok(answer);
        }
        // A question first raised by PermissionRequest: its `allow` + `updatedInput` body is read only
        // statically (2.1.288, 2.1.287) and has no ledger row, so the human answers it.
        let unanswerable = raise.continuation && raise.kind == DialogKind::Question;
        let hold = self.cli_verified
            && state.pending.is_none()
            && !unanswerable
            && self.wheel.holder() == Wheel::Driver;
        let dialog_id = self.register(&mut state, &raise, hold)?;
        if raise.hook == HookEvent::PreToolUse
            && let Some(tool) = raise.tool
        {
            state.arms.retain(|a| a.tool != tool);
            state.arms.push(Arm {
                tool,
                dialog_id,
                input: raise.input.clone(),
                outcome: if hold {
                    Outcome::Open
                } else {
                    Outcome::Unanswered
                },
            });
        }
        if !hold {
            return Ok(reply(dialog_id, None));
        }
        Ok(self.await_answer(state, dialog_id))
    }

    /// The dialog event, appended through the wait feed before any reply, and the `pending_dialog`
    /// snapshot field when it is held, written before the event so whoever sees the line on disk
    /// already sees the held dialog; cleared again when the append fails.
    #[instrument(
        skip_all,
        name = "run.dialog_register",
        fields(dialog_kind = raise.kind.as_str(), pending_conflict = state.pending.is_some())
    )]
    fn register(&self, state: &mut State, raise: &Raise, hold: bool) -> Result<u64, ProtocolError> {
        let dialog_id = state.next_id;
        let mut data = raise.data.clone();
        data.insert("dialog_id".to_owned(), json!(dialog_id));
        let line = EventLine::new(
            &self.name,
            raise.kind.event_kind(),
            Source::Hook,
            Value::Object(data),
            Utc::now(),
        );
        if hold {
            self.write_pending(Some(PendingDialog {
                dialog_id,
                kind: raise.kind.as_str().to_owned(),
            }));
        }
        let appended = self.feed.appending_dialog(hold, || {
            append_event_at(&self.instance_dir, |_| line.clone())
        });
        if appended.is_err() {
            if hold {
                self.write_pending(None);
            }
            return Err(ProtocolError::Internal);
        }
        state.next_id = dialog_id.saturating_add(1);
        obs_event!(
            INFO,
            ObsEvent::DialogRaised,
            corr = dialog_id,
            dialog_kind = raise.kind.as_str(),
            hook_event = raise.hook.as_str(),
        );
        if hold {
            state.pending = Some(Pending {
                dialog_id,
                kind: raise.kind,
                raised: self.clock.now(),
                answer: None,
                handed_back: false,
            });
        }
        Ok(dialog_id)
    }

    /// Held until an answer arrives, the hold passes or the dialog is handed back; each way the
    /// pending dialog is cleared, and the PreToolUse arm learns the outcome. A hold that ran out is
    /// recorded once, and the reply stays `null`: the dialog is left to the human.
    #[instrument(
        skip_all,
        name = "run.dialog_await",
        fields(deadline_ms = millis(self.hold), deadline_hit = tracing::field::Empty)
    )]
    fn await_answer(&self, mut state: MutexGuard<'_, State>, dialog_id: u64) -> Value {
        let deadline = self.clock.now() + self.hold;
        let (answer, deadline_hit) = loop {
            if let Some(answer) = state.pending.as_mut().and_then(|p| p.answer.take()) {
                break (Some(answer), false);
            }
            if state.pending.as_ref().is_some_and(|p| p.handed_back) {
                break (None, false);
            }
            if self.clock.now() >= deadline {
                break (None, true);
            }
            state = self
                .answered
                .wait_timeout(state, STEP)
                .unwrap_or_else(PoisonError::into_inner)
                .0;
        };
        tracing::Span::current().record("deadline_hit", deadline_hit);
        let expired = state.pending.take().filter(|_| deadline_hit);
        self.feed.dialog_settled();
        self.write_pending(None);
        if let Some(pending) = expired {
            self.record_expiry(&pending);
        }
        if let Some(arm) = state.arms.iter_mut().find(|a| a.dialog_id == dialog_id) {
            arm.outcome = answer
                .clone()
                .map_or(Outcome::Unanswered, Outcome::Answered);
        }
        reply(dialog_id, answer.as_ref())
    }

    /// One `dialog-expired` line through the wait feed, so a parked `wait` wakes on it, and the
    /// wrapper's `dialog-answered` with `deadline_hit` (obs-plan §4). A failed append changes
    /// nothing else: the reply is `null` either way.
    fn record_expiry(&self, pending: &Pending) {
        let line = EventLine::new(
            &self.name,
            EventKind::DialogExpired,
            Source::Wrapper,
            json!({"dialog_id": pending.dialog_id, "dialog_kind": pending.kind.as_str()}),
            Utc::now(),
        );
        let _ = self
            .feed
            .appending(&line, || append_event(&self.instance_dir, &line));
        obs_event!(
            INFO,
            ObsEvent::DialogAnswered,
            corr = pending.dialog_id,
            dialog_kind = pending.kind.as_str(),
            deadline_hit = true,
            duration_ms = millis(self.clock.now().saturating_duration_since(pending.raised)),
        );
    }

    /// `snapshot.json` with `pending_dialog` set or cleared, through the wrapper's one snapshot
    /// holder.
    fn write_pending(&self, pending: Option<PendingDialog>) {
        let _ = self.snapshots.update(|s| s.pending_dialog = pending);
    }

    /// The human took the wheel: a held dialog ends unanswered at once, left to the human.
    pub(crate) fn hand_back(&self) {
        if let Some(pending) = self.state().pending.as_mut() {
            pending.handed_back = true;
        }
        self.answered.notify_all();
    }

    /// `answer` `{dialog_id, from?, response}` in the documented order: the params, the free text,
    /// the wheel, the stamp, the pending id, then the response's kind. An answer nothing refused,
    /// from another instance, links that instance to this one before it is taken (architecture
    /// [Links]); a refused answer links nothing.
    pub(crate) fn answer(&self, call: &Call<'_>) -> Result<Value, ProtocolError> {
        let params = call.params;
        let dialog_id = params["dialog_id"]
            .as_u64()
            .ok_or(ProtocolError::InvalidParams)?;
        let from = parse_from(params)?;
        let response =
            Response::parse(&params["response"]).map_err(|_| ProtocolError::InvalidParams)?;
        if let Some(detail) = response
            .free_text()
            .into_iter()
            .find_map(|t| viola_core::validate_paste_text(t).err())
        {
            return Ok(refusal(RefusalReason::NotDelivered, Some(detail.as_str())));
        }
        if let Some(detail) = self.wheel.human_typing() {
            return Ok(refusal(
                RefusalReason::HumanTyping,
                detail.map(HumanTyping::as_str),
            ));
        }
        if !self.cli_verified {
            return Ok(refusal(RefusalReason::UnverifiedCli, None));
        }
        let mut state = self.state();
        let Some(pending) = state
            .pending
            .as_mut()
            .filter(|p| p.dialog_id == dialog_id && p.answer.is_none())
        else {
            return Ok(refusal(
                RefusalReason::NotDelivered,
                Some(NotDelivered::UnknownDialog.as_str()),
            ));
        };
        if response.kind() != pending.kind {
            return Err(ProtocolError::InvalidParams);
        }
        if let (Some(links), Some(from)) = (self.links.get(), from.as_ref())
            && *from != self.name
        {
            links.record(from).map_err(|_| ProtocolError::Internal)?;
        }
        let from: Option<&str> = from.as_ref().map(AsRef::as_ref);
        obs_event!(
            INFO,
            ObsEvent::DialogAnswered,
            corr = dialog_id,
            dialog_kind = pending.kind.as_str(),
            from = from,
            from_trust = from.map(|_| "self-reported"),
            deadline_hit = false,
            duration_ms = millis(self.clock.now().saturating_duration_since(pending.raised)),
        );
        pending.answer = Some(response);
        self.answered.notify_all();
        Ok(json!({"ok": {}}))
    }
}

/// The armed dialog a continuation repeats (same tool, equal `tool_input`), consumed: a `plan`
/// answered `revise` is answered from that answer, every other outcome with `null`. `None` when no
/// arm matches: the continuation is then raised as its own dialog.
fn continued(state: &mut State, raise: &Raise) -> Option<Value> {
    let at = state
        .arms
        .iter()
        .position(|a| Some(a.tool) == raise.tool && a.input == raise.input)?;
    let arm = state.arms.remove(at);
    let revise = match &arm.outcome {
        Outcome::Answered(
            response @ Response::Plan {
                behavior: Verdict::Revise,
                ..
            },
        ) => Some(response),
        Outcome::Open | Outcome::Answered(_) | Outcome::Unanswered => None,
    };
    Some(reply(arm.dialog_id, revise))
}

/// The highest `dialog_id` any dialog event in the log holds.
pub(crate) fn highest_dialog_id(line: &LoggedLine, highest: Option<u64>) -> Option<u64> {
    let dialog = DialogKind::ALL
        .iter()
        .any(|k| line.value["kind"] == k.as_str());
    let id = line.value["data"]["dialog_id"].as_u64().filter(|_| dialog);
    highest.max(id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    use std::path::Path;

    use rstest::rstest;
    use viola_state::snapshot::{InstanceSnapshot, read_snapshot};

    use crate::run::wheel::Recorder;

    /// Every reading a second past the previous one: a held dialog expires within a minute of
    /// readings.
    struct JumpClock(Mutex<Instant>);

    impl Clock for JumpClock {
        fn now(&self) -> Instant {
            let mut now = self.0.lock().expect("clock");
            *now += Duration::from_secs(1);
            *now
        }
    }

    /// Never moves: a held dialog waits for its answer.
    #[derive(Clone, Copy)]
    struct FixedClock(Instant);

    impl Clock for FixedClock {
        fn now(&self) -> Instant {
            self.0
        }
    }

    fn name() -> ViolaName {
        ViolaName::try_new("builder".to_owned()).expect("valid")
    }

    fn seeded_snapshots(dir: &Path) -> Arc<Snapshots> {
        let snapshots = Arc::new(Snapshots::new(dir.to_path_buf()));
        snapshots
            .init(InstanceSnapshot {
                endpoint: None,
                pid: 1,
                started_at: "s".to_owned(),
                pinned_bin: "b".to_owned(),
                cli_verified: true,
                cli_version: None,
                wheel: Wheel::Driver,
                budget_paused: false,
                budget_override_until: None,
                links: Vec::new(),
                child_pid: Some(2),
                pending_dialog: None,
                cwd: None,
                statusline_command: None,
                agent_session_id: None,
            })
            .expect("snapshot");
        snapshots
    }

    fn slot_on(
        dir: &Path,
        clock: impl Clock + 'static,
        verified: bool,
        wheel: Arc<WheelSlot>,
        snapshots: Arc<Snapshots>,
    ) -> DialogSlot {
        held_slot_on(
            dir,
            clock,
            viola_core::DIALOG_HOLD_DEFAULT,
            verified,
            wheel,
            snapshots,
        )
    }

    fn held_slot_on(
        dir: &Path,
        clock: impl Clock + 'static,
        hold: Duration,
        verified: bool,
        wheel: Arc<WheelSlot>,
        snapshots: Arc<Snapshots>,
    ) -> DialogSlot {
        let feed = Arc::new(WaitFeed::new(JumpClock(Mutex::new(Instant::now()))));
        DialogSlot::new(
            clock,
            verified,
            name(),
            dir.to_path_buf(),
            feed,
            wheel,
            snapshots,
        )
        .hold_for(hold)
    }

    fn slot(dir: &Path, clock: impl Clock + 'static, verified: bool) -> DialogSlot {
        slot_on(dir, clock, verified, Arc::default(), seeded_snapshots(dir))
    }

    fn events(dir: &Path) -> Vec<Value> {
        std::fs::read_to_string(dir.join("events.ndjson"))
            .unwrap_or_default()
            .lines()
            .map(|l| serde_json::from_str(l).expect("one JSON object per line"))
            .collect()
    }

    const PLAN_INPUT: &str = "the plan text";

    fn plan(hook: &str, continuation: bool) -> Value {
        json!({"v": 1, "kind": "plan", "data": {"plan": PLAN_INPUT}, "hook_event": hook,
            "tool": "ExitPlanMode", "input": {"plan": PLAN_INPUT}, "continuation": continuation})
    }

    fn question(hook: &str, continuation: bool) -> Value {
        json!({"v": 1, "kind": "question", "hook_event": hook, "tool": "AskUserQuestion",
            "data": {"questions": [{"question": "Which color?", "options": ["red"],
                "option_details": [{"description": null, "preview": null}], "multi_select": false}]},
            "input": {"questions": [{"question": "Which color?"}]}, "continuation": continuation})
    }

    fn permission() -> Value {
        json!({"v": 1, "kind": "permission", "hook_event": "permission-request",
            "data": {"tool": "Bash", "input": {"command": "ls"}}, "input": {"command": "ls"}})
    }

    fn call(params: &Value) -> Call<'_> {
        Call {
            method: "answer",
            params,
            id: Some(1),
            conn: None,
            srv_conn: None,
        }
    }

    fn pending_on_disk(dir: &Path) -> Option<PendingDialog> {
        read_snapshot(dir).expect("snapshot").pending_dialog
    }

    /// Raises `params` on a thread, answers the held dialog with `response`, and returns the
    /// `hook.dialog` reply, the `answer` reply and what the snapshot held in between.
    fn raise_and_answer(
        slot: &DialogSlot,
        dir: &Path,
        params: &Value,
        response: Value,
    ) -> (Value, Value, Option<PendingDialog>) {
        std::thread::scope(|s| {
            let raising = s.spawn(|| slot.hook_dialog(params));
            let dialog_id = loop {
                if let Some(p) = slot.state().pending.as_ref() {
                    break p.dialog_id;
                }
                std::thread::yield_now();
            };
            let held = pending_on_disk(dir);
            let params = json!({"v": 1, "dialog_id": dialog_id, "response": response});
            let answered = slot.answer(&call(&params)).expect("answered");
            let raised = raising.join().expect("thread").expect("replied");
            (raised, answered, held)
        })
    }

    #[test]
    fn hook_dialog_on_an_unverified_cli_logs_it_and_replies_null_at_once() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = slot(tmp.path(), FixedClock(Instant::now()), false);
        let got = slot.hook_dialog(&plan("pre-tool-use", false));
        assert_eq!(got, Ok(json!({"ok": {"dialog_id": 1, "response": null}})));
        let lines = events(tmp.path());
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0]["kind"], "plan");
        assert_eq!(lines[0]["source"], "hook");
        assert_eq!(
            lines[0]["data"],
            json!({"plan": PLAN_INPUT, "dialog_id": 1})
        );
        assert_eq!(pending_on_disk(tmp.path()), None);
    }

    #[test]
    fn hook_dialog_held_until_answered_replies_the_answer_and_clears_the_pending_dialog() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = slot(tmp.path(), FixedClock(Instant::now()), true);
        let response = json!({"answers": {"Which color?": "red"}});
        let (raised, answered, held) = raise_and_answer(
            &slot,
            tmp.path(),
            &question("pre-tool-use", false),
            response.clone(),
        );
        assert_eq!(answered, json!({"ok": {}}));
        assert_eq!(
            raised,
            json!({"ok": {"dialog_id": 1, "response": response}})
        );
        assert_eq!(
            held,
            Some(PendingDialog {
                dialog_id: 1,
                kind: "question".to_owned()
            })
        );
        assert_eq!(pending_on_disk(tmp.path()), None);
        assert!(slot.state().pending.is_none());
        assert_eq!(events(tmp.path()).len(), 1);
    }

    /// A second dialog while one is held is logged with its own id and left to the human.
    #[test]
    fn hook_dialog_while_another_is_pending_is_logged_and_null() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = slot(tmp.path(), FixedClock(Instant::now()), true);
        std::thread::scope(|s| {
            let first = s.spawn(|| slot.hook_dialog(&plan("pre-tool-use", false)));
            while slot.state().pending.is_none() {
                std::thread::yield_now();
            }
            let second = slot.hook_dialog(&permission());
            assert_eq!(
                second,
                Ok(json!({"ok": {"dialog_id": 2, "response": null}}))
            );
            let late = json!({"v": 1, "dialog_id": 2, "response": {"behavior": "allow"}});
            assert_eq!(
                slot.answer(&call(&late)),
                Ok(json!({"refusal": "not-delivered", "detail": "unknown-dialog"}))
            );
            let answer = json!({"v": 1, "dialog_id": 1, "response": {"behavior": "approve"}});
            slot.answer(&call(&answer)).expect("answered");
            let first = first.join().expect("thread").expect("replied");
            assert_eq!(first["ok"]["response"], json!({"behavior": "approve"}));
        });
        let kinds: Vec<Value> = events(tmp.path())
            .iter()
            .map(|e| e["kind"].clone())
            .collect();
        assert_eq!(kinds, ["plan", "permission"]);
        assert_eq!(events(tmp.path())[1]["data"]["dialog_id"], 2);
    }

    #[test]
    fn hook_dialog_expires_at_the_deadline_on_the_injected_clock() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = slot(tmp.path(), JumpClock(Mutex::new(Instant::now())), true);
        let got = slot.hook_dialog(&permission());
        assert_eq!(got, Ok(json!({"ok": {"dialog_id": 1, "response": null}})));
        assert!(slot.state().pending.is_none());
        assert_eq!(pending_on_disk(tmp.path()), None);
        let late = json!({"v": 1, "dialog_id": 1, "response": {"behavior": "allow"}});
        assert_eq!(
            slot.answer(&call(&late)),
            Ok(json!({"refusal": "not-delivered", "detail": "unknown-dialog"}))
        );
    }

    fn hold_of(config: Option<&str>) -> (u64, Vec<ConfigRejection>) {
        let tmp = tempfile::tempdir().expect("tempdir");
        if let Some(config) = config {
            std::fs::write(tmp.path().join("config.json"), config).expect("write");
        }
        let (hold, rejections) = read_hold(tmp.path());
        (hold.duration().as_secs(), rejections)
    }

    #[rstest]
    #[case::absent_document(None, 60)]
    #[case::absent_key(Some(r#"{"v":1}"#), 60)]
    #[case::sixty(Some(r#"{"v":1,"dialog_hold_s":60}"#), 60)]
    #[case::six_hundred(Some(r#"{"v":1,"dialog_hold_s":600}"#), 600)]
    #[case::the_ceiling(Some(r#"{"v":1,"dialog_hold_s":3600}"#), 3600)]
    #[case::another_version(Some(r#"{"v":2,"dialog_hold_s":600}"#), 60)]
    fn read_hold_takes_a_whole_number_of_seconds_in_range(
        #[case] config: Option<&str>,
        #[case] secs: u64,
    ) {
        assert_eq!(hold_of(config), (secs, Vec::new()));
    }

    #[rstest]
    #[case::zero("0")]
    #[case::fifty_nine("59")]
    #[case::past_the_ceiling("3601")]
    #[case::negative("-1")]
    #[case::a_fraction("600.5")]
    #[case::a_string("\"600\"")]
    #[case::null("null")]
    #[case::a_bool("true")]
    fn read_hold_falls_back_to_sixty_with_one_rejection(#[case] bad: &str) {
        let config = format!(r#"{{"v":1,"dialog_hold_s":{bad}}}"#);
        assert_eq!(
            hold_of(Some(&config)),
            (60, vec![ConfigRejection::Malformed]),
            "{config}"
        );
    }

    /// Stands where the case puts it, and counts its readings.
    #[derive(Default)]
    struct Stepped {
        at: Mutex<Option<Instant>>,
        reads: std::sync::atomic::AtomicUsize,
    }

    #[derive(Clone)]
    struct StepClock(Arc<Stepped>, Instant);

    impl StepClock {
        fn at(base: Instant) -> Self {
            Self(Arc::default(), base)
        }

        fn set(&self, past_base: Duration) {
            *self.0.at.lock().expect("clock") = Some(self.1 + past_base);
        }

        fn reads(&self) -> usize {
            self.0.reads.load(std::sync::atomic::Ordering::SeqCst)
        }
    }

    impl Clock for StepClock {
        fn now(&self) -> Instant {
            let at = self.0.at.lock().expect("clock").unwrap_or(self.1);
            self.0
                .reads
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            at
        }
    }

    /// Waits until the held dialog has read `clock` twice more: the second reading came after the
    /// case moved the clock.
    fn read_twice_more(clock: &StepClock) {
        let seen = clock.reads();
        while clock.reads() < seen + 2 {
            std::thread::yield_now();
        }
    }

    #[test]
    fn hook_dialog_holds_until_the_configured_hold_on_the_injected_clock() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let clock = StepClock::at(Instant::now());
        let slot = held_slot_on(
            tmp.path(),
            clock.clone(),
            Duration::from_secs(120),
            true,
            Arc::default(),
            seeded_snapshots(tmp.path()),
        );
        std::thread::scope(|s| {
            let raising = s.spawn(|| slot.hook_dialog(&permission()));
            while slot.state().pending.is_none() {
                std::thread::yield_now();
            }
            clock.set(Duration::from_secs(120) - STEP);
            read_twice_more(&clock);
            assert!(slot.state().pending.is_some(), "held one step before");
            assert!(pending_on_disk(tmp.path()).is_some());
            clock.set(Duration::from_secs(120));
            let raised = raising.join().expect("thread");
            assert_eq!(
                raised,
                Ok(json!({"ok": {"dialog_id": 1, "response": null}}))
            );
        });
        assert!(slot.state().pending.is_none());
        assert_eq!(pending_on_disk(tmp.path()), None);
    }

    /// Every line the calling thread emits, its fields as JSON.
    #[derive(Clone, Default)]
    struct Lines(Arc<Mutex<Vec<Value>>>);

    impl<S: tracing::Subscriber> tracing_subscriber::Layer<S> for Lines {
        fn on_event(
            &self,
            event: &tracing::Event<'_>,
            _: tracing_subscriber::layer::Context<'_, S>,
        ) {
            let mut fields = viola_channel::test_support::JsonFields::default();
            event.record(&mut fields);
            self.0.lock().expect("lines").push(Value::Object(fields.0));
        }
    }

    fn capture_lines<R>(f: impl FnOnce() -> R) -> (R, Vec<Value>) {
        use tracing_subscriber::layer::SubscriberExt as _;
        let lines = Lines::default();
        let subscriber = tracing_subscriber::registry().with(lines.clone());
        let out = crate::test_support::with_scoped_default(subscriber, f);
        let got = lines.0.lock().expect("lines").clone();
        (out, got)
    }

    #[test]
    fn hook_dialog_expiry_writes_one_dialog_expired_line_and_one_answered_line() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = slot(tmp.path(), JumpClock(Mutex::new(Instant::now())), true);
        let (got, lines) = capture_lines(|| slot.hook_dialog(&question("pre-tool-use", false)));
        assert_eq!(got, Ok(json!({"ok": {"dialog_id": 1, "response": null}})));
        let logged = events(tmp.path());
        assert_eq!(kinds(tmp.path()), ["question", "dialog-expired"]);
        let expired = &logged[1];
        assert_eq!(expired["source"], "wrapper");
        assert_eq!(expired["instance"], "builder");
        assert_eq!(
            expired["data"],
            json!({"dialog_id": 1, "dialog_kind": "question"})
        );
        assert_eq!(expired["data"]["dialog_id"], logged[0]["data"]["dialog_id"]);
        assert_eq!(expired["data"]["dialog_kind"], logged[0]["kind"]);
        let answered: Vec<&Value> = lines
            .iter()
            .filter(|l| l["event"] == "dialog-answered")
            .collect();
        assert_eq!(answered.len(), 1, "{lines:?}");
        assert_eq!(answered[0]["corr"], 1);
        assert_eq!(answered[0]["dialog_kind"], "question");
        assert_eq!(answered[0]["deadline_hit"], true);
        assert!(answered[0].get("from").is_none(), "{:?}", answered[0]);
        let late =
            json!({"v": 1, "dialog_id": 1, "response": {"answers": {"Which color?": "red"}}});
        assert_eq!(
            slot.answer(&call(&late)),
            Ok(json!({"refusal": "not-delivered", "detail": "unknown-dialog"}))
        );
    }

    #[test]
    fn hook_dialog_hand_back_writes_no_dialog_expired_line() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = slot(tmp.path(), FixedClock(Instant::now()), true);
        std::thread::scope(|s| {
            let raising = s.spawn(|| slot.hook_dialog(&permission()));
            while slot.state().pending.is_none() {
                std::thread::yield_now();
            }
            slot.hand_back();
            assert_eq!(
                raising.join().expect("thread"),
                Ok(json!({"ok": {"dialog_id": 1, "response": null}}))
            );
        });
        raise_and_answer(
            &slot,
            tmp.path(),
            &plan("pre-tool-use", false),
            json!({"behavior": "approve"}),
        );
        assert_eq!(kinds(tmp.path()), ["permission", "plan"]);
    }

    /// The plan revise rides the PermissionRequest that repeats the PreToolUse; the repeat logs
    /// nothing.
    #[test]
    fn continuation_of_a_revised_plan_carries_the_revise_and_logs_nothing() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = slot(tmp.path(), FixedClock(Instant::now()), true);
        let revise = json!({"behavior": "revise", "message": "rename it"});
        let (raised, _, _) = raise_and_answer(
            &slot,
            tmp.path(),
            &plan("pre-tool-use", false),
            revise.clone(),
        );
        assert_eq!(raised["ok"]["response"], revise);
        let continued = slot.hook_dialog(&plan("permission-request", true));
        assert_eq!(
            continued,
            Ok(json!({"ok": {"dialog_id": 1, "response": revise}}))
        );
        assert_eq!(events(tmp.path()).len(), 1);
        assert!(slot.state().arms.is_empty(), "the arm is consumed");
    }

    #[rstest]
    #[case::approved_plan(plan("pre-tool-use", false), json!({"behavior": "approve"}), plan("permission-request", true))]
    #[case::answered_question(question("pre-tool-use", false), json!({"answers": {"Which color?": "red"}}), question("permission-request", true))]
    fn continuation_of_any_other_outcome_is_null_and_logs_nothing(
        #[case] raise: Value,
        #[case] response: Value,
        #[case] repeat: Value,
    ) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = slot(tmp.path(), FixedClock(Instant::now()), true);
        raise_and_answer(&slot, tmp.path(), &raise, response);
        assert_eq!(
            slot.hook_dialog(&repeat),
            Ok(json!({"ok": {"dialog_id": 1, "response": null}}))
        );
        assert_eq!(events(tmp.path()).len(), 1);
    }

    #[test]
    fn continuation_of_an_unanswered_dialog_is_null_and_logs_nothing() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = slot(tmp.path(), FixedClock(Instant::now()), false);
        slot.hook_dialog(&plan("pre-tool-use", false))
            .expect("raised");
        assert_eq!(
            slot.hook_dialog(&plan("permission-request", true)),
            Ok(json!({"ok": {"dialog_id": 1, "response": null}}))
        );
        assert_eq!(events(tmp.path()).len(), 1);
    }

    /// No arm (a restart between the hooks) or an unequal `tool_input`: the repeat is raised as its
    /// own dialog, and a plan so raised is held for an answer.
    #[test]
    fn continuation_without_a_matching_arm_is_raised_and_logged_once() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = slot(tmp.path(), FixedClock(Instant::now()), false);
        slot.hook_dialog(&plan("pre-tool-use", false))
            .expect("raised");
        let mut other = plan("permission-request", true);
        other["input"] = json!({"plan": "another plan"});
        assert_eq!(
            slot.hook_dialog(&other),
            Ok(json!({"ok": {"dialog_id": 2, "response": null}}))
        );
        assert_eq!(events(tmp.path()).len(), 2);

        let tmp = tempfile::tempdir().expect("tempdir");
        let held_slot = super::tests::slot(tmp.path(), FixedClock(Instant::now()), true);
        let revise = json!({"behavior": "revise", "message": "m"});
        let (raised, _, held) = raise_and_answer(
            &held_slot,
            tmp.path(),
            &plan("permission-request", true),
            revise.clone(),
        );
        assert_eq!(raised["ok"]["response"], revise);
        assert!(held.is_some());
        assert_eq!(events(tmp.path())[0]["kind"], "plan");
    }

    /// A PreToolUse for another tool leaves the first tool's arm in place: its continuation still
    /// takes the first dialog's id and logs nothing.
    #[test]
    fn continuation_survives_a_pre_tool_use_for_another_tool() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = slot(tmp.path(), FixedClock(Instant::now()), false);
        slot.hook_dialog(&plan("pre-tool-use", false))
            .expect("plan raised");
        slot.hook_dialog(&question("pre-tool-use", false))
            .expect("question raised");
        assert_eq!(
            slot.hook_dialog(&plan("permission-request", true)),
            Ok(json!({"ok": {"dialog_id": 1, "response": null}}))
        );
        assert_eq!(events(tmp.path()).len(), 2);
    }

    #[test]
    fn millis_is_the_whole_milliseconds_capped_at_u64_max() {
        assert_eq!(millis(Duration::from_millis(1500)), 1500);
        assert_eq!(millis(Duration::MAX), u64::MAX);
    }

    /// A question first raised by PermissionRequest is logged and left to the human, even on a
    /// stamped CLI: its body has no ledger row (read only statically on 2.1.288 / 2.1.287).
    #[test]
    fn continuation_raised_as_a_question_is_null_at_once() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = slot(tmp.path(), FixedClock(Instant::now()), true);
        assert_eq!(
            slot.hook_dialog(&question("permission-request", true)),
            Ok(json!({"ok": {"dialog_id": 1, "response": null}}))
        );
        assert!(slot.state().pending.is_none());
        assert_eq!(events(tmp.path())[0]["kind"], "question");
    }

    #[test]
    fn restore_continues_past_the_highest_logged_id() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = slot(tmp.path(), FixedClock(Instant::now()), false);
        slot.restore(Some(41));
        let got = slot.hook_dialog(&permission()).expect("raised");
        assert_eq!(got["ok"]["dialog_id"], 42);
        slot.restore(None);
        assert_eq!(slot.state().next_id, 1);
    }

    fn logged(value: Value) -> LoggedLine {
        LoggedLine {
            start: 0,
            end: 1,
            value,
        }
    }

    #[test]
    fn highest_dialog_id_reads_only_dialog_kinds() {
        let fold = |lines: &[Value]| {
            lines
                .iter()
                .fold(None, |h, l| highest_dialog_id(&logged(l.clone()), h))
        };
        let lines = [
            json!({"kind": "plan", "data": {"dialog_id": 7}}),
            json!({"kind": "turn-ended", "data": {"dialog_id": 99}}),
            json!({"kind": "question", "data": {"dialog_id": 3}}),
            json!({"kind": "permission", "data": {"dialog_id": "x"}}),
        ];
        assert_eq!(fold(&lines), Some(7));
        assert_eq!(fold(&lines[1..]), Some(3));
        assert_eq!(
            fold(&[json!({"kind": "permission", "data": {"dialog_id": 12}})]),
            Some(12)
        );
        assert_eq!(fold(&[]), None);
    }

    /// The `answer` order, written out: the params, control-character ahead of everything, then
    /// the stamp, then the pending id, then the response's kind.
    #[rstest]
    #[case::control_character_before_unverified(
        false,
        json!({"v": 1, "dialog_id": 1, "response": {"behavior": "deny", "message": "a\u{1b}b"}}),
        Ok(json!({"refusal": "not-delivered", "detail": "control-character"}))
    )]
    #[case::control_character_in_an_annotation(
        true,
        json!({"v": 1, "dialog_id": 9, "response": {"answers": {"q": "a"}, "annotations": {"q": {"notes": "\u{7f}"}}}}),
        Ok(json!({"refusal": "not-delivered", "detail": "control-character"}))
    )]
    #[case::unverified_before_unknown(
        false,
        json!({"v": 1, "dialog_id": 9, "response": {"behavior": "allow"}}),
        Ok(json!({"refusal": "unverified-cli", "detail": null}))
    )]
    #[case::unknown_dialog(
        true,
        json!({"v": 1, "dialog_id": 9, "response": {"behavior": "allow", "message": "ok\nfine\t"}}),
        Ok(json!({"refusal": "not-delivered", "detail": "unknown-dialog"}))
    )]
    #[case::id_not_u64(true, json!({"v": 1, "dialog_id": "1", "response": {"behavior": "allow"}}), Err(ProtocolError::InvalidParams))]
    #[case::id_negative(true, json!({"v": 1, "dialog_id": -1, "response": {"behavior": "allow"}}), Err(ProtocolError::InvalidParams))]
    #[case::no_response(true, json!({"v": 1, "dialog_id": 1}), Err(ProtocolError::InvalidParams))]
    #[case::behavior_not_enum(true, json!({"v": 1, "dialog_id": 1, "response": {"behavior": "maybe"}}), Err(ProtocolError::InvalidParams))]
    #[case::from_not_a_name(true, json!({"v": 1, "dialog_id": 1, "from": "Bad", "response": {"behavior": "allow"}}), Err(ProtocolError::InvalidParams))]
    fn answer_refusal_order(
        #[case] verified: bool,
        #[case] params: Value,
        #[case] expected: Result<Value, ProtocolError>,
    ) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = slot(tmp.path(), FixedClock(Instant::now()), verified);
        assert_eq!(slot.answer(&call(&params)), expected);
    }

    /// A response of another kind than the pending dialog's is `-32602`, and the dialog stays held.
    #[test]
    fn answer_of_the_wrong_kind_is_invalid_params() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = slot(tmp.path(), FixedClock(Instant::now()), true);
        std::thread::scope(|s| {
            let raising = s.spawn(|| slot.hook_dialog(&plan("pre-tool-use", false)));
            while slot.state().pending.is_none() {
                std::thread::yield_now();
            }
            let wrong = json!({"v": 1, "dialog_id": 1, "response": {"behavior": "allow"}});
            assert_eq!(
                slot.answer(&call(&wrong)),
                Err(ProtocolError::InvalidParams)
            );
            assert!(slot.state().pending.is_some());
            let right = json!({"v": 1, "dialog_id": 1, "from": "overseer",
                "response": {"behavior": "approve"}});
            slot.answer(&call(&right)).expect("answered");
            raising.join().expect("thread").expect("replied");
        });
    }

    #[rstest]
    #[case::no_kind(json!({"v": 1, "data": {}, "hook_event": "pre-tool-use"}))]
    #[case::unknown_kind(json!({"v": 1, "kind": "turn-ended", "data": {}, "hook_event": "pre-tool-use"}))]
    #[case::data_not_object(json!({"v": 1, "kind": "plan", "data": [1], "hook_event": "pre-tool-use"}))]
    #[case::spine_hook(json!({"v": 1, "kind": "plan", "data": {}, "hook_event": "stop"}))]
    #[case::unknown_tool(json!({"v": 1, "kind": "plan", "data": {}, "hook_event": "pre-tool-use", "tool": "Bash"}))]
    #[case::tool_of_another_kind(json!({"v": 1, "kind": "plan", "data": {}, "hook_event": "pre-tool-use", "tool": "AskUserQuestion"}))]
    #[case::continuation_without_tool(json!({"v": 1, "kind": "plan", "data": {}, "hook_event": "permission-request", "continuation": true}))]
    #[case::continuation_not_bool(json!({"v": 1, "kind": "plan", "data": {}, "hook_event": "permission-request", "tool": "ExitPlanMode", "continuation": 1}))]
    fn hook_dialog_params_it_cannot_take_are_invalid_and_log_nothing(#[case] params: Value) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = slot(tmp.path(), FixedClock(Instant::now()), true);
        assert_eq!(slot.hook_dialog(&params), Err(ProtocolError::InvalidParams));
        assert!(events(tmp.path()).is_empty());
    }

    #[test]
    fn hook_dialog_append_that_failed_is_internal() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let missing = tmp.path().join("missing");
        let slot = slot_on(
            &missing,
            FixedClock(Instant::now()),
            true,
            Arc::default(),
            Arc::new(Snapshots::new(missing.clone())),
        );
        assert_eq!(
            slot.hook_dialog(&permission()),
            Err(ProtocolError::Internal)
        );
        assert!(slot.state().pending.is_none());
    }

    /// A held dialog's append that fails leaves no `pending_dialog` behind on the snapshot it was
    /// first written to.
    #[test]
    fn hook_dialog_append_that_failed_clears_the_pending_dialog() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = slot_on(
            &tmp.path().join("missing"),
            FixedClock(Instant::now()),
            true,
            Arc::default(),
            seeded_snapshots(tmp.path()),
        );
        assert_eq!(
            slot.hook_dialog(&permission()),
            Err(ProtocolError::Internal)
        );
        assert_eq!(pending_on_disk(tmp.path()), None);
        assert!(slot.state().pending.is_none());
    }

    /// What `pending_dialog` read on disk at each `dialog-raised` line the calling thread emits.
    #[derive(Clone)]
    struct RaisedReads(PathBuf, Arc<Mutex<Vec<Option<PendingDialog>>>>);

    impl<S: tracing::Subscriber> tracing_subscriber::Layer<S> for RaisedReads {
        fn on_event(
            &self,
            event: &tracing::Event<'_>,
            _: tracing_subscriber::layer::Context<'_, S>,
        ) {
            let mut fields = viola_channel::test_support::JsonFields::default();
            event.record(&mut fields);
            if fields.0.get("event").and_then(Value::as_str) == Some("dialog-raised") {
                let held = pending_on_disk(&self.0);
                self.1.lock().expect("reads").push(held);
            }
        }
    }

    /// Whoever sees a held dialog's line on disk already sees `pending_dialog` on the snapshot: it
    /// is written before the line, so the `dialog-raised` line that follows the append reads it.
    #[test]
    fn hook_dialog_held_is_on_the_snapshot_when_its_line_is_logged() {
        use tracing_subscriber::layer::SubscriberExt as _;
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = slot(tmp.path(), JumpClock(Mutex::new(Instant::now())), true);
        let reads = RaisedReads(tmp.path().to_path_buf(), Arc::default());
        let subscriber = tracing_subscriber::registry().with(reads.clone());
        let got = crate::test_support::with_scoped_default(subscriber, || {
            slot.hook_dialog(&permission())
        });
        assert_eq!(got, Ok(json!({"ok": {"dialog_id": 1, "response": null}})));
        assert_eq!(
            *reads.1.lock().expect("reads"),
            [Some(PendingDialog {
                dialog_id: 1,
                kind: "permission".to_owned()
            })]
        );
    }

    /// The human takes the wheel (a `pause`, recorded through the wheel's thread) while a dialog
    /// is held: it is answered `null` at once, long before the deadline, `pending_dialog` leaves
    /// the snapshot, and its PermissionRequest repeat is left to the human too.
    #[test]
    fn hook_dialog_held_is_handed_back_null_at_once_on_a_wheel_move() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let snapshots = seeded_snapshots(tmp.path());
        let wheel = Arc::new(WheelSlot::default());
        let slot = Arc::new(slot_on(
            tmp.path(),
            FixedClock(Instant::now()),
            true,
            Arc::clone(&wheel),
            Arc::clone(&snapshots),
        ));
        wheel.record_with(Recorder {
            name: name(),
            instance_dir: tmp.path().to_path_buf(),
            snapshots,
            dialogs: Arc::clone(&slot),
        });
        std::thread::scope(|s| {
            let raising = s.spawn(|| slot.hook_dialog(&plan("pre-tool-use", false)));
            while slot.state().pending.is_none() {
                std::thread::yield_now();
            }
            assert!(pending_on_disk(tmp.path()).is_some());
            let none = json!({"v": 1});
            let paused = wheel.pause(&Call {
                method: "pause",
                params: &none,
                id: Some(1),
                conn: None,
                srv_conn: None,
            });
            assert_eq!(paused, Ok(json!({"ok": {"wheel": "human"}})));
            let raised = raising.join().expect("thread");
            assert_eq!(
                raised,
                Ok(json!({"ok": {"dialog_id": 1, "response": null}}))
            );
        });
        assert!(slot.state().pending.is_none());
        assert_eq!(pending_on_disk(tmp.path()), None);
        assert_eq!(
            slot.hook_dialog(&plan("permission-request", true)),
            Ok(json!({"ok": {"dialog_id": 1, "response": null}}))
        );
        let kinds: Vec<Value> = events(tmp.path())
            .iter()
            .map(|e| e["kind"].clone())
            .collect();
        assert_eq!(kinds, ["plan", "wheel"]);
    }

    /// Under a human wheel a dialog is still logged once with its own id, and left to the human.
    #[test]
    fn hook_dialog_under_a_human_wheel_is_logged_once_and_null_at_once() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let wheel = Arc::new(WheelSlot::default());
        wheel.human_input();
        let slot = slot_on(
            tmp.path(),
            FixedClock(Instant::now()),
            true,
            wheel,
            seeded_snapshots(tmp.path()),
        );
        assert_eq!(
            slot.hook_dialog(&permission()),
            Ok(json!({"ok": {"dialog_id": 1, "response": null}}))
        );
        assert!(slot.state().pending.is_none());
        assert_eq!(pending_on_disk(tmp.path()), None);
        let lines = events(tmp.path());
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0]["kind"], "permission");
        assert_eq!(lines[0]["data"]["dialog_id"], 1);
    }

    /// A human wheel, taken by a key or (`paused`) by `viola pause`.
    fn human_wheel(paused: bool) -> Arc<WheelSlot> {
        let wheel = Arc::new(WheelSlot::default());
        wheel.human_input();
        if paused {
            let none = json!({"v": 1});
            wheel
                .pause(&Call {
                    method: "pause",
                    params: &none,
                    id: Some(1),
                    conn: None,
                    srv_conn: None,
                })
                .expect("paused");
        }
        wheel
    }

    /// The `answer` order under a human wheel, written out: control-character first, then
    /// human-typing ahead of the stamp and of the pending id.
    #[rstest]
    #[case::control_character_before_human_typing(
        true,
        true,
        json!({"v": 1, "dialog_id": 1, "response": {"behavior": "deny", "message": "a\u{1b}b"}}),
        json!({"refusal": "not-delivered", "detail": "control-character"})
    )]
    #[case::human_typing_before_unverified(
        false,
        false,
        json!({"v": 1, "dialog_id": 1, "response": {"behavior": "allow"}}),
        json!({"refusal": "human-typing", "detail": null})
    )]
    #[case::manual_pause_before_unverified(
        true,
        false,
        json!({"v": 1, "dialog_id": 1, "response": {"behavior": "allow"}}),
        json!({"refusal": "human-typing", "detail": "manual-pause"})
    )]
    #[case::human_typing_before_unknown_dialog(
        false,
        true,
        json!({"v": 1, "dialog_id": 9, "response": {"behavior": "allow"}}),
        json!({"refusal": "human-typing", "detail": null})
    )]
    #[case::manual_pause_before_unknown_dialog(
        true,
        true,
        json!({"v": 1, "dialog_id": 9, "response": {"behavior": "allow"}}),
        json!({"refusal": "human-typing", "detail": "manual-pause"})
    )]
    fn answer_refusal_order_under_a_human_wheel(
        #[case] paused: bool,
        #[case] verified: bool,
        #[case] params: Value,
        #[case] expected: Value,
    ) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = slot_on(
            tmp.path(),
            FixedClock(Instant::now()),
            verified,
            human_wheel(paused),
            seeded_snapshots(tmp.path()),
        );
        assert_eq!(slot.answer(&call(&params)), Ok(expected));
    }

    /// A parked `wait` with no `after` while a dialog is held returns that dialog at once.
    #[test]
    fn a_held_dialog_is_returned_by_a_wait_without_after() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = slot(tmp.path(), FixedClock(Instant::now()), true);
        let (seen_tx, seen) = mpsc::channel();
        std::thread::scope(|s| {
            let raising = s.spawn(|| slot.hook_dialog(&permission()));
            while slot.state().pending.is_none() {
                std::thread::yield_now();
            }
            let woken = slot
                .feed
                .wait(tmp.path(), &json!({"v": 1, "timeout_ms": 1}))
                .expect("answered");
            seen_tx.send(woken).expect("send");
            let answer = json!({"v": 1, "dialog_id": 1, "response": {"behavior": "deny"}});
            slot.answer(&call(&answer)).expect("answered");
            raising.join().expect("thread").expect("replied");
        });
        let woken = seen.recv().expect("woken");
        assert_eq!(woken["ok"]["event"]["kind"], "permission");
        assert_eq!(woken["ok"]["event"]["data"]["dialog_id"], 1);
        let after = slot
            .feed
            .wait(tmp.path(), &json!({"v": 1, "timeout_ms": 1}))
            .expect("answered");
        assert_eq!(after, json!({"ok": {"timed_out": true}}));
    }

    /// A verified slot over `dir` that was handed `builder`'s link set over `links_dir`.
    fn linking(dir: &Path, links_dir: &Path, wheel: Arc<WheelSlot>) -> DialogSlot {
        let snapshots = seeded_snapshots(dir);
        let clock = FixedClock(Instant::now());
        let slot = slot_on(dir, clock, true, wheel, Arc::clone(&snapshots));
        let links = LinkSet::new(name(), links_dir.to_path_buf(), snapshots);
        slot.links_with(Arc::new(links));
        slot
    }

    /// Raises `params` on a thread and sends the held dialog each of `answers` in turn, `dialog_id`
    /// filled in: the `hook.dialog` reply and every `answer` reply.
    fn raise_and_send(
        slot: &DialogSlot,
        params: &Value,
        answers: &[Value],
    ) -> (Value, Vec<Result<Value, ProtocolError>>) {
        std::thread::scope(|s| {
            let raising = s.spawn(|| slot.hook_dialog(params));
            let dialog_id = loop {
                if let Some(p) = slot.state().pending.as_ref() {
                    break p.dialog_id;
                }
                std::thread::yield_now();
            };
            let replies = answers
                .iter()
                .map(|answer| {
                    let mut answer = answer.clone();
                    answer["dialog_id"] = json!(dialog_id);
                    slot.answer(&call(&answer))
                })
                .collect();
            (raising.join().expect("thread").expect("replied"), replies)
        })
    }

    fn kinds(dir: &Path) -> Vec<Value> {
        events(dir).iter().map(|e| e["kind"].clone()).collect()
    }

    fn links_on_disk(dir: &Path) -> Vec<Value> {
        read_snapshot(dir).expect("snapshot").links
    }

    #[test]
    fn answer_link_a_taken_answer_from_another_instance_appends_one_link() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = linking(tmp.path(), tmp.path(), Arc::default());
        let response = json!({"answers": {"Which color?": "red"}});
        let answer = json!({"v": 1, "from": "overseer", "response": response});
        let (raised, replies) = raise_and_send(&slot, &question("pre-tool-use", false), &[answer]);
        assert_eq!(replies, [Ok(json!({"ok": {}}))]);
        assert_eq!(raised["ok"]["response"], response, "the answer is taken");
        assert_eq!(kinds(tmp.path()), ["question", "link"]);
        let link = &events(tmp.path())[1];
        assert_eq!(link["source"], "wrapper");
        assert_eq!(link["instance"], "builder");
        assert_eq!(
            link["data"],
            json!({"driver": "overseer", "driven": "builder"})
        );
        assert_eq!(
            links_on_disk(tmp.path()),
            [json!({"driver": "overseer", "driven": "builder", "since": link["ts"]})]
        );
    }

    #[test]
    fn answer_link_a_second_taken_answer_adds_no_second_line() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = linking(tmp.path(), tmp.path(), Arc::default());
        let approve = json!({"v": 1, "from": "overseer", "response": {"behavior": "approve"}});
        let allow = json!({"v": 1, "from": "overseer", "response": {"behavior": "allow"}});
        raise_and_send(&slot, &plan("pre-tool-use", false), &[approve]);
        let (_, replies) = raise_and_send(&slot, &permission(), &[allow]);
        assert_eq!(replies, [Ok(json!({"ok": {}}))]);
        assert_eq!(kinds(tmp.path()), ["plan", "link", "permission"]);
        assert_eq!(links_on_disk(tmp.path()).len(), 1);
    }

    /// An answer refused with no dialog held links nothing, whichever rung refuses it.
    #[rstest]
    #[case::unknown_dialog(false, json!({"refusal": "not-delivered", "detail": "unknown-dialog"}))]
    #[case::human_typing(true, json!({"refusal": "human-typing", "detail": null}))]
    fn answer_link_a_refused_answer_appends_none(#[case] human_key: bool, #[case] refused: Value) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let wheel = Arc::new(WheelSlot::default());
        if human_key {
            wheel.human_input();
        }
        let slot = linking(tmp.path(), tmp.path(), wheel);
        let params = json!({"v": 1, "dialog_id": 9, "from": "overseer",
            "response": {"behavior": "allow"}});
        assert_eq!(slot.answer(&call(&params)), Ok(refused));
        assert!(events(tmp.path()).is_empty());
        assert_eq!(links_on_disk(tmp.path()), Vec::<Value>::new());
    }

    /// While a dialog is held, an answer of another kind than its own is `-32602` and links
    /// nothing; so does an answer with no `from`, and one under the wrapper's own name.
    #[test]
    fn answer_link_a_wrong_kind_no_from_and_the_wrappers_own_name_append_none() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = linking(tmp.path(), tmp.path(), Arc::default());
        let wrong = json!({"v": 1, "from": "overseer", "response": {"behavior": "allow"}});
        let approve = json!({"v": 1, "response": {"behavior": "approve"}});
        let (_, replies) = raise_and_send(&slot, &plan("pre-tool-use", false), &[wrong, approve]);
        assert_eq!(
            replies,
            [Err(ProtocolError::InvalidParams), Ok(json!({"ok": {}}))]
        );
        let own = json!({"v": 1, "from": "builder", "response": {"behavior": "allow"}});
        let (_, replies) = raise_and_send(&slot, &permission(), &[own]);
        assert_eq!(replies, [Ok(json!({"ok": {}}))]);
        assert_eq!(kinds(tmp.path()), ["plan", "permission"]);
        assert_eq!(links_on_disk(tmp.path()), Vec::<Value>::new());
    }

    /// A link that cannot be recorded answers `-32603` and the answer is not taken: the dialog
    /// stays held for the next answer.
    #[test]
    fn answer_link_a_link_that_cannot_be_recorded_is_internal_and_the_answer_is_not_taken() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let slot = linking(tmp.path(), &tmp.path().join("missing"), Arc::default());
        let unlinked = json!({"v": 1, "from": "overseer", "response": {"behavior": "approve"}});
        let revise = json!({"behavior": "revise", "message": "rename it"});
        let taken = json!({"v": 1, "response": revise});
        let (raised, replies) =
            raise_and_send(&slot, &plan("pre-tool-use", false), &[unlinked, taken]);
        assert_eq!(
            replies,
            [Err(ProtocolError::Internal), Ok(json!({"ok": {}}))]
        );
        assert_eq!(raised["ok"]["response"], revise);
        assert_eq!(kinds(tmp.path()), ["plan"]);
    }
}
