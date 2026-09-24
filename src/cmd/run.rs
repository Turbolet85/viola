use std::ffi::OsString;
use std::path::Path;
use std::process::ExitCode;

use viola_core::ViolaName;
use viola_core::obs::ObsProcess;

use crate::{obs, run};

#[derive(clap::Args)]
pub(crate) struct RunArgs {
    /// Instance name: [a-z0-9-], 1-32 characters, starting with a letter
    #[arg(value_parser = parse_name)]
    name: ViolaName,
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

    let (program, program_args) = args
        .program
        .split_first()
        .expect("clap requires at least one program word");
    match run::spawn_child(program, program_args) {
        Ok(mut child) => {
            run::log_child_start(child.id());
            let status = child.wait()?;
            run::log_child_exit(status.code());
            run::log_self_exit(0, None);
            Ok(ExitCode::SUCCESS)
        }
        Err(_) => {
            run::log_self_exit(1, Some("internal-error"));
            Ok(ExitCode::from(1))
        }
    }
}
