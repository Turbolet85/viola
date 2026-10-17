//! `viola-harness`: parse, call `viola_e2e::harness`, print one JSON document, exit 0 · 1 · 2.
//! (`logs` streams ndjson instead; `supervise` prints nothing — its document is its file.)
//! `schema-check`, `secret-scan` and `gate` are internal CI gate bodies, not agent commands.

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Args, Parser, Subcommand};
use viola_e2e::harness::boot::{AgentsMode, BootOptions, DEFAULT_CLI_VERSION, InstanceSpec, boot};
use viola_e2e::harness::cleanup::{Target, cleanup};
use viola_e2e::harness::gate::{gate, parse_require};
use viola_e2e::harness::logs::{Query, query};
use viola_e2e::harness::pre_push::{PRE_PUSH_HOST_SUPPORTED, pre_push};
use viola_e2e::harness::run::{Selection, run_forwarding, run_with};
use viola_e2e::harness::schema_check::schema_check;
use viola_e2e::harness::secret_scan::secret_scan;
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
        /// Skip boot step 4: the home is not stamped through `viola verify`.
        #[arg(long)]
        unstamped: bool,
        /// Plant the stamped home's statusline source naming the fake agent's `statusline-echo`.
        #[arg(long, conflicts_with = "unstamped")]
        statusline_echo: bool,
        /// How the session's fake agent answers `claude agents --json`.
        #[arg(long, value_enum, default_value_t = AgentsMode::Recorded)]
        agents_mode: AgentsMode,
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
        #[arg(long)]
        kind: Option<String>,
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
    },
    PrePush,
}

#[derive(Args)]
struct RunArgs {
    #[arg(long)]
    unit: bool,
    #[arg(long)]
    integration: bool,
    #[arg(long)]
    e2e: bool,
    #[arg(long)]
    mutants: bool,
    #[arg(long)]
    coverage: bool,
    #[arg(long)]
    fuzz_replay: bool,
    #[arg(long)]
    perf: bool,
    /// One `viola verify` against the real `claude`; refused under `CI`.
    #[arg(long)]
    local_live: bool,
    #[arg(long)]
    all: bool,
    #[arg(long)]
    filter: Option<String>,
    /// Scopes the mutation run to this source (repeatable): the inner loop, never a verdict.
    #[arg(long = "file", requires = "mutants")]
    files: Vec<String>,
    /// Mutates this whole member instead of the chunk diff (the boundary tier's form).
    #[arg(long, requires = "mutants")]
    package: Option<String>,
}

const COMMANDS: [&str; 10] = [
    "boot",
    "run",
    "status",
    "cleanup",
    "logs",
    "supervise",
    "schema-check",
    "secret-scan",
    "gate",
    "pre-push",
];

fn emit(outcome: Outcome) -> ExitCode {
    println!("{}", outcome.doc);
    ExitCode::from(outcome.code)
}

/// The boot flags past the session and its instances.
struct BootFlags {
    cli_version: String,
    unstamped: bool,
    statusline_echo: bool,
    agents_mode: AgentsMode,
}

/// No `--instance` boots the overseer + builder pair; `--unstamped` skips the stamp. `None` when an
/// instance does not parse.
fn boot_options(
    ws: Workspace,
    session: String,
    instances: Vec<String>,
    flags: BootFlags,
) -> Option<BootOptions> {
    let raw = if instances.is_empty() {
        vec!["overseer".to_owned(), "builder".to_owned()]
    } else {
        instances
    };
    let instances = raw
        .iter()
        .map(|r| InstanceSpec::parse(r))
        .collect::<Option<_>>()?;
    Some(BootOptions {
        bin_dir: ws.harness_bins(),
        ws,
        session,
        instances,
        cli_version: flags.cli_version,
        build: true,
        stamp: !flags.unstamped,
        statusline_echo: flags.statusline_echo,
        agents_mode: flags.agents_mode,
    })
}

fn boot_cmd(ws: Workspace, session: String, instances: Vec<String>, flags: BootFlags) -> ExitCode {
    match boot_options(ws, session, instances, flags) {
        Some(opts) => emit(boot(&opts)),
        None => emit(Outcome::usage(Some("boot"), "invalid-instance")),
    }
}

fn run_cmd(ws: &Workspace, args: RunArgs) -> ExitCode {
    let named = Selection {
        unit: args.unit,
        integration: args.integration,
        e2e: args.e2e,
        mutants: args.mutants,
        coverage: args.coverage,
        fuzz_replay: args.fuzz_replay,
        perf: args.perf,
        local_live: args.local_live,
        files: args.files,
        package: args.package,
    };
    emit(run_with(
        ws,
        Selection::from_flags(named, args.all),
        args.filter.as_deref(),
        std::env::var("AGENT_RUN_CHUNK_BASE").ok(),
        std::env::var_os("CI").is_some(),
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
fn logs_cmd(ws: &Workspace, session: &str, filter: &Query<'_>) -> ExitCode {
    match query(ws, session, filter) {
        Ok(lines) => {
            for line in lines {
                println!("{line}");
            }
            ExitCode::SUCCESS
        }
        Err(out) => emit(out),
    }
}

fn gate_cmd(ws: &Workspace, require: &str, artifacts: Option<PathBuf>) -> ExitCode {
    let Some(require) = parse_require(require) else {
        return emit(Outcome::usage(Some("gate"), "unknown-suite"));
    };
    let artifacts = artifacts.unwrap_or_else(|| ws.artifacts());
    emit(gate(&artifacts, &require))
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
            unstamped,
            statusline_echo,
            agents_mode,
        } => {
            let flags = BootFlags {
                cli_version,
                unstamped,
                statusline_echo,
                agents_mode,
            };
            boot_cmd(ws, session, instances, flags)
        }
        Cmd::Run(args) => run_cmd(&ws, args),
        Cmd::Status { session } => emit(status(&ws, &session)),
        Cmd::Cleanup { session, all } => cleanup_cmd(&ws, session.as_deref(), all),
        Cmd::Logs {
            session,
            instance,
            process,
            kind,
        } => {
            let filter = Query {
                instance: instance.as_deref(),
                process: process.as_deref(),
                kind: kind.as_deref(),
            };
            logs_cmd(&ws, &session, &filter)
        }
        Cmd::Supervise { session } => ExitCode::from(supervise(&ws, &session).code),
        Cmd::SchemaCheck => emit(schema_check(&ws)),
        Cmd::SecretScan => emit(secret_scan(&ws)),
        Cmd::Gate { require, artifacts } => gate_cmd(&ws, &require, artifacts),
        Cmd::PrePush => emit(pre_push(&ws, PRE_PUSH_HOST_SUPPORTED, &mut run_forwarding)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_args(argv: &[&str]) -> Option<RunArgs> {
        let argv = ["viola-harness", "run"].iter().chain(argv);
        match Cli::try_parse_from(argv).ok()?.command {
            Cmd::Run(args) => Some(args),
            _ => None,
        }
    }

    #[test]
    fn run_file_is_repeatable_and_needs_mutants() {
        let args = run_args(&["--mutants", "--file", "a.rs", "--file", "b/c.rs"]).expect("parsed");
        assert_eq!(args.files, ["a.rs", "b/c.rs"]);
        assert!(run_args(&["--mutants"]).expect("parsed").files.is_empty());
        assert!(run_args(&["--unit", "--file", "a.rs"]).is_none());
    }

    #[test]
    fn run_package_needs_mutants() {
        let args = run_args(&["--mutants", "--package", "viola-e2e"]).expect("parsed");
        assert_eq!(args.package.as_deref(), Some("viola-e2e"));
        assert!(run_args(&["--mutants"]).expect("parsed").package.is_none());
        assert!(run_args(&["--unit", "--package", "viola-e2e"]).is_none());
        assert!(run_args(&["--package", "viola-e2e"]).is_none());
    }

    #[test]
    fn run_perf_is_a_flag_of_its_own() {
        assert!(run_args(&["--perf"]).expect("parsed").perf);
        assert!(!run_args(&["--all"]).expect("parsed").perf);
    }

    #[test]
    fn run_local_live_is_a_flag_of_its_own() {
        assert!(run_args(&["--local-live"]).expect("parsed").local_live);
        assert!(!run_args(&["--all"]).expect("parsed").local_live);
    }

    #[test]
    fn boot_stamps_unless_unstamped() {
        let unstamped = |argv: &[&str]| {
            let argv = ["viola-harness", "boot"].iter().chain(argv);
            match Cli::try_parse_from(argv).expect("parsed").command {
                Cmd::Boot { unstamped, .. } => unstamped,
                _ => panic!("not boot"),
            }
        };
        assert!(!unstamped(&[]));
        assert!(unstamped(&["--unstamped"]));
    }

    fn boot_opts(argv: &[&str]) -> Option<BootOptions> {
        let argv = ["viola-harness", "boot"].iter().chain(argv);
        match Cli::try_parse_from(argv).expect("parsed").command {
            Cmd::Boot {
                session,
                instances,
                cli_version,
                unstamped,
                statusline_echo,
                agents_mode,
            } => boot_options(
                Workspace::from_build(),
                session,
                instances,
                BootFlags {
                    cli_version,
                    unstamped,
                    statusline_echo,
                    agents_mode,
                },
            ),
            _ => panic!("not boot"),
        }
    }

    #[test]
    fn boot_flags_agents_mode_parses_and_defaults_to_recorded() {
        assert_eq!(
            boot_opts(&[]).expect("options").agents_mode,
            AgentsMode::Recorded
        );
        for (word, mode) in [
            ("recorded", AgentsMode::Recorded),
            ("oversize", AgentsMode::Oversize),
            ("malformed", AgentsMode::Malformed),
        ] {
            let opts = boot_opts(&["--agents-mode", word]).expect("options");
            assert_eq!(opts.agents_mode, mode, "{word}");
        }
        let argv = ["viola-harness", "boot", "--agents-mode", "loud"];
        assert!(Cli::try_parse_from(argv).is_err(), "an unknown mode");
    }

    #[test]
    fn boot_flags_statusline_echo_parses() {
        assert!(!boot_opts(&[]).expect("options").statusline_echo);
        let opts = boot_opts(&["--statusline-echo"]).expect("options");
        assert!(opts.statusline_echo && opts.stamp);
    }

    #[test]
    fn boot_flags_statusline_echo_with_unstamped_is_refused() {
        let argv = ["viola-harness", "boot", "--statusline-echo", "--unstamped"];
        assert!(Cli::try_parse_from(argv).is_err());
    }

    #[test]
    fn run_e2e_is_a_flag_of_its_own() {
        assert!(run_args(&["--e2e"]).expect("parsed").e2e);
        assert!(!run_args(&["--all"]).expect("parsed").e2e);
    }

    #[test]
    fn boot_options_stamp_unless_unstamped_and_default_to_the_pair() {
        let stamped = boot_opts(&[]).expect("options");
        assert!(stamped.stamp && stamped.build);
        let names: Vec<&str> = stamped.instances.iter().map(|i| i.name.as_str()).collect();
        assert_eq!(names, ["overseer", "builder"]);
        assert!(!boot_opts(&["--unstamped"]).expect("options").stamp);
        let one = boot_opts(&["--instance", "builder:--mode x"]).expect("options");
        assert_eq!(one.instances.len(), 1);
        assert_eq!(one.instances[0].fake_args, ["--mode", "x"]);
        assert!(boot_opts(&["--instance", "../x"]).is_none());
    }
}
