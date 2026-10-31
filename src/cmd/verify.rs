//! `viola verify [--record <dir>] [-- <program> [args…]]`: measures the capability ledger against
//! the local `claude` (or the program after `--`) with one print-mode probe through a capture
//! plugin and five interactive PTY runs (`typed`: untrusted, trusted, dialogs, plan — those two
//! answered by their own capture hook — and resume) with one `claude agents --json` read while the
//! trusted run's session is live, prints one step line per row and the `stamped`
//! summary on stdout, and writes the stamps as their only writer (security-plan §Data Protection;
//! architecture [CLI Version Compatibility]). With `--record`, a clean run's payloads become
//! scrubbed fixtures and its screens signature-only `Screen.<phase>.json` fixtures. Stopped by
//! Ctrl-C or TERM before its rows are checked, it ends its child with no key, removes the dirs that
//! run made, prints one refusal pair and exits 1 with no stamp written.

mod stop;
mod typed;

use std::ffi::OsString;
use std::fs::File;
use std::io::Read as _;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::Arc;
use std::time::{Duration, Instant};

use chrono::Utc;
use tracing::instrument;
use viola_agent_claude::hook::HookEvent;
use viola_agent_claude::ledger::{
    self, Capture, DialogRun, DialogRuns, LedgerRow, PROBE_ANSWERS, PROBE_PROMPT, ProbePlugin,
    ProbeRun, Probes, ShellVerdict, StatusLineCapture, TypedRun,
};
use viola_agent_claude::statusline;
use viola_agent_claude::{AgentError, CASE_INSENSITIVE, PLUGIN_DIR_FLAG, PROGRAM, StripPlan};
use viola_core::obs::{ObsEvent, ObsProcess};
use viola_core::{Clock, MAX_FRAME, SystemClock, ViolaName, obs_event};
use viola_state::fs::{FILE_MODE, create_private_dir, replace_private};
use viola_state::pin::{PinError, pin_exe};
use viola_state::stamps::{StampsError, ledger_dir, update_stamps};
use viola_state::strict::check_home_tree;

use crate::run::version_gate::{Bounded, VERSION_DEADLINE, run_bounded};
use crate::{human, obs, run};

/// The bound on the print-mode probe: one short Haiku turn.
const PROBE_DEADLINE: Duration = Duration::from_secs(120);

#[derive(clap::Args)]
pub(crate) struct VerifyArgs {
    /// Record the probe's hook payloads, scrubbed, under <DIR>/<cli version>/
    #[arg(long, value_name = "DIR")]
    record: Option<PathBuf>,
    /// The CLI to verify and its arguments, after `--` (default: claude)
    #[arg(last = true)]
    program: Vec<OsString>,
}

/// With an instance (`VIOLA_NAME`) the run is logged to `cli-<name>.ndjson`; without one no
/// process-log file is written.
#[instrument(skip_all, name = "cli.verify")]
pub(crate) fn verify(
    home: &Path,
    instance: Option<&ViolaName>,
    args: &VerifyArgs,
) -> anyhow::Result<ExitCode> {
    let home = std::path::absolute(home)?;
    // Before the log opens and before any probe: opening the log narrows the home to its owner,
    // which would erase the very state the check refuses. A refused check prints the home's pair
    // and nothing else, and writes no line.
    if check_home_tree(&home, None, None).is_err() {
        refuse_home();
        return Ok(ExitCode::from(1));
    }
    if let Some(name) = instance {
        let (level, rejection) = obs::read_diagnostics_level(&home);
        obs::viola_obs_init(&home, ObsProcess::Cli, Some(name.clone()), level)?;
        run::log_self_start();
        if let Some(rejection) = rejection {
            obs::log_config_rejection(rejection);
        }
    }
    let (code, detail) = match measure(&home, args)? {
        Ended::Exit(code) => (code, None),
        Ended::Interrupted => {
            human::refuse(
                "verify was stopped before it finished",
                "run viola verify again to stamp this CLI",
            );
            (1, Some("interrupted"))
        }
    };
    if instance.is_some() {
        run::log_self_exit(code, detail);
    }
    Ok(ExitCode::from(code))
}

/// The home's pair: the home or a folder `verify` trusts can be written or read by another user.
fn refuse_home() {
    let (unable, hint) = human::home_state_pair();
    human::refuse(unable, hint);
}

/// How a measurement ended: with an exit code, or stopped before its rows were checked.
enum Ended {
    Exit(u8),
    Interrupted,
}

/// `run_bounded` between a `process-start` / `process-exit` pair for `subject` (obs-plan §6 Child /
/// shell spawns), ended early by the stop. The pair lives at the call site, not in `run_bounded`,
/// which `run`'s version gate shares and logs itself. Without an instance no subscriber is
/// installed and both are no-ops.
fn run_logged(
    subject: &'static str,
    program: &Path,
    args: &[OsString],
    cwd: &Path,
    strip: &StripPlan,
    deadline: Duration,
) -> std::io::Result<Bounded> {
    let started = Instant::now();
    obs_event!(INFO, ObsEvent::ProcessStart, subject = subject);
    let ran = run_bounded(program, args, cwd, strip, deadline, stop::stopped);
    let status = ran
        .as_ref()
        .ok()
        .and_then(|r| r.status)
        .and_then(|s| s.code());
    obs_event!(
        INFO,
        ObsEvent::ProcessExit,
        subject = subject,
        child_exit_status = status,
        duration_ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX),
    );
    ran
}

/// The stop is read at each child's end and once more after the interactive runs; once the rows
/// are being checked the run completes.
fn measure(home: &Path, args: &VerifyArgs) -> anyhow::Result<Ended> {
    stop::install();
    let (program, program_args) = match args.program.split_first() {
        Some((program, rest)) => (program.clone(), rest.to_vec()),
        None => (OsString::from(PROGRAM), Vec::new()),
    };
    let cwd = std::env::current_dir()?;
    // A given path resolves as given, so its existence is checked here.
    let resolved = run::resolve_program(&program, &cwd).and_then(|p| {
        if p.is_file() {
            Ok(p)
        } else {
            Err(AgentError::ProgramNotFound)
        }
    });
    let program = match resolved {
        Ok(program) => program,
        Err(AgentError::BatchScriptChild) => {
            human::refuse(
                "the claude CLI is a .cmd or .bat script",
                "pass the real executable, not a .cmd or .bat shim",
            );
            return Ok(Ended::Exit(1));
        }
        Err(_) => {
            human::refuse(
                "the claude CLI was not found",
                "install Claude Code or put it on PATH",
            );
            return Ok(Ended::Exit(1));
        }
    };
    let pinned = match pin_exe(home, &std::env::current_exe()?) {
        Ok(pinned) => pinned,
        Err(PinError::HashMismatch) => {
            super::run::refuse_tampered_pin();
            return Ok(Ended::Exit(1));
        }
        Err(error) => return Err(error.into()),
    };
    let (persistent, rejection) = run::persistent_names(home);
    if let Some(rejection) = rejection {
        obs::log_config_rejection(rejection);
    }
    let strip = viola_agent_claude::plan_strip(std::env::vars_os().map(|(k, _)| k), &persistent);

    let mut version_args = program_args.clone();
    version_args.push("--version".into());
    let answer = run_logged(
        "version-probe",
        &program,
        &version_args,
        &cwd,
        &strip,
        VERSION_DEADLINE,
    )?;
    if answer.stopped {
        return Ok(Ended::Interrupted);
    }
    let Some(version) = ledger::parse_version(&answer.stdout) else {
        human::refuse(
            "the CLI version could not be read",
            "run claude --version to check the install",
        );
        return Ok(Ended::Exit(1));
    };

    let probe = ProbeDir::create(home, &pinned.path_fwd)?;
    let mut probe_args = program_args.clone();
    probe_args
        .extend(["-p", PROBE_PROMPT, "--model", "haiku", PLUGIN_DIR_FLAG].map(OsString::from));
    probe_args.push(probe.plugin().into_os_string());
    probe_args.push("--no-session-persistence".into());
    let printed = run_logged(
        "verify-probe",
        &program,
        &probe_args,
        &probe.dir,
        &strip,
        PROBE_DEADLINE,
    )?;
    if printed.stopped {
        return Ok(Ended::Interrupted);
    }
    let print = ProbeRun {
        program_is_script: viola_agent_claude::is_script(&program),
        version_answered: true,
        captures: read_captures(&probe.captures()),
    };
    let clock: Arc<dyn Clock> = Arc::new(SystemClock);
    let [trusted, questions, plan, resume] = [
        probe.typed(),
        probe.questions(),
        probe.plan(),
        probe.resume(),
    ]
    .map(|root| (root.join("plugin"), root.join("captures")));
    let statusline_settings = probe.statusline_settings();
    let typed = typed::measure(
        &clock,
        &typed::Inputs {
            program: &program,
            program_args: &program_args,
            cwd: &cwd,
            strip: &strip,
            stop: stop::stopped,
            trusted: dirs(&trusted),
            statusline_settings: statusline_settings
                .is_file()
                .then_some(statusline_settings.as_path()),
            questions: dirs(&questions),
            plan: dirs(&plan),
            resume: dirs(&resume),
        },
    )?;
    let Some((typed, agents)) = typed else {
        return Ok(Ended::Interrupted);
    };
    let dialogs = DialogRuns {
        questions: read_captures(&questions.1),
        plan: read_captures(&plan.1),
    };
    let probes = Probes {
        print,
        typed,
        trusted: read_captures(&trusted.1),
        dialogs,
        statusline: read_statusline(&probe.statusline_captures()),
        resumed: read_captures(&resume.1),
        agents,
    };
    drop(probe);
    if stop::stopped() {
        return Ok(Ended::Interrupted);
    }

    let results = check_rows(&probes);
    let written_at = obs::timestamp(Utc::now());
    let largest = ledger::largest(&probes.print.captures);
    let written = update_stamps(home, |existing| {
        ledger::merge_stamp(
            existing,
            &version,
            &results,
            &largest,
            &probes.typed,
            ledger::parallel_both_before_first_post(&probes.dialogs.questions),
            &written_at,
        )
    });
    match written {
        Ok(()) => {}
        Err(StampsError::StrictModes(_)) => {
            refuse_home();
            return Ok(Ended::Exit(1));
        }
        Err(error) => return Err(error.into()),
    }
    let failed = results.iter().filter(|(_, pass)| !pass).count();
    human::result(&format!(
        "stamped {version}  {} pass  {failed} fail",
        results.len() - failed
    ));
    if failed > 0 {
        return Ok(Ended::Exit(1));
    }
    match &args.record {
        Some(dir) => record(dir, &version, &probes).map(Ended::Exit),
        None => Ok(Ended::Exit(0)),
    }
}

/// `<home>/ledger/probes/<pid>/` (0700) with the capture plugin in `plugin/` and an empty
/// `captures/`, the same pair under `typed/` for the trusted interactive run with its empty
/// `statusline/` and, where the status line is wrapped, its probe settings `statusline.json`, under
/// `questions/` and `plan/` a dialog plugin, its captures and its `answers/` (written from the
/// compiled answer table) for the dialog and plan runs, and under `resume/` the capture plugin and
/// its captures for the resume run; removed whole when dropped, on every exit path.
struct ProbeDir {
    dir: PathBuf,
}

/// A run's `(plugin, captures)` pair as the runs take it.
fn dirs((plugin, captures): &(PathBuf, PathBuf)) -> typed::RunDirs<'_> {
    typed::RunDirs { plugin, captures }
}

/// `path` with forward slashes, the form the plugin's `hooks.json` carries.
fn fwd(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

impl ProbeDir {
    fn create(home: &Path, pinned_bin_fwd: &str) -> anyhow::Result<Self> {
        let dir = ledger_dir(home)
            .join("probes")
            .join(std::process::id().to_string());
        let _ = std::fs::remove_dir_all(&dir);
        let probe = Self { dir };
        for root in [probe.dir.clone(), probe.typed(), probe.resume()] {
            write_plugin(&root, pinned_bin_fwd, None)?;
        }
        let statusline = probe.statusline_captures();
        create_private_dir(&statusline)?;
        if let Some(document) = statusline::probe_document(pinned_bin_fwd, &fwd(&statusline)) {
            replace_private(&probe.statusline_settings(), document.as_bytes(), FILE_MODE)?;
        }
        for (root, run) in [
            (probe.questions(), DialogRun::Questions),
            (probe.plan(), DialogRun::Plan),
        ] {
            let answers = root.join("answers");
            create_private_dir(&answers)?;
            for (_, event, ordinal, answer) in PROBE_ANSWERS.iter().filter(|a| a.0 == run) {
                let path = answers.join(ledger::answer_file_name(*event, *ordinal));
                replace_private(&path, answer.id().as_bytes(), FILE_MODE)?;
            }
            write_plugin(&root, pinned_bin_fwd, Some(&fwd(&answers)))?;
        }
        Ok(probe)
    }

    fn typed(&self) -> PathBuf {
        self.dir.join("typed")
    }

    fn statusline_captures(&self) -> PathBuf {
        self.typed().join("statusline")
    }

    fn statusline_settings(&self) -> PathBuf {
        self.typed().join("statusline.json")
    }

    fn questions(&self) -> PathBuf {
        self.dir.join("questions")
    }

    fn plan(&self) -> PathBuf {
        self.dir.join("plan")
    }

    fn resume(&self) -> PathBuf {
        self.dir.join("resume")
    }

    fn plugin(&self) -> PathBuf {
        self.dir.join("plugin")
    }

    fn captures(&self) -> PathBuf {
        self.dir.join("captures")
    }
}

impl Drop for ProbeDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}

/// `<root>/captures/` and `<root>/plugin/`: a spine plugin, or with `answers` a dialog plugin.
fn write_plugin(root: &Path, pinned_bin_fwd: &str, answers: Option<&str>) -> anyhow::Result<()> {
    let captures = root.join("captures");
    create_private_dir(&captures)?;
    let kind = match answers {
        Some(answers_dir_fwd) => ProbePlugin::Dialog { answers_dir_fwd },
        None => ProbePlugin::Spine,
    };
    for (rel, content) in ledger::capture_plugin_files(pinned_bin_fwd, &fwd(&captures), kind) {
        let path = root.join("plugin").join(rel);
        create_private_dir(path.parent().unwrap_or(root))?;
        replace_private(&path, content.as_bytes(), FILE_MODE)?;
    }
    Ok(())
}

/// Every `<Event>.<k>.json` in `dir`, in `k` order (arrival order), each read through the frame
/// cap; other names are skipped.
fn read_captures(dir: &Path) -> Vec<Capture> {
    let mut found: Vec<(u32, HookEvent, PathBuf)> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|entry| {
            let (event, k) = ledger::parse_capture_file_name(entry.file_name().to_str()?)?;
            Some((k, event, entry.path()))
        })
        .collect();
    found.sort_by_key(|(k, _, _)| *k);
    found
        .into_iter()
        .filter_map(|(_, event, path)| {
            let mut bytes = Vec::new();
            File::open(path)
                .ok()?
                .take(MAX_FRAME + 1)
                .read_to_end(&mut bytes)
                .ok()?;
            Some(Capture::read(event, &bytes))
        })
        .collect()
}

/// The bound on a shell verdict file: `bin-sh` and `other` are shorter.
const SHELL_CAP: u64 = 16;

/// Every `StatusLine.<k>.json` in `dir`, in `k` order, each read through the frame cap with the
/// verdict its `StatusLine.<k>.shell` holds; an absent or unreadable verdict reads `Other`.
fn read_statusline(dir: &Path) -> Vec<StatusLineCapture> {
    let mut found: Vec<(u32, PathBuf)> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|entry| {
            let k = ledger::parse_statusline_capture_file_name(entry.file_name().to_str()?)?;
            Some((k, entry.path()))
        })
        .collect();
    found.sort_by_key(|(k, _)| *k);
    found
        .into_iter()
        .filter_map(|(k, path)| {
            let mut bytes = Vec::new();
            File::open(path)
                .ok()?
                .take(MAX_FRAME + 1)
                .read_to_end(&mut bytes)
                .ok()?;
            let mut word = Vec::new();
            let shell = File::open(dir.join(ledger::statusline_shell_file_name(k)))
                .and_then(|f| f.take(SHELL_CAP).read_to_end(&mut word))
                .map_or(ShellVerdict::Other, |_| ShellVerdict::read(&word));
            Some(StatusLineCapture { bytes, shell })
        })
        .collect()
}

/// Every row relied on this OS checked in order, its step line printed as it is checked.
fn check_rows(probes: &Probes) -> Vec<(LedgerRow, bool)> {
    let relied = LedgerRow::relied();
    let total = relied.len();
    relied
        .into_iter()
        .enumerate()
        .map(|(i, row)| (row, check_step(i + 1, total, row, probes)))
        .collect()
}

#[instrument(skip_all, name = "cli.verify_step", fields(row = row.id()))]
fn check_step(n: usize, total: usize, row: LedgerRow, probes: &Probes) -> bool {
    let pass = ledger::check(row, probes);
    human::result(&step_line(n, total, row, pass));
    pass
}

/// `[NN/NN] <row id> <row words>  pass|fail` (design-system §Surface: cli, the step counter).
fn step_line(n: usize, total: usize, row: LedgerRow, pass: bool) -> String {
    let verdict = if pass { "pass" } else { "fail" };
    format!(
        "[{n:02}/{total:02}] {} {}  {verdict}",
        row.id(),
        row.words()
    )
}

/// The first capture of each spine event, scrubbed of the user's home and name, written as
/// `<dir>/<version>/<Event>.default.json`; each dialog run's captures the same way as
/// `<Event>.<variant>.json` (`ledger::dialog_variants`), and so the trusted run's paste prompts and
/// the hooks its local command fired (`ledger::framing_variants`), its status line's two payloads
/// (`ledger::statusline_variants`) and the resume run's SessionStart (`ledger::resume_variants`); and
/// each recorded screen as
/// `Screen.<phase>.json` holding only its signature rows. One payload that stays unclean after the
/// scrub, or one screen whose kept rows or their seams hold a path, the username or an email,
/// refuses the whole recording before any file is written; the refusal names the file and the
/// check, never the content.
fn record(dir: &Path, version: &str, probes: &Probes) -> anyhow::Result<u8> {
    let user_home = std::env::home_dir().unwrap_or_default();
    let home = user_home.to_string_lossy().into_owned();
    let user = user_home
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    // Every payload and every screen is checked before the first file is written.
    let mut files = match scrubbed_payloads(named_payloads(probes), &home, &user) {
        Ok(files) => files,
        Err(named) => return Ok(refuse_recording(&named)),
    };
    match signature_screens(&probes.typed, &home, &user) {
        Ok(screens) => files.extend(screens),
        Err(named) => return Ok(refuse_recording(&named)),
    }
    write_recording(&dir.join(version), files)?;
    Ok(0)
}

/// A fixture `record` writes: its file name and its document.
type Recorded = (String, serde_json::Value);

/// The payloads `record` names, in file order: the spine's first of each event, each dialog run's
/// variants, the trusted run's framing variants, its status line variants, the resume run's variant
/// and the trusted run's own row of its agents read. A payload that does not parse is left out.
fn named_payloads(probes: &Probes) -> Vec<(String, serde_json::Value)> {
    let mut payloads: Vec<(String, Option<serde_json::Value>)> = Vec::new();
    let mut add = |name: String, capture: &Capture| payloads.push((name, capture.payload.clone()));
    for event in ledger::CAPTURE_EVENTS {
        if let Some(first) = probes.print.captures.iter().find(|c| c.event == event) {
            add(format!("{}.default.json", ledger::event_name(event)), first);
        }
    }
    for (variant, capture) in ledger::dialog_variants(&probes.dialogs) {
        add(
            format!("{}.{variant}.json", ledger::event_name(capture.event)),
            capture,
        );
    }
    for (variant, capture) in ledger::framing_variants(&probes.trusted) {
        add(
            format!("{}.{variant}.json", ledger::event_name(capture.event)),
            capture,
        );
    }
    for (name, bytes) in ledger::statusline_variants(&probes.statusline) {
        payloads.push((name, serde_json::from_slice(bytes).ok()));
    }
    for (variant, capture) in ledger::resume_variants(&probes.resumed) {
        payloads.push((
            format!("{}.{variant}.json", ledger::event_name(capture.event)),
            capture.payload.clone(),
        ));
    }
    if let Some(agents) = probes.agents.as_ref() {
        payloads.push((
            ledger::AGENTS_RECORDED.to_owned(),
            ledger::agents_variant(agents),
        ));
    }
    payloads
        .into_iter()
        .filter_map(|(name, payload)| Some((name, payload?)))
        .collect()
}

/// Each payload scrubbed; `Err` names the first one that stays unclean and its check code.
fn scrubbed_payloads(
    payloads: Vec<(String, serde_json::Value)>,
    home: &str,
    user: &str,
) -> Result<Vec<Recorded>, String> {
    let mut files = Vec::new();
    for (name, payload) in payloads {
        let scrubbed = ledger::scrub(&payload, home, user, CASE_INSENSITIVE);
        if let Some(why) = ledger::unclean(&scrubbed, user) {
            return Err(format!("{name} {}", why.code()));
        }
        files.push((name, scrubbed));
    }
    Ok(files)
}

/// Each recorded screen cut to its signature rows; `Err` names the first screen with a fault, its
/// row and its check code.
fn signature_screens(typed: &TypedRun, home: &str, user: &str) -> Result<Vec<Recorded>, String> {
    let mut files = Vec::new();
    for (phase, rows) in screens(typed) {
        let kept = ledger::signature_rows(rows);
        let name = format!("Screen.{phase}.json");
        if let Some(fault) = ledger::screen_fault(rows, &kept, home, user) {
            let at = if fault.seam { " seam" } else { "" };
            return Err(format!("{name} row {}{at} {}", fault.row, fault.why.code()));
        }
        let screen = serde_json::json!({"screen_phase": phase, "cols": 80, "rows": kept});
        files.push((name, screen));
    }
    Ok(files)
}

/// `files` under `out`, one JSON document and a newline each.
fn write_recording(out: &Path, files: Vec<Recorded>) -> std::io::Result<()> {
    std::fs::create_dir_all(out)?;
    for (name, doc) in files {
        let mut text = doc.to_string();
        text.push('\n');
        std::fs::write(out.join(name), text)?;
    }
    Ok(())
}

/// `named` is the refused file and its check code (a screen's: its row index, `seam` when the row's
/// join with a live neighbour holds it), never the content that failed.
fn refuse_recording(named: &str) -> u8 {
    human::refuse(
        &format!("a recorded fixture is not clean: {named}"),
        "record with a viola home under your user home",
    );
    1
}

/// The recorded screens by phase; a phase the run never reached is absent.
fn screens(typed: &TypedRun) -> Vec<(&'static str, &[String])> {
    [
        ("modal", &typed.modal),
        ("ready", &typed.ready),
        ("turn", &typed.turn),
    ]
    .into_iter()
    .filter_map(|(phase, rows)| rows.as_deref().map(|rows| (phase, rows)))
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs;

    #[test]
    fn step_line_is_the_counter_the_row_and_the_verdict() {
        assert_eq!(
            step_line(1, 18, LedgerRow::ShimResolution, true),
            "[01/18] shim-resolution claude resolves to a real executable  pass"
        );
        assert_eq!(
            step_line(6, 18, LedgerRow::LargestHookPayload, false),
            "[06/18] largest-hook-payload every hook payload fits the frame cap  fail"
        );
        assert_eq!(
            step_line(12, 18, LedgerRow::PlanApproveRevise, false),
            "[12/18] plan-approve-revise a plan revise and approve each take effect  fail"
        );
        assert_eq!(
            step_line(14, 18, LedgerRow::DialogConcurrency, true),
            "[14/18] dialog-concurrency two parallel questions each raise a dialog  pass"
        );
        assert_eq!(
            step_line(15, 18, LedgerRow::LongPasteWrapper, true),
            "[15/18] long-paste-wrapper a long paste unwraps to the text as pasted  pass"
        );
        assert_eq!(
            step_line(16, 18, LedgerRow::TagEscaping, false),
            "[16/18] tag-escaping tag-like text un-escapes to the text as pasted  fail"
        );
        let clear = step_line(17, 18, LedgerRow::LocalCommandClear, true);
        assert!(clear.starts_with("[17/18] local-command-clear ") && clear.ends_with("  pass"));
        assert_eq!(
            step_line(18, 18, LedgerRow::PasteNewline, true),
            "[18/18] paste-newline an LF inside a paste stays inside the one prompt submitted  pass"
        );
    }

    /// The counter's total is the relied count: twenty-four rows on Unix, twenty-one on Windows, where
    /// the status line is not wrapped.
    #[test]
    fn step_lines_count_to_the_relied_total() {
        let probes = Probes {
            print: ProbeRun {
                program_is_script: true,
                version_answered: false,
                captures: Vec::new(),
            },
            typed: TypedRun::default(),
            trusted: Vec::new(),
            dialogs: DialogRuns::default(),
            statusline: Vec::new(),
            resumed: Vec::new(),
            agents: None,
        };
        let results = check_rows(&probes);
        #[cfg(unix)]
        {
            assert_eq!(results.len(), 24);
            assert_eq!(
                step_line(19, 24, LedgerRow::StatusLineOverride, false),
                "[19/24] statusline-override a --settings statusLine runs in the status line's place  fail"
            );
            assert_eq!(
                step_line(20, 24, LedgerRow::StatusLineShell, true),
                "[20/24] statusline-shell a statusline command runs through /bin/sh  pass"
            );
            assert_eq!(
                step_line(21, 24, LedgerRow::StatusLineRateLimits, true),
                "[21/24] statusline-rate-limits the statusline payload carries two whole-number rate_limits windows  pass"
            );
            assert_eq!(
                step_line(22, 24, LedgerRow::ResumeSession, false),
                "[22/24] resume-session --resume <id> reopens the session under that id  fail"
            );
            assert_eq!(
                step_line(23, 24, LedgerRow::AgentsJoin, false),
                "[23/24] agents-join claude agents --json names the live session by pid and session id  fail"
            );
            assert_eq!(
                step_line(24, 24, LedgerRow::LocalCommandCompact, false),
                "[24/24] local-command-compact /compact keeps the session id and submits no prompt  fail"
            );
        }
        #[cfg(windows)]
        {
            assert_eq!(results.len(), 21);
            assert_eq!(
                step_line(19, 21, LedgerRow::ResumeSession, true),
                "[19/21] resume-session --resume <id> reopens the session under that id  pass"
            );
            assert_eq!(
                step_line(20, 21, LedgerRow::AgentsJoin, true),
                "[20/21] agents-join claude agents --json names the live session by pid and session id  pass"
            );
            assert_eq!(
                step_line(21, 21, LedgerRow::LocalCommandCompact, true),
                "[21/21] local-command-compact /compact keeps the session id and submits no prompt  pass"
            );
        }
        let rows: Vec<LedgerRow> = results.iter().map(|(row, _)| *row).collect();
        assert_eq!(rows, LedgerRow::relied());
        // No capture is over the frame cap, so only the largest-payload row passes an empty probe.
        let passed: Vec<LedgerRow> = results
            .iter()
            .filter(|(_, pass)| *pass)
            .map(|(row, _)| *row)
            .collect();
        assert_eq!(passed, [LedgerRow::LargestHookPayload]);
    }

    #[test]
    fn read_statusline_orders_by_k_and_reads_each_shell_verdict() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let dir = tmp.path();
        fs::write(dir.join("StatusLine.2.json"), b"{\"n\":2}").expect("2");
        fs::write(dir.join("StatusLine.2.shell"), b"other").expect("2");
        fs::write(dir.join("StatusLine.1.json"), b"{\"n\":1}").expect("1");
        fs::write(dir.join("StatusLine.1.shell"), b"bin-sh").expect("1");
        fs::write(dir.join("StatusLine.3.json"), b"x").expect("3, no verdict");
        fs::write(dir.join("StatusLine.4.json"), b"y").expect("4");
        fs::write(dir.join("StatusLine.4.shell"), b"bin-sh and more").expect("4");
        fs::write(dir.join("Stop.5.json"), b"{}").expect("another arm's name");
        fs::write(dir.join("StatusLine.6.shell"), b"bin-sh").expect("a lone verdict");
        let got = read_statusline(dir);
        let read: Vec<(&[u8], ShellVerdict)> =
            got.iter().map(|c| (c.bytes.as_slice(), c.shell)).collect();
        assert_eq!(
            read,
            [
                (&b"{\"n\":1}"[..], ShellVerdict::BinSh),
                (&b"{\"n\":2}"[..], ShellVerdict::Other),
                (&b"x"[..], ShellVerdict::Other),
                (&b"y"[..], ShellVerdict::Other),
            ]
        );
        assert!(read_statusline(&dir.join("missing")).is_empty());
    }

    #[test]
    fn read_statusline_reads_one_byte_past_the_cap() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let cap = usize::try_from(MAX_FRAME).expect("fits");
        fs::write(tmp.path().join("StatusLine.1.json"), vec![b' '; cap + 5]).expect("big");
        assert_eq!(read_statusline(tmp.path())[0].bytes.len(), cap + 1);
    }

    #[test]
    fn probe_dir_holds_the_statusline_dir_and_on_unix_its_probe_settings() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let probe = ProbeDir::create(tmp.path(), "/h/bin/k/viola").expect("probe dir");
        let captures = probe.dir.join("typed").join("statusline");
        assert!(captures.is_dir());
        assert_eq!(probe.statusline_captures(), captures);
        let settings = probe.dir.join("typed").join("statusline.json");
        assert_eq!(probe.statusline_settings(), settings);
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            let doc: serde_json::Value =
                serde_json::from_slice(&fs::read(&settings).expect("settings")).expect("json");
            let command = format!(
                "/h/bin/k/viola hook statusline --capture {} --shell \"$0\"",
                fwd(&captures)
            );
            assert_eq!(
                doc,
                json!({"statusLine": {"type": "command", "command": command}})
            );
            let mode = fs::metadata(&settings).expect("meta").permissions().mode();
            assert_eq!(mode & 0o777, 0o600);
        }
        #[cfg(windows)]
        assert!(!settings.exists());
        let unplain = ProbeDir::create(tmp.path(), "/h/bin k/viola").expect("probe dir");
        assert!(!unplain.statusline_settings().exists());
        assert!(unplain.statusline_captures().is_dir());
    }

    #[test]
    fn read_captures_orders_by_k_and_skips_other_names() {
        let tmp = tempfile::tempdir().expect("tempdir");
        fs::write(tmp.path().join("Stop.3.json"), b"{\"n\":3}").expect("stop");
        fs::write(tmp.path().join("SessionStart.1.json"), b"{\"n\":1}").expect("start");
        fs::write(tmp.path().join("UserPromptSubmit.2.json"), b"x").expect("prompt");
        fs::write(tmp.path().join("notes.txt"), b"{}").expect("other");
        fs::create_dir(tmp.path().join("SessionEnd.4.json")).expect("a dir");
        let got = read_captures(tmp.path());
        let events: Vec<HookEvent> = got.iter().map(|c| c.event).collect();
        assert_eq!(
            events,
            [
                HookEvent::SessionStart,
                HookEvent::UserPromptSubmit,
                HookEvent::Stop
            ]
        );
        assert_eq!(got[0].payload, Some(json!({"n": 1})));
        assert_eq!(got[1].payload, None);
        assert_eq!(got[1].bytes, 1);
        assert!(read_captures(&tmp.path().join("missing")).is_empty());
    }

    #[test]
    fn read_captures_reads_one_byte_past_the_cap() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let cap = usize::try_from(MAX_FRAME).expect("fits");
        fs::write(tmp.path().join("Stop.1.json"), vec![b' '; cap + 5]).expect("big");
        let got = read_captures(tmp.path());
        assert_eq!(got[0].bytes, cap + 1);
    }

    #[test]
    fn probe_dir_holds_the_plugin_and_is_removed_on_drop() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let probe = ProbeDir::create(tmp.path(), "C:/h/viola.exe").expect("probe dir");
        let dir = probe.dir.clone();
        assert!(dir.starts_with(tmp.path().join("ledger").join("probes")));
        assert!(probe.captures().is_dir());
        let hooks = fs::read_to_string(probe.plugin().join("hooks").join("hooks.json"))
            .expect("hooks.json");
        assert!(hooks.contains("\"--capture\""));
        assert!(
            probe
                .plugin()
                .join(".claude-plugin")
                .join("plugin.json")
                .is_file()
        );
        fs::write(probe.captures().join("Stop.1.json"), b"{}").expect("a capture");
        drop(probe);
        assert!(!dir.exists());
    }

    /// Built without `create`, so no case writes a run's dirs relative to the test's cwd.
    #[test]
    fn probe_dir_names_each_run_its_own_dir_inside_it() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let dir = tmp.path().join("probe");
        let probe = ProbeDir { dir: dir.clone() };
        assert_eq!(probe.typed(), dir.join("typed"));
        assert_eq!(probe.questions(), dir.join("questions"));
        assert_eq!(probe.plan(), dir.join("plan"));
        assert_eq!(probe.plugin(), dir.join("plugin"));
        assert_eq!(probe.captures(), dir.join("captures"));
    }

    #[test]
    fn probe_dir_starts_empty_over_a_leftover_of_the_same_pid() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let first = ProbeDir::create(tmp.path(), "C:/h/viola.exe").expect("first");
        fs::write(first.captures().join("Stop.1.json"), b"{}").expect("leftover");
        std::mem::forget(first);
        let second = ProbeDir::create(tmp.path(), "C:/h/viola.exe").expect("second");
        assert_eq!(fs::read_dir(second.captures()).expect("dir").count(), 0);
    }
}
