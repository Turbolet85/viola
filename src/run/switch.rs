//! The wrapper's `switch`: a running session moved to another model and effort by an in-place
//! restart. Read idle and the human off the keys, under the one flight guard a send takes, the
//! wrapper ends its own child, appends one `launch` line once that child has exited, and resumes
//! the session's id on the new pair in the same PTY, directory and terminal. The window, socket,
//! wheel holder, links and lineage stay; nothing is typed into the session and nothing is written
//! to the user's Claude settings or config. A later `revive` or `up` replays the last `launch`
//! line, so it resumes on the new pair.

use std::ffi::OsString;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock, PoisonError};

use chrono::Utc;
use serde_json::{Value, json};
use viola_agent_claude::{StripPlan, is_effort_level, is_session_id, resume_args};
use viola_channel::{Call, ProtocolError};
use viola_core::{EventKind, HumanTyping, RefusalReason, ViolaName, is_model_word};
use viola_pty::{PtyError, Respawn, Size, SpawnSpec};
use viola_state::events::{EventLine, Source, append_event};
use viola_state::pin::Pinned;
use viola_state::replay::{Recovered, read_snapshot_or_replay};
use viola_state::snapshot::{InstanceSnapshot, Window};

use crate::run::launch::{self, Pair};
use crate::run::send::SendSlot;
use crate::run::snapshot::Snapshots;
use crate::run::wheel::WheelSlot;

/// Ends the current child, runs the hook once it has exited, then spawns the spec in its place.
pub(crate) type Respawner =
    Box<dyn Fn(SpawnSpec, Box<dyn FnOnce() + Send>) -> Result<Respawn, PtyError> + Send + Sync>;

/// What a switch's child is started from: the start's own program, directory, pinned copy, plugin
/// folder, inherited `PATH`, strip plan and version gate reading.
pub(crate) struct ChildPlan {
    pub(crate) program: PathBuf,
    pub(crate) cwd: PathBuf,
    pub(crate) pinned: Pinned,
    pub(crate) plugin_dir: PathBuf,
    pub(crate) path: Option<OsString>,
    pub(crate) strip: StripPlan,
    pub(crate) cli_version: Option<String>,
    pub(crate) cli_verified: bool,
}

pub(crate) struct SwitchSlot {
    name: ViolaName,
    instance_dir: PathBuf,
    send: Arc<SendSlot>,
    wheel: Arc<WheelSlot>,
    snapshots: Arc<Snapshots>,
    child: ChildPlan,
    respawner: OnceLock<Respawner>,
}

/// The first rung that refuses a switch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rung {
    HumanTyping(Option<HumanTyping>),
    TurnRunning,
    DialogPending,
    UnverifiedCli,
    NoSession,
}

impl Rung {
    fn reply(self) -> Value {
        let (refusal, detail) = match self {
            Self::HumanTyping(detail) => {
                (RefusalReason::HumanTyping, detail.map(HumanTyping::as_str))
            }
            Self::TurnRunning => (RefusalReason::NotDelivered, Some("turn-running")),
            Self::DialogPending => (RefusalReason::NotDelivered, Some("dialog-pending")),
            Self::UnverifiedCli => (RefusalReason::UnverifiedCli, None),
            Self::NoSession => (RefusalReason::NotDelivered, Some("no-session")),
        };
        refusal_reply(refusal, detail)
    }
}

fn refusal_reply(refusal: RefusalReason, detail: Option<&str>) -> Value {
    json!({"refusal": refusal.as_str(), "detail": detail})
}

/// What the refusal rungs read, under the flight guard.
struct Reads<'a> {
    human: Option<Option<HumanTyping>>,
    /// A turn runs, a send is in flight, or another switch stands.
    busy: bool,
    dialog_pending: bool,
    cli_verified: bool,
    session: Option<&'a str>,
}

/// The session id to resume, or the first rung that refuses, in the documented order.
fn first_refusal(reads: &Reads<'_>) -> Result<String, Rung> {
    if let Some(detail) = reads.human {
        return Err(Rung::HumanTyping(detail));
    }
    if reads.busy {
        return Err(Rung::TurnRunning);
    }
    if reads.dialog_pending {
        return Err(Rung::DialogPending);
    }
    if !reads.cli_verified {
        return Err(Rung::UnverifiedCli);
    }
    reads
        .session
        .filter(|id| is_session_id(id))
        .map(str::to_owned)
        .ok_or(Rung::NoSession)
}

/// The given halves, each checked again here: neither given, a half of another type or one
/// failing its check is invalid params.
fn halves(params: &Value) -> Result<(Option<String>, Option<String>), ProtocolError> {
    let half = |key: &str, ok: fn(&str) -> bool| match params.get(key) {
        None => Ok(None),
        Some(Value::String(word)) if ok(word) => Ok(Some(word.clone())),
        Some(_) => Err(ProtocolError::InvalidParams),
    };
    let model = half("model", is_model_word)?;
    let effort = half("effort", is_effort_level)?;
    if model.is_none() && effort.is_none() {
        return Err(ProtocolError::InvalidParams);
    }
    Ok((model, effort))
}

/// Each half given, else the recorded one; with no record, the compiled default's.
fn pair_of(given: (Option<String>, Option<String>), recorded: Option<Pair>) -> Pair {
    let recorded = recorded.unwrap_or_else(launch::compiled);
    Pair {
        model: given.0.unwrap_or(recorded.model),
        effort: given.1.unwrap_or(recorded.effort),
    }
}

impl SwitchSlot {
    pub(crate) fn new(
        name: ViolaName,
        instance_dir: PathBuf,
        send: Arc<SendSlot>,
        wheel: Arc<WheelSlot>,
        snapshots: Arc<Snapshots>,
        child: ChildPlan,
    ) -> Self {
        Self {
            name,
            instance_dir,
            send,
            wheel,
            snapshots,
            child,
            respawner: OnceLock::new(),
        }
    }

    /// The first call wins; the pump's caller sets it once, before the pump starts.
    pub(crate) fn respawn_with(&self, respawner: Respawner) {
        let _ = self.respawner.set(respawner);
    }

    /// The `switch` method: the params, the refusal rungs under the flight guard, then the
    /// respawn, its `launch` line appended between the old child's exit and the new spawn.
    pub(crate) fn switch(&self, call: &Call<'_>) -> Result<Value, ProtocolError> {
        let given = halves(call.params)?;
        let snapshot = match read_snapshot_or_replay(&self.instance_dir) {
            Ok(Recovered::Snapshot(snapshot)) => Some(*snapshot),
            _ => None,
        };
        let id = self.send.begin_switch(|busy| {
            first_refusal(&Reads {
                human: self.wheel.human_typing(),
                busy: busy || self.wheel.turn_running(),
                dialog_pending: snapshot
                    .as_ref()
                    .is_some_and(|s| s.pending_dialog.is_some()),
                cli_verified: self.child.cli_verified,
                session: snapshot
                    .as_ref()
                    .and_then(|s| s.agent_session_id.as_deref()),
            })
        });
        let id = match id {
            Ok(id) => id,
            Err(rung) => return Ok(rung.reply()),
        };
        let pair = pair_of(given, snapshot.as_ref().and_then(|s| s.launch.clone()));
        let outcome = self.respawn(&id, &pair);
        match outcome {
            Ok((Respawn::Respawned { exit, child_pid }, appended)) => {
                self.send.switched_to(child_pid);
                crate::run::log_child_exit(exit);
                crate::run::log_child_start(
                    child_pid,
                    &self.child.strip,
                    self.child.cli_version.as_deref(),
                    self.child.cli_verified,
                );
                appended.map_err(|()| ProtocolError::Internal)?;
                self.recorded(snapshot.as_ref(), &pair, child_pid)
                    .map_err(|()| ProtocolError::Internal)?;
                Ok(json!({"ok": {"model": pair.model, "effort": pair.effort}}))
            }
            Ok((Respawn::NotEnded, _)) => {
                self.send.end_switch();
                Ok(refusal_reply(
                    RefusalReason::NotDelivered,
                    Some("child-not-ended"),
                ))
            }
            Ok((Respawn::SpawnFailed, _)) => {
                self.send.end_switch();
                Ok(refusal_reply(
                    RefusalReason::NotDelivered,
                    Some("resume-failed"),
                ))
            }
            Err(_) => {
                self.send.end_switch();
                Err(ProtocolError::Internal)
            }
        }
    }

    /// The respawn of `id` on `pair`, its `launch` line appended (and the held wake count cleared,
    /// as a `session-start` clears it) once the old child has exited and before the new one is
    /// spawned: a wrapper ended past that point leaves the new pair recorded. With it, whether
    /// that append landed.
    fn respawn(&self, id: &str, pair: &Pair) -> Result<(Respawn, Result<(), ()>), PtyError> {
        let launched = crate::run::child_launch(
            &self.name,
            &self.instance_dir,
            &self.child.pinned,
            &self.child.plugin_dir,
            self.child.path.clone(),
            pair,
            &resume_args(id, false),
        );
        let spec = SpawnSpec {
            program: self.child.program.clone(),
            args: launched.args,
            cwd: self.child.cwd.clone(),
            env_set: launched.env_set,
            env_remove: self.child.strip.remove.clone(),
            size: Size::DEFAULT,
        };
        let appended = Arc::new(Mutex::new(Err(())));
        let (slot, name, dir, wheel) = (
            Arc::clone(&appended),
            self.name.clone(),
            self.instance_dir.clone(),
            Arc::clone(&self.wheel),
        );
        let data = json!({"model": pair.model, "effort": pair.effort});
        let between = move || {
            wheel.count_wakes(None);
            let line = EventLine::new(&name, EventKind::Launch, Source::Wrapper, data, Utc::now());
            *slot.lock().unwrap_or_else(PoisonError::into_inner) =
                append_event(&dir, &line).map_err(|_| ());
        };
        let respawner = self.respawner.get().ok_or(PtyError::NoInput)?;
        let respawned = respawner(spec, Box::new(between))?;
        let appended = *appended.lock().unwrap_or_else(PoisonError::into_inner);
        Ok((respawned, appended))
    }

    /// After the respawn: the recorded window's line again (a `launch` line clears the replayed
    /// window), then the snapshot's pair and child.
    fn recorded(
        &self,
        snapshot: Option<&InstanceSnapshot>,
        pair: &Pair,
        child_pid: Option<u32>,
    ) -> Result<(), ()> {
        if let Some(Window {
            address,
            workspace,
            placement,
            origin,
        }) = snapshot.and_then(|s| s.window.clone())
        {
            let data = json!({"address": address, "workspace": workspace, "placement": placement,
                "origin": origin});
            let line = EventLine::new(
                &self.name,
                EventKind::Window,
                Source::Wrapper,
                data,
                Utc::now(),
            );
            append_event(&self.instance_dir, &line).map_err(|_| ())?;
        }
        self.snapshots
            .update(|s| {
                s.launch = Some(pair.clone());
                s.child_pid = child_pid;
            })
            .map_err(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::sync::atomic::{AtomicBool, Ordering};

    use rstest::rstest;
    use viola_core::SystemClock;
    use viola_pty::{Exit, ExitSource};
    use viola_state::snapshot::PendingDialog;

    use crate::run::send::append_hook_event;
    use crate::run::wait::WaitFeed;

    const ID: &str = "4e678ce3-c94d-472b-92ab-9734c27f57dc";

    fn name() -> ViolaName {
        ViolaName::try_new("builder".to_owned()).expect("valid")
    }

    fn reads(
        human: bool,
        busy: bool,
        dialog: bool,
        verified: bool,
        session: bool,
    ) -> Reads<'static> {
        Reads {
            human: human.then_some(None),
            busy,
            dialog_pending: dialog,
            cli_verified: verified,
            session: session.then_some(ID),
        }
    }

    /// The refusal order, written out: human typing, a running turn, a pending dialog, an
    /// unverified CLI, no session; the first rung that holds wins.
    #[rstest]
    #[case::human_typing_first(reads(true, true, true, false, false), Err(Rung::HumanTyping(None)))]
    #[case::turn_running_before_dialog(
        reads(false, true, true, false, false),
        Err(Rung::TurnRunning)
    )]
    #[case::dialog_before_unverified(
        reads(false, false, true, false, false),
        Err(Rung::DialogPending)
    )]
    #[case::unverified_before_no_session(
        reads(false, false, false, false, false),
        Err(Rung::UnverifiedCli)
    )]
    #[case::no_session_last(reads(false, false, false, true, false), Err(Rung::NoSession))]
    #[case::idle_and_verified_resumes(reads(false, false, false, true, true), Ok(ID.to_owned()))]
    fn switch_refusal_order_cases(
        #[case] reads: Reads<'static>,
        #[case] want: Result<String, Rung>,
    ) {
        assert_eq!(first_refusal(&reads), want);
    }

    #[test]
    fn switch_refuses_a_manual_pause_with_its_detail() {
        let mut paused = reads(false, false, false, true, true);
        paused.human = Some(Some(HumanTyping::ManualPause));
        assert_eq!(
            first_refusal(&paused).map_err(Rung::reply),
            Err(json!({"refusal": "human-typing", "detail": "manual-pause"}))
        );
    }

    #[test]
    fn switch_refuses_a_session_id_of_another_shape_as_no_session() {
        let mut odd = reads(false, false, false, true, false);
        odd.session = Some("s-1");
        assert_eq!(first_refusal(&odd), Err(Rung::NoSession));
    }

    fn pair(model: &str, effort: &str) -> Pair {
        Pair {
            model: model.to_owned(),
            effort: effort.to_owned(),
        }
    }

    #[rstest]
    #[case::both_given((Some("sonnet"), Some("high")), Some(pair("opus", "low")), pair("sonnet", "high"))]
    #[case::model_given((Some("sonnet"), None), Some(pair("opus", "low")), pair("sonnet", "low"))]
    #[case::effort_given((None, Some("max")), Some(pair("opus", "low")), pair("opus", "max"))]
    #[case::no_record((None, Some("max")), None, pair("opus", "max"))]
    fn switch_pair_cases(
        #[case] given: (Option<&str>, Option<&str>),
        #[case] recorded: Option<Pair>,
        #[case] want: Pair,
    ) {
        let given = (given.0.map(str::to_owned), given.1.map(str::to_owned));
        assert_eq!(pair_of(given, recorded), want);
    }

    #[rstest]
    #[case::neither(json!({"v": 1}))]
    #[case::null_model(json!({"v": 1, "model": null}))]
    #[case::number_effort(json!({"v": 1, "effort": 3}))]
    #[case::bad_model(json!({"v": 1, "model": "Opus"}))]
    #[case::bad_effort(json!({"v": 1, "effort": "huge"}))]
    #[case::bad_half_beside_a_good_one(json!({"v": 1, "model": "opus", "effort": "huge"}))]
    fn switch_params_refused_as_invalid(#[case] params: Value) {
        assert_eq!(halves(&params), Err(ProtocolError::InvalidParams));
    }

    #[test]
    fn switch_params_take_either_half() {
        assert_eq!(
            halves(&json!({"v": 1, "effort": "xhigh"})),
            Ok((None, Some("xhigh".to_owned())))
        );
    }

    struct Rig {
        _tmp: tempfile::TempDir,
        dir: PathBuf,
        slot: SwitchSlot,
        send: Arc<SendSlot>,
    }

    fn rig(window: bool, dialog: bool) -> Rig {
        let tmp = tempfile::tempdir().expect("tempdir");
        let dir = tmp.path().join("builder");
        viola_state::fs::create_private_dir(&dir).expect("instance dir");
        let snapshots = Arc::new(Snapshots::new(dir.clone()));
        let mut first = crate::run::first();
        first.agent_session_id = Some(ID.to_owned());
        first.launch = Some(pair("opus", "medium"));
        first.window = window.then(|| Window {
            address: Some("0x1".to_owned()),
            workspace: Some(2),
            placement: "placed".to_owned(),
            origin: "cli".to_owned(),
        });
        first.pending_dialog = dialog.then(|| PendingDialog {
            dialog_id: 1,
            kind: "plan".to_owned(),
        });
        snapshots.init(first).expect("snapshot");
        let wheel = Arc::new(WheelSlot::default());
        let send = Arc::new(SendSlot::new(SystemClock, true, Arc::clone(&wheel)));
        let pinned = Pinned {
            key: "k".to_owned(),
            path: tmp.path().join("bin").join("viola"),
            path_fwd: "b".to_owned(),
        };
        let child = ChildPlan {
            program: PathBuf::from("/bin/true"),
            cwd: tmp.path().to_path_buf(),
            pinned,
            plugin_dir: tmp.path().join("plugin"),
            path: None,
            strip: StripPlan::default(),
            cli_version: None,
            cli_verified: true,
        };
        let slot = SwitchSlot::new(
            name(),
            dir.clone(),
            Arc::clone(&send),
            wheel,
            snapshots,
            child,
        );
        Rig {
            _tmp: tmp,
            dir,
            slot,
            send,
        }
    }

    fn kinds(dir: &Path) -> Vec<String> {
        let text = std::fs::read_to_string(dir.join("events.ndjson")).unwrap_or_default();
        text.lines()
            .map(|l| {
                let v: Value = serde_json::from_str(l).expect("line");
                v["kind"].as_str().expect("kind").to_owned()
            })
            .collect()
    }

    fn call(params: &Value) -> Call<'_> {
        Call {
            method: "switch",
            params,
            id: Some(1),
            conn: None,
            srv_conn: None,
        }
    }

    fn exit() -> Exit {
        Exit {
            code: Some(0),
            source: ExitSource::KillFallback,
        }
    }

    /// The `launch` line stands on disk when the new child is spawned, and a recorded window's
    /// line follows it; the snapshot then holds the new pair.
    #[rstest]
    #[case::no_window(false)]
    #[case::with_window(true)]
    fn switch_appends_launch_before_the_spawn(#[case] window: bool) {
        let rig = rig(window, false);
        let at_spawn = Arc::new(Mutex::new(Vec::new()));
        let (seen, dir) = (Arc::clone(&at_spawn), rig.dir.clone());
        rig.slot.respawn_with(Box::new(move |spec, between| {
            between();
            assert!(spec.args.iter().any(|a| a == "--resume"));
            *seen.lock().expect("seen") = kinds(&dir);
            Ok(Respawn::Respawned {
                exit: exit(),
                child_pid: Some(7),
            })
        }));
        let params = json!({"v": 1, "model": "sonnet"});
        let reply = rig.slot.switch(&call(&params)).expect("reply");
        assert_eq!(
            reply,
            json!({"ok": {"model": "sonnet", "effort": "medium"}})
        );
        assert_eq!(*at_spawn.lock().expect("seen"), ["launch"]);
        let want: &[&str] = if window {
            &["launch", "window"]
        } else {
            &["launch"]
        };
        assert_eq!(kinds(&rig.dir), want);
        let disk = viola_state::snapshot::read_snapshot(&rig.dir).expect("snapshot");
        assert_eq!(disk.launch, Some(pair("sonnet", "medium")));
        assert_eq!(disk.child_pid, Some(7));
    }

    /// A child that does not end, or a resume that fails to start, appends no `launch` line on the
    /// first and refuses both with their closed details.
    #[rstest]
    #[case::not_ended(Respawn::NotEnded, "child-not-ended", false)]
    #[case::spawn_failed(Respawn::SpawnFailed, "resume-failed", true)]
    fn switch_respawn_outcomes_are_refusals(
        #[case] outcome: Respawn,
        #[case] detail: &str,
        #[case] reaped: bool,
    ) {
        let rig = rig(false, false);
        rig.slot.respawn_with(Box::new(move |_, between| {
            if reaped {
                between();
            }
            Ok(outcome)
        }));
        let params = json!({"v": 1, "effort": "high"});
        let reply = rig.slot.switch(&call(&params)).expect("reply");
        assert_eq!(reply, json!({"refusal": "not-delivered", "detail": detail}));
        let want: &[&str] = if reaped { &["launch"] } else { &[] };
        assert_eq!(kinds(&rig.dir), want);
        assert!(!rig.send.flight_switching(), "the switch mark stands");
    }

    /// A pending dialog refuses with nothing appended and the child left alone.
    #[test]
    fn switch_with_a_pending_dialog_respawns_nothing() {
        let rig = rig(false, true);
        let asked = Arc::new(AtomicBool::new(false));
        let flag = Arc::clone(&asked);
        rig.slot.respawn_with(Box::new(move |_, _| {
            flag.store(true, Ordering::SeqCst);
            Ok(Respawn::NotEnded)
        }));
        let params = json!({"v": 1, "model": "opus"});
        let reply = rig.slot.switch(&call(&params)).expect("reply");
        assert_eq!(
            reply,
            json!({"refusal": "not-delivered", "detail": "dialog-pending"})
        );
        assert!(!asked.load(Ordering::SeqCst));
        assert!(kinds(&rig.dir).is_empty());
    }

    fn line(kind: EventKind, id: &str) -> EventLine {
        EventLine::new(
            &name(),
            kind,
            Source::Hook,
            json!({"agent_session_id": id, "cause": "resume"}),
            Utc::now(),
        )
    }

    /// The old child's `session-end` of the switched id is appended before the new child's
    /// `session-start`, and screened once after it while the new child runs.
    #[rstest]
    #[case::before_the_new_start(false, vec!["session-end", "session-start"])]
    #[case::after_the_new_start(true, vec!["session-start"])]
    fn switch_drops_the_old_childs_late_session_end(#[case] late: bool, #[case] want: Vec<&str>) {
        let rig = rig(false, false);
        let feed = WaitFeed::new(SystemClock);
        rig.send
            .begin_switch(|_| Ok::<_, ()>(ID.to_owned()))
            .expect("armed");
        rig.send.switched_to(Some(std::process::id()));
        let append = |kind| {
            append_hook_event(&rig.send, &feed, &rig.dir, line(kind, ID), None).expect("append");
        };
        if late {
            append(EventKind::SessionStart);
            append(EventKind::SessionEnd);
        } else {
            append(EventKind::SessionEnd);
            append(EventKind::SessionStart);
        }
        assert_eq!(kinds(&rig.dir), want);
        // The screen is spent: the next end of the id is the new child's own.
        append(EventKind::SessionEnd);
        assert_eq!(
            kinds(&rig.dir).last().map(String::as_str),
            Some("session-end")
        );
    }
}
