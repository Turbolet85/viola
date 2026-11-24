use std::ffi::{OsStr, OsString};
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
use viola_agent_claude::trust::StartWord;
use viola_channel::{Call, ChannelError, Dispatch, ProtocolError, Server, Serving};
use viola_core::obs::{ObsEvent, ObsProcess};
use viola_core::{EventKind, SystemClock, TurnEndCause, ViolaName, WheelCause, obs_event};
use viola_pty::{HostTerminal, PasteHandle, PortablePty, PumpEnd, RespawnHandle, Size, SpawnSpec};
use viola_state::events::{EventLine, Source, append_event};
use viola_state::fs::{FILE_MODE, create_private_dir, replace_private_shared};
use viola_state::heartbeat::{Heartbeat, beat_age, touch_heartbeat};
use viola_state::liveness::{Liveness, STALE_AFTER, classify, own_start, same_process};
use viola_state::pin::{PinError, Pinned, pin_bytes, pin_key};
use viola_state::replay::{Recovered, read_snapshot_or_replay};
use viola_state::snapshot::{InstanceSnapshot, Lineage, Wheel, Window};
use viola_state::strict::check_endpoint_dir;
use viola_state::strict::{TreeRefused, check_home_tree};

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
pub(super) struct WindowAsk {
    pub(super) origin: String,
    pub(super) place: Option<i32>,
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
    trust_folder(cli_verified, &strip, &launch.spawn_dir);
    let window = launch
        .window
        .as_ref()
        .map(|ask| open_window(ask, &cwd, &strip));
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

/// The window step of a start `viola open` asked for, over the compositor seam: `hyprctl` looked up
/// on `PATH` (none reads `unsupported`, nothing spawned), the terminal the wrapper's parent, the
/// R8 strip's names taken out of each read's environment. Its bound never stops the start.
fn open_window(ask: &WindowAsk, cwd: &Path, strip: &StripPlan) -> Window {
    let hyprctl = run::resolve_program(OsStr::new(viola_hyprland::PROGRAM), cwd).ok();
    let mut read = |request: &viola_hyprland::Request| {
        viola_hyprland::read(hyprctl.as_deref(), request, &strip.remove).reply
    };
    run::window::window_step(
        &mut read,
        std::os::unix::process::parent_id(),
        ask.place,
        &ask.origin,
        run::window::WINDOW_WAIT,
    )
}

/// The child's folder trusted for the CLI before its spawn, on a verified CLI only: one
/// `trust-record` line with the step's closed word, no path, key or value. The word never refuses
/// the start; a folder left untrusted is the human's dialog.
fn trust_folder(cli_verified: bool, strip: &StripPlan, spawn_dir: &Path) {
    let word = trust_word(cli_verified, strip, spawn_dir);
    obs_event!(INFO, ObsEvent::TrustRecord, detail = word.as_str());
}

fn trust_word(cli_verified: bool, strip: &StripPlan, spawn_dir: &Path) -> StartWord {
    if !cli_verified {
        return StartWord::UnverifiedCli;
    }
    let file = match trust_record::trust_config_file(strip) {
        Ok(file) => file,
        Err(word) => return StartWord::of(word),
    };
    let Ok(cwd) = std::fs::canonicalize(spawn_dir) else {
        return StartWord::ConfigUnreadable;
    };
    let git_root = trust_record::git_root(&cwd);
    let user_home = std::env::home_dir().and_then(|home| std::fs::canonicalize(home).ok());
    trust_record::trust_for_start(&file, &cwd, git_root.as_deref(), user_home.as_deref())
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
    match end? {
        PumpEnd::Exited(exit) => {
            run::log_child_exit(exit);
            run::log_self_exit(0, None);
            Ok(ExitCode::SUCCESS)
        }
        PumpEnd::WorkerPanicked(exit) => {
            run::log_child_exit(exit);
            run::log_self_exit(1, Some("internal-error"));
            Ok(ExitCode::from(1))
        }
    }
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
mod tests {
    use super::*;
    use std::sync::Mutex;
    use tracing::span::{Attributes, Id, Record};
    use tracing_subscriber::layer::{Context, Layer, SubscriberExt as _};
    use tracing_subscriber::registry::LookupSpan;
    use viola_channel::test_support::JsonFields as Fields;
    use viola_pty::Pty as _;

    #[test]
    fn trust_word_on_an_unverified_cli_reads_no_file() {
        let strip = viola_agent_claude::plan_strip(std::iter::empty(), &[]);
        let gone = Path::new("/nonexistent/viola-trust-step");
        assert_eq!(trust_word(false, &strip, gone), StartWord::UnverifiedCli);
        assert_eq!(
            trust_word(true, &strip, gone),
            StartWord::ConfigUnreadable,
            "the verified arm reaches the folder"
        );
    }

    /// Every span the calling thread opens, in order: `{name, parent, fields}`, with values
    /// recorded after creation merged in.
    #[derive(Clone, Default)]
    struct Spans(Arc<Mutex<Vec<(u64, Value)>>>);

    impl<S: tracing::Subscriber + for<'a> LookupSpan<'a>> Layer<S> for Spans {
        fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
            let mut fields = Fields::default();
            attrs.record(&mut fields);
            let parent = ctx
                .span(id)
                .and_then(|s| s.parent())
                .map(|p| p.name().to_owned());
            let span =
                json!({"name": attrs.metadata().name(), "parent": parent, "fields": fields.0});
            self.0.lock().expect("spans").push((id.into_u64(), span));
        }

        fn on_record(&self, id: &Id, values: &Record<'_>, _: Context<'_, S>) {
            let mut fields = Fields::default();
            values.record(&mut fields);
            let mut spans = self.0.lock().expect("spans");
            if let Some((_, span)) = spans.iter_mut().rev().find(|(i, _)| *i == id.into_u64()) {
                for (key, value) in fields.0 {
                    span["fields"][key] = value;
                }
            }
        }
    }

    impl Spans {
        fn all(&self) -> Vec<Value> {
            let spans = self.0.lock().expect("spans");
            spans.iter().map(|(_, span)| span.clone()).collect()
        }
    }

    fn capture_spans<R>(f: impl FnOnce() -> R) -> (R, Spans) {
        let spans = Spans::default();
        let subscriber = tracing_subscriber::registry().with(spans.clone());
        (
            crate::test_support::with_scoped_default(subscriber, f),
            spans,
        )
    }

    fn methods(instance_dir: &Path) -> Methods {
        let name = ViolaName::try_new("builder".to_owned()).expect("valid");
        let wait = Arc::new(WaitFeed::new(SystemClock));
        let wheel = Arc::new(WheelSlot::default());
        let snapshots = Arc::new(Snapshots::new(instance_dir.to_path_buf()));
        let dialogs = DialogSlot::new(
            SystemClock,
            false,
            name.clone(),
            instance_dir.to_path_buf(),
            Arc::clone(&wait),
            Arc::clone(&wheel),
            Arc::clone(&snapshots),
        );
        let send = Arc::new(SendSlot::new(SystemClock, false, Arc::clone(&wheel)));
        let links = Arc::new(LinkSet::new(
            name.clone(),
            instance_dir.to_path_buf(),
            Arc::clone(&snapshots),
        ));
        let switch = Arc::new(SwitchSlot::new(
            name.clone(),
            instance_dir.to_path_buf(),
            Arc::clone(&send),
            Arc::clone(&wheel),
            snapshots,
            ChildPlan {
                program: PathBuf::from("/bin/true"),
                cwd: instance_dir.to_path_buf(),
                pinned: viola_state::pin::Pinned {
                    key: "k".to_owned(),
                    path: instance_dir.join("viola"),
                    path_fwd: "b".to_owned(),
                },
                plugin_dir: instance_dir.join("plugin"),
                path: None,
                strip: StripPlan::default(),
                cli_version: None,
                cli_verified: false,
            },
        ));
        Methods {
            name,
            instance_dir: instance_dir.to_path_buf(),
            send,
            wait,
            dialogs: Arc::new(dialogs),
            wheel,
            links,
            switch,
        }
    }

    fn events_in(instance_dir: &Path) -> Vec<Value> {
        std::fs::read_to_string(instance_dir.join("events.ndjson"))
            .unwrap_or_default()
            .lines()
            .map(|l| serde_json::from_str(l).expect("one JSON object per line"))
            .collect()
    }

    #[test]
    fn methods_answer_method_not_found_for_every_other_method() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let params = json!({"v": 1});
        assert_eq!(
            methods(tmp.path()).dispatch_call(&call("anything", &params)),
            Err(ProtocolError::MethodNotFound)
        );
        assert!(events_in(tmp.path()).is_empty());
    }

    fn call<'a>(method: &'a str, params: &'a Value) -> Call<'a> {
        Call {
            method,
            params,
            id: Some(1),
            conn: None,
            srv_conn: Some("srv-1"),
        }
    }

    /// `link` and `unlink` are answered by the link set: one line each, in that order.
    #[test]
    fn methods_route_link_and_unlink_to_the_link_set() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let methods = methods(tmp.path());
        let params = json!({"v": 1, "driver": "overseer", "driven": "builder"});
        let linked = methods
            .dispatch_call(&call("link", &params))
            .expect("linked");
        assert_eq!(linked["ok"]["already_linked"], false);
        assert_eq!(
            methods.dispatch_call(&call("unlink", &params)),
            Ok(json!({"ok": {}}))
        );
        let kinds: Vec<Value> = events_in(tmp.path())
            .into_iter()
            .map(|line| line["kind"].clone())
            .collect();
        assert_eq!(kinds, [json!("link"), json!("unlink")]);
    }

    /// `wait` and `last` are answered by the wait feed, and a `hook.event` line reaches it.
    #[test]
    fn methods_route_wait_and_last_to_the_wait_feed() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let methods = methods(tmp.path());
        let params = json!({"v": 1, "after": 0, "timeout_ms": 1});
        assert_eq!(
            methods.dispatch_call(&call("wait", &params)),
            Ok(json!({"ok": {"timed_out": true}}))
        );
        let none = json!({"ok": {"last_assistant_message": null, "ts": null}});
        assert_eq!(
            methods.dispatch_call(&call("last", &json!({"v": 1}))),
            Ok(none)
        );
        let event = json!({"v": 1, "event": {"kind": "turn-ended", "data": {"last_assistant_message": "done"}}});
        methods
            .dispatch_call(&call("hook.event", &event))
            .expect("appended");
        let last = methods
            .dispatch_call(&call("last", &json!({"v": 1})))
            .expect("answered");
        assert_eq!(last["ok"]["last_assistant_message"], "done");
        assert_eq!(last["ok"]["ts"], events_in(tmp.path())[0]["ts"]);
        let woken = methods
            .dispatch_call(&call("wait", &json!({"v": 1, "after": 0})))
            .expect("answered");
        assert_eq!(woken["ok"]["event"]["kind"], "turn-ended");
    }

    /// `hook.dialog` and `answer` are the dialog slot's: on an unverified CLI a dialog is logged and
    /// answered `null` at once, and an `answer` is refused `unverified-cli`.
    #[test]
    fn methods_route_hook_dialog_and_answer_to_the_dialog_slot() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let methods = methods(tmp.path());
        let dialog = json!({"v": 1, "kind": "permission", "hook_event": "permission-request",
            "data": {"tool": "Bash", "input": {}}});
        assert_eq!(
            methods.dispatch_call(&call("hook.dialog", &dialog)),
            Ok(json!({"ok": {"dialog_id": 1, "response": null}}))
        );
        assert_eq!(events_in(tmp.path())[0]["kind"], "permission");
        let answer = json!({"v": 1, "dialog_id": 1, "response": {"behavior": "allow"}});
        assert_eq!(
            methods.dispatch_call(&call("answer", &answer)),
            Ok(json!({"refusal": "unverified-cli", "detail": null}))
        );
        // Neither is a notification: without an id they are not dispatched as one.
        assert_eq!(
            methods.dispatch("hook.dialog", &dialog),
            Err(ProtocolError::MethodNotFound)
        );
    }

    /// Every line the calling thread emits, its fields as JSON.
    #[derive(Clone, Default)]
    struct Lines(Arc<Mutex<Vec<Value>>>);

    impl<S: tracing::Subscriber> Layer<S> for Lines {
        fn on_event(&self, event: &tracing::Event<'_>, _: Context<'_, S>) {
            let mut fields = Fields::default();
            event.record(&mut fields);
            self.0.lock().expect("lines").push(Value::Object(fields.0));
        }
    }

    fn capture_lines<R>(f: impl FnOnce() -> R) -> (R, Vec<Value>) {
        let lines = Lines::default();
        let subscriber = tracing_subscriber::registry().with(lines.clone());
        let out = crate::test_support::with_scoped_default(subscriber, f);
        let got = lines.0.lock().expect("lines").clone();
        (out, got)
    }

    fn cli_call<'a>(method: &'a str, params: &'a Value) -> Call<'a> {
        Call {
            method,
            params,
            id: Some(4),
            conn: Some("cli-1-2-3"),
            srv_conn: Some("srv-1"),
        }
    }

    /// `pause` takes the wheel for the human (`send` is then `human-typing` / `manual-pause`),
    /// a `release` carrying `budget` is refused with the wheel left, and a plain `release` hands it
    /// back.
    #[test]
    fn methods_pause_and_release_move_the_wheel() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let methods = methods(tmp.path());
        let pause = json!({"v": 1, "from": "overseer"});
        assert_eq!(
            methods.dispatch_call(&cli_call("pause", &pause)),
            Ok(json!({"ok": {"wheel": "human"}}))
        );
        let send = json!({"v": 1, "text": "hello"});
        assert_eq!(
            methods.dispatch_call(&cli_call("send", &send)),
            Ok(json!({"refusal": "human-typing", "detail": "manual-pause"}))
        );
        let budget = json!({"v": 1, "budget": true});
        assert_eq!(
            methods.dispatch_call(&cli_call("release", &budget)),
            Err(ProtocolError::InvalidParams)
        );
        assert_eq!(methods.wheel.holder(), Wheel::Human);
        let release = json!({"v": 1, "from": null});
        assert_eq!(
            methods.dispatch_call(&cli_call("release", &release)),
            Ok(json!({"ok": {"wheel": "driver"}}))
        );
        assert_eq!(methods.wheel.holder(), Wheel::Driver);
        assert_eq!(
            methods.dispatch_call(&cli_call("send", &send)),
            Ok(json!({"refusal": "not-delivered", "detail": "input-not-ready"}))
        );
    }

    #[rstest::rstest]
    #[case::pause_from_not_a_name("pause", json!({"v": 1, "from": "../x"}))]
    #[case::pause_from_not_string("pause", json!({"v": 1, "from": 3}))]
    #[case::release_from_not_string("release", json!({"v": 1, "from": 3}))]
    #[case::release_budget_string("release", json!({"v": 1, "budget": "yes"}))]
    #[case::release_budget_number("release", json!({"v": 1, "budget": 1}))]
    #[case::release_budget_null("release", json!({"v": 1, "budget": null}))]
    fn methods_wheel_params_they_cannot_take_are_invalid(
        #[case] method: &str,
        #[case] params: Value,
    ) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let methods = methods(tmp.path());
        assert_eq!(
            methods.dispatch_call(&cli_call(method, &params)),
            Err(ProtocolError::InvalidParams)
        );
        assert_eq!(methods.wheel.holder(), Wheel::Driver);
    }

    /// A `release` carrying a string `from` is a driver's: `-32602` `release-from-driver`, the
    /// wheel unmoved, and one `release-from-driver` line (its `from` only when it is a name). The
    /// refusal comes before `budget` is read: with `true`, and with a value that is no bool.
    #[rstest::rstest]
    #[case::a_name("overseer", Some("overseer"))]
    #[case::not_a_name("Not A Name", None)]
    fn methods_release_from_a_driver_is_refused_and_logged(
        #[case] from: &str,
        #[case] logged: Option<&str>,
        #[values(json!(true), json!("ignored"))] budget: Value,
    ) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let methods = methods(tmp.path());
        let none = json!({"v": 1});
        methods
            .dispatch_call(&cli_call("pause", &none))
            .expect("paused");
        let params = json!({"v": 1, "from": from, "budget": budget});
        let (reply, lines) = capture_lines(|| methods.dispatch_call(&cli_call("release", &params)));
        assert_eq!(reply, Err(ProtocolError::ReleaseFromDriver));
        assert_eq!(methods.wheel.holder(), Wheel::Human);
        let refused: Vec<&Value> = lines
            .iter()
            .filter(|l| l["event"] == "release-from-driver")
            .collect();
        assert_eq!(refused.len(), 1, "{lines:?}");
        let mut want = json!({"event": "release-from-driver", "message": "release-from-driver",
            "corr": 4, "conn": "cli-1-2-3", "from_trust": "self-reported"});
        if let Some(name) = logged {
            want["from"] = json!(name);
        }
        let mut got = refused[0].clone();
        for absent in ["process", "instance"] {
            got.as_object_mut().expect("object").remove(absent);
        }
        assert_eq!(got, want);
    }

    /// A valid `hook.event` becomes one `source:"hook"` line stamped by the wrapper.
    #[test]
    fn methods_append_a_hook_event() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let params = json!({"v": 1, "sender": "0.1.0", "ts": "t",
            "event": {"kind": "turn-ended", "data": {"last_assistant_message": null}}});
        assert_eq!(
            methods(tmp.path()).dispatch("hook.event", &params),
            Ok(Value::Null)
        );
        let lines = events_in(tmp.path());
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0]["v"], 1);
        assert_eq!(lines[0]["instance"], "builder");
        assert_eq!(lines[0]["kind"], "turn-ended");
        assert_eq!(lines[0]["source"], "hook");
        assert_eq!(lines[0]["data"], json!({"last_assistant_message": null}));
        let ts = lines[0]["ts"].as_str().expect("ts");
        assert!(ts.ends_with('Z') && ts.len() == 24, "{ts}");
    }

    #[rstest::rstest]
    #[case::session_start(json!({"kind": "session-start", "data": {"cause": "startup", "agent_session_id": null}}))]
    #[case::prompt(json!({"kind": "prompt-submitted", "data": {"text": "hi", "origin": "human"}}))]
    #[case::harness(json!({"kind": "prompt-submitted", "data": {"text": "", "origin": "harness"}}))]
    #[case::session_end(json!({"kind": "session-end", "data": {}}))]
    #[case::activity(json!({"kind": "activity", "data": {"tool": "Bash"}}))]
    fn methods_take_every_hook_kind(#[case] event: Value) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let kind = event["kind"].clone();
        methods(tmp.path())
            .dispatch("hook.event", &json!({"v": 1, "event": event}))
            .expect("dispatched");
        let lines = events_in(tmp.path());
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0]["kind"], kind);
    }

    #[rstest::rstest]
    #[case::no_event(json!({"v": 1}))]
    #[case::wrapper_kind(json!({"v": 1, "event": {"kind": "wheel", "data": {}}}))]
    #[case::unknown_kind(json!({"v": 1, "event": {"kind": "question", "data": {}}}))]
    #[case::no_data(json!({"v": 1, "event": {"kind": "session-end"}}))]
    #[case::data_not_object(json!({"v": 1, "event": {"kind": "session-end", "data": [1]}}))]
    #[case::driver_origin(json!({"v": 1, "event": {"kind": "prompt-submitted", "data": {"text": "x", "origin": "driver"}}}))]
    #[case::no_origin(json!({"v": 1, "event": {"kind": "prompt-submitted", "data": {"text": "x"}}}))]
    #[case::text_not_string(json!({"v": 1, "event": {"kind": "prompt-submitted", "data": {"text": 1, "origin": "human"}}}))]
    fn methods_refuse_an_event_that_fails_the_checks(#[case] params: Value) {
        let tmp = tempfile::tempdir().expect("tempdir");
        assert_eq!(
            methods(tmp.path()).dispatch("hook.event", &params),
            Ok(Value::Null)
        );
        assert!(!tmp.path().join("events.ndjson").exists());
    }

    /// A `turn-ended` is appended only with its `cause` absent or a closed `TurnEndCause` word.
    #[rstest::rstest]
    #[case::rate_limited(Some(json!("rate-limited")), true)]
    #[case::overloaded(Some(json!("overloaded")), true)]
    #[case::auth_failed(Some(json!("auth-failed")), true)]
    #[case::billing(Some(json!("billing")), true)]
    #[case::invalid_request(Some(json!("invalid-request")), true)]
    #[case::model_not_found(Some(json!("model-not-found")), true)]
    #[case::server_error(Some(json!("server-error")), true)]
    #[case::output_limit(Some(json!("output-limit")), true)]
    #[case::api_error(Some(json!("api-error")), true)]
    #[case::absent(None, true)]
    #[case::raw_server_error(Some(json!("server_error")), false)]
    #[case::raw_rate_limit(Some(json!("rate_limit")), false)]
    #[case::a_number(Some(json!(7)), false)]
    #[case::null(Some(Value::Null), false)]
    #[case::error_text(Some(json!("API Error: Connection lost mid-response")), false)]
    fn run_hook_event_turn_ended_with_a_cause_outside_the_closed_set_is_not_appended(
        #[case] cause: Option<Value>,
        #[case] appended: bool,
    ) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let mut data = json!({"last_assistant_message": null, "background_tasks": null});
        if let Some(cause) = cause {
            data["cause"] = cause;
        }
        let params = json!({"v": 1, "event": {"kind": "turn-ended", "data": data.clone()}});
        assert_eq!(
            methods(tmp.path()).dispatch("hook.event", &params),
            Ok(Value::Null)
        );
        if appended {
            let lines = events_in(tmp.path());
            assert_eq!(lines.len(), 1);
            assert_eq!(lines[0]["kind"], "turn-ended");
            assert_eq!(lines[0]["data"], data);
        } else {
            assert!(!tmp.path().join("events.ndjson").exists());
        }
    }

    /// A `transcript` beside the event is taken only for a `session-start`, only as a string passing
    /// `transcript_path_ok` against the line's own id; the line is taken either way and its data
    /// never holds the path.
    #[rstest::rstest]
    #[case::its_own_shape("session-start", json!("/t/SID.jsonl"), true)]
    #[case::relative("session-start", json!("t/SID.jsonl"), false)]
    #[case::a_parent_component("session-start", json!("/t/../SID.jsonl"), false)]
    #[case::another_session_s_file("session-start", json!("/t/1b1c2d3e-4f50-6172-8394-a5b6c7d8e9f0.jsonl"), false)]
    #[case::not_a_string("session-start", json!(7), false)]
    #[case::on_another_kind("session-end", json!("/t/SID.jsonl"), false)]
    fn run_hook_event_transcript_is_re_validated(
        #[case] kind: &str,
        #[case] transcript: Value,
        #[case] taken: bool,
    ) {
        const SID: &str = "0b1c2d3e-4f50-6172-8394-a5b6c7d8e9f0";
        let tmp = tempfile::tempdir().expect("tempdir");
        let transcript = match transcript.as_str() {
            Some(path) => json!(path.replace("SID", SID)),
            None => transcript,
        };
        let data = json!({"cause": "startup", "agent_session_id": SID});
        let params =
            json!({"v": 1, "event": {"kind": kind, "data": data}, "transcript": transcript});
        let (line, kept) = methods(tmp.path())
            .hook_event_line(&params)
            .expect("the line is taken");
        assert_eq!(line.data, data);
        let want = taken.then(|| PathBuf::from(transcript.as_str().expect("a path")));
        assert_eq!(kept, want);
    }

    #[test]
    fn methods_report_an_append_that_failed_as_internal() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let params = json!({"v": 1, "event": {"kind": "session-end", "data": {}}});
        assert_eq!(
            methods(&tmp.path().join("missing")).dispatch("hook.event", &params),
            Err(ProtocolError::Internal)
        );
    }

    /// Every event line the calling thread emits, its fields as JSON.
    #[derive(Clone, Default)]
    struct Events(Arc<Mutex<Vec<Value>>>);

    impl<S: tracing::Subscriber> Layer<S> for Events {
        fn on_event(&self, event: &tracing::Event<'_>, _: Context<'_, S>) {
            let mut fields = Fields::default();
            event.record(&mut fields);
            self.0.lock().expect("events").push(Value::Object(fields.0));
        }
    }

    /// A live wrapper's first snapshot, cut short or re-versioned to `v` 99 from the writer's own
    /// bytes, its beat aged as given: the check takes the replay, which records no pid, so the beat
    /// alone decides — over a stale beat the name is free, over a fresh one it is `already-live` —
    /// and the replay writes its one `state-recovered` line.
    #[rstest::rstest]
    #[case::cut_short_stale("cut", 10, "snapshot-replayed", true)]
    #[case::cut_short_fresh("cut", 0, "snapshot-replayed", false)]
    #[case::unsupported_v_stale("v99", 10, "snapshot-unsupported-v", true)]
    #[case::unsupported_v_fresh("v99", 0, "snapshot-unsupported-v", false)]
    fn collision_check_over_a_replayed_snapshot_reads_the_beat_alone(
        #[case] fault: &str,
        #[case] beat_secs: u64,
        #[case] detail: &str,
        #[case] free: bool,
    ) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let dir = tmp.path().join("instances").join("builder");
        create_private_dir(&dir).expect("instance dir");
        first_snapshot(&dir, tmp.path());
        let path = dir.join(viola_state::snapshot::SNAPSHOT);
        let bytes = std::fs::read(&path).expect("bytes");
        let faulted = if fault == "cut" {
            bytes[..bytes.len() / 2].to_vec()
        } else {
            String::from_utf8(bytes)
                .expect("utf-8")
                .replacen("\"v\":1", "\"v\":99", 1)
                .into_bytes()
        };
        std::fs::write(&path, faulted).expect("faulted");
        std::fs::OpenOptions::new()
            .append(true)
            .open(dir.join("heartbeat"))
            .expect("the beat")
            .set_modified(SystemTime::now() - std::time::Duration::from_secs(beat_secs))
            .expect("mtime");
        let events = Events::default();
        let subscriber = tracing_subscriber::registry().with(events.clone());
        let name = ViolaName::try_new("builder".to_owned()).expect("valid");
        let taken = crate::test_support::with_scoped_default(subscriber, || {
            let recovered = read_snapshot_or_replay(&dir).ok();
            collision_check(&name, &dir, recovered.as_ref(), true).is_some()
        });
        assert_eq!(taken, !free);
        let lines = events.0.lock().expect("events").clone();
        let recovered: Vec<&Value> = lines
            .iter()
            .filter(|l| l["event"] == "state-recovered")
            .collect();
        assert_eq!(recovered.len(), 1, "{lines:?}");
        assert_eq!(recovered[0]["detail"], detail);
    }

    /// The first snapshot `start_state` writes for a child spawned in `spawn_dir`, read back from
    /// the instance directory.
    fn first_snapshot(instance_dir: &Path, spawn_dir: &Path) -> InstanceSnapshot {
        first_snapshot_of(instance_dir, spawn_dir, Lineage::default())
    }

    fn first_snapshot_of(
        instance_dir: &Path,
        spawn_dir: &Path,
        lineage: Lineage,
    ) -> InstanceSnapshot {
        first_snapshot_with(instance_dir, spawn_dir, lineage, run::launch::compiled())
    }

    fn first_snapshot_with(
        instance_dir: &Path,
        spawn_dir: &Path,
        lineage: Lineage,
        launch: Pair,
    ) -> InstanceSnapshot {
        first_snapshot_opened(instance_dir, spawn_dir, lineage, launch, None)
    }

    fn first_snapshot_opened(
        instance_dir: &Path,
        spawn_dir: &Path,
        lineage: Lineage,
        launch: Pair,
        window: Option<Window>,
    ) -> InstanceSnapshot {
        let name = ViolaName::try_new("builder".to_owned()).expect("valid");
        let pinned = Pinned {
            key: "k".to_owned(),
            path: PathBuf::from("bin").join("k").join("viola"),
            path_fwd: "bin/k/viola".to_owned(),
        };
        let gate = Gate::default();
        let snapshots = Arc::new(Snapshots::new(instance_dir.to_path_buf()));
        let links = LinkSet::new(
            name.clone(),
            instance_dir.to_path_buf(),
            Arc::clone(&snapshots),
        );
        links.restore().expect("the link set");
        let (_beat, returned) = start_state(
            &name,
            instance_dir,
            &snapshots,
            &pinned,
            "endpoint".to_owned(),
            gate,
            Recorded {
                spawn_dir,
                links: links.current(),
                lineage,
                launch,
                window,
            },
        )
        .expect("the start state");
        assert_eq!(
            viola_state::snapshot::read_snapshot(instance_dir),
            Some(returned.clone())
        );
        returned
    }

    /// A start over a log that holds a link: the first snapshot holds that pair with its first
    /// `since`, and the start appends its one record and no `link`.
    #[test]
    fn links_set_at_a_start_over_a_log_with_a_link_is_in_the_first_snapshot() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let instance_dir = tmp.path().join("instance");
        create_private_dir(&instance_dir).expect("the instance dir");
        let name = ViolaName::try_new("builder".to_owned()).expect("valid");
        let linked = EventLine::new(
            &name,
            EventKind::Link,
            Source::Wrapper,
            json!({"driver": "overseer", "driven": "builder"}),
            Utc::now(),
        );
        append_event(&instance_dir, &linked).expect("append");

        let snapshot = first_snapshot(&instance_dir, tmp.path());
        assert_eq!(
            snapshot.links,
            [json!({"driver": "overseer", "driven": "builder", "since": linked.ts})]
        );
        let kinds: Vec<Value> = events_in(&instance_dir)
            .iter()
            .map(|e| e["kind"].clone())
            .collect();
        assert_eq!(kinds, ["link", "wheel", "lineage", "launch"]);
    }

    /// The first snapshot records where the child is spawned, and the start event stands as it
    /// was: the wheel starts with the driver.
    #[test]
    fn snapshot_cwd_of_a_start_is_the_spawn_directory() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let instance_dir = tmp.path().join("instance");
        let spawn_dir = tmp.path().join("work dir");
        let snapshot = first_snapshot(&instance_dir, &spawn_dir);
        assert_eq!(
            snapshot.cwd,
            Some(spawn_dir.to_str().expect("utf-8").to_owned())
        );
        let events = events_in(&instance_dir);
        let read: Vec<(&Value, &Value)> = events
            .iter()
            .filter(|e| e["kind"] != "launch")
            .map(|e| (&e["kind"], &e["data"]))
            .collect();
        assert_eq!(
            read,
            [
                (
                    &json!("wheel"),
                    &json!({"holder": "driver", "cause": "start"})
                ),
                (&json!("lineage"), &json!({"parent": null, "role": null})),
            ]
        );
    }

    /// The snapshot holds the lineage before the start's lines; the `lineage` line, `source`
    /// `wrapper`, follows the `wheel` start line directly.
    #[test]
    fn start_state_writes_the_lineage_line_after_the_wheel_line() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let instance_dir = tmp.path().join("instance");
        let lineage = Lineage {
            parent: Some("overseer".to_owned()),
            role: Some("builder".to_owned()),
        };
        let snapshot = first_snapshot_of(&instance_dir, tmp.path(), lineage.clone());
        assert_eq!(snapshot.lineage, Some(lineage));
        let events = events_in(&instance_dir);
        let read: Vec<(&Value, &Value, &Value)> = events
            .iter()
            .filter(|e| e["kind"] != "launch")
            .map(|e| (&e["kind"], &e["source"], &e["data"]))
            .collect();
        assert_eq!(
            read,
            [
                (
                    &json!("wheel"),
                    &json!("wrapper"),
                    &json!({"holder": "driver", "cause": "start"})
                ),
                (
                    &json!("lineage"),
                    &json!("wrapper"),
                    &json!({"parent": "overseer", "role": "builder"})
                ),
            ]
        );
    }

    /// The snapshot holds the pair before the start's lines; the `launch` line, `source`
    /// `wrapper`, follows the `lineage` line directly and names the pair handed.
    #[test]
    fn start_state_writes_the_launch_line_after_the_lineage_line() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let instance_dir = tmp.path().join("instance");
        let pair = Pair {
            model: "haiku".to_owned(),
            effort: "low".to_owned(),
        };
        let snapshot =
            first_snapshot_with(&instance_dir, tmp.path(), Lineage::default(), pair.clone());
        assert_eq!(snapshot.launch, Some(pair));
        let events = events_in(&instance_dir);
        let read: Vec<(&Value, &Value)> =
            events.iter().map(|e| (&e["kind"], &e["source"])).collect();
        assert_eq!(
            read,
            [
                (&json!("wheel"), &json!("wrapper")),
                (&json!("lineage"), &json!("wrapper")),
                (&json!("launch"), &json!("wrapper")),
            ]
        );
        assert_eq!(
            events[2]["data"],
            json!({"model": "haiku", "effort": "low"})
        );
    }

    /// An opened start: the snapshot holds the window before the start's lines, and the `window`
    /// line, `source` `wrapper`, follows the `launch` line directly.
    #[test]
    fn start_state_writes_the_window_line_after_the_launch_line() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let instance_dir = tmp.path().join("instance");
        let window = Window {
            address: Some("0x555aa16bde50".to_owned()),
            workspace: Some(1),
            placement: "placed".to_owned(),
            origin: "cli".to_owned(),
        };
        let snapshot = first_snapshot_opened(
            &instance_dir,
            tmp.path(),
            Lineage::default(),
            run::launch::compiled(),
            Some(window.clone()),
        );
        assert_eq!(snapshot.window, Some(window));
        let events = events_in(&instance_dir);
        let read: Vec<(&Value, &Value)> =
            events.iter().map(|e| (&e["kind"], &e["source"])).collect();
        assert_eq!(
            read,
            [
                (&json!("wheel"), &json!("wrapper")),
                (&json!("lineage"), &json!("wrapper")),
                (&json!("launch"), &json!("wrapper")),
                (&json!("window"), &json!("wrapper")),
            ]
        );
        assert_eq!(
            events[3]["data"],
            json!({"address": "0x555aa16bde50", "workspace": 1, "placement": "placed",
                "origin": "cli"})
        );
    }

    #[test]
    fn run_args_refuse_a_place_without_an_origin() {
        assert_eq!(
            parse(&["builder", "--place", "1", "--", "claude"]).err(),
            Some(clap::error::ErrorKind::MissingRequiredArgument)
        );
        assert_eq!(
            parse(&["builder", "--origin", "human", "--", "claude"]).err(),
            Some(clap::error::ErrorKind::ValueValidation)
        );
        assert_eq!(
            parse(&["builder", "--origin", "cli", "--place", "x", "--", "claude"]).err(),
            Some(clap::error::ErrorKind::ValueValidation)
        );
    }

    #[test]
    fn run_args_take_a_hidden_origin_and_place() {
        let args = parse(&[
            "builder", "--origin", "agent", "--place", "3", "--", "claude",
        ])
        .expect("args");
        assert_eq!(
            (args.origin.as_deref(), args.place),
            (Some("agent"), Some(3))
        );
        let args = parse(&["builder", "--origin", "cli", "--", "claude"]).expect("args");
        assert_eq!((args.origin.as_deref(), args.place), (Some("cli"), None));
        let args = parse(&["builder", "--", "claude"]).expect("args");
        assert_eq!((args.origin, args.place), (None, None));
        use clap::CommandFactory as _;
        let help = Line::command().render_help().to_string();
        assert!(
            !help.contains("--origin") && !help.contains("--place"),
            "{help}"
        );
    }

    #[derive(clap::Parser)]
    struct Line {
        #[command(flatten)]
        args: RunArgs,
    }

    fn parse(words: &[&str]) -> Result<RunArgs, clap::error::ErrorKind> {
        use clap::Parser as _;
        Line::try_parse_from(std::iter::once("run").chain(words.iter().copied()))
            .map_err(|error| error.kind())
            .and_then(|line| checked(line.args).map_err(|error| error.kind()))
    }

    fn named(text: &str) -> ViolaName {
        ViolaName::try_new(text.to_owned()).expect("valid")
    }

    #[rstest::rstest]
    #[case::both(&["builder", "--parent", "overseer", "--role", "builder", "--", "claude"], Ok((Some("overseer"), Some("builder"))))]
    #[case::parent_alone(&["builder", "--parent", "overseer", "--", "claude"], Ok((Some("overseer"), None)))]
    #[case::neither(&["builder", "--", "claude"], Ok((None, None)))]
    #[case::bad_name(&["builder", "--parent", "Over Seer", "--", "claude"], Err(clap::error::ErrorKind::ValueValidation))]
    #[case::bad_role(&["builder", "--role", "Builder!", "--", "claude"], Err(clap::error::ErrorKind::ValueValidation))]
    #[case::self_parent(&["builder", "--parent", "builder", "--", "claude"], Err(clap::error::ErrorKind::ArgumentConflict))]
    fn run_args_take_parent_and_role(
        #[case] words: &[&str],
        #[case] expected: Result<(Option<&str>, Option<&str>), clap::error::ErrorKind>,
    ) {
        let parsed =
            parse(words).map(|args| (args.parent.map(|name| name.as_ref().to_owned()), args.role));
        let expected =
            expected.map(|(parent, role)| (parent.map(str::to_owned), role.map(str::to_owned)));
        assert_eq!(parsed, expected);
        if let Err(kind) = parsed {
            let error = clap::Error::new(kind);
            assert_eq!(error.exit_code(), 2, "{kind:?}");
        }
    }

    #[rstest::rstest]
    #[case::both(&["builder", "--model", "haiku", "--effort", "low", "--", "claude"], Ok((Some("haiku"), Some("low"))))]
    #[case::model_alone(&["builder", "--model", "claude-opus-5-5", "--", "claude"], Ok((Some("claude-opus-5-5"), None)))]
    #[case::effort_alone(&["builder", "--effort", "xhigh", "--", "claude"], Ok((None, Some("xhigh"))))]
    #[case::neither(&["builder", "--", "claude"], Ok((None, None)))]
    #[case::bad_effort(&["builder", "--effort", "huge", "--", "claude"], Err(clap::error::ErrorKind::ValueValidation))]
    #[case::bad_model(&["builder", "--model", "Opus", "--", "claude"], Err(clap::error::ErrorKind::ValueValidation))]
    fn run_args_take_model_and_effort(
        #[case] words: &[&str],
        #[case] expected: Result<(Option<&str>, Option<&str>), clap::error::ErrorKind>,
    ) {
        let parsed = parse(words).map(|args| (args.model, args.effort));
        let expected =
            expected.map(|(model, effort)| (model.map(str::to_owned), effort.map(str::to_owned)));
        assert_eq!(parsed, expected);
        if let Err(kind) = parsed {
            assert_eq!(clap::Error::new(kind).exit_code(), 2, "{kind:?}");
        }
    }

    #[rstest::rstest]
    #[case::spaced(&["builder", "--", "claude", "--model", "x"], false)]
    #[case::effort_equals(&["builder", "--", "claude", "--effort=low"], false)]
    #[case::model_equals(&["builder", "--model", "haiku", "--", "claude", "--model=x"], false)]
    #[case::a_longer_word(&["builder", "--", "claude", "--modelx"], true)]
    fn run_args_refuse_a_pair_in_the_callers_words(#[case] words: &[&str], #[case] passes: bool) {
        assert_eq!(parse(words).is_ok(), passes, "{words:?}");
        if !passes {
            assert_eq!(
                parse(words).err(),
                Some(clap::error::ErrorKind::ArgumentConflict)
            );
            let line = pair_after_dashes().to_string();
            assert!(line.contains("'--model <MODEL>'") && line.contains("'--effort <EFFORT>'"));
            assert!(
                !line.contains("--model=x") && !line.contains("claude"),
                "{line}"
            );
        }
    }

    #[test]
    fn run_lineage_takes_the_flag_over_the_inherited_name() {
        let args = parse(&["builder", "--parent", "overseer", "--", "claude"]).expect("args");
        assert_eq!(
            lineage_of(&args, Some(named("tester"))),
            Lineage {
                parent: Some("overseer".to_owned()),
                role: None,
            }
        );
    }

    #[test]
    fn run_lineage_an_inherited_name_is_the_parent_when_no_flag() {
        let args = parse(&["builder", "--role", "builder", "--", "claude"]).expect("args");
        assert_eq!(
            lineage_of(&args, Some(named("overseer"))),
            Lineage {
                parent: Some("overseer".to_owned()),
                role: Some("builder".to_owned()),
            }
        );
        assert_eq!(lineage_of(&args, None).parent, None);
    }

    /// A `viola run builder` typed inside `builder`'s own session: the inherited name is dropped,
    /// never refused.
    #[test]
    fn run_lineage_an_inherited_own_name_is_no_parent() {
        let args = parse(&["builder", "--", "claude"]).expect("args");
        assert_eq!(
            lineage_of(&args, Some(named("builder"))),
            Lineage::default()
        );
    }

    #[cfg(unix)]
    #[test]
    fn snapshot_cwd_of_a_start_in_a_directory_that_is_not_utf8_is_absent() {
        use std::os::unix::ffi::OsStrExt as _;
        let tmp = tempfile::tempdir().expect("tempdir");
        let spawn_dir = tmp.path().join(std::ffi::OsStr::from_bytes(b"work-\xff"));
        let snapshot = first_snapshot(&tmp.path().join("instance"), &spawn_dir);
        assert_eq!(snapshot.cwd, None);
    }

    fn field<'a>(spans: &'a [Value], name: &str) -> &'a Value {
        spans
            .iter()
            .find(|s| s["name"] == name)
            .map(|s| &s["fields"])
            .unwrap_or_else(|| panic!("no span {name} in {spans:?}"))
    }

    /// One in-process start through the spawn (obs-plan §4 Scenario 1): every step's span, in the
    /// documented order, under `run.start`, with its required fields; then the seam's own spans.
    /// The home is outside `target/e2e-home`: its pinned copy is this test binary, not `viola`.
    /// The child is a host program, never this test binary. The wrapper's plugin flag comes first
    /// in a child's arguments, libtest refuses it and exits by itself, and under coverage a kill
    /// that lands in that exit's profile write leaves a profile `llvm-profdata` refuses. `whoami`
    /// refuses the flag too, and writes no profile (test-plan §10 Zero-flakiness budget).
    #[test]
    fn start_opens_the_scenario_one_spans_under_run_start() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let launch = Launch {
            name: ViolaName::try_new("builder".to_owned()).expect("valid"),
            program: OsString::from("whoami"),
            args: Vec::new(),
            spawn_dir: std::env::current_dir().expect("cwd"),
            json: false,
            lineage: Lineage::default(),
            given: (None, None),
            window: None,
        };
        let exe = std::fs::read(std::env::current_exe().expect("exe")).expect("exe bytes");
        let (started, spans) =
            capture_spans(|| start(&tmp.path().join("home"), &launch, &[], &exe));
        let Started::Launched(mut launched) = started.expect("started") else {
            panic!("the start was refused");
        };
        let (_, seam) = capture_spans(|| {
            let _ = launched.pty.resize(Size { cols: 90, rows: 30 });
            let _ = launched.pty.kill();
        });
        drop(launched);

        let spans = spans.all();
        let names: Vec<&str> = spans.iter().filter_map(|s| s["name"].as_str()).collect();
        assert_eq!(
            names,
            [
                "run.start",
                "state.snapshot_recover",
                "run.collision_check",
                "run.pin_copy",
                "run.version_gate",
                "channel.bind",
                "state.snapshot_write",
                "state.heartbeat_start",
                "pty.spawn",
                "state.snapshot_write",
            ]
        );
        assert!(spans[0]["parent"].is_null());
        assert!(
            spans[1..].iter().all(|s| s["parent"] == "run.start"),
            "{spans:?}"
        );
        assert_eq!(field(&spans, "run.start")["instance"], "builder");
        assert_eq!(field(&spans, "run.collision_check")["outcome"], "free");
        assert_eq!(field(&spans, "run.pin_copy")["outcome"], "ok");
        assert_eq!(field(&spans, "run.version_gate")["cli_verified"], false);
        assert!(
            field(&spans, "run.version_gate")
                .get("cli_version")
                .is_none()
        );
        assert_eq!(
            field(&spans, "channel.bind")["endpoint_kind"],
            viola_channel::ENDPOINT_KIND
        );
        assert_eq!(field(&spans, "state.snapshot_write")["v"], 1);
        let spawn = field(&spans, "pty.spawn");
        assert_eq!(spawn["pty_backend"], viola_pty::pty_backend());
        assert!(spawn["env_stripped_count"].is_u64(), "{spawn}");

        let seam: Vec<Value> = seam.all();
        let names: Vec<&str> = seam.iter().filter_map(|s| s["name"].as_str()).collect();
        assert_eq!(names, ["pty.resize", "pty.kill"]);
    }
}
