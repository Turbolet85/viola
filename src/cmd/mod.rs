mod answer;
mod client;
mod hook;
mod last;
mod pause;
mod release;
mod run;
mod send;
mod verify;
mod wait;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::Context as _;
use clap::{Parser, Subcommand};
use viola_core::ViolaName;
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
    /// Type a prompt into the named instance and read it back; the text comes from stdin or --file
    Send(send::SendArgs),
    /// Wait for the named instance's next turn end, dialog or session end at or after a cursor
    Wait(wait::WaitArgs),
    /// Print the named instance's newest turn message
    Last(last::LastArgs),
    /// Answer the named instance's pending dialog by its id; the response comes from stdin or --file
    Answer(answer::AnswerArgs),
    /// Take the named instance's wheel for the human without a keystroke
    Pause(pause::PauseArgs),
    /// Hand the named instance's wheel back to the driver: the human's verb, refused to a driver
    Release(release::ReleaseArgs),
    /// Measure the local claude CLI against the capability ledger and stamp its version
    Verify(verify::VerifyArgs),
    /// Hand a Claude Code hook's payload to the wrapper (run by the plugin, never by a person)
    #[command(hide = true)]
    Hook(hook::HookArgs),
}

/// A dispatch error and, once the home and the instance resolved, where its chain may go.
pub(crate) struct Failure {
    pub(crate) error: anyhow::Error,
    pub(crate) sink: Option<DetailSink>,
}

pub(crate) fn dispatch(cli: Cli) -> Result<ExitCode, Failure> {
    match cli.command {
        // The hook's home is its session's, from `VIOLA_DIR`, never `--home`.
        Command::Hook(args) => hook::hook(&args),
        Command::Run(args) => {
            let home = resolve_home(cli.home)?;
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
        Command::Send(args) => {
            let home = resolve_home(cli.home)?;
            let sink = cli_sink(&home, &args.name);
            send::send(&home, &args).map_err(|error| Failure { error, sink })
        }
        Command::Wait(args) => {
            let home = resolve_home(cli.home)?;
            let sink = cli_sink(&home, &args.name);
            wait::wait(&home, &args).map_err(|error| Failure { error, sink })
        }
        Command::Last(args) => {
            let home = resolve_home(cli.home)?;
            let sink = cli_sink(&home, &args.name);
            last::last(&home, &args).map_err(|error| Failure { error, sink })
        }
        Command::Answer(args) => {
            let home = resolve_home(cli.home)?;
            let sink = cli_sink(&home, &args.name);
            answer::answer(&home, &args).map_err(|error| Failure { error, sink })
        }
        Command::Pause(args) => {
            let home = resolve_home(cli.home)?;
            let sink = cli_sink(&home, &args.name);
            pause::pause(&home, &args).map_err(|error| Failure { error, sink })
        }
        Command::Release(args) => {
            let home = resolve_home(cli.home)?;
            let sink = cli_sink(&home, &args.name);
            release::release(&home, &args).map_err(|error| Failure { error, sink })
        }
        Command::Verify(args) => {
            let home = resolve_home(cli.home)?;
            let instance = std::env::var_os("VIOLA_NAME")
                .and_then(|name| name.into_string().ok())
                .and_then(|name| ViolaName::try_new(name).ok());
            let sink = instance.clone().map(|instance| DetailSink {
                home: home.clone(),
                instance,
                process: ObsProcess::Cli,
            });
            verify::verify(&home, instance.as_ref(), &args).map_err(|error| Failure { error, sink })
        }
    }
}

/// A `cli` verb's detail sink: its chain goes there, and its one `error: internal error` line is
/// printed by the catch site alone (obs-plan §7 Per-role behaviour).
fn cli_sink(home: &Path, instance: &ViolaName) -> Option<DetailSink> {
    Some(DetailSink {
        home: home.to_path_buf(),
        instance: instance.clone(),
        process: ObsProcess::Cli,
    })
}

fn resolve_home(home: Option<PathBuf>) -> Result<PathBuf, Failure> {
    match home {
        Some(home) => Ok(home),
        None => Ok(std::env::home_dir()
            .context("no user home directory")
            .map_err(|error| Failure { error, sink: None })?
            .join(".viola")),
    }
}
