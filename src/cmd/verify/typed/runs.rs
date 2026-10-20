//! The five runs, each with its own arguments and its own dir.

use std::ffi::OsString;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::mpsc;

use viola_agent_claude::PLUGIN_DIR_FLAG;
use viola_agent_claude::hook::HookEvent;
use viola_agent_claude::ledger::{
    self, AgentsCapture, DIALOG_PROMPT_PARALLEL, DIALOG_PROMPT_PERMISSION, DIALOG_PROMPT_QUESTIONS,
    DIALOG_SETTINGS, PLAN_PROMPT, TypedRun,
};
use viola_agent_claude::statusline::SETTINGS_FLAG;
use viola_core::Clock;
use viola_state::fs::create_private_dir;

use super::Inputs;
use super::pty::KeyPipe;
use super::run::Run;
use super::turns::{
    agents_read, count_of, framing_turns, input_box_up, ms, probe_until, turn, wait_capture,
    wait_for,
};

/// Run A: the first settled screen in a fresh dir under the OS temp dir, then a kill.
pub(super) fn untrusted_run(
    clock: &Arc<dyn Clock>,
    inputs: &Inputs<'_>,
    args: &[OsString],
    typed: &mut TypedRun,
) -> anyhow::Result<()> {
    let untrusted = tempfile::Builder::new().prefix("viola-verify-").tempdir()?;
    let run = Run::spawn(clock, inputs, args, untrusted.path(), Box::new(io::empty()))?;
    typed.modal = run.settle(run.spawned_at).and_then(|s| s.rows);
    run.end_by_kill();
    drop(untrusted);
    Ok(())
}

/// Run B: the settled input box, one typed turn, the agents read once that turn settled, the five
/// framing pastes, then Ctrl-C; its status line runs the probe settings' capture command where they
/// were written.
pub(super) fn trusted_run(
    clock: &Arc<dyn Clock>,
    inputs: &Inputs<'_>,
    args: &[OsString],
    typed: &mut TypedRun,
) -> anyhow::Result<Option<AgentsCapture>> {
    let trusted = TrustedDir::create(inputs.cwd, "")?;
    let args = trusted_args(args, inputs);
    let (keys, pipe) = mpsc::channel();
    let run = Run::spawn(
        clock,
        inputs,
        &args,
        &trusted.0,
        Box::new(KeyPipe::new(pipe)),
    )?;
    let ready = run.settle(run.spawned_at);
    typed.ready_settle_ms = ready
        .as_ref()
        .map(|s| ms(s.at.saturating_duration_since(run.spawned_at)));
    typed.ready = ready.and_then(|s| s.rows);
    if !input_box_up(typed.ready.as_deref()) {
        run.end_by_kill();
        return Ok(None);
    }
    // A stop during the turn, the pastes or the agents read ends the run by a kill inside
    // `end_by_ctrl_c`.
    turn(&run, inputs.trusted.captures, typed);
    let agents = typed
        .turn
        .is_some()
        .then(|| agents_read(&run, inputs, &trusted.0))
        .flatten();
    typed.compact_latency_ms = framing_turns(
        &run,
        inputs.trusted.captures,
        typed.turn.clone(),
        &mut typed.long_turn,
    );
    run.end_by_ctrl_c(&keys);
    drop(keys);
    drop(trusted);
    Ok(agents)
}

/// Run B's arguments: the run's own, its plugin dir, then its one `--settings` when given.
fn trusted_args(args: &[OsString], inputs: &Inputs<'_>) -> Vec<OsString> {
    let mut args = args.to_vec();
    args.push(PLUGIN_DIR_FLAG.into());
    args.push(inputs.trusted.plugin.as_os_str().to_owned());
    if let Some(settings) = inputs.statusline_settings {
        args.push(SETTINGS_FLAG.into());
        args.push(settings.as_os_str().to_owned());
    }
    args
}

/// Run C: the three dialog prompts, each pasted once the previous turn's Stop is captured, under the
/// session `ask` rule; once the last Stop is captured the screen settles (as Run B's turn does), so
/// the kill never hangs up a Stop hook still exiting.
pub(super) fn dialog_run(
    clock: &Arc<dyn Clock>,
    inputs: &Inputs<'_>,
    args: &[OsString],
) -> anyhow::Result<()> {
    let dir = TrustedDir::create(inputs.cwd, "-dialogs")?;
    let mut args = args.to_vec();
    args.extend([SETTINGS_FLAG, DIALOG_SETTINGS, PLUGIN_DIR_FLAG].map(OsString::from));
    args.push(inputs.questions.plugin.as_os_str().to_owned());
    let (keys, pipe) = mpsc::channel::<Vec<u8>>();
    let run = Run::spawn(clock, inputs, &args, &dir.0, Box::new(KeyPipe::new(pipe)))?;
    if run.input_box_settles() {
        let captures = inputs.questions.captures;
        for prompt in [
            DIALOG_PROMPT_QUESTIONS,
            DIALOG_PROMPT_PARALLEL,
            DIALOG_PROMPT_PERMISSION,
        ] {
            let stops = count_of(captures, HookEvent::Stop);
            if !run.paste_text(prompt) {
                break;
            }
            let until = probe_until(clock.as_ref());
            let stopped = wait_for(clock.as_ref(), until, run.stop, || {
                count_of(captures, HookEvent::Stop) > stops
            });
            let Some(stopped) = stopped else {
                break;
            };
            if prompt == DIALOG_PROMPT_PERMISSION {
                run.settle(stopped);
            }
        }
    }
    run.end_by_kill();
    drop(keys);
    drop(dir);
    Ok(())
}

/// Run D: plan mode, the plan prompt, until the turn's Stop is captured (after the plan's
/// PostToolUse, which the rows read) and the screen settles after it; the CLI's plan file goes into
/// the run's own `plans/`; ended by a kill.
pub(super) fn plan_run(
    clock: &Arc<dyn Clock>,
    inputs: &Inputs<'_>,
    args: &[OsString],
) -> anyhow::Result<()> {
    let dir = TrustedDir::create(inputs.cwd, "-plan")?;
    let plans = dir.0.join("plans");
    create_private_dir(&plans)?;
    let mut args = args.to_vec();
    args.extend(["--permission-mode", "plan", SETTINGS_FLAG].map(OsString::from));
    args.push(ledger::plan_settings(&plans.to_string_lossy()).into());
    args.push(PLUGIN_DIR_FLAG.into());
    args.push(inputs.plan.plugin.as_os_str().to_owned());
    let (keys, pipe) = mpsc::channel::<Vec<u8>>();
    let run = Run::spawn(clock, inputs, &args, &dir.0, Box::new(KeyPipe::new(pipe)))?;
    if run.input_box_settles() && run.paste_text(PLAN_PROMPT) {
        let until = probe_until(clock.as_ref());
        let stopped = wait_for(clock.as_ref(), until, run.stop, || {
            count_of(inputs.plan.captures, HookEvent::Stop) > 0
        });
        if let Some(stopped) = stopped {
            run.settle(stopped);
        }
    }
    run.end_by_kill();
    drop(keys);
    drop(dir);
    Ok(())
}

/// Run E: Run B's first session resumed by its id (`ledger::resume_target`; none, and no child
/// starts) in its own dir, until one SessionStart is captured and the screen settles after it; ended
/// by a kill with no key typed.
pub(super) fn resume_run(
    clock: &Arc<dyn Clock>,
    inputs: &Inputs<'_>,
    args: &[OsString],
) -> anyhow::Result<()> {
    let trusted = super::super::read_captures(inputs.trusted.captures);
    let Some(id) = ledger::resume_target(&trusted) else {
        return Ok(());
    };
    let dir = TrustedDir::create(inputs.cwd, "-resume")?;
    let mut args = args.to_vec();
    args.push(PLUGIN_DIR_FLAG.into());
    args.push(inputs.resume.plugin.as_os_str().to_owned());
    args.extend(viola_agent_claude::resume_args(id, false));
    let run = Run::spawn(clock, inputs, &args, &dir.0, Box::new(io::empty()))?;
    let started = wait_capture(
        inputs.resume.captures,
        HookEvent::SessionStart,
        clock.as_ref(),
        probe_until(clock.as_ref()),
        run.stop,
    );
    if let Some(started) = started {
        run.settle(started);
    }
    run.end_by_kill();
    drop(dir);
    Ok(())
}

/// `<cwd>/.viola-verify-<pid><suffix>/` (0700), removed whole when dropped, on every exit path.
struct TrustedDir(PathBuf);

impl TrustedDir {
    fn create(cwd: &Path, suffix: &str) -> io::Result<Self> {
        let dir = cwd.join(format!(".viola-verify-{}{suffix}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let guard = Self(dir);
        create_private_dir(&guard.0)?;
        Ok(guard)
    }
}

impl Drop for TrustedDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trusted_dir_is_private_and_removed_on_drop() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let dir = TrustedDir::create(tmp.path(), "").expect("dir");
        let path = dir.0.clone();
        assert_eq!(
            path,
            tmp.path()
                .join(format!(".viola-verify-{}", std::process::id()))
        );
        let plan = TrustedDir::create(tmp.path(), "-plan").expect("plan dir");
        assert_eq!(
            plan.0,
            tmp.path()
                .join(format!(".viola-verify-{}-plan", std::process::id()))
        );
        std::fs::write(path.join("left"), b"x").expect("a file");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            let mode = std::fs::metadata(&path).expect("meta").permissions().mode();
            assert_eq!(mode & 0o777, 0o700);
        }
        drop(dir);
        assert!(!path.exists());
    }
}
