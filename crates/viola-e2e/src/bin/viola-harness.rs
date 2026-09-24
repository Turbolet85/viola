//! `viola-harness`: parse, call `viola_e2e::harness`, print one JSON document, exit 0 · 1 · 2.
//! (`logs` streams ndjson instead; `supervise` prints nothing — its document is its file.)
//! `schema-check`, `secret-scan` and `gate` are internal CI gate bodies, not agent commands.

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Args, Parser, Subcommand};
use viola_e2e::harness::boot::{BootOptions, DEFAULT_CLI_VERSION, InstanceSpec, boot};
use viola_e2e::harness::cleanup::{Target, cleanup};
use viola_e2e::harness::gate::{gate, parse_legs, parse_require};
use viola_e2e::harness::logs::{Filter, logs};
use viola_e2e::harness::run::{Selection, run_forwarding, run_with};
use viola_e2e::harness::schema_check::schema_check;
use viola_e2e::harness::secret_scan::secret_scan;
use viola_e2e::harness::status::status;
use viola_e2e::harness::supervise::supervise;
use viola_e2e::harness::{Outcome, Workspace, valid_session_id};

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
    Run(RunArgs),
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
    SchemaCheck,
    SecretScan,
    Gate {
        #[arg(long)]
        require: String,
        #[arg(long)]
        artifacts: Option<PathBuf>,
        #[arg(long)]
        mutants_legs: Option<String>,
    },
}

#[derive(Args)]
struct RunArgs {
    #[arg(long)]
    unit: bool,
    #[arg(long)]
    integration: bool,
    #[arg(long)]
    mutants: bool,
    #[arg(long)]
    coverage: bool,
    #[arg(long)]
    fuzz_replay: bool,
    #[arg(long)]
    all: bool,
    #[arg(long)]
    filter: Option<String>,
    /// A mutation leg whose verdict the `gate` union decides.
    #[arg(long, requires = "mutants")]
    leg: Option<String>,
}

const COMMANDS: [&str; 9] = [
    "boot",
    "run",
    "status",
    "cleanup",
    "logs",
    "supervise",
    "schema-check",
    "secret-scan",
    "gate",
];

fn emit(outcome: Outcome) -> ExitCode {
    println!("{}", outcome.doc);
    ExitCode::from(outcome.code)
}

/// No `--instance` boots the overseer + builder pair.
fn boot_cmd(
    ws: Workspace,
    session: String,
    instances: Vec<String>,
    cli_version: String,
) -> ExitCode {
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

fn run_cmd(ws: &Workspace, args: RunArgs) -> ExitCode {
    if args.leg.as_deref().is_some_and(|l| !valid_session_id(l)) {
        return emit(Outcome::usage(Some("run"), "invalid-leg"));
    }
    let named = Selection {
        unit: args.unit,
        integration: args.integration,
        mutants: args.mutants,
        coverage: args.coverage,
        fuzz_replay: args.fuzz_replay,
    };
    emit(run_with(
        ws,
        Selection::from_flags(named, args.all),
        args.filter.as_deref(),
        std::env::var("AGENT_RUN_CHUNK_BASE").ok(),
        args.leg.as_deref(),
        &mut run_forwarding,
    ))
}

fn cleanup_cmd(ws: &Workspace, session: Option<&str>, all: bool) -> ExitCode {
    let keep = std::env::var("AGENT_RUN_KEEP_HOMES").is_ok_and(|v| v == "1");
    let target = if all {
        Target::All
    } else {
        Target::Session(session.unwrap_or("default"))
    };
    emit(cleanup(ws, target, keep))
}

/// `logs` streams ndjson lines instead of one document.
fn logs_cmd(ws: &Workspace, session: &str, filter: &Filter<'_>) -> ExitCode {
    match logs(ws, session, filter) {
        Ok(lines) => {
            for line in lines {
                println!("{line}");
            }
            ExitCode::SUCCESS
        }
        Err(out) => emit(out),
    }
}

fn gate_cmd(
    ws: &Workspace,
    require: &str,
    artifacts: Option<PathBuf>,
    mutants_legs: Option<&str>,
) -> ExitCode {
    let Some(require) = parse_require(require) else {
        return emit(Outcome::usage(Some("gate"), "unknown-suite"));
    };
    let legs = match mutants_legs.map(parse_legs) {
        Some(None) => return emit(Outcome::usage(Some("gate"), "invalid-leg")),
        Some(Some(legs)) => Some(legs),
        None => None,
    };
    let artifacts = artifacts.unwrap_or_else(|| ws.artifacts());
    emit(gate(&artifacts, &require, legs.as_deref()))
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
        } => boot_cmd(ws, session, instances, cli_version),
        Cmd::Run(args) => run_cmd(&ws, args),
        Cmd::Status { session } => emit(status(&ws, &session)),
        Cmd::Cleanup { session, all } => cleanup_cmd(&ws, session.as_deref(), all),
        Cmd::Logs {
            session,
            instance,
            process,
        } => {
            let filter = Filter {
                instance: instance.as_deref(),
                process: process.as_deref(),
            };
            logs_cmd(&ws, &session, &filter)
        }
        Cmd::Supervise { session } => ExitCode::from(supervise(&ws, &session).code),
        Cmd::SchemaCheck => emit(schema_check(&ws)),
        Cmd::SecretScan => emit(secret_scan(&ws)),
        Cmd::Gate {
            require,
            artifacts,
            mutants_legs,
        } => gate_cmd(&ws, &require, artifacts, mutants_legs.as_deref()),
    }
}
