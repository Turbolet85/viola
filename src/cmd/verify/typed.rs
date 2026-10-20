//! `viola verify`'s five interactive runs of the CLI under a PTY at a fixed 80×24 (architecture
//! [Screen Model]). The untrusted run (A) starts in a fresh dir under the OS temp dir, records its
//! first settled screen (`modal`) and is ended by a kill. The trusted run (B) starts in
//! `<cwd>/.viola-verify-<pid>/`, records its settled input box (`ready`), types the probe prompt as
//! one bracketed paste, times the turn and records its end (`turn`), reads `claude agents --json`
//! once while its session is live, then pastes the long text, the tag-like text, the newline text,
//! `/compact` and the probed local command, each into the settled input box.
//! The dialog run (C, in
//! `<cwd>/.viola-verify-<pid>-dialogs/`) pastes the three dialog prompts, each after the previous
//! turn's Stop; the plan run (D, in `<cwd>/.viola-verify-<pid>-plan/`, plan mode) pastes the plan
//! prompt. In C and D the probe's capture hook answers each dialog (the founder's ruling,
//! 2026-10-05); both are ended by a kill. The resume run (E, in `<cwd>/.viola-verify-<pid>-resume/`)
//! resumes Run B's first session by its id, waits for its SessionStart and is ended by a kill. No
//! run ever types into a dialog: the untrusted and resume runs' input is empty, a trusted start that
//! shows a modal is killed, and only prompts are pasted. Every wait
//! but the pump's after a kill ends at the stop; a stopped run is ended by a kill, pastes nothing
//! more, and no later run starts.

use std::cell::RefCell;
use std::ffi::OsString;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use viola_agent_claude::StripPlan;
use viola_agent_claude::ledger::{AgentsCapture, TypedRun};
use viola_core::Clock;

mod pty;
mod run;
mod runs;
mod turns;

use runs::{dialog_run, plan_run, resume_run, trusted_run, untrusted_run};

const POLL: Duration = Duration::from_millis(5);

/// One run's capture plugin and capture dir.
pub(super) struct RunDirs<'a> {
    pub(super) plugin: &'a Path,
    pub(super) captures: &'a Path,
}

/// What the runs need from `verify`: the resolved CLI and its leading arguments, verify's cwd, the
/// R8 strip, the stop, the trusted, dialog, plan and resume runs' plugins and capture dirs, and the
/// trusted run's statusline probe settings where the status line is wrapped.
pub(super) struct Inputs<'a> {
    pub(super) program: &'a Path,
    pub(super) program_args: &'a [OsString],
    pub(super) cwd: &'a Path,
    pub(super) strip: &'a StripPlan,
    pub(super) stop: fn() -> bool,
    pub(super) trusted: RunDirs<'a>,
    pub(super) statusline_settings: Option<&'a Path>,
    pub(super) questions: RunDirs<'a>,
    pub(super) plan: RunDirs<'a>,
    pub(super) resume: RunDirs<'a>,
}

/// One of the five runs, adding what it measured.
type RunStep<'a> = dyn Fn(&mut TypedRun) -> anyhow::Result<()> + 'a;

/// The five runs in order, with Run B's agents read when it made one; every dir any run used is
/// gone on return. Runs C, D and E leave only their captures, which `verify` reads. `None` when the
/// stop was set before a run would start: the run it ended has been killed and its dir removed.
pub(super) fn measure(
    clock: &Arc<dyn Clock>,
    inputs: &Inputs<'_>,
) -> anyhow::Result<Option<(TypedRun, Option<AgentsCapture>)>> {
    let mut typed = TypedRun::default();
    let agents = RefCell::new(None);
    let mut args = inputs.program_args.to_vec();
    args.extend(["--model", "haiku"].map(OsString::from));
    let runs: [&RunStep<'_>; 5] = [
        &|typed| untrusted_run(clock, inputs, &args, typed),
        &|typed| {
            *agents.borrow_mut() = trusted_run(clock, inputs, &args, typed)?;
            Ok(())
        },
        &|_| dialog_run(clock, inputs, &args),
        &|_| plan_run(clock, inputs, &args),
        &|_| resume_run(clock, inputs, &args),
    ];
    for run in runs {
        if (inputs.stop)() {
            return Ok(None);
        }
        run(&mut typed)?;
    }
    Ok(Some((typed, agents.into_inner())))
}

#[cfg(test)]
mod test_support;
