//! `viola-harness`: parse, call `viola_e2e::harness`, print one JSON document, exit 0 · 1 · 2.
//! (`logs` streams ndjson instead; `supervise` prints nothing — its document is its file.)

use std::process::ExitCode;

use clap::{Parser, Subcommand};
use viola_e2e::harness::boot::{BootOptions, DEFAULT_CLI_VERSION, InstanceSpec, boot};
use viola_e2e::harness::cleanup::{Target, cleanup};
use viola_e2e::harness::logs::{Filter, logs};
use viola_e2e::harness::run::{Selection, run};
use viola_e2e::harness::status::status;
use viola_e2e::harness::supervise::supervise;
use viola_e2e::harness::{Outcome, Workspace};

#[derive(Parser)]
#[command(name = "viola-harness")]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Boot {
        #[arg(long, default_value = "default")]
        session: String,
        #[arg(long = "instance")]
        instances: Vec<String>,
        #[arg(long, default_value = DEFAULT_CLI_VERSION)]
        cli_version: String,
    },
    Run {
        #[arg(long)]
        unit: bool,
        #[arg(long)]
        integration: bool,
        #[arg(long)]
        mutants: bool,
        #[arg(long)]
        all: bool,
        #[arg(long)]
        filter: Option<String>,
    },
    Status {
        #[arg(long, default_value = "default")]
        session: String,
    },
    Cleanup {
        #[arg(long, conflicts_with = "all")]
        session: Option<String>,
        #[arg(long)]
        all: bool,
    },
    Logs {
        #[arg(long, default_value = "default")]
        session: String,
        #[arg(long)]
        instance: Option<String>,
        #[arg(long)]
        process: Option<String>,
    },
    Supervise {
        #[arg(long)]
        session: String,
    },
}

const COMMANDS: [&str; 6] = ["boot", "run", "status", "cleanup", "logs", "supervise"];

fn emit(outcome: Outcome) -> ExitCode {
    println!("{}", outcome.doc);
    ExitCode::from(outcome.code)
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let cli = match Cli::try_parse_from(&args) {
        Ok(cli) => cli,
        Err(e) => {
            eprintln!("{e}");
            let cmd = args
                .get(1)
                .map(String::as_str)
                .filter(|c| COMMANDS.contains(c));
            return emit(Outcome::usage(cmd, "arguments"));
        }
    };
    let ws = Workspace::from_build();
    match cli.command {
        Cmd::Boot {
            session,
            instances,
            cli_version,
        } => {
            let raw = if instances.is_empty() {
                vec!["overseer".to_owned(), "builder".to_owned()]
            } else {
                instances
            };
            let Some(instances) = raw.iter().map(|r| InstanceSpec::parse(r)).collect() else {
                return emit(Outcome::usage(Some("boot"), "invalid-instance"));
            };
            emit(boot(&BootOptions {
                bin_dir: ws.harness_bins(),
                ws,
                session,
                instances,
                cli_version,
                build: true,
            }))
        }
        Cmd::Run {
            unit,
            integration,
            mutants,
            all,
            filter,
        } => emit(run(
            &ws,
            Selection::from_flags(unit, integration, mutants, all),
            filter.as_deref(),
            std::env::var("AGENT_RUN_CHUNK_BASE").ok(),
        )),
        Cmd::Status { session } => emit(status(&ws, &session)),
        Cmd::Cleanup { session, all } => {
            let keep = std::env::var("AGENT_RUN_KEEP_HOMES").is_ok_and(|v| v == "1");
            let target = if all {
                Target::All
            } else {
                Target::Session(session.as_deref().unwrap_or("default"))
            };
            emit(cleanup(&ws, target, keep))
        }
        Cmd::Logs {
            session,
            instance,
            process,
        } => {
            let filter = Filter {
                instance: instance.as_deref(),
                process: process.as_deref(),
            };
            match logs(&ws, &session, &filter) {
                Ok(lines) => {
                    for line in lines {
                        println!("{line}");
                    }
                    ExitCode::SUCCESS
                }
                Err(out) => emit(out),
            }
        }
        Cmd::Supervise { session } => ExitCode::from(supervise(&ws, &session).code),
    }
}
