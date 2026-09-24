//! The root copy of the fixture chain. The `viola_e2e::fixtures` copy lands with its first E2E
//! consumer; both follow one contract (test-plan §3 `run` step 2).

use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, ExitStatus, Stdio};
use std::time::{Duration, Instant};

use rstest::fixture;
use serde_json::Value;

use super::fake;

pub const VIOLA: &str = env!("CARGO_BIN_EXE_viola");
const READY_WITHIN: Duration = Duration::from_secs(20);

/// Keep a test's home for the CI gates (`AGENT_RUN_KEEP_HOMES=1`) or for a failing test's
/// post-mortem (`AGENT_RUN_KEEP_FAILED=1`); remove it otherwise.
pub fn keep_decision(keep_homes: bool, keep_failed: bool, panicking: bool) -> bool {
    keep_homes || (keep_failed && panicking)
}

fn flag(name: &str) -> bool {
    std::env::var(name).is_ok_and(|v| v == "1")
}

/// `<workspace>/target/e2e-home/viola-test-*/` with the home at `home/`, which is never created
/// here: viola creates it with its own modes.
pub struct TestHome {
    dir: Option<tempfile::TempDir>,
    home: PathBuf,
}

impl TestHome {
    pub fn new() -> Self {
        let base = workspace_path("target/e2e-home");
        fs::create_dir_all(&base).expect("e2e-home");
        let dir = tempfile::Builder::new()
            .prefix("viola-test-")
            .tempdir_in(base)
            .expect("tempdir");
        let home = dir.path().join("home");
        Self {
            dir: Some(dir),
            home,
        }
    }

    pub fn path(&self) -> &Path {
        &self.home
    }

    /// The per-test scratch dir the home sits in (for files that are not viola state).
    pub fn scratch(&self) -> &Path {
        self.home.parent().expect("home has a parent")
    }
}

impl Drop for TestHome {
    fn drop(&mut self) {
        let keep = keep_decision(
            flag("AGENT_RUN_KEEP_HOMES"),
            flag("AGENT_RUN_KEEP_FAILED"),
            std::thread::panicking(),
        );
        if let (true, Some(dir)) = (keep, self.dir.take()) {
            let _ = dir.keep();
        }
    }
}

/// A workspace-relative path resolved against the root package's manifest dir.
pub fn workspace_path(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

#[fixture]
pub fn home() -> TestHome {
    TestHome::new()
}

#[fixture]
pub fn fake_agent_path() -> PathBuf {
    PathBuf::from(fake::FAKE)
}

/// Interim seam: stamps come only from `viola verify` against the fake agent (test-plan §12
/// Stamps conflict), and that verb does not exist yet — so this home is NOT stamped, and nothing
/// here writes `ledger/stamps.json`.
pub struct StampedHome {
    pub home: TestHome,
    pub fake: PathBuf,
    pub stamped: bool,
}

#[fixture]
pub fn stamped_home(home: TestHome, fake_agent_path: PathBuf) -> StampedHome {
    StampedHome {
        home,
        fake: fake_agent_path,
        stamped: false,
    }
}

/// `viola run <name> -- <fake agent> --control … --receipt …` on a piped stdin (the PTY seam
/// replaces the pipe when `viola-pty` lands).
pub struct Wrapper {
    pub stamped: StampedHome,
    pub name: String,
    child: Child,
    stdin: Option<ChildStdin>,
}

impl Wrapper {
    /// `script` is workspace-relative, resolved here the way the harness resolves it.
    pub fn boot(stamped: StampedHome, name: &str, script: Option<&str>, extra: &[&str]) -> Self {
        let home = stamped.home.path().to_path_buf();
        let mut cmd = Command::new(VIOLA);
        cmd.arg("--home")
            .arg(&home)
            .args(["run", name, "--"])
            .arg(&stamped.fake)
            .arg("--control")
            .arg(fake::control_path(&home, name))
            .arg("--receipt")
            .arg(fake::receipt_path(&home, name));
        if let Some(script) = script {
            cmd.arg("--script").arg(workspace_path(script));
        }
        let mut child = cmd
            .args(extra)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("viola run");
        let stdin = child.stdin.take();
        let wrapper = Self {
            stamped,
            name: name.to_owned(),
            child,
            stdin,
        };
        wrapper.wait_ready();
        wrapper
    }

    /// Interim readiness (verification-harness rules): `process-start` for `self` and `claude-child`.
    fn wait_ready(&self) {
        let role = self
            .home()
            .join("diagnostics")
            .join(format!("run-{}.ndjson", self.name));
        let deadline = Instant::now() + READY_WITHIN;
        loop {
            let lines: Vec<Value> = fs::read_to_string(&role)
                .unwrap_or_default()
                .lines()
                .filter_map(|l| serde_json::from_str(l).ok())
                .collect();
            let started = |subject: &str| {
                lines
                    .iter()
                    .any(|l| l["event"] == "process-start" && l["subject"] == subject)
            };
            if started("self") && started("claude-child") {
                return;
            }
            assert!(Instant::now() < deadline, "wrapper {} not ready", self.name);
            std::thread::yield_now();
        }
    }

    pub fn home(&self) -> &Path {
        self.stamped.home.path()
    }

    pub fn send(&mut self, bytes: &[u8]) {
        let stdin = self.stdin.as_mut().expect("stdin open");
        stdin.write_all(bytes).expect("write");
        stdin.flush().expect("flush");
    }

    pub fn release(&self) {
        fake::release(&fake::control_path(self.home(), &self.name));
    }

    pub fn receipt(&self) -> PathBuf {
        fake::receipt_path(self.home(), &self.name)
    }

    /// Ctrl-C into the child, then the wrapper's own exit.
    pub fn stop(mut self) -> ExitStatus {
        if let Some(mut stdin) = self.stdin.take() {
            let _ = stdin.write_all(b"\x03");
        }
        self.child.wait().expect("viola exits")
    }
}

impl Drop for Wrapper {
    fn drop(&mut self) {
        if let Ok(None) = self.child.try_wait() {
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }
}

#[fixture]
pub fn booted_wrapper(stamped_home: StampedHome) -> Wrapper {
    Wrapper::boot(stamped_home, "builder", None, &[])
}
