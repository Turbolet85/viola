//! `viola verify [--record <dir>] [-- <program> [args…]]`: measures the capability ledger against
//! the local `claude` (or the program after `--`) with one print-mode probe through a capture
//! plugin, prints one step line per row and the `stamped` summary on stdout, and writes the stamps
//! as their only writer (security-plan §Data Protection; architecture [CLI Version
//! Compatibility]). With `--record`, a clean run's payloads become scrubbed fixtures.

use std::ffi::OsString;
use std::fs::File;
use std::io::Read as _;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{Duration, Instant};

use chrono::Utc;
use tracing::instrument;
use viola_agent_claude::hook::HookEvent;
use viola_agent_claude::ledger::{self, Capture, LedgerRow, PROBE_PROMPT, ProbeRun};
use viola_agent_claude::{CASE_INSENSITIVE, PLUGIN_DIR_FLAG, Refusal, StripPlan};
use viola_core::obs::{ObsEvent, ObsProcess};
use viola_core::{MAX_FRAME, ViolaName, obs_event};
use viola_state::fs::{FILE_MODE, create_private_dir, replace_private};
use viola_state::pin::{PinError, pin_exe};
use viola_state::stamps::{ledger_dir, update_stamps};

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
    if let Some(name) = instance {
        let (level, rejection) = obs::read_diagnostics_level(&home);
        obs::viola_obs_init(&home, ObsProcess::Cli, Some(name.clone()), level)?;
        run::log_self_start();
        if let Some(rejection) = rejection {
            obs::log_config_rejection(rejection);
        }
    }
    let code = measure(&home, args)?;
    if instance.is_some() {
        run::log_self_exit(code, None);
    }
    Ok(ExitCode::from(code))
}

/// `run_bounded` between a `process-start` / `process-exit` pair for `subject` (obs-plan §6 Child /
/// shell spawns). The pair lives at the call site, not in `run_bounded`, which `run`'s version
/// gate shares and logs itself. Without an instance no subscriber is installed and both are no-ops.
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
    let ran = run_bounded(program, args, cwd, strip, deadline);
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

fn measure(home: &Path, args: &VerifyArgs) -> anyhow::Result<u8> {
    let (program, program_args) = match args.program.split_first() {
        Some((program, rest)) => (program.clone(), rest.to_vec()),
        None => (OsString::from("claude"), Vec::new()),
    };
    let cwd = std::env::current_dir()?;
    // A given path resolves as given, so its existence is checked here.
    let resolved = run::resolve_program(&program, &cwd).and_then(|p| {
        if p.is_file() {
            Ok(p)
        } else {
            Err(Refusal::NotFound)
        }
    });
    let program = match resolved {
        Ok(program) => program,
        Err(Refusal::BatchScriptChild) => {
            human::refuse(
                "the claude CLI is a .cmd or .bat script",
                "pass the real executable, not a .cmd or .bat shim",
            );
            return Ok(1);
        }
        Err(Refusal::NotFound) => {
            human::refuse(
                "the claude CLI was not found",
                "install Claude Code or put it on PATH",
            );
            return Ok(1);
        }
    };
    let pinned = match pin_exe(home, &std::env::current_exe()?) {
        Ok(pinned) => pinned,
        Err(PinError::HashMismatch) => {
            super::run::refuse_tampered_pin();
            return Ok(1);
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
    let Some(version) = ledger::parse_version(&answer.stdout) else {
        human::refuse(
            "the CLI version could not be read",
            "run claude --version to check the install",
        );
        return Ok(1);
    };

    let probe = ProbeDir::create(home, &pinned.path_fwd)?;
    let mut probe_args = program_args;
    probe_args
        .extend(["-p", PROBE_PROMPT, "--model", "haiku", PLUGIN_DIR_FLAG].map(OsString::from));
    probe_args.push(probe.plugin().into_os_string());
    probe_args.push("--no-session-persistence".into());
    run_logged(
        "verify-probe",
        &program,
        &probe_args,
        &probe.dir,
        &strip,
        PROBE_DEADLINE,
    )?;
    let probe_run = ProbeRun {
        program_is_script: viola_agent_claude::is_script(&program),
        version_answered: true,
        captures: read_captures(&probe.captures()),
    };
    drop(probe);

    let results = check_rows(&probe_run);
    let written_at = obs::timestamp(Utc::now());
    let largest = ledger::largest(&probe_run.captures);
    update_stamps(home, |existing| {
        ledger::merge_stamp(existing, &version, &results, &largest, &written_at)
    })?;
    let failed = results.iter().filter(|(_, pass)| !pass).count();
    human::result(&format!(
        "stamped {version}  {} pass  {failed} fail",
        results.len() - failed
    ));
    if failed > 0 {
        return Ok(1);
    }
    match &args.record {
        Some(dir) => record(dir, &version, &probe_run.captures),
        None => Ok(0),
    }
}

/// `<home>/ledger/probes/<pid>/` (0700) with the capture plugin in `plugin/` and an empty
/// `captures/`; removed whole when dropped, on every exit path.
struct ProbeDir {
    dir: PathBuf,
}

impl ProbeDir {
    fn create(home: &Path, pinned_bin_fwd: &str) -> anyhow::Result<Self> {
        let dir = ledger_dir(home)
            .join("probes")
            .join(std::process::id().to_string());
        let _ = std::fs::remove_dir_all(&dir);
        let probe = Self { dir };
        create_private_dir(&probe.captures())?;
        let captures_fwd = probe.captures().to_string_lossy().replace('\\', "/");
        for (rel, content) in ledger::capture_plugin_files(pinned_bin_fwd, &captures_fwd) {
            let path = probe.plugin().join(rel);
            create_private_dir(path.parent().unwrap_or(&probe.dir))?;
            replace_private(&path, content.as_bytes(), FILE_MODE)?;
        }
        Ok(probe)
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

/// Every row checked in order, its step line printed as it is checked.
fn check_rows(probe_run: &ProbeRun) -> Vec<(LedgerRow, bool)> {
    let total = LedgerRow::ALL.len();
    LedgerRow::ALL
        .iter()
        .enumerate()
        .map(|(i, row)| (*row, check_step(i + 1, total, *row, probe_run)))
        .collect()
}

#[instrument(skip_all, name = "cli.verify_step", fields(row = row.id()))]
fn check_step(n: usize, total: usize, row: LedgerRow, probe_run: &ProbeRun) -> bool {
    let pass = ledger::check(row, probe_run);
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
/// `<dir>/<version>/<Event>.default.json`. One payload that stays unclean after the scrub refuses
/// the whole recording before any file is written.
fn record(dir: &Path, version: &str, captures: &[Capture]) -> anyhow::Result<u8> {
    let user_home = std::env::home_dir().unwrap_or_default();
    let home = user_home.to_string_lossy().into_owned();
    let user = user_home
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let mut files = Vec::new();
    for event in ledger::CAPTURE_EVENTS {
        let first = captures.iter().find(|c| c.event == event);
        let Some(payload) = first.and_then(|c| c.payload.as_ref()) else {
            continue;
        };
        let scrubbed = ledger::scrub(payload, &home, &user, CASE_INSENSITIVE);
        if !ledger::is_clean(&scrubbed, &user) {
            human::refuse(
                "a recorded payload still holds a path or a username",
                "record with a viola home under your user home",
            );
            return Ok(1);
        }
        files.push((event, scrubbed));
    }
    let out = dir.join(version);
    std::fs::create_dir_all(&out)?;
    for (event, payload) in files {
        let mut text = payload.to_string();
        text.push('\n');
        let name = format!("{}.default.json", ledger::event_name(event));
        std::fs::write(out.join(name), text)?;
    }
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs;

    #[test]
    fn step_line_is_the_counter_the_row_and_the_verdict() {
        assert_eq!(
            step_line(1, 6, LedgerRow::ShimResolution, true),
            "[01/06] shim-resolution claude resolves to a real executable  pass"
        );
        assert_eq!(
            step_line(6, 6, LedgerRow::LargestHookPayload, false),
            "[06/06] largest-hook-payload every hook payload fits the frame cap  fail"
        );
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
