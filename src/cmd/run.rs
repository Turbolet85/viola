use std::ffi::OsString;
use std::path::Path;
use std::process::ExitCode;

use viola_core::ViolaName;

use crate::run;

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
    let role_file = run::open_role_file(home, &args.name)?;
    run::init_role_logger(role_file, "run", &args.name);
    run::log_self_start(&args.name);

    let (program, program_args) = args
        .program
        .split_first()
        .expect("clap requires at least one program word");
    match run::spawn_child(program, program_args) {
        Ok(mut child) => {
            run::log_child_start(&args.name, child.id());
            let status = child.wait()?;
            run::log_child_exit(&args.name, status.code());
            run::log_self_exit(&args.name, 0, None);
            Ok(ExitCode::SUCCESS)
        }
        Err(_) => {
            run::log_self_exit(&args.name, 1, Some("internal-error"));
            Ok(ExitCode::from(1))
        }
    }
}
