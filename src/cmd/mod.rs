mod run;

use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::Context as _;
use clap::{Parser, Subcommand};
use viola_core::obs::ObsProcess;

use crate::obs::DetailSink;

#[derive(Parser)]
#[command(
    name = "viola",
    version,
    about = "Drive one Claude Code session from another"
)]
pub(crate) struct Cli {
    /// The viola home (default: <user home>/.viola)
    #[arg(long, global = true)]
    home: Option<PathBuf>,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Wrap a program as the named instance
    Run(run::RunArgs),
}

/// A dispatch error and, once the home and the instance resolved, where its chain may go.
pub(crate) struct Failure {
    pub(crate) error: anyhow::Error,
    pub(crate) sink: Option<DetailSink>,
}

pub(crate) fn dispatch(cli: Cli) -> Result<ExitCode, Failure> {
    let home = match cli.home {
        Some(home) => home,
        None => std::env::home_dir()
            .context("no user home directory")
            .map_err(|error| Failure { error, sink: None })?
            .join(".viola"),
    };
    match cli.command {
        Command::Run(args) => {
            let sink = DetailSink {
                home: home.clone(),
                instance: args.name.clone(),
                process: ObsProcess::Run,
            };
            run::run(&home, args).map_err(|error| Failure {
                error,
                sink: Some(sink),
            })
        }
    }
}
