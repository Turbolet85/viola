pub mod boot;
pub mod cleanup;
pub mod gate;
pub mod logs;
pub mod run;
pub mod schema_check;
pub mod secret_scan;
pub mod status;
pub mod supervise;

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sysinfo::{Pid, ProcessStatus, ProcessesToUpdate, System};

/// The bounded file-state / pid probe interval (test-plan §3 readiness signal).
pub const POLL: Duration = Duration::from_millis(100);

#[derive(Debug, thiserror::Error)]
pub enum HarnessError {
    #[error("harness file i/o failed")]
    Io(#[from] io::Error),
    #[error("harness record is malformed")]
    Json(#[from] serde_json::Error),
}

/// One command's result: the single JSON document it prints and its exit code (0 · 1 · 2).
#[derive(Debug)]
pub struct Outcome {
    pub doc: Value,
    pub code: u8,
}

impl Outcome {
    pub fn new(doc: Value, ok: bool) -> Self {
        Self {
            doc,
            code: if ok { 0 } else { 1 },
        }
    }

    pub fn usage(cmd: Option<&str>, detail: &str) -> Self {
        Self {
            doc: json!({"v": 1, "cmd": cmd, "ok": false, "reason": "usage", "detail": detail}),
            code: 2,
        }
    }
}

/// The repository the harness was built from; every harness path hangs off it.
#[derive(Debug, Clone)]
pub struct Workspace {
    pub root: PathBuf,
}

impl Workspace {
    pub fn from_build() -> Self {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest
            .parent()
            .and_then(Path::parent)
            .unwrap_or(manifest)
            .to_path_buf();
        Self { root }
    }

    pub fn agent_run(&self) -> PathBuf {
        self.root.join("target").join("agent-run")
    }

    pub fn session_dir(&self, session: &str) -> PathBuf {
        self.agent_run().join(session)
    }

    pub fn artifacts(&self) -> PathBuf {
        self.agent_run().join("artifacts")
    }

    pub fn e2e_home(&self) -> PathBuf {
        self.root.join("target").join("e2e-home")
    }

    /// The harness builds and tests in its own target dir: `viola-harness` itself runs from
    /// `target/debug`, and Windows cannot relink a running executable.
    pub fn cargo_target(&self) -> PathBuf {
        self.root.join("target").join("harness")
    }

    /// Where `boot` step 1 puts the binaries a CLI-driven session runs.
    pub fn harness_bins(&self) -> PathBuf {
        self.cargo_target().join("debug")
    }
}

/// A session id becomes a directory name, so it is held to a path-safe charset.
pub fn valid_session_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 128
        && !id.starts_with('.')
        && id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.'))
}

pub fn exe(dir: &Path, name: &str) -> PathBuf {
    dir.join(format!("{name}{}", std::env::consts::EXE_SUFFIX))
}

/// The directory holding the workspace binaries, found from a harness or test executable: a test
/// binary lives one level down, in `deps/` (test-plan §3 `run` step 2).
pub fn bin_dir_from_exe(exe: &Path) -> PathBuf {
    let parent = exe.parent().unwrap_or(exe);
    match parent.file_name() {
        Some(name) if name == "deps" => parent.parent().unwrap_or(parent).to_path_buf(),
        _ => parent.to_path_buf(),
    }
}

pub fn expired(deadline: Instant) -> bool {
    Instant::now().checked_duration_since(deadline).is_some()
}

pub fn write_json(path: &Path, value: &impl Serialize) -> Result<(), HarnessError> {
    let mut text = serde_json::to_string_pretty(value)?;
    text.push('\n');
    fs::write(path, text)?;
    Ok(())
}

pub fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, HarnessError> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}

/// A process named by pid AND start time, so a reused pid is never mistaken for it (test-plan §3 PID file).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcessId {
    pub pid: u32,
    pub started_at: u64,
}

impl ProcessId {
    /// Identifies a running process now; `None` when it is gone or a zombie.
    pub fn of(pid: u32) -> Option<Self> {
        let mut sys = System::new();
        let key = Pid::from_u32(pid);
        sys.refresh_processes(ProcessesToUpdate::Some(&[key]), true);
        sys.process(key)
            .filter(|p| p.status() != ProcessStatus::Zombie)
            .map(|p| Self {
                pid,
                started_at: p.start_time(),
            })
    }

    pub fn alive(&self) -> bool {
        Self::of(self.pid) == Some(*self)
    }

    /// Kills the process only if it is still this same, live process: an exited-but-unreaped
    /// Unix child (a zombie) is already gone and is never reported as killed.
    pub fn kill(&self) -> bool {
        let mut sys = System::new();
        let key = Pid::from_u32(self.pid);
        sys.refresh_processes(ProcessesToUpdate::Some(&[key]), true);
        sys.process(key)
            .filter(|p| p.status() != ProcessStatus::Zombie && p.start_time() == self.started_at)
            .is_some_and(|p| p.kill())
    }

    /// Polls until the process is gone or the deadline passes; true when gone.
    pub fn wait_gone(&self, deadline: Instant) -> bool {
        loop {
            if !self.alive() {
                return true;
            }
            if expired(deadline) {
                return false;
            }
            std::thread::sleep(POLL);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceRecord {
    pub name: String,
    pub wrapper_pid: u32,
    pub started_at: u64,
    pub child_pid: u32,
    pub child_started_at: u64,
    pub fake_args: Vec<String>,
}

impl InstanceRecord {
    pub fn wrapper(&self) -> ProcessId {
        ProcessId {
            pid: self.wrapper_pid,
            started_at: self.started_at,
        }
    }

    pub fn child(&self) -> ProcessId {
        ProcessId {
            pid: self.child_pid,
            started_at: self.child_started_at,
        }
    }
}

/// `target/agent-run/<session>/session.json`, written by `boot` once readiness passes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SessionRecord {
    pub v: u32,
    pub session: String,
    pub home: PathBuf,
    pub instances: Vec<InstanceRecord>,
    pub ui: Option<Value>,
    pub supervisor_pid: u32,
    pub supervisor_started_at: u64,
    pub cookie_file: Option<PathBuf>,
}

impl SessionRecord {
    pub fn supervisor(&self) -> ProcessId {
        ProcessId {
            pid: self.supervisor_pid,
            started_at: self.supervisor_started_at,
        }
    }
}

pub fn session_record_path(ws: &Workspace, session: &str) -> PathBuf {
    ws.session_dir(session).join("session.json")
}

/// The session record, or the one usage answer every reader gives for an unknown session.
pub fn load_session(ws: &Workspace, cmd: &str, session: &str) -> Result<SessionRecord, Outcome> {
    if !valid_session_id(session) {
        return Err(Outcome::usage(Some(cmd), "invalid-session-id"));
    }
    read_json(&session_record_path(ws, session)).map_err(|_| Outcome {
        doc: json!({"v": 1, "cmd": cmd, "ok": false, "reason": "unknown-session"}),
        code: 2,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_session_id_accepts_test_style_ids() {
        assert!(valid_session_id("default"));
        assert!(valid_session_id("path2_send-p2-1234"));
        assert!(valid_session_id("a.b"));
        assert!(valid_session_id(&"a".repeat(128)));
    }

    #[test]
    fn valid_session_id_rejects_path_escapes() {
        for bad in ["", ".", "..", ".hidden", "a/b", "a\\b", "a b", "é"] {
            assert!(!valid_session_id(bad), "{bad:?}");
        }
        assert!(!valid_session_id(&"a".repeat(129)));
    }

    #[test]
    fn bin_dir_from_test_exe_skips_deps() {
        let base = std::env::temp_dir().join("target").join("debug");
        assert_eq!(bin_dir_from_exe(&base.join("deps").join("t-123")), base);
        assert_eq!(bin_dir_from_exe(&base.join("viola-harness")), base);
    }

    #[test]
    fn exe_appends_platform_suffix() {
        let dir = Path::new("bin");
        let expected = format!("viola{}", std::env::consts::EXE_SUFFIX);
        assert_eq!(exe(dir, "viola"), dir.join(expected));
    }

    #[test]
    fn expired_tracks_the_deadline() {
        assert!(expired(Instant::now()));
        assert!(!expired(Instant::now() + Duration::from_secs(60)));
    }

    #[test]
    fn workspace_root_holds_the_root_manifest() {
        let ws = Workspace::from_build();
        assert!(ws.root.join("Cargo.toml").is_file());
        assert!(ws.root.join("crates").join("viola-e2e").is_dir());
        assert_eq!(ws.session_dir("s"), ws.root.join("target/agent-run/s"));
        assert_eq!(ws.artifacts(), ws.root.join("target/agent-run/artifacts"));
        assert_eq!(ws.e2e_home(), ws.root.join("target/e2e-home"));
        assert_eq!(ws.harness_bins(), ws.root.join("target/harness/debug"));
    }

    #[test]
    fn process_id_sees_itself_alive_and_a_wrong_start_time_dead() {
        let me = ProcessId::of(std::process::id()).expect("this process");
        assert!(me.alive());
        let other = ProcessId {
            pid: me.pid,
            started_at: me.started_at + 1,
        };
        assert!(!other.alive());
        assert!(!other.kill());
        assert!(other.wait_gone(Instant::now()));
    }

    #[test]
    fn process_id_kill_stops_a_child_and_wait_gone_sees_it() {
        let mut child = std::process::Command::new(exe(
            &bin_dir_from_exe(&std::env::current_exe().expect("exe")),
            "viola-fake-agent",
        ))
        .stdin(std::process::Stdio::piped())
        .spawn()
        .expect("fake agent built with the workspace");
        let id = ProcessId::of(child.id()).expect("running");
        assert!(!id.wait_gone(Instant::now() + Duration::from_millis(300)));
        assert!(id.kill());
        let _ = child.wait();
        assert!(id.wait_gone(Instant::now() + Duration::from_secs(10)));
    }

    #[test]
    fn load_session_unknown_is_exit_2() {
        let ws = Workspace::from_build();
        let out = load_session(&ws, "status", "no-such-session-x9").expect_err("unknown");
        assert_eq!(out.code, 2);
        assert_eq!(out.doc["reason"], "unknown-session");
        let bad = load_session(&ws, "status", "../x").expect_err("invalid");
        assert_eq!(bad.code, 2);
        assert_eq!(bad.doc["reason"], "usage");
    }

    #[test]
    fn json_round_trips_through_files() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = tmp.path().join("r.json");
        let rec = ProcessId {
            pid: 7,
            started_at: 9,
        };
        write_json(&path, &rec).expect("write");
        let back: ProcessId = read_json(&path).expect("read");
        assert_eq!(back, rec);
        assert!(read_json::<ProcessId>(&tmp.path().join("missing.json")).is_err());
    }

    #[test]
    fn outcome_codes_follow_ok() {
        assert_eq!(Outcome::new(json!({}), true).code, 0);
        assert_eq!(Outcome::new(json!({}), false).code, 1);
        let usage = Outcome::usage(Some("run"), "x");
        assert_eq!(usage.code, 2);
        assert_eq!(usage.doc["cmd"], "run");
    }
}
