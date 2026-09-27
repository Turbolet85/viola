//! The root copy of the fixture chain. The `viola_e2e::fixtures` copy lands with its first E2E
//! consumer; both follow one contract (test-plan §3 `run` step 2).

use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime};

use rstest::fixture;
use serde_json::{Value, json};
use sysinfo::{Pid, ProcessStatus, ProcessesToUpdate, System};

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

/// The owner record beside a test's home: the test process's pid and OS start time.
const OWNER: &str = "owner.json";

/// `pid`'s OS start time; `None` when it is gone or a zombie.
pub fn process_start(pid: u32) -> Option<u64> {
    let mut sys = System::new();
    let key = Pid::from_u32(pid);
    sys.refresh_processes(ProcessesToUpdate::Some(&[key]), true);
    sys.process(key)
        .filter(|p| p.status() != ProcessStatus::Zombie)
        .map(sysinfo::Process::start_time)
}

/// Removes every dir under `base` whose owner record names a test process that is verifiably gone
/// (its pid dead, or alive with another start time). A dir without a readable record is never
/// touched, whatever its name or age: a sibling may still be writing its record. Returns how many
/// dirs were removed.
pub fn sweep_gone_owners(base: &Path) -> usize {
    let mut removed = 0;
    for entry in fs::read_dir(base).into_iter().flatten().flatten() {
        if !entry.file_type().is_ok_and(|t| t.is_dir()) {
            continue;
        }
        let dir = entry.path();
        let Some(owner) = fs::read(dir.join(OWNER))
            .ok()
            .and_then(|b| serde_json::from_slice::<Value>(&b).ok())
        else {
            continue;
        };
        let (Some(pid), Some(started_at)) = (
            owner["pid"].as_u64().and_then(|p| u32::try_from(p).ok()),
            owner["started_at"].as_u64(),
        ) else {
            continue;
        };
        if process_start(pid) == Some(started_at) {
            continue;
        }
        if fs::remove_dir_all(&dir).is_ok() {
            removed += 1;
        }
    }
    removed
}

/// Records this test process as the owner of `dir`.
pub fn write_owner(dir: &Path) {
    let pid = std::process::id();
    let started_at = process_start(pid).expect("own start time");
    let record = json!({"pid": pid, "started_at": started_at});
    fs::write(dir.join(OWNER), record.to_string()).expect("owner record");
}

/// `<workspace>/target/e2e-home/viola-test-*/` with the home at `home/`, which is never created
/// here: viola creates it with its own modes.
pub struct TestHome {
    dir: Option<tempfile::TempDir>,
    home: PathBuf,
}

impl TestHome {
    /// A test killed mid-run (nextest's fail-fast terminate) never drops its home, and each home
    /// holds a pinned copy of the viola binary: homes whose owner is gone are swept first, unless
    /// homes are being kept.
    pub fn new() -> Self {
        let base = workspace_path("target/e2e-home");
        fs::create_dir_all(&base).expect("e2e-home");
        if !flag("AGENT_RUN_KEEP_HOMES") && !flag("AGENT_RUN_KEEP_FAILED") {
            sweep_gone_owners(&base);
        }
        let dir = tempfile::Builder::new()
            .prefix("viola-test-")
            .tempdir_in(base)
            .expect("tempdir");
        write_owner(dir.path());
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
            // A kept home is no longer owned: no later sweep may take it.
            let _ = fs::remove_file(dir.path().join(OWNER));
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
        let before = Starts::read(&home, name);
        let pty = OuterPty::spawn(Path::new(VIOLA), &args, &[]);
        let mut wrapper = Self {
            stamped,
            name: name.to_owned(),
            pty,
        };
        wrapper.wait_ready(&before);
        wrapper
    }

    /// Readiness (test-plan §3 boot): `process-start` for `self` and `claude-child` and the fake
    /// agent's `start` receipt, each new since `before` (a gone name's files are reused), then the
    /// snapshot with `pid`, `started_at` and `child_pid`, and a heartbeat under 5 s old. The
    /// receipt is written once the agent's terminal is raw, so no key sent after this is
    /// swallowed. A wrapper that exits first fails at once (`run-exited`).
    fn wait_ready(&mut self, before: &Starts) {
        let deadline = Instant::now() + READY_WITHIN;
        loop {
            let now = Starts::read(self.home(), &self.name);
            let started = now.wrapper > before.wrapper
                && now.child > before.child
                && now.receipt > before.receipt;
            if started && snapshot_ready(&self.instance_dir()) && beat_fresh(&self.instance_dir()) {
                return;
            }
            if let Some(code) = self.pty.try_wait() {
                panic!("wrapper {} exited before ready: {code}", self.name);
            }
            assert!(Instant::now() < deadline, "wrapper {} not ready", self.name);
            std::thread::yield_now();
        }
    }

    pub fn instance_dir(&self) -> PathBuf {
        self.home().join("instances").join(&self.name)
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
    pub fn stop(self) -> Stopped {
        self.stop_keep().0
    }

    /// `stop`, handing the home back for a next start of the same name.
    pub fn stop_keep(self) -> (Stopped, StampedHome) {
        let Self {
            stamped, mut pty, ..
        } = self;
        pty.write(b"\x03");
        let code = pty.wait_exit(EXIT_WITHIN);
        drop(pty);
        (Stopped(Some(code)), stamped)
    }
}

/// How many starts the role file and the receipt hold so far.
struct Starts {
    wrapper: usize,
    child: usize,
    receipt: usize,
}

impl Starts {
    fn read(home: &Path, name: &str) -> Self {
        let role = home.join("diagnostics").join(format!("run-{name}.ndjson"));
        let lines: Vec<Value> = fs::read_to_string(role)
            .unwrap_or_default()
            .lines()
            .filter_map(|l| serde_json::from_str(l).ok())
            .collect();
        let started = |subject: &str| {
            lines
                .iter()
                .filter(|l| l["event"] == "process-start" && l["subject"] == subject)
                .count()
        };
        let receipt = fake::receipt(&fake::receipt_path(home, name));
        Self {
            wrapper: started("self"),
            child: started("claude-child"),
            receipt: fake::of_kind(&receipt, "start").len(),
        }
    }
}

/// The instance snapshot's `data`, when it parses.
pub fn snapshot_data(instance_dir: &Path) -> Option<Value> {
    let text = fs::read_to_string(instance_dir.join("snapshot.json")).ok()?;
    let doc: Value = serde_json::from_str(&text).ok()?;
    Some(doc["data"].clone())
}

fn snapshot_ready(instance_dir: &Path) -> bool {
    snapshot_data(instance_dir).is_some_and(|d| {
        d["pid"].is_u64() && d["started_at"].is_string() && d["child_pid"].is_u64()
    })
}

/// The heartbeat's age; `None` when it is absent.
pub fn beat_age(instance_dir: &Path) -> Option<Duration> {
    let modified = fs::metadata(instance_dir.join("heartbeat"))
        .and_then(|m| m.modified())
        .ok()?;
    Some(
        SystemTime::now()
            .duration_since(modified)
            .unwrap_or(Duration::ZERO),
    )
}

fn beat_fresh(instance_dir: &Path) -> bool {
    beat_age(instance_dir).is_some_and(|age| age < Duration::from_secs(5))
}

#[fixture]
pub fn booted_wrapper(stamped_home: StampedHome) -> Wrapper {
    Wrapper::boot(stamped_home, "builder", None, &[])
}
