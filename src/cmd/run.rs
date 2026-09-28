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
use viola_agent_claude::{Refusal, StripPlan};
use viola_channel::{ChannelError, Dispatch, ProtocolError, Server, Serving};
use viola_core::obs::ObsProcess;
use viola_core::{EventKind, ViolaName};
use viola_pty::{HostTerminal, PortablePty, PumpEnd, Size, SpawnSpec};
use viola_state::events::{EventLine, Source, append_event};
use viola_state::fs::{FILE_MODE, create_private_dir, replace_private_shared};
use viola_state::heartbeat::{Heartbeat, beat_age, touch_heartbeat};
use viola_state::liveness::{Liveness, classify, own_start, same_process};
use viola_state::pin::{PinError, Pinned, pin_exe};
use viola_state::snapshot::{InstanceSnapshot, Wheel, read_snapshot, write_snapshot};

use crate::run::ChildLaunch;
use crate::run::version_gate::{Gate, version_gate};
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

/// The wrapper's answers: the `hook.event` notification appends its event; every other method is
/// `-32601` until its chunk lands.
pub(crate) struct Methods {
    name: ViolaName,
    instance_dir: PathBuf,
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
            append_event(&self.instance_dir, &line).map_err(|_| ProtocolError::Internal)?;
        }
        Ok(Value::Null)
    }
}

pub(crate) fn run(home: &Path, args: RunArgs) -> anyhow::Result<ExitCode> {
    let (level, rejection) = obs::read_diagnostics_level(home);
    obs::viola_obs_init(home, ObsProcess::Run, Some(args.name.clone()), level)?;
    run::log_self_start();
    if let Some(rejection) = rejection {
        obs::log_config_rejection(rejection);
    }
    let (persistent, keep_rejection) = run::persistent_names(home);
    if let Some(rejection) = keep_rejection {
        obs::log_config_rejection(rejection);
    }
    match start(home, &args, &persistent)? {
        Started::Launched(launched) => pump_child(*launched),
        Started::Refused(code) => Ok(code),
    }
}

/// Everything the pump needs, and the guards that must outlive it: the heartbeat thread and the
/// served endpoint.
struct Launched {
    pty: PortablePty,
    terminal: Option<HostTerminal>,
    size: Size,
    _beat: Heartbeat,
    _serving: Serving,
}

enum Started {
    Launched(Box<Launched>),
    Refused(ExitCode),
}

/// The documented start order (architecture §Established Decisions [Session Liveness]): program
/// resolution, collision check, pinned copy + plugin folder, the version gate, endpoint bind, first
/// snapshot + heartbeat, start events, then the spawn.
#[instrument(skip_all, name = "run.start", fields(instance = name_str(&args.name)))]
fn start(home: &Path, args: &RunArgs, persistent: &[String]) -> anyhow::Result<Started> {
    let (program, program_args) = args
        .program
        .split_first()
        .expect("clap requires at least one program word");
    let cwd = std::env::current_dir()?;
    let program = match run::resolve_program(program, &cwd) {
        Ok(program) => program,
        Err(Refusal::BatchScriptChild) => {
            refuse_batch_script(&args.name);
            return Ok(refused("batch-script-child"));
        }
        Err(Refusal::NotFound) => return Ok(refused("internal-error")),
    };

    let home = std::path::absolute(home)?;
    let instance_dir = home.join("instances").join(name_str(&args.name));
    if let Some(refusal) = collision_check(&args.name, &instance_dir) {
        return Ok(refusal);
    }
    let Some((pinned, plugin_dir)) = pin_and_plugin(&home)? else {
        refuse_tampered_pin();
        return Ok(refused("pinned-hash-mismatch"));
    };
    let strip = viola_agent_claude::plan_strip(std::env::vars_os().map(|(k, _)| k), persistent);
    let gate = version_gate(&home, &program, &cwd, &strip);
    let Some((endpoint, server)) = bind_endpoint(&args.name, &home)? else {
        refuse_squatted(&args.name);
        return Ok(refused("squatted-name"));
    };
    let serving = server.serve(Arc::new(Methods {
        name: args.name.clone(),
        instance_dir: instance_dir.clone(),
    }));
    let (beat, snapshot) = start_state(&args.name, &instance_dir, &pinned, endpoint, gate)?;
    let launch = run::child_launch(
        &args.name,
        &instance_dir,
        &pinned,
        &plugin_dir,
        std::env::var_os("PATH"),
        program_args,
    );
    let (pty, terminal, size) = spawn_child(program, cwd, launch, &strip, &instance_dir, snapshot)?;
    Ok(Started::Launched(Box::new(Launched {
        pty,
        terminal,
        size,
        _beat: beat,
        _serving: serving,
    })))
}

fn refused(detail: &'static str) -> Started {
    run::log_self_exit(1, Some(detail));
    Started::Refused(ExitCode::from(1))
}

/// A `live` or `stale` name refuses before anything is written; a `gone` one is taken over.
#[instrument(skip_all, name = "run.collision_check", fields(outcome = tracing::field::Empty))]
fn collision_check(name: &ViolaName, instance_dir: &Path) -> Option<Started> {
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

/// The first snapshot (with the endpoint and the version gate's reading), the heartbeat and its
/// thread, then the two start events; the returned guard keeps the heartbeat running.
fn start_state(
    name: &ViolaName,
    instance_dir: &Path,
    pinned: &Pinned,
    endpoint: String,
    gate: Gate,
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
    };
    write_snapshot(instance_dir, &snapshot)?;
    touch_heartbeat(instance_dir)?;
    let beat = Heartbeat::start(instance_dir.to_path_buf());
    for (kind, data) in [
        (
            EventKind::Wheel,
            json!({"holder": "driver", "cause": "start"}),
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
    instance_dir: &Path,
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
    snapshot.child_pid = viola_pty::Pty::child_pid(&pty);
    write_snapshot(instance_dir, &snapshot)?;
    run::log_child_start(
        snapshot.child_pid,
        strip,
        snapshot.cli_version.as_deref(),
        snapshot.cli_verified,
    );
    Ok((pty, terminal, spec.size))
}

fn pump_child(launched: Launched) -> anyhow::Result<ExitCode> {
    let Launched {
        mut pty,
        terminal,
        size,
        _beat,
        _serving,
    } = launched;
    #[cfg(feature = "fake-agent")]
    hold_pump_start();
    let end = viola_pty::pump(
        &mut pty,
        Box::new(io::stdin()),
        Box::new(io::stdout()),
        size,
        &mut viola_pty::host_size,
    );
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
        Methods {
            name: ViolaName::try_new("builder".to_owned()).expect("valid"),
            instance_dir: instance_dir.to_path_buf(),
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
        for method in ["send", "wait", "hook.dialog", "anything"] {
            assert_eq!(
                methods(tmp.path()).dispatch(method, &json!({"v": 1})),
                Err(ProtocolError::MethodNotFound)
            );
        }
        assert!(events_in(tmp.path()).is_empty());
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
    #[test]
    fn start_opens_the_scenario_one_spans_under_run_start() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let exe = std::env::current_exe().expect("test exe");
        let args = RunArgs {
            name: ViolaName::try_new("builder".to_owned()).expect("valid"),
            program: vec![exe.into_os_string(), OsString::from("--list")],
        };
        let (started, spans) = capture_spans(|| start(&tmp.path().join("home"), &args, &[]));
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
        assert_eq!(spawn["pty_backend"], viola_pty::PTY_BACKEND);
        assert!(spawn["env_stripped_count"].is_u64(), "{spawn}");

        let seam: Vec<Value> = seam.all();
        let names: Vec<&str> = seam.iter().filter_map(|s| s["name"].as_str()).collect();
        assert_eq!(names, ["pty.resize", "pty.kill"]);
    }
}
