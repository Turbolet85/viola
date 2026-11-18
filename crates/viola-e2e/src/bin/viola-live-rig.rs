//! `viola-live-rig`: the standing live rig every live round reuses (test-only, never in a release
//! build). It hosts one child on a pty it owns, reads its screen into closed words, types only the
//! granted keys, keeps its frames under the gitignored `target/live-rig/`, and owns its isolation
//! and its processes.
//!
//! - `rehearse <r1|r2|hold|r3b|r3|im|v|vc> --program <fake agent> [-- <child args>]`: one start over the fake
//!   agent (CI runs these); `{cwd}` and `{config}` in a child argument stand for the start's cwd and
//!   private config dir.
//! - `live <R1|R2|R3B|R3|IM|S0|S0R|S0B|S0C|…-2|R3-326|R3D> --cli <version> --cap <n> --spare <n> --ledger <file>`: one live start
//!   of the stamped `claude`, refused under `CI`; `S0` / `S0R` / `S0B` run `viola verify` over it, `S0C`
//!   reads its first frame at 80×24 and stops it with no key.
//! - `census [--kill]`: the processes an earlier, dead rig left, and their kill.
//! - `capture --out <file>` (hidden): a wake start's UserPromptSubmit hook, one closed-word line
//!   per payload.
//!
//! Exit: 0 a whole start · 1 a start that read red · 2 usage · 3 refused before the ledger line.

use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Duration;

use clap::{Args, Parser, Subcommand};
use viola_e2e::rig::keys::Grant;
use viola_e2e::rig::procs::{self, Scope};
use viola_e2e::rig::wake::Wake;
use viola_e2e::rig::{self, Arm, Plan, capture, isolate};

#[derive(Parser)]
#[command(name = "viola-live-rig")]
struct Cli {
    #[command(subcommand)]
    verb: Verb,
}

#[derive(Subcommand)]
enum Verb {
    Rehearse {
        arm: String,
        #[arg(long)]
        program: PathBuf,
        #[arg(long)]
        viola: Option<PathBuf>,
        #[command(flatten)]
        common: Common,
        #[arg(last = true)]
        child_args: Vec<String>,
    },
    Live {
        label: String,
        #[arg(long)]
        cli: String,
        #[command(flatten)]
        common: Common,
    },
    Census {
        #[arg(long)]
        kill: bool,
    },
    #[command(hide = true)]
    Capture {
        #[arg(long)]
        out: PathBuf,
    },
}

#[derive(Args)]
struct Common {
    #[arg(long)]
    grant: Option<String>,
    #[arg(long)]
    ledger: Option<PathBuf>,
    #[arg(long)]
    cap: Option<usize>,
    #[arg(long, default_value_t = 0)]
    spare: usize,
    #[arg(long)]
    credentials: Option<PathBuf>,
    #[arg(long)]
    user_config: Option<PathBuf>,
    #[arg(long)]
    scratch_base: Option<PathBuf>,
    #[arg(long)]
    out: Option<PathBuf>,
    /// Seconds before the watchdog stops the start.
    #[arg(long, default_value_t = 600)]
    deadline: u64,
    /// Seconds one settle waits at most.
    #[arg(long, default_value_t = 60)]
    settle: u64,
    /// R3: the wake form its send asks for, `interval` or `dynamic`.
    #[arg(long, default_value = "interval")]
    wake: String,
    /// R3: seconds the start stands after its send for the wake to fire.
    #[arg(long, default_value_t = 180)]
    stand: u64,
}

fn user_home() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_default()
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// The plan's paths and bounds from the shared options.
fn plan_of(label: String, arm: Arm, live: bool, common: Common) -> Option<Plan> {
    Some(Plan {
        label,
        arm,
        live,
        program: PathBuf::new(),
        child_args: Vec::new(),
        viola: None,
        cli_dir: None,
        cli_version: None,
        grant: Grant::parse(common.grant.as_deref())?,
        ledger: common.ledger,
        cap: common.cap,
        spare: common.spare,
        credentials: common
            .credentials
            .unwrap_or_else(|| user_home().join(".claude/.credentials.json")),
        user_config: common
            .user_config
            .unwrap_or_else(|| user_home().join(".claude.json")),
        scratch_base: common
            .scratch_base
            .unwrap_or_else(isolate::default_scratch_base),
        out: common
            .out
            .unwrap_or_else(|| repo_root().join("target/live-rig")),
        deadline: Duration::from_secs(common.deadline),
        settle: Duration::from_secs(common.settle),
        wake: Wake::parse(&common.wake)?,
        stand: Duration::from_secs(common.stand),
        rig: std::env::current_exe()
            .ok()
            .and_then(|exe| std::fs::canonicalize(exe).ok()),
    })
}

fn usage() -> ExitCode {
    println!("refused: usage");
    ExitCode::from(2)
}

fn main() -> ExitCode {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(_) => return usage(),
    };
    let plan = match cli.verb {
        Verb::Census { kill } => {
            if kill {
                let (killed, left) = procs::sweep(Scope::Orphans);
                println!("census: {killed} killed, {left} left");
                return ExitCode::from(u8::from(left > 0));
            }
            println!("census: {}", procs::census(Scope::Orphans).len());
            return ExitCode::SUCCESS;
        }
        Verb::Capture { out } => {
            capture::run(std::io::stdin().lock(), &out);
            return ExitCode::SUCCESS;
        }
        Verb::Rehearse {
            arm,
            program,
            viola,
            common,
            child_args,
        } => {
            let Some(arm) = Arm::parse(&arm) else {
                return usage();
            };
            let Some(mut plan) = plan_of(format!("rehearse-{arm:?}"), arm, false, common) else {
                return usage();
            };
            plan.program = program;
            plan.child_args = child_args;
            plan.viola = viola;
            plan
        }
        Verb::Live { label, cli, common } => {
            let Some(arm) = Arm::of_label(&label) else {
                return usage();
            };
            let Some(mut plan) = plan_of(label, arm, true, common) else {
                return usage();
            };
            let dir = user_home()
                .join(".local/share/mise/installs/claude")
                .join(&cli);
            plan.program = dir.join("claude");
            // Run F's argv for `VC`: the program and `--model haiku` alone.
            if arm.wrapped().is_some() || arm == Arm::VC {
                plan.child_args = vec!["--model".to_owned(), "haiku".to_owned()];
            }
            if arm.has_vhome() {
                plan.viola = std::env::current_exe()
                    .ok()
                    .and_then(|exe| exe.parent().map(|d| d.join("viola")));
            }
            plan.cli_dir = Some(dir);
            plan.cli_version = Some(cli);
            plan
        }
    };
    ExitCode::from(rig::start(&plan))
}
