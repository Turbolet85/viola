use std::ffi::OsString;
use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::Arc;
use std::time::SystemTime;

use anyhow::Context as _;
use chrono::Utc;
use serde_json::{Value, json};
use tracing::instrument;
use viola_agent_claude::StripPlan;
use viola_agent_claude::ledger::transcript_path_ok;
use viola_agent_claude::screen::PASTE_BOUND;
use viola_channel::{Call, ChannelError, Dispatch, ProtocolError, Server, Serving};
use viola_core::obs::ObsProcess;
use viola_core::{EventKind, SystemClock, TurnEndCause, ViolaName, WheelCause};
use viola_pty::{HostTerminal, PasteHandle, PortablePty, PumpEnd, RespawnHandle, Size, SpawnSpec};
use viola_state::events::{EventLine, Source, append_event};
use viola_state::fs::{FILE_MODE, create_private_dir, replace_private_shared};
use viola_state::heartbeat::{Heartbeat, beat_age, touch_heartbeat};
use viola_state::liveness::{Liveness, STALE_AFTER, classify, own_start, same_process};
use viola_state::pin::{PinError, Pinned, pin_bytes, pin_key};
use viola_state::replay::{Recovered, read_snapshot_or_replay};
use viola_state::snapshot::{InstanceSnapshot, Lineage, Wheel, Window};
use viola_state::strict::{TreeRefused, check_home_tree};
use viola_state::strict::{check_endpoint_dir, check_instance};

use crate::run::ChildLaunch;
use crate::run::auto_verify::{self, Attempt, Outcome, auto_verify};
use crate::run::dialog::DialogSlot;
use crate::run::gate::{self, Tee};
use crate::run::launch::Pair;
use crate::run::links::LinkSet;
use crate::run::send::{self, SendSlot};
use crate::run::snapshot::Snapshots;
use crate::run::switch::{ChildPlan, SwitchSlot};
use crate::run::version_gate::{Gate, version_gate};
use crate::run::wait::WaitFeed;
use crate::run::wheel::{Observed, Recorder, WheelSlot};
use crate::{human, obs, run, trust_record};

#[derive(clap::Args)]
pub(crate) struct RunArgs {
    /// Instance name: [a-z0-9-], 1-32 characters, starting with a letter
    #[arg(value_parser = super::send::parse_name)]
    pub(super) name: ViolaName,
    /// The session this one was launched by, a name; else the launching session's own
    #[arg(long, value_name = "NAME", value_parser = super::send::parse_name)]
    parent: Option<ViolaName>,
    /// A word naming this session's role: [a-z0-9-], starting with a letter
    #[arg(long, value_name = "WORD", value_parser = parse_role)]
    role: Option<String>,
    /// The model the session starts on: an alias or a full name; else the configured default
    #[arg(long, value_name = "MODEL", value_parser = parse_model)]
    model: Option<String>,
    /// The effort the session starts on: low, medium, high, xhigh or max; else the configured default
    #[arg(long, value_name = "EFFORT", value_parser = parse_effort)]
    effort: Option<String>,
    /// Who asked `viola open` for this start, self-reported: the window step runs only with it
    #[arg(long, hide = true, value_name = "WORD", value_parser = parse_origin)]
    origin: Option<String>,
    /// The workspace `viola open` chose for this start's window
    #[arg(long, hide = true, value_name = "N", allow_negative_numbers = true)]
    place: Option<i32>,
    /// The program to wrap and its arguments, after `--`
    #[arg(last = true, required = true)]
    program: Vec<OsString>,
}

fn parse_origin(word: &str) -> Result<String, String> {
    if viola_core::ORIGIN_WORDS.contains(&word) {
        Ok(word.to_owned())
    } else {
        Err("an origin is cli or agent".to_owned())
    }
}

fn parse_role(word: &str) -> Result<String, String> {
    if viola_core::is_code_word(word) {
        Ok(word.to_owned())
    } else {
        Err("a role is [a-z0-9-], 1-64 characters, starting with a letter".to_owned())
    }
}

pub(super) fn parse_model(word: &str) -> Result<String, String> {
    if viola_core::is_model_word(word) {
        Ok(word.to_owned())
    } else {
        Err("a model is [a-z0-9.-], 1-64 characters, starting with a letter".to_owned())
    }
}

pub(super) fn parse_effort(word: &str) -> Result<String, String> {
    if viola_agent_claude::is_effort_level(word) {
        Ok(word.to_owned())
    } else {
        Err("an effort is one of low, medium, high, xhigh, max".to_owned())
    }
}

/// The usage error of a model or an effort typed after `--`: one fixed line, none of the words.
pub(super) fn pair_after_dashes() -> clap::Error {
    clap::Error::raw(
        clap::error::ErrorKind::ArgumentConflict,
        run::launch::PAIR_AFTER_DASHES,
    )
}

/// The arguments clap cannot check alone: a `--parent` naming the session itself, a place with no
/// origin, and a model or an effort among the program's words, are usage errors.
fn checked(args: RunArgs) -> Result<RunArgs, clap::Error> {
    if args.parent.as_ref() == Some(&args.name) {
        return Err(clap::Error::raw(
            clap::error::ErrorKind::ArgumentConflict,
            "the argument '--parent <NAME>' cannot name the session itself\n",
        ));
    }
    if args.place.is_some() && args.origin.is_none() {
        return Err(clap::Error::raw(
            clap::error::ErrorKind::MissingRequiredArgument,
            "the argument '--place <N>' needs '--origin <WORD>'\n",
        ));
    }
    if run::launch::names_pair(&args.program) {
        return Err(pair_after_dashes());
    }
    Ok(args)
}

/// The lineage a `run` start records: the `--parent` given, else the inherited name of the session
/// it was typed in (read as the parent only, never as this instance), dropped when it is this
/// instance's own; the role as given.
fn lineage_of(args: &RunArgs, inherited: Option<ViolaName>) -> Lineage {
    let parent = args
        .parent
        .clone()
        .or_else(|| inherited.filter(|own| own != &args.name));
    Lineage {
        parent: parent.map(|name| name.as_ref().to_owned()),
        role: args.role.clone(),
    }
}

fn name_str(name: &ViolaName) -> &str {
    name.as_ref()
}

/// Test-only seam, compiled only with the test-only `fake-agent` feature and so absent from any
/// release build: `FAKE_AGENT_PUMP_DELAY_MS` holds the pump back after the child starts, so a test can
/// land a host resize between the spawn sizing and the pump's first look without a timing bet.
#[cfg(feature = "fake-agent")]
fn hold_pump_start() {
    let ms = std::env::var("FAKE_AGENT_PUMP_DELAY_MS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok());
    if let Some(ms) = ms {
        std::thread::sleep(std::time::Duration::from_millis(ms.min(5_000)));
    }
}

/// The wrapper's answers: `send`, `wait`, `last`, `hook.dialog`, `answer`, `pause`, `release`,
/// `link`, `unlink`, `switch`, and the `hook.event` notification, which appends its event; every other method
/// is `-32601`.
pub(crate) struct Methods {
    name: ViolaName,
    instance_dir: PathBuf,
    send: Arc<SendSlot>,
    wait: Arc<WaitFeed>,
    dialogs: Arc<DialogSlot>,
    wheel: Arc<WheelSlot>,
    links: Arc<LinkSet>,
    switch: Arc<SwitchSlot>,
}

/// The kinds a hook may hand the wrapper.
const HOOK_KINDS: [EventKind; 5] = [
    EventKind::SessionStart,
    EventKind::PromptSubmitted,
    EventKind::TurnEnded,
    EventKind::SessionEnd,
    EventKind::Activity,
];

impl Methods {
    /// The event a `hook.event` carries, re-validated here (the wrapper re-runs validators): a hook
    /// kind, an object `data`, for `prompt-submitted` a string `text` and a closed `origin`, and
    /// for `turn-ended` a `cause` absent or a closed `TurnEndCause` word. A `session-start`'s
    /// sibling `transcript` comes with it only as a string passing `transcript_path_ok` against the
    /// line's own `agent_session_id`; any other `transcript` is dropped and the line still taken.
    fn hook_event_line(&self, params: &Value) -> Option<(EventLine, Option<PathBuf>)> {
        let event = &params["event"];
        let kind = HOOK_KINDS
            .into_iter()
            .find(|k| event["kind"].as_str() == Some(k.as_str()))?;
        let data = event.get("data").filter(|d| d.is_object())?;
        let prompt_ok = data["text"].is_string()
            && matches!(data["origin"].as_str(), Some("harness" | "human"));
        if kind == EventKind::PromptSubmitted && !prompt_ok {
            return None;
        }
        let cause_ok = data
            .get("cause")
            .is_none_or(|c| c.as_str().and_then(TurnEndCause::from_name).is_some());
        if kind == EventKind::TurnEnded && !cause_ok {
            return None;
        }
        let transcript = params["transcript"]
            .as_str()
            .filter(|path| {
                kind == EventKind::SessionStart
                    && data["agent_session_id"]
                        .as_str()
                        .is_some_and(|id| transcript_path_ok(path, id))
            })
            .map(PathBuf::from);
        let line = EventLine::new(&self.name, kind, Source::Hook, data.clone(), Utc::now());
        Some((line, transcript))
    }
}

impl Dispatch for Methods {
    fn dispatch(&self, method: &str, params: &Value) -> Result<Value, ProtocolError> {
        if method != "hook.event" {
            return Err(ProtocolError::MethodNotFound);
        }
        // A notification is never answered: an event that fails the checks is simply not appended.
        if let Some((line, transcript)) = self.hook_event_line(params) {
            send::append_hook_event(&self.send, &self.wait, &self.instance_dir, line, transcript)
                .map_err(|_| ProtocolError::Internal)?;
        }
        Ok(Value::Null)
    }

    fn dispatch_call(&self, call: &Call<'_>) -> Result<Value, ProtocolError> {
        match call.method {
            "send" => send::send(&self.send, &self.name, &self.instance_dir, call),
            "wait" => self.wait.wait(&self.instance_dir, call.params),
            "last" => self.wait.last(call.params),
            "hook.dialog" => self.dialogs.hook_dialog(call.params),
            "answer" => self.dialogs.answer(call),
            "pause" => self.wheel.pause(call),
            "release" => self.wheel.release(call),
            "link" => self.links.link(call.params),
            "unlink" => self.links.unlink(call.params),
            "switch" => self.switch.switch(call),
            _ => self.dispatch(call.method, call.params),
        }
    }
}

/// What a start launches. `run` builds it from its own arguments and the directory it is typed
/// in; `revive` from a dead instance's record.
pub(super) struct Launch {
    pub(super) name: ViolaName,
    /// The program word, looked up from the starting process's own `PATH` and current directory.
    pub(super) program: OsString,
    /// The child's arguments, after the wrapper's plugin flag.
    pub(super) args: Vec<OsString>,
    /// Where the child is spawned, and nothing else: the program is never looked up there.
    pub(super) spawn_dir: PathBuf,
    /// A start refusal prints its one `--json` document in place of its pair.
    pub(super) json: bool,
    /// The `{parent, role}` the start's `lineage` line records.
    pub(super) lineage: Lineage,
    /// The `(model, effort)` halves the start was given; a missing half is the default's.
    pub(super) given: (Option<String>, Option<String>),
    /// What `viola open` asked of this start's window; `None` runs no window step.
    pub(super) window: Option<WindowAsk>,
}

/// Who asked for the window, self-reported, and the workspace it goes to when one was chosen.
pub(crate) struct WindowAsk {
    pub(crate) origin: String,
    pub(crate) place: Option<i32>,
}

pub(crate) fn run(home: &Path, args: RunArgs) -> anyhow::Result<ExitCode> {
    let args = checked(args).unwrap_or_else(|usage| usage.exit());
    let lineage = lineage_of(&args, super::client::own_name());
    let exe = match checked_exe(home, &args.name)? {
        Ok(exe) => exe,
        Err(refused) => {
            refuse_tree(&args.name, refused);
            return Ok(ExitCode::from(1));
        }
    };
    let persistent = open_wrapper_log(home, &args.name)?;
    let mut words = args.program.into_iter();
    let launch = Launch {
        name: args.name,
        program: words
            .next()
            .expect("clap requires at least one program word"),
        args: words.collect(),
        spawn_dir: std::env::current_dir()?,
        json: false,
        lineage,
        given: (args.model, args.effort),
        window: args.origin.map(|origin| WindowAsk {
            origin,
            place: args.place,
        }),
    };
    match start(home, &launch, &persistent, &exe)? {
        Started::Launched(launched) => pump_child(*launched),
        Started::Refused(code) => Ok(code),
    }
}

/// This exe's bytes, read once, behind the strict-modes check of the home tree a start trusts
/// (the home, the instance, `ledger/`, `bin/`, `plugin/` and the folders and copy named by the
/// bytes' key): run before the wrapper log opens, since opening it narrows the home to its owner
/// and would erase the very state the check refuses.
pub(super) fn checked_exe(
    home: &Path,
    name: &ViolaName,
) -> anyhow::Result<Result<Vec<u8>, TreeRefused>> {
    let exe = std::fs::read(std::env::current_exe()?)?;
    let home = std::path::absolute(home)?;
    Ok(check_home_tree(&home, Some(name), Some(&pin_key(&exe))).map(|()| exe))
}

/// Whether the instance's home and files pass the strict-modes check.
pub(super) fn trusted(home: &Path, name: &ViolaName) -> anyhow::Result<bool> {
    let home = std::path::absolute(home)?;
    let instance_dir = home.join("instances").join(AsRef::<str>::as_ref(name));
    Ok(check_instance(&home, &instance_dir).is_ok())
}

/// What a `revive` or an `adopt` start holds once its prologue has passed: this exe's bytes and
/// the persistent environment's names.
pub(super) struct Prologue {
    pub(super) exe: Vec<u8>,
    pub(super) persistent: Vec<String>,
}

/// The first steps of a `revive` or an `adopt` start, in their order: the instance check, the
/// home tree check, then the wrapper log. `strict_modes` writes the verb's own strict-modes
/// refusal, a document when given `true`. Before the log opens: opening it narrows the home to
/// its owner, which would erase the very state this check refuses. A refused check opens nothing
/// under the home and writes no line.
pub(super) fn start_prologue(
    home: &Path,
    name: &ViolaName,
    json: bool,
    strict_modes: impl Fn(bool),
) -> anyhow::Result<Result<Prologue, Started>> {
    if !trusted(home, name)? {
        strict_modes(json);
        return Ok(Err(Started::Refused(ExitCode::from(1))));
    }
    let exe = match checked_exe(home, name)? {
        Ok(exe) => exe,
        Err(_) if json => {
            strict_modes(true);
            return Ok(Err(Started::Refused(ExitCode::from(1))));
        }
        Err(refused) => {
            refuse_tree(name, refused);
            return Ok(Err(Started::Refused(ExitCode::from(1))));
        }
    };
    let persistent = open_wrapper_log(home, name)?;
    Ok(Ok(Prologue { exe, persistent }))
}

/// A refused tree's stderr pair, the home's or the instance's: nothing under the home opened, no
/// line written.
pub(super) fn refuse_tree(name: &ViolaName, refused: TreeRefused) {
    match refused {
        TreeRefused::HomeLevel(_) => {
            let (unable, hint) = human::home_state_pair();
            human::refuse(unable, hint);
        }
        TreeRefused::InstanceLevel(_) => {
            let (unable, hint) = human::instance_state_pair(name.as_ref());
            human::refuse(&unable, &hint);
        }
    }
}

/// The wrapper's process log with its own start line, then the persistent environment's names:
/// what a wrapper does before its start order.
pub(super) fn open_wrapper_log(home: &Path, name: &ViolaName) -> anyhow::Result<Vec<String>> {
    let (level, rejection) = obs::read_diagnostics_level(home);
    obs::viola_obs_init(home, ObsProcess::Run, Some(name.clone()), level)?;
    run::log_self_start();
    if let Some(rejection) = rejection {
        obs::log_config_rejection(rejection);
    }
    let (persistent, keep_rejection) = run::persistent_names(home);
    if let Some(rejection) = keep_rejection {
        obs::log_config_rejection(rejection);
    }
    Ok(persistent)
}

/// Everything the pump needs, and the guards that must outlive it: the heartbeat thread and the
/// served endpoint.
pub(super) struct Launched {
    pty: PortablePty,
    terminal: Option<HostTerminal>,
    size: Size,
    /// The version gate's reading: the readiness gate reads the compiled signatures only then.
    cli_verified: bool,
    send: Arc<SendSlot>,
    wheel: Arc<WheelSlot>,
    switch: Arc<SwitchSlot>,
    _beat: Heartbeat,
    _serving: Serving,
}

pub(super) enum Started {
    Launched(Box<Launched>),
    Refused(ExitCode),
}

/// The documented start order (architecture §Established Decisions [Session Liveness]): program
/// resolution, collision check, pinned copy + plugin folder, the version gate, endpoint bind, the
/// automatic verify (`verify_new_cli`), the trust step on the final verdict, first snapshot +
/// heartbeat, start events, then the spawn.
#[instrument(skip_all, name = "run.start", fields(instance = name_str(&launch.name)))]
pub(super) fn start(
    home: &Path,
    launch: &Launch,
    persistent: &[String],
    exe: &[u8],
) -> anyhow::Result<Started> {
    let name = &launch.name;
    let cwd = std::env::current_dir()?;
    let program = match run::resolve_program(&launch.program, &cwd) {
        Ok(program) => program,
        Err(_) => return Ok(refused_with(launch.json, "internal-error", || {})),
    };

    // The log's open created the home, so this is its one canonical spelling: the endpoint, the
    // child's `VIOLA_DIR` and every path below are built over it.
    let home = super::canonical_home(home);
    let instance_dir = home.join("instances").join(name_str(name));
    let recovered = read_snapshot_or_replay(&instance_dir).ok();
    if let Some(refusal) = collision_check(name, &instance_dir, recovered.as_ref(), launch.json) {
        return Ok(refusal);
    }
    let Some((pinned, plugin_dir)) = pin_and_plugin(&home, exe)? else {
        return Ok(refused_with(
            launch.json,
            "pinned-hash-mismatch",
            refuse_tampered_pin,
        ));
    };
    let strip = viola_agent_claude::plan_strip(std::env::vars_os().map(|(k, _)| k), persistent);
    let gate = version_gate(&home, &program, &cwd, &strip);
    let (endpoint, server) = match bind_endpoint(name, &home)? {
        Bound::Serving(endpoint, server) => (endpoint, server),
        Bound::Taken => {
            return Ok(refused_with(launch.json, "squatted-name", || {
                refuse_squatted(name);
            }));
        }
        Bound::SocketDirRefused => {
            return Ok(refused_with(launch.json, "socket-dir-refused", || {
                refuse_socket_dir(name);
            }));
        }
    };
    // After the bind, so a refused socket directory still refuses before any child; the endpoint
    // is not served and no snapshot names it while the verify runs.
    let attempt = Attempt {
        home: &home,
        pinned: &pinned.path,
        program: &program,
        cwd: &cwd,
        strip: &strip,
        json: launch.json,
    };
    let gate = match auto_verify(&attempt, gate) {
        Outcome::Gate(gate) => gate,
        Outcome::Stopped(signal) => {
            drop(server);
            auto_verify::end_by(signal);
            return Ok(Started::Refused(ExitCode::from(1)));
        }
    };
    let cli_verified = gate.cli_verified;
    trust_record::trust_folder(cli_verified, &strip, &launch.spawn_dir);
    let window = launch
        .window
        .as_ref()
        .map(|ask| run::window::open_window(ask, &cwd, &strip));
    let wheel = Arc::new(WheelSlot::default());
    let snapshots = Arc::new(Snapshots::new(instance_dir.clone()));
    let (hold, rejections) = run::dialog::read_hold(&home);
    for rejection in rejections {
        obs::log_config_rejection(rejection);
    }
    let (default, rejections) = run::launch::read_default(&home);
    for rejection in rejections {
        obs::log_config_rejection(rejection);
    }
    let pair = run::launch::resolve(launch.given.0.clone(), launch.given.1.clone(), default);
    let send = Arc::new(SendSlot::new(SystemClock, cli_verified, Arc::clone(&wheel)));
    // Rebuilt before the endpoint serves: the first `last` already sees the newest logged turn, the
    // first dialog takes an id past every logged one, and a pair the log holds is not linked again.
    let wait = Arc::new(WaitFeed::new(SystemClock));
    let highest_dialog = wait.rebuild(&instance_dir)?;
    let links = Arc::new(LinkSet::new(
        name.clone(),
        instance_dir.clone(),
        Arc::clone(&snapshots),
    ));
    links.restore()?;
    send.links_with(Arc::clone(&links));
    send.snapshots_with(Arc::clone(&snapshots));
    let dialogs = Arc::new(
        DialogSlot::new(
            SystemClock,
            gate.cli_verified,
            name.clone(),
            instance_dir.clone(),
            Arc::clone(&wait),
            Arc::clone(&wheel),
            Arc::clone(&snapshots),
        )
        .hold_for(hold.duration()),
    );
    dialogs.restore(highest_dialog);
    dialogs.links_with(Arc::clone(&links));
    wheel.record_with(Recorder {
        name: name.clone(),
        instance_dir: instance_dir.clone(),
        snapshots: Arc::clone(&snapshots),
        dialogs: Arc::clone(&dialogs),
    });
    let switch = Arc::new(SwitchSlot::new(
        name.clone(),
        instance_dir.clone(),
        Arc::clone(&send),
        Arc::clone(&wheel),
        Arc::clone(&snapshots),
        ChildPlan {
            program: program.clone(),
            cwd: launch.spawn_dir.clone(),
            pinned: pinned.clone(),
            plugin_dir: plugin_dir.clone(),
            path: std::env::var_os("PATH"),
            strip: strip.clone(),
            cli_version: gate.cli_version.clone(),
            cli_verified,
        },
    ));
    let serving = server.serve(Arc::new(Methods {
        name: name.clone(),
        instance_dir: instance_dir.clone(),
        send: Arc::clone(&send),
        wait,
        dialogs,
        wheel: Arc::clone(&wheel),
        links: Arc::clone(&links),
        switch: Arc::clone(&switch),
    }));
    let (beat, snapshot) = start_state(
        name,
        &instance_dir,
        &snapshots,
        &pinned,
        endpoint,
        gate,
        Recorded {
            spawn_dir: &launch.spawn_dir,
            links: links.current(),
            lineage: launch.lineage.clone(),
            launch: pair.clone(),
            window,
        },
    )?;
    let child = run::child_launch(
        name,
        &instance_dir,
        &pinned,
        &plugin_dir,
        std::env::var_os("PATH"),
        &pair,
        &launch.args,
    );
    let spawned = spawn_child(
        program,
        launch.spawn_dir.clone(),
        child,
        &strip,
        &snapshots,
        snapshot,
    );
    let (pty, terminal, size) = spawned?;
    Ok(Started::Launched(Box::new(Launched {
        pty,
        terminal,
        size,
        cli_verified,
        send,
        wheel,
        switch,
        _beat: beat,
        _serving: serving,
    })))
}

pub(super) fn refused(detail: &'static str) -> Started {
    run::log_self_exit(1, Some(detail));
    Started::Refused(ExitCode::from(1))
}

/// A refusal of the start, before any child: the stderr pair from `human`, or with `json` the
/// code's one document on stdout in place of it. The line is the same either way.
fn refused_with(json: bool, detail: &'static str, human: impl FnOnce()) -> Started {
    if json {
        super::client::refused_document(detail);
    } else {
        human();
    }
    refused(detail)
}

/// A `live` or `stale` name refuses before anything is written; a `gone` one is taken over. The
/// snapshot is the caller's one read of it through the replay (`None` when that read failed). With
/// `json` the refusal is its code's document on stdout in place of the stderr pair. The span's
/// `outcome` says a holder stands, whichever; the cause is the exit line's `detail`.
#[instrument(skip_all, name = "run.collision_check", fields(outcome = tracing::field::Empty))]
pub(super) fn collision_check(
    name: &ViolaName,
    instance_dir: &Path,
    recovered: Option<&Recovered>,
    json: bool,
) -> Option<Started> {
    let age = || beat_age(instance_dir, SystemTime::now());
    let taken = match recovered {
        Some(Recovered::Snapshot(snapshot)) => Some(classify(age(), same_process(snapshot))),
        // A replayed snapshot records no pid: the beat alone says whether a wrapper still runs.
        Some(Recovered::Replayed { .. }) => age()
            .is_some_and(|age| age <= STALE_AFTER)
            .then_some(Liveness::Live),
        Some(Recovered::Absent) | None => None,
    };
    let outcome = match taken {
        None | Some(Liveness::Gone) => "free",
        Some(Liveness::Live | Liveness::Stale) => "already-live",
    };
    tracing::Span::current().record("outcome", outcome);
    match taken? {
        Liveness::Gone => None,
        Liveness::Live => Some(refused_with(json, "already-live", || refuse_live(name))),
        Liveness::Stale => Some(refused_with(json, "stale-heartbeat", || {
            refuse_stale(name);
        })),
    }
}

/// The pinned copy of this exe's bytes, then `plugin/<key>/` rewritten whole; `None` when the
/// pinned copy failed its re-hash.
#[instrument(skip_all, name = "run.pin_copy", fields(outcome = tracing::field::Empty))]
fn pin_and_plugin(home: &Path, exe: &[u8]) -> anyhow::Result<Option<(Pinned, PathBuf)>> {
    let pinned = match pin_bytes(home, exe) {
        Ok(pinned) => pinned,
        Err(PinError::HashMismatch) => {
            tracing::Span::current().record("outcome", "pinned-hash-mismatch");
            return Ok(None);
        }
        Err(error) => return Err(error.into()),
    };
    tracing::Span::current().record("outcome", "ok");
    let plugin_dir = home.join("plugin").join(&pinned.key);
    for (rel, content) in viola_agent_claude::plugin_files(&pinned.path_fwd) {
        let path = plugin_dir.join(rel);
        create_private_dir(path.parent().unwrap_or(&plugin_dir))?;
        replace_private_shared(&path, content.as_bytes(), FILE_MODE)?;
    }
    Ok(Some((pinned, plugin_dir)))
}

/// What a bind came to.
enum Bound {
    Serving(String, Server),
    /// Another process holds the endpoint.
    Taken,
    /// The directory the endpoint lies in failed its check, or could not be made.
    SocketDirRefused,
}

/// The instance's endpoint, bound before the first snapshot so the child's first hook finds it
/// listening, its directory verified first. The exclusive bind is the arbiter of two starts of one
/// name.
fn bind_endpoint(name: &ViolaName, home: &Path) -> anyhow::Result<Bound> {
    let endpoint = viola_channel::endpoint_path(name, home)?;
    if !socket_dir_ready(&endpoint) {
        return Ok(Bound::SocketDirRefused);
    }
    match Server::bind(&endpoint) {
        Ok(server) => Ok(Bound::Serving(endpoint, server)),
        Err(ChannelError::BindTaken) => Ok(Bound::Taken),
        Err(error) => Err(error.into()),
    }
}

/// The endpoint's directory, verified before anything touches it: `create_private_dir` chmods an
/// existing path, which would erase what the check refuses, so it runs only for an absent one, and
/// the check runs again so the loser of a race to make it reads the same.
fn socket_dir_ready(endpoint: &str) -> bool {
    if check_endpoint_dir(endpoint).is_err() {
        return false;
    }
    let Some(dir) = Path::new(endpoint).parent() else {
        return false;
    };
    let absent = std::fs::symlink_metadata(dir).is_err();
    (!absent || create_private_dir(dir).is_ok()) && check_endpoint_dir(endpoint).is_ok()
}

/// What the first snapshot records of the launch, and the link set the log gave.
struct Recorded<'a> {
    spawn_dir: &'a Path,
    links: Vec<Value>,
    lineage: Lineage,
    launch: Pair,
    window: Option<Window>,
}

/// The first snapshot (with the endpoint, the version gate's reading, the restored link set, the
/// child's spawn directory when it is valid UTF-8, the launch's lineage and its pair), the
/// heartbeat and its thread, then the start's `wheel` event, its `lineage` event, its `launch`
/// event and, for an opened start, its `window` event; the returned guard keeps the heartbeat
/// running.
fn start_state(
    name: &ViolaName,
    instance_dir: &Path,
    snapshots: &Snapshots,
    pinned: &Pinned,
    endpoint: String,
    gate: Gate,
    recorded: Recorded<'_>,
) -> anyhow::Result<(Heartbeat, InstanceSnapshot)> {
    create_private_dir(instance_dir)?;
    let pid = std::process::id();
    let snapshot = InstanceSnapshot {
        endpoint: Some(endpoint),
        pid,
        started_at: own_start(pid).context("own start time unreadable")?,
        pinned_bin: pinned.path_fwd.clone(),
        cli_verified: gate.cli_verified,
        cli_version: gate.cli_version,
        wheel: Wheel::Driver,
        links: recorded.links,
        child_pid: None,
        pending_dialog: None,
        cwd: recorded.spawn_dir.to_str().map(str::to_owned),
        agent_session_id: None,
        lineage: Some(recorded.lineage.clone()),
        launch: Some(recorded.launch.clone()),
        window: recorded.window.clone(),
    };
    snapshots.init(snapshot.clone())?;
    touch_heartbeat(instance_dir)?;
    let beat = Heartbeat::start(instance_dir.to_path_buf());
    let data = json!({"holder": Wheel::Driver.as_str(), "cause": WheelCause::Start.as_str()});
    let line = EventLine::new(name, EventKind::Wheel, Source::Wrapper, data, Utc::now());
    append_event(instance_dir, &line)?;
    let Lineage { parent, role } = recorded.lineage;
    let data = json!({"parent": parent, "role": role});
    let line = EventLine::new(name, EventKind::Lineage, Source::Wrapper, data, Utc::now());
    append_event(instance_dir, &line)?;
    let Pair { model, effort } = recorded.launch;
    let data = json!({"model": model, "effort": effort});
    let line = EventLine::new(name, EventKind::Launch, Source::Wrapper, data, Utc::now());
    append_event(instance_dir, &line)?;
    if let Some(Window {
        address,
        workspace,
        placement,
        origin,
    }) = recorded.window
    {
        let data = json!({"address": address, "workspace": workspace, "placement": placement,
            "origin": origin});
        let line = EventLine::new(name, EventKind::Window, Source::Wrapper, data, Utc::now());
        append_event(instance_dir, &line)?;
    }
    Ok((beat, snapshot))
}

/// The child under a PTY, the host terminal raw first, then its pid in the snapshot.
fn spawn_child(
    program: PathBuf,
    cwd: PathBuf,
    launch: ChildLaunch,
    strip: &StripPlan,
    snapshots: &Snapshots,
    mut snapshot: InstanceSnapshot,
) -> anyhow::Result<(PortablePty, Option<HostTerminal>, Size)> {
    let spec = SpawnSpec {
        program,
        args: launch.args,
        cwd,
        env_set: launch.env_set,
        env_remove: strip.remove.clone(),
        size: viola_pty::host_size().unwrap_or(Size::DEFAULT),
    };
    // Raw before the spawn: every key the human types from here on reaches the child as typed.
    let terminal = HostTerminal::enter();
    let pty = viola_pty::spawn(&spec)?;
    let child_pid = viola_pty::Pty::child_pid(&pty);
    snapshot.child_pid = child_pid;
    snapshots.update(|s| s.child_pid = child_pid)?;
    run::log_child_start(
        snapshot.child_pid,
        strip,
        snapshot.cli_version.as_deref(),
        snapshot.cli_verified,
    );
    Ok((pty, terminal, spec.size))
}

pub(super) fn pump_child(launched: Launched) -> anyhow::Result<ExitCode> {
    match pump_end(launched)? {
        PumpEnd::Exited(_) => {
            run::log_self_exit(0, None);
            Ok(ExitCode::SUCCESS)
        }
        PumpEnd::WorkerPanicked(_) => {
            run::log_self_exit(1, Some("internal-error"));
            Ok(ExitCode::from(1))
        }
    }
}

/// `pump_child`'s pump with the child's exit logged, its own exit left to the caller. It returns
/// after the host terminal is restored.
pub(super) fn pump_end(launched: Launched) -> anyhow::Result<PumpEnd> {
    let Launched {
        mut pty,
        terminal,
        size,
        cli_verified,
        send,
        wheel,
        switch,
        _beat,
        _serving,
    } = launched;
    #[cfg(feature = "fake-agent")]
    hold_pump_start();
    let sigs = cli_verified.then_some(&viola_agent_claude::screen::SIGNATURES);
    let (feed, gate, _feed_thread) = gate::start(SystemClock, size, sigs);
    let paste = PasteHandle::default();
    let typed = paste.clone();
    send.attach(Box::new(move |text| typed.paste(text, PASTE_BOUND)), gate);
    let respawn = RespawnHandle::default();
    let respawning = respawn.clone();
    switch.respawn_with(Box::new(move |spec, between| {
        respawning.respawn(spec, between)
    }));
    let sizes = feed.clone();
    let mut host_size = move || {
        let size = viola_pty::host_size();
        if let Some(size) = size {
            sizes.size(size);
        }
        size
    };
    let end = viola_pty::pump_with_paste(
        &mut pty,
        Box::new(Observed::new(viola_pty::host_stdin(), Arc::clone(&wheel))),
        Box::new(Tee::new(io::stdout(), feed)),
        size,
        &mut host_size,
        &paste,
        &respawn,
    );
    // Before the terminal leaves raw mode: a key pressed while the last moves are recorded is a
    // byte, never a signal that ends the wrapper mid-exit.
    wheel.flush();
    drop(terminal);
    let end = end?;
    let (PumpEnd::Exited(exit) | PumpEnd::WorkerPanicked(exit)) = &end;
    run::log_child_exit(*exit);
    Ok(end)
}

fn refuse_live(name: &ViolaName) {
    let name: &str = name.as_ref();
    human::refuse(&format!("{name} is already live"), "viola list");
}

fn refuse_stale(name: &ViolaName) {
    let name: &str = name.as_ref();
    human::refuse(
        &format!("{name} is still running but not answering"),
        &format!("viola list shows it as stale; stop that process before starting {name} again"),
    );
}

pub(super) fn refuse_tampered_pin() {
    human::refuse(
        "the pinned viola copy failed its integrity check",
        "the pinned copy was changed after it was written, so viola will not run it",
    );
}

/// The directory lies outside the home, so this is no state-file pair: its own, naming no path.
fn refuse_socket_dir(name: &ViolaName) {
    let name = name_str(name);
    human::refuse(
        &format!("the socket directory for {name} cannot be trusted"),
        &human::send_hint(name, "socket-dir").unwrap_or_default(),
    );
}

/// The bind loser cannot tell a racing viola from any other holder, so it names neither.
fn refuse_squatted(name: &ViolaName) {
    let name = name_str(name);
    human::refuse(
        &format!("the endpoint for {name} is held by another process"),
        "another process holds this name's endpoint; stop it or pick another name",
    );
}

#[cfg(test)]
#[path = "run/test_cases.rs"]
mod tests;
