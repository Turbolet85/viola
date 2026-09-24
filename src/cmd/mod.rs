mod run;

use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::Context as _;
use clap::{Parser, Subcommand};

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

pub(crate) fn dispatch(cli: Cli) -> anyhow::Result<ExitCode> {
    let home = match cli.home {
        Some(home) => home,
        None => std::env::home_dir()
            .context("no user home directory")?
            .join(".viola"),
    };
    match cli.command {
        Command::Run(args) => run::run(&home, args),
    }
}
