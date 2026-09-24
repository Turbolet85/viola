//! The `run` wrapper's process side: its role lines and the one child-spawn function (the PTY
//! seam replaces its body; nothing else depends on how it spawns).

use std::ffi::OsString;
use std::io;
use std::process::{Child, Command};

use viola_core::obs::ObsEvent;
use viola_core::{SERVICE_NAME, VERSION, obs_event};

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

pub(crate) fn log_child_start(child_pid: u32) {
    obs_event!(
        INFO,
        ObsEvent::ProcessStart,
        subject = "claude-child",
        child_pid = child_pid,
    );
}

pub(crate) fn log_child_exit(status: Option<i32>) {
    obs_event!(
        INFO,
        ObsEvent::ProcessExit,
        subject = "claude-child",
        child_exit_status = status,
        exit_source = "handle-wait",
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

/// The child is spawned as given, directly: no shell (architecture §Cross-cutting Patterns).
pub(crate) fn spawn_child(program: &OsString, args: &[OsString]) -> io::Result<Child> {
    Command::new(program).args(args).spawn()
}
