use std::ffi::OsString;
use std::io::{self, Write as _};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::SystemTime;

use anyhow::Context as _;
use chrono::Utc;
use serde_json::json;
use viola_agent_claude::{Refusal, StripPlan};
use viola_core::obs::ObsProcess;
use viola_core::{EventKind, ViolaName};
use viola_pty::{HostTerminal, PumpEnd, Size, SpawnSpec};
use viola_state::events::{EventLine, Source, append_event};
use viola_state::fs::{FILE_MODE, create_private_dir, replace_private_shared};
use viola_state::heartbeat::{Heartbeat, beat_age, touch_heartbeat};
use viola_state::liveness::{Liveness, classify, own_start, same_process};
use viola_state::pin::{PinError, Pinned, pin_exe};
use viola_state::snapshot::{InstanceSnapshot, Wheel, read_snapshot, write_snapshot};

use crate::run::ChildLaunch;
use crate::{obs, run};

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

/// The documented start order (architecture §Established Decisions [Session Liveness]): program
/// resolution, collision check, pinned copy + plugin folder, first snapshot + heartbeat, start
/// events, then the spawn. The version gate and the endpoint bind have no step yet.
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

    let (program, program_args) = args
        .program
        .split_first()
        .expect("clap requires at least one program word");
    let cwd = std::env::current_dir()?;
    let program = match run::resolve_program(program, &cwd) {
        Ok(program) => program,
        Err(Refusal::BatchScriptChild) => {
            refuse_batch_script(&args.name);
            run::log_self_exit(1, Some("batch-script-child"));
            return Ok(ExitCode::from(1));
        }
        Err(Refusal::NotFound) => {
            run::log_self_exit(1, Some("internal-error"));
            return Ok(ExitCode::from(1));
        }
    };

    let home = std::path::absolute(home)?;
    let instance_dir = home.join("instances").join(args.name.as_ref());
    if let Some(refused) = collision_check(&args.name, &instance_dir) {
        return Ok(refused);
    }
    let Some((pinned, plugin_dir)) = pin_and_plugin(&home)? else {
        refuse_tampered_pin();
        run::log_self_exit(1, Some("pinned-hash-mismatch"));
        return Ok(ExitCode::from(1));
    };
    let (_beat, snapshot) = start_state(&args.name, &instance_dir, &pinned)?;
    let launch = run::child_launch(
        &args.name,
        &instance_dir,
        &pinned,
        &plugin_dir,
        std::env::var_os("PATH"),
        program_args,
    );
    let strip = viola_agent_claude::plan_strip(std::env::vars_os().map(|(k, _)| k), &persistent);
    spawn_and_pump(program, cwd, launch, &strip, &instance_dir, snapshot)
}

/// A `live` or `stale` name refuses before anything is written; a `gone` one is taken over.
fn collision_check(name: &ViolaName, instance_dir: &Path) -> Option<ExitCode> {
    let snapshot = read_snapshot(instance_dir)?;
    let age = beat_age(instance_dir, SystemTime::now());
    match classify(age, same_process(&snapshot)) {
        Liveness::Gone => return None,
        Liveness::Live => refuse_live(name),
        Liveness::Stale => refuse_stale(name),
    }
    run::log_self_exit(1, Some("already-live"));
    Some(ExitCode::from(1))
}

/// The pinned copy of this exe, then `plugin/<key>/` rewritten whole; `None` when the pinned copy
/// failed its re-hash.
fn pin_and_plugin(home: &Path) -> anyhow::Result<Option<(Pinned, PathBuf)>> {
    let exe = std::env::current_exe()?;
    let pinned = match pin_exe(home, &exe) {
        Ok(pinned) => pinned,
        Err(PinError::HashMismatch) => return Ok(None),
        Err(error) => return Err(error.into()),
    };
    let plugin_dir = home.join("plugin").join(&pinned.key);
    for (rel, content) in viola_agent_claude::plugin_files(&pinned.path_fwd) {
        let path = plugin_dir.join(rel);
        create_private_dir(path.parent().unwrap_or(&plugin_dir))?;
        replace_private_shared(&path, content.as_bytes(), FILE_MODE)?;
    }
    Ok(Some((pinned, plugin_dir)))
}

/// The first snapshot, the heartbeat and its thread, then the two start events; the returned
/// guard keeps the heartbeat running.
fn start_state(
    name: &ViolaName,
    instance_dir: &Path,
    pinned: &Pinned,
) -> anyhow::Result<(Heartbeat, InstanceSnapshot)> {
    create_private_dir(instance_dir)?;
    let pid = std::process::id();
    let snapshot = InstanceSnapshot {
        endpoint: None,
        pid,
        started_at: own_start(pid).context("own start time unreadable")?,
        pinned_bin: pinned.path_fwd.clone(),
        cli_verified: false,
        cli_version: None,
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

fn spawn_and_pump(
    program: PathBuf,
    cwd: PathBuf,
    launch: ChildLaunch,
    strip: &StripPlan,
    instance_dir: &Path,
    mut snapshot: InstanceSnapshot,
) -> anyhow::Result<ExitCode> {
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
    let mut pty = viola_pty::spawn(&spec)?;
    snapshot.child_pid = viola_pty::Pty::child_pid(&pty);
    write_snapshot(instance_dir, &snapshot)?;
    run::log_child_start(snapshot.child_pid, strip);
    #[cfg(feature = "fake-agent")]
    hold_pump_start();
    let end = viola_pty::pump(
        &mut pty,
        Box::new(io::stdin()),
        Box::new(io::stdout()),
        spec.size,
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

/// A start refusal's two fixed lines on stderr (design-system cli pattern 2): no path, no pid.
fn refuse(unable: &str, hint: &str) {
    let mut err = io::stderr().lock();
    let _ = writeln!(err, "unable: {unable}");
    let _ = writeln!(err, "hint: {hint}");
}

fn refuse_batch_script(name: &ViolaName) {
    let name: &str = name.as_ref();
    refuse(
        &format!("{name}'s command is a .cmd or .bat script"),
        "pass the real executable, not a .cmd or .bat shim",
    );
}

fn refuse_live(name: &ViolaName) {
    let name: &str = name.as_ref();
    refuse(&format!("{name} is already live"), "viola list");
}

fn refuse_stale(name: &ViolaName) {
    let name: &str = name.as_ref();
    refuse(
        &format!("{name} is still running but not answering"),
        &format!("viola list shows it as stale; stop that process before starting {name} again"),
    );
}

fn refuse_tampered_pin() {
    refuse(
        "the pinned viola copy failed its integrity check",
        "the pinned copy was changed after it was written, so viola will not run it",
    );
}
