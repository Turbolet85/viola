//! The one `claude agents --json` read `viola list`, `viola revive` and `viola verify` make
//! (architecture §Standard Contracts → Session liveness): the program resolved by `run`'s rule,
//! spawned directly under the R8 strip with stdin null, its stdout read through one byte past the
//! frame cap, killed and reaped at the version probe's bound. A failed read is never an error: the
//! callers carry on without it.

use std::ffi::{OsStr, OsString};
use std::path::Path;
use std::time::Instant;

use viola_agent_claude::agents::{self, AgentsFailure, AgentsList};
use viola_agent_claude::{PROGRAM, StripPlan};
use viola_core::obs::ObsEvent;
use viola_core::{MAX_FRAME, obs_event};

use super::version_gate::{VERSION_DEADLINE, run_capped};

/// Why a read gave no rows: the output was over the frame cap, was not one JSON array, or never
/// came (the program not found or refused, a failed spawn, the deadline, a non-zero exit).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ReadFailure {
    Oversize,
    Malformed,
    Unreadable,
}

/// One read: the parsed list or why there is none, the raw stdout for a recorder, and whether the
/// stop ended it.
#[derive(Debug)]
pub(crate) struct AgentsRead {
    pub(crate) list: Result<AgentsList, ReadFailure>,
    pub(crate) stdout: Vec<u8>,
    pub(crate) stopped: bool,
}

/// `program agents --json` in `cwd`, between an `agents-probe` spawn pair (no row text, id, `cwd`
/// or path), with one `parse-rejected{parser:"claude-agents-json"}` when the output does not parse.
/// Without an open log every line is a no-op.
pub(crate) fn read_logged(
    program: &Path,
    cwd: &Path,
    strip: &StripPlan,
    stopped: fn() -> bool,
) -> AgentsRead {
    let started = Instant::now();
    obs_event!(INFO, ObsEvent::ProcessStart, subject = "agents-probe");
    let args = agents::ARGS.map(OsString::from);
    let ran = run_capped(
        program,
        &args,
        cwd,
        strip,
        VERSION_DEADLINE,
        stopped,
        MAX_FRAME + 1,
    );
    let status = ran
        .as_ref()
        .ok()
        .and_then(|r| r.status)
        .and_then(|s| s.code());
    obs_event!(
        INFO,
        ObsEvent::ProcessExit,
        subject = "agents-probe",
        child_exit_status = status,
        duration_ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX),
    );
    let Ok(ran) = ran else {
        return unread(false);
    };
    if ran.stopped || !ran.status.is_some_and(|s| s.success()) {
        return AgentsRead {
            stdout: ran.stdout,
            ..unread(ran.stopped)
        };
    }
    let list = agents::parse(&ran.stdout).map_err(|failure| {
        obs_event!(
            WARN,
            ObsEvent::ParseRejected,
            parser = "claude-agents-json",
            detail = failure.detail(),
            count = 1u64,
        );
        match failure {
            AgentsFailure::Oversize => ReadFailure::Oversize,
            AgentsFailure::Malformed => ReadFailure::Malformed,
        }
    });
    AgentsRead {
        list,
        stdout: ran.stdout,
        stopped: false,
    }
}

fn unread(stopped: bool) -> AgentsRead {
    AgentsRead {
        list: Err(ReadFailure::Unreadable),
        stdout: Vec::new(),
        stopped,
    }
}

/// The read `list` and `revive` make: `claude` looked up on this process's `PATH` from `cwd`, the
/// strip planned against the home's pass list. A program not found or refused is a failed read
/// with no spawn and no line.
pub(crate) fn read_here(home: &Path, cwd: &Path) -> Result<AgentsList, ReadFailure> {
    let Ok(program) = super::resolve_program(OsStr::new(PROGRAM), cwd) else {
        return Err(ReadFailure::Unreadable);
    };
    let (persistent, _) = super::persistent_names(home);
    let strip = viola_agent_claude::plan_strip(std::env::vars_os().map(|(k, _)| k), &persistent);
    read_logged(&program, cwd, &strip, super::version_gate::never_stopped).list
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `echo` prints its arguments and exits 0: a text that is not JSON reads malformed.
    #[cfg(unix)]
    #[test]
    fn read_logged_of_a_program_whose_output_is_no_array_is_malformed() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let got = read_logged(
            Path::new("/bin/echo"),
            tmp.path(),
            &StripPlan::default(),
            || false,
        );
        assert_eq!(got.list, Err(ReadFailure::Malformed));
        assert_eq!(got.stdout, b"agents --json\n");
        assert!(!got.stopped);
    }

    #[test]
    fn read_logged_of_a_program_that_cannot_start_is_unreadable() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let got = read_logged(
            &tmp.path().join("absent"),
            tmp.path(),
            &StripPlan::default(),
            || false,
        );
        assert_eq!(got.list, Err(ReadFailure::Unreadable));
        assert!(got.stdout.is_empty());
    }

    #[test]
    fn read_logged_with_the_stop_set_reads_stopped_and_unreadable() {
        let exe = std::env::current_exe().expect("test exe");
        let cwd = std::env::current_dir().expect("cwd");
        let got = read_logged(&exe, &cwd, &StripPlan::default(), || true);
        assert_eq!(got.list, Err(ReadFailure::Unreadable));
        assert!(got.stopped);
    }

    /// `sh` given `agents` opens no such script and exits non-zero: a failed read, never a parse.
    #[cfg(unix)]
    #[test]
    fn read_logged_of_a_failing_program_is_unreadable() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let got = read_logged(
            Path::new("/bin/sh"),
            tmp.path(),
            &StripPlan::default(),
            || false,
        );
        assert_eq!(got.list, Err(ReadFailure::Unreadable));
        assert!(!got.stopped);
    }
}
