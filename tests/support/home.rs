//! The root copy of the fixture chain. The `viola_e2e::fixtures` copy lands with its first E2E
//! consumer; both follow one contract (test-plan §3 `run` step 2).

use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use rstest::fixture;
use serde_json::Value;

use super::fake;
use super::outer_pty::{EXIT_WITHIN, OuterPty};

pub const VIOLA: &str = env!("CARGO_BIN_EXE_viola");
/// Below cargo-mutants' 20 s auto-timeout floor, so a mutant that never gets ready is caught, not
/// graded Timeout; readiness itself takes milliseconds.
const READY_WITHIN: Duration = Duration::from_secs(10);

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

/// `viola run <name> -- <fake agent> --control … --receipt …` under an outer PTY, the way a
/// human's terminal hosts it.
pub struct Wrapper {
    pub stamped: StampedHome,
    pub name: String,
    pty: OuterPty,
}

/// The wrapper's exit code once it stopped.
pub struct Stopped(Option<u32>);

impl Stopped {
    pub fn code(&self) -> Option<i32> {
        self.0.and_then(|c| i32::try_from(c).ok())
    }
}

impl Wrapper {
    /// `script` is workspace-relative, resolved here the way the harness resolves it.
    pub fn boot(stamped: StampedHome, name: &str, script: Option<&str>, extra: &[&str]) -> Self {
        let home = stamped.home.path().to_path_buf();
        let mut args: Vec<OsString> = vec!["--home".into(), home.clone().into()];
        args.extend(["run", name, "--"].map(OsString::from));
        args.push(stamped.fake.clone().into());
        args.push("--control".into());
        args.push(fake::control_path(&home, name).into());
        args.push("--receipt".into());
        args.push(fake::receipt_path(&home, name).into());
        if let Some(script) = script {
            args.push("--script".into());
            args.push(workspace_path(script).into());
        }
        args.extend(extra.iter().map(OsString::from));
        let pty = OuterPty::spawn(Path::new(VIOLA), &args, &[]);
        let mut wrapper = Self {
            stamped,
            name: name.to_owned(),
            pty,
        };
        wrapper.wait_ready();
        wrapper
    }

    /// Interim readiness (verification-harness rules): `process-start` for `self` and
    /// `claude-child`, then the fake agent's `start` receipt, written once its terminal is raw so
    /// that no key sent after this can be swallowed. A wrapper that exits first fails at once
    /// (test-plan §3 Readiness: `run-exited`).
    fn wait_ready(&mut self) {
        let role = self
            .home()
            .join("diagnostics")
            .join(format!("run-{}.ndjson", self.name));
        let receipt = self.receipt();
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
            let raw = fake::receipt(&receipt).iter().any(|l| l["kind"] == "start");
            if started("self") && started("claude-child") && raw {
                return;
            }
            if let Some(code) = self.pty.try_wait() {
                panic!("wrapper {} exited before ready: {code}", self.name);
            }
            assert!(Instant::now() < deadline, "wrapper {} not ready", self.name);
            std::thread::yield_now();
        }
    }

    pub fn home(&self) -> &Path {
        self.stamped.home.path()
    }

    pub fn send(&mut self, bytes: &[u8]) {
        self.pty.write(bytes);
    }

    pub fn release(&self) {
        fake::release(&fake::control_path(self.home(), &self.name));
    }

    pub fn receipt(&self) -> PathBuf {
        fake::receipt_path(self.home(), &self.name)
    }

    /// Ctrl-C into the terminal, then the wrapper's own exit.
    pub fn stop(mut self) -> Stopped {
        self.pty.write(b"\x03");
        Stopped(Some(self.pty.wait_exit(EXIT_WITHIN)))
    }
}

#[fixture]
pub fn booted_wrapper(stamped_home: StampedHome) -> Wrapper {
    Wrapper::boot(stamped_home, "builder", None, &[])
}
