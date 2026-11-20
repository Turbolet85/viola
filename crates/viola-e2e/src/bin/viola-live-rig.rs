//! `viola-live-rig`: the standing live rig every live round reuses (test-only, never in a release
//! build). It hosts one child on a pty it owns, reads its screen into closed words, types only the
//! granted keys, keeps its frames under the gitignored `target/live-rig/`, and owns its isolation
//! and its processes.
//!
//! - `rehearse <r1|r2|hold|r3b|r3|im|v|vc|ts|tc|vr|mt|vh|cx|sf|hb|cs|cc|ws|r3w> --program <fake agent> [--viola <exe>]
//!   [--tap <exe>] [--target-home <dir> --claude-dir <dir> --user-config <file>] [--contain]
//!   [-- <child args>]`: one start over the fake agent (CI runs these); `vh` refuses `usage` without
//!   all three of its paths; `--contain` (`sf` alone, else `usage`) runs the child inside the
//!   network-closed `bwrap` sandbox every live `SF` start runs in; `{cwd}` and `{config}` in a child argument stand for the start's cwd and private config
//!   dir; `cs` is `v` keeping its stamp in `<out>/context-stamp.json` and reading its private
//!   config's top level, `cc` is `cx` seeding that kept stamp, then sending a prompt, then
//!   `/context`, then reading the hint's cells around one `x` and one DEL; `ws` is `v` keeping its
//!   stamp in `<out>/wake-stamp.json`, `r3w` is `r3` seeding that kept stamp and reading viola's own
//!   `wheel` lines after the fired wake, then sending again; `hb` reads a fresh box's hint cells
//!   around one `x` and one DEL, unwrapped, with no send.
//! - `live <R1|R2|R3B|R3|IM|S0|S0R|S0B|S0C|S1|S2|BP|BK|R4|R4M|L1|L2|SF|CS|CC|WS|HB|R3W|…-2|R3-326|R3D> --cli <version> --cap <n> --spare <n> --ledger <file>`: one live start
//!   of the stamped `claude`, refused under `CI`; `S0` / `S0R` / `S0B` run `viola verify` over it, `S0C`
//!   reads its first frame at 80×24 and stops it with no key; `S1` runs `viola verify` in a trusted
//!   git folder, then `viola run` in a fresh untrusted one, and `S2` a plain `claude` in one, each
//!   first frame read; `SF` runs `viola run` inside the network-closed sandbox, refused
//!   `sandbox` before its ledger line where the sandbox cannot be made; `CS` runs `viola verify`
//!   and keeps its stamp, and `CC` runs `viola run` under that kept stamp, refused `stamp` before
//!   its ledger line where none stands; `WS` runs `viola verify` and keeps its stamp, `R3W` runs
//!   `viola run` under that kept stamp, refused `stamp` before its ledger line where none stands,
//!   and `HB` a plain `claude` whose fresh box it reads, no send.
//! - `census [--kill]`: the processes an earlier, dead rig left, and their kill.
//! - `capture --out <file>` (hidden): a wake start's UserPromptSubmit hook and `SF`'s StopFailure
//!   hook, one closed-word line per payload.
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
        /// `mt`: the `viola-mcp-tap` its MCP server runs.
        #[arg(long)]
        tap: Option<PathBuf>,
        /// `vh`: the viola home verify stamps; required, so a rehearsal never reaches a real one.
        #[arg(long)]
        target_home: Option<PathBuf>,
        /// `vh`: the Claude dir whose `settings*.json` are read by key names; required.
        #[arg(long)]
        claude_dir: Option<PathBuf>,
        /// `sf`: the child inside the network-closed sandbox.
        #[arg(long)]
        contain: bool,
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
        run_args: Vec::new(),
        viola: None,
        tap: None,
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
        target_home: None,
        claude_dir: None,
        cwd: None,
        contain: false,
    })
}

/// The rig's current directory, canonical: `VH`'s verify cwd.
fn current_dir() -> Option<PathBuf> {
    std::env::current_dir()
        .ok()
        .and_then(|d| std::fs::canonicalize(d).ok())
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
            tap,
            target_home,
            claude_dir,
            contain,
            common,
            child_args,
        } => {
            let (arm, label) = match arm.as_str() {
                "cs" => (Arm::V, rig::REHEARSE_CS.to_owned()),
                "cc" => (Arm::CX, rig::REHEARSE_CC.to_owned()),
                "ws" => (Arm::V, rig::REHEARSE_WS.to_owned()),
                "r3w" => (Arm::R3, rig::REHEARSE_R3W.to_owned()),
                word => match Arm::parse(word) {
                    Some(arm) => (arm, format!("rehearse-{arm:?}")),
                    None => return usage(),
                },
            };
            if contain && arm != Arm::SF {
                return usage();
            }
            if arm == Arm::VH
                && (target_home.is_none() || claude_dir.is_none() || common.user_config.is_none())
            {
                return usage();
            }
            let Some(mut plan) = plan_of(label, arm, false, common) else {
                return usage();
            };
            plan.program = program;
            plan.child_args = child_args;
            plan.viola = viola;
            plan.tap = tap;
            plan.contain = contain;
            if arm == Arm::VH {
                plan.target_home = target_home;
                plan.claude_dir = claude_dir;
                plan.cwd = current_dir();
            }
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
            // Run F's argv for `VC`: the program and `--model haiku` alone; `S1`'s verify takes
            // none and its `viola run` step that pair; `S2` the pair.
            if arm.wrapped().is_some() || matches!(arm, Arm::VC | Arm::TC) {
                plan.child_args = vec!["--model".to_owned(), "haiku".to_owned()];
            }
            if arm == Arm::TS {
                plan.run_args = vec!["--model".to_owned(), "haiku".to_owned()];
            }
            let sibling = |name: &str| {
                std::env::current_exe()
                    .ok()
                    .and_then(|exe| exe.parent().map(|d| d.join(name)))
            };
            if arm.has_vhome() {
                plan.viola = sibling("viola");
            }
            if arm == Arm::MT {
                plan.tap = sibling("viola-mcp-tap");
            }
            if arm == Arm::VH {
                // The real home's verify runs the release build of the tree, from its root.
                plan.viola = Some(repo_root().join("target/release/viola"));
                plan.target_home = Some(user_home().join(".viola"));
                plan.claude_dir = Some(user_home().join(".claude"));
                plan.cwd = current_dir();
            }
            // Every live `SF` start is contained, never by an option.
            plan.contain = arm == Arm::SF;
            plan.cli_dir = Some(dir);
            plan.cli_version = Some(cli);
            plan
        }
    };
    ExitCode::from(rig::start(&plan))
}
