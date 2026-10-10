use std::ffi::OsString;
use std::fs::File;
use std::io::{self, Read as _};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::Arc;
use std::time::SystemTime;

use anyhow::Context as _;
use chrono::Utc;
use serde_json::{Value, json};
use tracing::instrument;
use viola_agent_claude::{Refusal, StripPlan, statusline};
use viola_channel::{Call, ChannelError, Dispatch, ProtocolError, Server, Serving};
use viola_core::obs::ObsProcess;
use viola_core::{EventKind, MAX_FRAME, SystemClock, ViolaName, WheelCause};
use viola_pty::{HostTerminal, PasteHandle, PortablePty, PumpEnd, Size, SpawnSpec};
use viola_state::events::{EventLine, Source, append_event};
use viola_state::fs::{FILE_MODE, create_private_dir, replace_private, replace_private_shared};
use viola_state::heartbeat::{Heartbeat, beat_age, touch_heartbeat};
use viola_state::liveness::{Liveness, classify, own_start, same_process};
use viola_state::pin::{PinError, Pinned, pin_exe};
use viola_state::snapshot::{InstanceSnapshot, Wheel, read_snapshot};

use crate::run::ChildLaunch;
use crate::run::dialog::DialogSlot;
use crate::run::gate::{self, Tee};
use crate::run::send::{self, SendSlot};
use crate::run::snapshot::Snapshots;
use crate::run::version_gate::{Gate, version_gate};
use crate::run::wait::WaitFeed;
use crate::run::wheel::{Observed, Recorder, WheelSlot};
use crate::{human, obs, run};

#[derive(clap::Args)]
pub(crate) struct RunArgs {
    /// Instance name: [a-z0-9-], 1-32 characters, starting with a letter
    #[arg(value_parser = parse_name)]
    pub(super) name: ViolaName,
    /// The program to wrap and its arguments, after `--`
    #[arg(last = true, required = true)]
    program: Vec<OsString>,
}

fn parse_name(raw: &str) -> Result<ViolaName, String> {
    ViolaName::try_new(raw.to_owned()).map_err(|_| "invalid instance name".to_owned())
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

/// The wrapper's answers: `send`, `wait`, `last`, `hook.dialog`, `answer`, `pause`, `release`, and
/// the `hook.event` notification, which appends its event; every other method is `-32601` until its
/// chunk lands.
pub(crate) struct Methods {
    name: ViolaName,
    instance_dir: PathBuf,
    send: Arc<SendSlot>,
    wait: Arc<WaitFeed>,
    dialogs: Arc<DialogSlot>,
    wheel: Arc<WheelSlot>,
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
    /// kind, an object `data`, and for `prompt-submitted` a string `text` and a closed `origin`.
    fn hook_event_line(&self, params: &Value) -> Option<EventLine> {
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
        Some(EventLine::new(
            &self.name,
            kind,
            Source::Hook,
            data.clone(),
            Utc::now(),
        ))
    }
}

impl Dispatch for Methods {
    fn dispatch(&self, method: &str, params: &Value) -> Result<Value, ProtocolError> {
        if method != "hook.event" {
            return Err(ProtocolError::MethodNotFound);
        }
        // A notification is never answered: an event that fails the checks is simply not appended.
        if let Some(line) = self.hook_event_line(params) {
            send::append_hook_event(&self.send, &self.wait, &self.instance_dir, line)
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
}

pub(crate) fn run(home: &Path, args: RunArgs) -> anyhow::Result<ExitCode> {
    let persistent = open_wrapper_log(home, &args.name)?;
    let mut words = args.program.into_iter();
    let launch = Launch {
        name: args.name,
        program: words
            .next()
            .expect("clap requires at least one program word"),
        args: words.collect(),
        spawn_dir: std::env::current_dir()?,
    };
    match start(home, &launch, &persistent)? {
        Started::Launched(launched) => pump_child(*launched),
        Started::Refused(code) => Ok(code),
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
    _beat: Heartbeat,
    _serving: Serving,
}

pub(super) enum Started {
    Launched(Box<Launched>),
    Refused(ExitCode),
}

/// The documented start order (architecture §Established Decisions [Session Liveness]): program
/// resolution, collision check, pinned copy + plugin folder, the version gate, endpoint bind, first
/// snapshot + heartbeat, start events, then the spawn.
#[instrument(skip_all, name = "run.start", fields(instance = name_str(&launch.name)))]
pub(super) fn start(
    home: &Path,
    launch: &Launch,
    persistent: &[String],
) -> anyhow::Result<Started> {
    let name = &launch.name;
    let cwd = std::env::current_dir()?;
    let program = match run::resolve_program(&launch.program, &cwd) {
        Ok(program) => program,
        Err(Refusal::BatchScriptChild) => {
            refuse_batch_script(name);
            return Ok(refused("batch-script-child"));
        }
        Err(Refusal::NotFound) => return Ok(refused("internal-error")),
    };

    let home = std::path::absolute(home)?;
    let instance_dir = home.join("instances").join(name_str(name));
    if let Some(refusal) = collision_check(name, &instance_dir) {
        return Ok(refusal);
    }
    let Some((pinned, plugin_dir)) = pin_and_plugin(&home)? else {
        refuse_tampered_pin();
        return Ok(refused("pinned-hash-mismatch"));
    };
    #[cfg(windows)]
    let sideload = conpty_sideload(&pinned);
    #[cfg(windows)]
    let sideload_fallback = sideload.fallback;
    #[cfg(not(windows))]
    let sideload_fallback = None;
    let statusline_command = statusline_command(&home, std::env::home_dir().as_deref());
    let strip = viola_agent_claude::plan_strip(std::env::vars_os().map(|(k, _)| k), persistent);
    let gate = version_gate(&home, &program, &cwd, &strip);
    let cli_verified = gate.cli_verified;
    let Some((endpoint, server)) = bind_endpoint(name, &home)? else {
        refuse_squatted(name);
        return Ok(refused("squatted-name"));
    };
    let wheel = Arc::new(WheelSlot::default());
    let snapshots = Arc::new(Snapshots::new(instance_dir.clone()));
    let send = Arc::new(SendSlot::new(SystemClock, cli_verified, Arc::clone(&wheel)));
    // Rebuilt before the endpoint serves: the first `last` already sees the newest logged turn, and
    // the first dialog takes an id past every logged one.
    let wait = Arc::new(WaitFeed::new(SystemClock));
    let highest_dialog = wait.rebuild(&instance_dir)?;
    let dialogs = Arc::new(DialogSlot::new(
        SystemClock,
        gate.cli_verified,
        name.clone(),
        instance_dir.clone(),
        Arc::clone(&wait),
        Arc::clone(&wheel),
        Arc::clone(&snapshots),
    ));
    dialogs.restore(highest_dialog);
    wheel.record_with(Recorder {
        name: name.clone(),
        instance_dir: instance_dir.clone(),
        snapshots: Arc::clone(&snapshots),
        dialogs: Arc::clone(&dialogs),
    });
    let serving = server.serve(Arc::new(Methods {
        name: name.clone(),
        instance_dir: instance_dir.clone(),
        send: Arc::clone(&send),
        wait,
        dialogs,
        wheel: Arc::clone(&wheel),
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
            statusline_command,
        },
    )?;
    let settings = write_override(&instance_dir, &pinned)?;
    let child = run::child_launch(
        name,
        &instance_dir,
        &pinned,
        &plugin_dir,
        settings.as_deref(),
        std::env::var_os("PATH"),
        &launch.args,
    );
    let spawned = spawn_child(
        program,
        launch.spawn_dir.clone(),
        child,
        &strip,
        &snapshots,
        snapshot,
        sideload_fallback,
    );
    // Held from the hash through the spawn: the bytes verified are the bytes loaded and launched.
    #[cfg(windows)]
    drop(sideload);
    let (pty, terminal, size) = spawned?;
    Ok(Started::Launched(Box::new(Launched {
        pty,
        terminal,
        size,
        cli_verified,
        send,
        wheel,
        _beat: beat,
        _serving: serving,
    })))
}

pub(super) fn refused(detail: &'static str) -> Started {
    run::log_self_exit(1, Some(detail));
    Started::Refused(ExitCode::from(1))
}

/// A `live` or `stale` name refuses before anything is written; a `gone` one is taken over.
#[instrument(skip_all, name = "run.collision_check", fields(outcome = tracing::field::Empty))]
pub(super) fn collision_check(name: &ViolaName, instance_dir: &Path) -> Option<Started> {
    let taken = read_snapshot(instance_dir).map(|snapshot| {
        let age = beat_age(instance_dir, SystemTime::now());
        classify(age, same_process(&snapshot))
    });
    let outcome = match taken {
        None | Some(Liveness::Gone) => "free",
        Some(Liveness::Live | Liveness::Stale) => "already-live",
    };
    tracing::Span::current().record("outcome", outcome);
    match taken? {
        Liveness::Gone => return None,
        Liveness::Live => refuse_live(name),
        Liveness::Stale => refuse_stale(name),
    }
    Some(refused("already-live"))
}

/// The pinned copy of this exe, then `plugin/<key>/` rewritten whole; `None` when the pinned copy
/// failed its re-hash.
#[instrument(skip_all, name = "run.pin_copy", fields(outcome = tracing::field::Empty))]
fn pin_and_plugin(home: &Path) -> anyhow::Result<Option<(Pinned, PathBuf)>> {
    let exe = std::env::current_exe()?;
    let pinned = match pin_exe(home, &exe) {
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

/// The pinned ConPTY companions, held while they are used; `fallback` is why the child runs on the
/// inbox ConPTY instead (obs-plan §6 `sideload_fallback`), `None` once the sideload is loaded.
#[cfg(windows)]
struct Sideload {
    fallback: Option<&'static str>,
    _held: Option<viola_state::pin::HeldCompanions>,
}

/// After the pinned copy, before the spawn: `OpenConsole.exe` and `conpty.dll` written when absent,
/// re-hashed and held, then `conpty.dll` pre-loaded by absolute path. It never refuses, prints or
/// changes the exit: any failure leaves the child on the inbox ConPTY, recorded as a code.
#[cfg(windows)]
#[instrument(
    skip_all,
    name = "run.conpty_sideload",
    fields(
        outcome = tracing::field::Empty,
        search_restricted = tracing::field::Empty
    )
)]
fn conpty_sideload(pinned: &Pinned) -> Sideload {
    let span = tracing::Span::current();
    span.record(
        "search_restricted",
        viola_pty::sideload::search_restricted(),
    );
    let (outcome, held) = sideload_outcome(pinned);
    span.record("outcome", outcome);
    Sideload {
        fallback: (outcome != "loaded").then_some(outcome),
        _held: held,
    }
}

/// `conpty::FILES` puts `OpenConsole.exe` first, so a dll with no verified host never loads.
#[cfg(all(windows, target_arch = "x86_64"))]
fn sideload_outcome(pinned: &Pinned) -> (&'static str, Option<viola_state::pin::HeldCompanions>) {
    use crate::conpty;
    let held = match viola_state::pin::pin_companions(pinned, conpty::SUBDIR, conpty::FILES) {
        Ok(held) => held,
        Err(PinError::HashMismatch) => return ("hash-mismatch", None),
        Err(PinError::State(_)) => return ("unreadable", None),
    };
    match viola_pty::sideload::preload(&held.dir.join(conpty::DLL)) {
        Ok(()) => ("loaded", Some(held)),
        Err(_) => ("load-failed", None),
    }
}

#[cfg(all(windows, not(target_arch = "x86_64")))]
fn sideload_outcome(_: &Pinned) -> (&'static str, Option<viola_state::pin::HeldCompanions>) {
    ("not-built", None)
}

/// The instance's endpoint, bound before the first snapshot so the child's first hook finds it
/// listening. The exclusive bind is the arbiter of two starts of one name: `None` when another
/// process holds it.
fn bind_endpoint(name: &ViolaName, home: &Path) -> anyhow::Result<Option<(String, Server)>> {
    let endpoint = viola_channel::endpoint_path(name, home)?;
    #[cfg(unix)]
    if let Some(dir) = Path::new(&endpoint).parent() {
        create_private_dir(dir)?;
    }
    match Server::bind(&endpoint) {
        Ok(server) => Ok(Some((endpoint, server))),
        Err(ChannelError::BindTaken) => Ok(None),
        Err(error) => Err(error.into()),
    }
}

/// The home's own file that names the user's statusline command, in the settings shape: the source
/// whenever it exists.
const STATUSLINE_SOURCE: &str = "statusline-source.json";

/// The per-session settings override, in the instance directory.
const SETTINGS_OVERRIDE: &str = "settings.json";

/// Which file names the user's statusline command for a start in `home` (absolute): the home's own
/// source when it exists; else, for the default home alone, the user-scope settings file; else
/// none, so a home named by `--home` never reads the user's settings.
fn statusline_source(home: &Path, user_home: Option<&Path>) -> Option<PathBuf> {
    let own = home.join(STATUSLINE_SOURCE);
    if own.exists() {
        return Some(own);
    }
    let user_home = user_home?;
    let default = std::path::absolute(super::default_home(user_home)).ok()?;
    (default == home).then(|| {
        statusline::USER_SETTINGS
            .iter()
            .fold(user_home.to_path_buf(), |path, part| path.join(part))
    })
}

/// The user's own statusline command for a start in `home`. A source that is absent, unreadable,
/// over the frame cap or holds no command yields none: never a refusal, a line or a log entry.
/// Both files are only ever read.
fn statusline_command(home: &Path, user_home: Option<&Path>) -> Option<String> {
    let source = statusline_source(home, user_home)?;
    let mut bytes = Vec::new();
    File::open(source)
        .ok()?
        .take(MAX_FRAME)
        .read_to_end(&mut bytes)
        .ok()?;
    statusline::user_command(&bytes)
}

/// The override that puts `viola hook statusline` in the status line's place, written whole on every
/// start whatever stands there, whether or not the user has a command of their own; `None` when
/// this start wraps no status line, and then nothing is written.
fn write_override(instance_dir: &Path, pinned: &Pinned) -> anyhow::Result<Option<PathBuf>> {
    let Some(document) = statusline::override_document(&pinned.path_fwd) else {
        return Ok(None);
    };
    let path = instance_dir.join(SETTINGS_OVERRIDE);
    replace_private(&path, document.as_bytes(), FILE_MODE)?;
    Ok(Some(path))
}

/// What the first snapshot records of the launch.
struct Recorded<'a> {
    spawn_dir: &'a Path,
    statusline_command: Option<String>,
}

/// The first snapshot (with the endpoint, the version gate's reading, the child's spawn directory
/// when it is valid UTF-8 and the user's statusline command when one was found), the heartbeat and
/// its thread, then the two start events; the returned guard keeps the heartbeat running.
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
        budget_paused: false,
        links: Vec::new(),
        child_pid: None,
        pending_dialog: None,
        cwd: recorded.spawn_dir.to_str().map(str::to_owned),
        statusline_command: recorded.statusline_command,
    };
    snapshots.init(snapshot.clone())?;
    touch_heartbeat(instance_dir)?;
    let beat = Heartbeat::start(instance_dir.to_path_buf());
    for (kind, data) in [
        (
            EventKind::Wheel,
            json!({"holder": Wheel::Driver.as_str(), "cause": WheelCause::Start.as_str()}),
        ),
        (EventKind::BudgetGate, json!({"paused": false})),
    ] {
        let line = EventLine::new(name, kind, Source::Wrapper, data, Utc::now());
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
    sideload_fallback: Option<&'static str>,
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
        sideload_fallback,
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
        _beat,
        _serving,
    } = launched;
    #[cfg(feature = "fake-agent")]
    hold_pump_start();
    let sigs = cli_verified.then_some(&viola_agent_claude::screen::SIGNATURES);
    let (feed, gate, _feed_thread) = gate::start(SystemClock, size, sigs);
    let paste = PasteHandle::default();
    let typed = paste.clone();
    send.attach(Box::new(move |text| typed.paste(text)), gate);
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

fn refuse_batch_script(name: &ViolaName) {
    let name: &str = name.as_ref();
    human::refuse(
        &format!("{name}'s command is a .cmd or .bat script"),
        "pass the real executable, not a .cmd or .bat shim",
    );
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
        (tracing::subscriber::with_default(subscriber, f), spans)
    }

    fn methods(instance_dir: &Path) -> Methods {
        let name = ViolaName::try_new("builder".to_owned()).expect("valid");
        let wait = Arc::new(WaitFeed::new(SystemClock));
        let wheel = Arc::new(WheelSlot::default());
        let dialogs = DialogSlot::new(
            SystemClock,
            false,
            name.clone(),
            instance_dir.to_path_buf(),
            Arc::clone(&wait),
            Arc::clone(&wheel),
            Arc::new(Snapshots::new(instance_dir.to_path_buf())),
        );
        Methods {
            name,
            instance_dir: instance_dir.to_path_buf(),
            send: Arc::new(SendSlot::new(SystemClock, false, Arc::clone(&wheel))),
            wait,
            dialogs: Arc::new(dialogs),
            wheel,
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
        for method in ["link", "unlink", "anything"] {
            assert_eq!(
                methods(tmp.path()).dispatch(method, &json!({"v": 1})),
                Err(ProtocolError::MethodNotFound)
            );
        }
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
        let out = tracing::subscriber::with_default(subscriber, f);
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
    /// `release` with `budget: true` leaves it, and a plain `release` hands it back.
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
            Ok(json!({"ok": {"wheel": "human", "budget_paused": false}}))
        );
        assert_eq!(methods.wheel.holder(), Wheel::Human);
        let release = json!({"v": 1, "budget": false, "from": null});
        assert_eq!(
            methods.dispatch_call(&cli_call("release", &release)),
            Ok(json!({"ok": {"wheel": "driver", "budget_paused": false}}))
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
    /// wheel unmoved, and one `release-from-driver` line (its `from` only when it is a name).
    #[rstest::rstest]
    #[case::a_name("overseer", Some("overseer"))]
    #[case::not_a_name("Not A Name", None)]
    fn methods_release_from_a_driver_is_refused_and_logged(
        #[case] from: &str,
        #[case] logged: Option<&str>,
    ) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let methods = methods(tmp.path());
        let none = json!({"v": 1});
        methods
            .dispatch_call(&cli_call("pause", &none))
            .expect("paused");
        let params = json!({"v": 1, "from": from, "budget": "ignored"});
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

    #[test]
    fn methods_report_an_append_that_failed_as_internal() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let params = json!({"v": 1, "event": {"kind": "session-end", "data": {}}});
        assert_eq!(
            methods(&tmp.path().join("missing")).dispatch("hook.event", &params),
            Err(ProtocolError::Internal)
        );
    }

    /// The first snapshot `start_state` writes for a child spawned in `spawn_dir`, read back from
    /// the instance directory.
    fn first_snapshot(instance_dir: &Path, spawn_dir: &Path) -> InstanceSnapshot {
        first_snapshot_recording(
            instance_dir,
            Recorded {
                spawn_dir,
                statusline_command: None,
            },
        )
    }

    fn first_snapshot_recording(instance_dir: &Path, recorded: Recorded<'_>) -> InstanceSnapshot {
        let name = ViolaName::try_new("builder".to_owned()).expect("valid");
        let pinned = Pinned {
            key: "k".to_owned(),
            path: PathBuf::from("bin").join("k").join("viola"),
            path_fwd: "bin/k/viola".to_owned(),
        };
        let gate = Gate {
            cli_version: None,
            cli_verified: false,
        };
        let snapshots = Snapshots::new(instance_dir.to_path_buf());
        let (_beat, returned) = start_state(
            &name,
            instance_dir,
            &snapshots,
            &pinned,
            "endpoint".to_owned(),
            gate,
            recorded,
        )
        .expect("the start state");
        assert_eq!(read_snapshot(instance_dir), Some(returned.clone()));
        returned
    }

    /// The first snapshot records where the child is spawned, and the two start events stand as
    /// they were: the wheel starts with the driver.
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
        let read: Vec<(&Value, &Value)> = events.iter().map(|e| (&e["kind"], &e["data"])).collect();
        assert_eq!(
            read,
            [
                (
                    &json!("wheel"),
                    &json!({"holder": "driver", "cause": "start"})
                ),
                (&json!("budget-gate"), &json!({"paused": false})),
            ]
        );
    }

    fn settings_with(command: &str) -> String {
        json!({"statusLine": {"type": "command", "command": command}}).to_string()
    }

    /// A viola home and a user home side by side in `root`, the user's settings file holding
    /// `user_command`.
    fn homes(root: &Path, user_command: &str) -> (PathBuf, PathBuf) {
        let user_home = root.join("user");
        let claude = user_home.join(".claude");
        std::fs::create_dir_all(&claude).expect("the user's claude dir");
        std::fs::write(claude.join("settings.json"), settings_with(user_command))
            .expect("the user's settings");
        let home = root.join("vhome");
        std::fs::create_dir_all(&home).expect("the viola home");
        (home, user_home)
    }

    #[test]
    fn statusline_source_the_home_s_own_file_wins_over_the_user_s_settings() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (_, user_home) = homes(tmp.path(), "user-line");
        let home = user_home.join(".viola");
        std::fs::create_dir_all(&home).expect("the default home");
        let own = home.join("statusline-source.json");
        std::fs::write(&own, settings_with("home-line")).expect("the home's source");
        assert_eq!(statusline_source(&home, Some(&user_home)), Some(own));
        assert_eq!(
            statusline_command(&home, Some(&user_home)).as_deref(),
            Some("home-line")
        );
    }

    #[test]
    fn statusline_source_a_home_named_by_home_reads_no_user_settings() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (home, user_home) = homes(tmp.path(), "user-line");
        assert_eq!(statusline_source(&home, Some(&user_home)), None);
        assert_eq!(statusline_command(&home, Some(&user_home)), None);
        let own = home.join("statusline-source.json");
        std::fs::write(&own, settings_with("home-line")).expect("the home's source");
        assert_eq!(statusline_source(&home, Some(&user_home)), Some(own));
        assert_eq!(
            statusline_command(&home, None).as_deref(),
            Some("home-line")
        );
    }

    #[test]
    fn statusline_source_the_default_home_falls_back_to_the_user_s_settings() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (_, user_home) = homes(tmp.path(), "user-line");
        let home = user_home.join(".viola");
        assert_eq!(
            statusline_source(&home, Some(&user_home)),
            Some(user_home.join(".claude").join("settings.json"))
        );
        assert_eq!(
            statusline_command(&home, Some(&user_home)).as_deref(),
            Some("user-line")
        );
        assert_eq!(statusline_source(&home, None), None);
        assert_eq!(statusline_command(&home, None), None);
    }

    #[test]
    fn statusline_source_that_is_absent_unreadable_or_holds_no_command_is_no_command() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let user_home = tmp.path().join("user");
        let home = user_home.join(".viola");
        std::fs::create_dir_all(&home).expect("the default home");
        assert_eq!(statusline_command(&home, Some(&user_home)), None);
        let own = home.join("statusline-source.json");
        std::fs::write(&own, b"not json").expect("the home's source");
        assert_eq!(statusline_command(&home, Some(&user_home)), None);
        std::fs::write(&own, br#"{"statusLine":{"type":"command"}}"#).expect("the home's source");
        assert_eq!(statusline_command(&home, Some(&user_home)), None);
        std::fs::remove_file(&own).expect("removed");
        std::fs::create_dir(&own).expect("a directory in its place");
        assert_eq!(statusline_command(&home, Some(&user_home)), None);
    }

    /// A source past the frame cap is read up to the cap alone, so its command is never taken.
    #[test]
    fn statusline_source_over_the_frame_cap_is_no_command() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (home, _) = homes(tmp.path(), "user-line");
        let own = home.join("statusline-source.json");
        let cap = usize::try_from(MAX_FRAME).expect("the cap");
        let doc = settings_with("home-line");
        std::fs::write(&own, " ".repeat(cap) + &doc).expect("the home's source");
        assert_eq!(statusline_command(&home, None), None);
        std::fs::write(&own, " ".repeat(cap - doc.len()) + &doc).expect("the home's source");
        assert_eq!(
            statusline_command(&home, None).as_deref(),
            Some("home-line")
        );
    }

    #[test]
    fn statusline_override_of_a_start_is_rewritten_whole_at_owner_only() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let pinned = |path_fwd: &str| Pinned {
            key: "k".to_owned(),
            path: PathBuf::from(path_fwd),
            path_fwd: path_fwd.to_owned(),
        };
        let written = write_override(tmp.path(), &pinned("/h/bin/k/viola")).expect("the write");
        let file = tmp.path().join("settings.json");
        if cfg!(unix) {
            assert_eq!(written.as_deref(), Some(file.as_path()));
            let first = std::fs::read(&file).expect("the override");
            assert_eq!(
                statusline::user_command(&first).as_deref(),
                Some("/h/bin/k/viola hook statusline")
            );
            std::fs::write(&file, b"sentinel").expect("altered");
            write_override(tmp.path(), &pinned("/h/bin/k/viola")).expect("the second write");
            assert_eq!(std::fs::read(&file).expect("the override"), first);
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt as _;
                let mode = std::fs::metadata(&file).expect("meta").permissions().mode();
                assert_eq!(mode & 0o777, 0o600);
            }
        } else {
            assert_eq!(written, None);
            assert!(!file.exists());
        }
        let unsafe_dir = tmp.path().join("unsafe");
        std::fs::create_dir(&unsafe_dir).expect("dir");
        let none = write_override(&unsafe_dir, &pinned("/h/bin k/viola")).expect("no write");
        assert_eq!(none, None);
        assert!(!unsafe_dir.join("settings.json").exists());
        if cfg!(unix) {
            let missing = tmp.path().join("missing");
            assert!(write_override(&missing, &pinned("/h/bin/k/viola")).is_err());
        }
    }

    /// The first snapshot records the command the start's source names, and none without one.
    #[test]
    fn snapshot_statusline_command_of_a_start_is_the_source_s_command() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (home, _) = homes(tmp.path(), "user-line");
        std::fs::write(
            home.join("statusline-source.json"),
            settings_with("home-line --wide"),
        )
        .expect("the home's source");
        let spawn_dir = tmp.path().join("work");
        let snapshot = first_snapshot_recording(
            &tmp.path().join("instance"),
            Recorded {
                spawn_dir: &spawn_dir,
                statusline_command: statusline_command(&home, None),
            },
        );
        assert_eq!(
            snapshot.statusline_command.as_deref(),
            Some("home-line --wide")
        );
        let without = first_snapshot(&tmp.path().join("other"), &spawn_dir);
        assert_eq!(without.statusline_command, None);
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
        };
        let (started, spans) = capture_spans(|| start(&tmp.path().join("home"), &launch, &[]));
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
        let sideload: &[&str] = if cfg!(windows) {
            &["run.conpty_sideload"]
        } else {
            &[]
        };
        let expected: Vec<&str> = ["run.start", "run.collision_check", "run.pin_copy"]
            .iter()
            .chain(sideload)
            .chain(&[
                "run.version_gate",
                "channel.bind",
                "state.snapshot_write",
                "state.heartbeat_start",
                "pty.spawn",
                "state.snapshot_write",
            ])
            .copied()
            .collect();
        assert_eq!(names, expected);
        if cfg!(all(windows, target_arch = "x86_64")) {
            let sideload = field(&spans, "run.conpty_sideload");
            assert_eq!(sideload["outcome"], "loaded");
            assert!(sideload["search_restricted"].is_boolean(), "{sideload}");
            assert_eq!(viola_pty::pty_backend(), "conpty-sideload");
        }
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
