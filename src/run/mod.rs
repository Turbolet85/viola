//! The `run` wrapper's process side: its role lines, the persistent-environment read behind the
//! R8 strip, and where the child's program is looked up.

mod env;

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use viola_agent_claude::{Refusal, Search, StripPlan};
use viola_core::obs::ObsEvent;
use viola_core::{SERVICE_NAME, VERSION, obs_event};
use viola_pty::Exit;

pub(crate) use env::persistent_names;

pub(crate) fn log_self_start() {
    obs_event!(
        INFO,
        ObsEvent::ProcessStart,
        subject = "self",
        service_name = SERVICE_NAME,
        version = VERSION,
        os = std::env::consts::OS,
        pid = std::process::id(),
    );
}

/// Names only: the stripped and kept `CLAUDE*` names, never a value (obs-plan D-14).
pub(crate) fn log_child_start(child_pid: Option<u32>, strip: &StripPlan) {
    let stripped = strip.removed_names();
    let kept = strip.kept_names();
    obs_event!(
        INFO,
        ObsEvent::ProcessStart,
        subject = "claude-child",
        child_pid = child_pid,
        pty_backend = viola_pty::PTY_BACKEND,
        env_stripped_count = strip.count(),
        env_stripped_known = Some(stripped.as_str()).filter(|s| !s.is_empty()),
        env_kept = Some(kept.as_str()).filter(|s| !s.is_empty()),
    );
}

pub(crate) fn log_child_exit(exit: Exit) {
    obs_event!(
        INFO,
        ObsEvent::ProcessExit,
        subject = "claude-child",
        child_exit_status = exit.code,
        exit_source = exit.source.as_str(),
    );
}

pub(crate) fn log_self_exit(exit_code: u8, detail: Option<&'static str>) {
    let duration_ms = crate::obs::duration_ms();
    match detail {
        Some(detail) => obs_event!(
            ERROR,
            ObsEvent::ProcessExit,
            subject = "self",
            exit_code = exit_code,
            detail = detail,
            duration_ms = duration_ms,
        ),
        None => obs_event!(
            INFO,
            ObsEvent::ProcessExit,
            subject = "self",
            exit_code = exit_code,
            duration_ms = duration_ms,
        ),
    }
}

/// The program after `--`, looked up by viola itself (never by portable-pty's own search, which
/// takes an extensionless sh shim first on Windows) against this process's `PATH`.
pub(crate) fn resolve_program(program: &OsStr, cwd: &Path) -> Result<PathBuf, Refusal> {
    let path = std::env::var_os("PATH");
    let pathext = std::env::var_os("PATHEXT");
    let search = Search {
        path: path.as_deref(),
        pathext: pathext.as_deref(),
        windows: Search::HOST_IS_WINDOWS,
    };
    viola_agent_claude::resolve_program(program, cwd, search, &|p: &Path| p.is_file())
}
