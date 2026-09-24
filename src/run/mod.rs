//! The `run` wrapper's process side: home and role-file setup, the role logger, and the one
//! child-spawn function (the PTY seam replaces its body; nothing else depends on how it spawns).

use std::ffi::OsString;
use std::fs::{self, File, OpenOptions};
use std::io;
use std::path::Path;
use std::process::{Child, Command};
use std::sync::Arc;

use chrono::{DateTime, SecondsFormat, Utc};
use tracing_subscriber::filter::Targets;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::layer::SubscriberExt as _;
use viola_core::{SERVICE_NAME, VERSION, ViolaName};

/// Creates `dir` (and missing parents) owner-only; an existing one is narrowed to 0700 on Unix.
fn ensure_private_dir(dir: &Path) -> io::Result<()> {
    let mut builder = fs::DirBuilder::new();
    builder.recursive(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::{DirBuilderExt as _, PermissionsExt as _};
        builder.mode(0o700).create(dir)?;
        fs::set_permissions(dir, fs::Permissions::from_mode(0o700))
    }
    #[cfg(not(unix))]
    builder.create(dir)
}

/// `<home>/diagnostics/run-<name>.ndjson`, append-only, 0600 on Unix.
pub(crate) fn open_role_file(home: &Path, name: &ViolaName) -> io::Result<Arc<File>> {
    ensure_private_dir(home)?;
    let dir = home.join("diagnostics");
    ensure_private_dir(&dir)?;
    let path = dir.join(format!("run-{}.ndjson", name.as_ref()));
    let mut options = OpenOptions::new();
    options.create(true).append(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::{OpenOptionsExt as _, PermissionsExt as _};
        options.mode(0o600);
        let file = options.open(&path)?;
        file.set_permissions(fs::Permissions::from_mode(0o600))?;
        Ok(Arc::new(file))
    }
    #[cfg(not(unix))]
    Ok(Arc::new(options.open(&path)?))
}

pub(crate) fn timestamp(at: DateTime<Utc>) -> String {
    at.to_rfc3339_opts(SecondsFormat::Millis, true)
}

struct MillisUtc;

impl FormatTime for MillisUtc {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        w.write_str(&timestamp(Utc::now()))
    }
}

/// One JSON object per line, one `write` per line, into the role file only (obs-plan §3).
pub(crate) fn init_role_logger(file: Arc<File>, process: &'static str, instance: &ViolaName) {
    crate::set_panic_sink(file.clone(), process, Some(instance.to_string()));
    let layer = tracing_subscriber::fmt::layer()
        .json()
        .flatten_event(true)
        .with_current_span(false)
        .with_span_list(false)
        .with_ansi(false)
        .log_internal_errors(false)
        .with_timer(MillisUtc)
        .with_writer(file);
    let subscriber = tracing_subscriber::registry()
        .with(Targets::new().with_target("viola", tracing::Level::INFO))
        .with(layer);
    let _ = tracing::subscriber::set_global_default(subscriber);
}

pub(crate) fn log_self_start(name: &ViolaName) {
    tracing::info!(
        event = "process-start",
        process = "run",
        instance = name.as_ref(),
        subject = "self",
        service_name = SERVICE_NAME,
        version = VERSION,
        os = std::env::consts::OS,
        pid = std::process::id(),
        "process-start"
    );
}

pub(crate) fn log_child_start(name: &ViolaName, child_pid: u32) {
    tracing::info!(
        event = "process-start",
        process = "run",
        instance = name.as_ref(),
        subject = "claude-child",
        child_pid,
        "process-start"
    );
}

pub(crate) fn log_child_exit(name: &ViolaName, status: Option<i32>) {
    match status {
        Some(code) => tracing::info!(
            event = "process-exit",
            process = "run",
            instance = name.as_ref(),
            subject = "claude-child",
            child_exit_status = code,
            exit_source = "handle-wait",
            "process-exit"
        ),
        None => tracing::info!(
            event = "process-exit",
            process = "run",
            instance = name.as_ref(),
            subject = "claude-child",
            exit_source = "handle-wait",
            "process-exit"
        ),
    }
}

pub(crate) fn log_self_exit(name: &ViolaName, exit_code: u8, detail: Option<&'static str>) {
    match detail {
        Some(detail) => tracing::error!(
            event = "process-exit",
            process = "run",
            instance = name.as_ref(),
            subject = "self",
            exit_code,
            detail,
            "process-exit"
        ),
        None => tracing::info!(
            event = "process-exit",
            process = "run",
            instance = name.as_ref(),
            subject = "self",
            exit_code,
            "process-exit"
        ),
    }
}

/// The child is spawned as given, directly: no shell (architecture §Cross-cutting Patterns).
pub(crate) fn spawn_child(program: &OsString, args: &[OsString]) -> io::Result<Child> {
    Command::new(program).args(args).spawn()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone as _;

    #[test]
    fn timestamp_is_rfc3339_utc_millis_with_z() {
        let at = Utc
            .with_ymd_and_hms(2026, 9, 24, 6, 7, 8)
            .single()
            .expect("valid date")
            + chrono::Duration::milliseconds(9);
        assert_eq!(timestamp(at), "2026-09-24T06:07:08.009Z");
    }

    #[test]
    fn millis_utc_writes_the_current_timestamp() {
        let mut out = String::new();
        MillisUtc
            .format_time(&mut Writer::new(&mut out))
            .expect("format");
        assert_eq!(out.len(), "2026-09-24T06:07:08.009Z".len());
        assert!(out.ends_with('Z'));
        assert!(out.starts_with("20"));
    }

    #[test]
    fn ensure_private_dir_creates_missing_parents() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let dir = tmp.path().join("a").join("b");
        ensure_private_dir(&dir).expect("create");
        assert!(dir.is_dir());
        ensure_private_dir(&dir).expect("idempotent");
    }
}
