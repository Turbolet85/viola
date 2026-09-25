use std::ffi::OsString;
use std::io::{self, Write as _};
use std::path::Path;
use std::process::ExitCode;

use viola_agent_claude::Refusal;
use viola_core::ViolaName;
use viola_core::obs::ObsProcess;
use viola_pty::{HostTerminal, PumpEnd, Size, SpawnSpec};

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
    let strip = viola_agent_claude::plan_strip(std::env::vars_os().map(|(k, _)| k), &persistent);
    let spec = SpawnSpec {
        program,
        args: program_args.to_vec(),
        cwd,
        env_set: Vec::new(),
        env_remove: strip.remove.clone(),
        size: viola_pty::host_size().unwrap_or(Size::DEFAULT),
    };
    // Raw before the spawn: every key the human types from here on reaches the child as typed.
    let terminal = HostTerminal::enter();
    let mut pty = viola_pty::spawn(&spec)?;
    run::log_child_start(viola_pty::Pty::child_pid(&pty), &strip);
    let end = viola_pty::pump(
        &mut pty,
        Box::new(io::stdin()),
        Box::new(io::stdout()),
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

/// The start refusal's two fixed lines on stderr (design-system cli pattern 2): no path, no pid.
fn refuse_batch_script(name: &ViolaName) {
    let name: &str = name.as_ref();
    let mut err = io::stderr().lock();
    let _ = writeln!(err, "unable: {name}'s command is a .cmd or .bat script");
    let _ = writeln!(
        err,
        "hint: pass the real executable, not a .cmd or .bat shim"
    );
}
