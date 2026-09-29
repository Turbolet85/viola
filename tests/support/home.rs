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
use super::watch::{WITHIN, Watch};

pub const VIOLA: &str = env!("CARGO_BIN_EXE_viola");

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

/// Windows x64: fills `<home>/bin/<key>/conpty/` from one per-run copy under
/// `target/conpty-seed/<key>/` (a hard link, a copy where linking fails) so a first `viola run` in a
/// fresh home finds its ConPTY companions present and only re-hashes them: a first start's two
/// atomic writes cost ~1.2 s under a parallel run on this host. It creates the home, so a test that
/// asserts viola creates nothing never calls it, and `tests/conpty_sideload.rs` (the product's own
/// write path, and tampering that would reach a shared link) never does either. Elsewhere a no-op.
pub fn seed_conpty(home: &Path) {
    #[cfg(all(windows, target_arch = "x86_64"))]
    {
        const FILES: [&str; 2] = ["OpenConsole.exe", "conpty.dll"];
        static KEY: std::sync::OnceLock<String> = std::sync::OnceLock::new();
        let key = KEY.get_or_init(|| {
            let exe = fs::read(VIOLA).expect("viola exe");
            format!(
                "{}-{}",
                viola_core::VERSION,
                viola_state::pin::content_key(&exe)
            )
        });
        let seed = workspace_path("target/conpty-seed").join(key);
        fs::create_dir_all(&seed).expect("seed dir");
        for name in FILES {
            let path = seed.join(name);
            if !path.is_file() {
                let bytes = fs::read(workspace_path("vendor/conpty/1.24.260710001/x64").join(name))
                    .expect("vendored companion");
                let mut tmp = tempfile::NamedTempFile::new_in(&seed).expect("seed temp");
                std::io::Write::write_all(&mut tmp, &bytes).expect("seed write");
                // Another test process may land the same bytes first.
                let _ = tmp.persist_noclobber(&path);
            }
        }
        let dest = home.join("bin").join(key).join("conpty");
        fs::create_dir_all(&dest).expect("conpty dir");
        for name in FILES {
            let (from, to) = (seed.join(name), dest.join(name));
            if !to.exists() && fs::hard_link(&from, &to).is_err() {
                fs::copy(&from, &to).expect("seed copy");
            }
        }
    }
    #[cfg(not(all(windows, target_arch = "x86_64")))]
    let _ = home;
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

/// A home stamped the one sanctioned way: `viola verify` against the fake agent over the committed
/// `fixtures/claude/` set at the recorded version (test-plan §7 Seed strategies). Nothing here
/// writes `ledger/stamps.json`.
pub struct StampedHome {
    pub home: TestHome,
    pub fake: PathBuf,
    pub stamped: bool,
}

impl StampedHome {
    /// A home for the tests that assert the unverified path: nothing is run and nothing written.
    pub fn unstamped(home: TestHome) -> Self {
        Self {
            home,
            fake: PathBuf::from(fake::FAKE),
            stamped: false,
        }
    }
}

/// `stamped` reads only the verb's result: exit 0 and a last line `stamped <recorded>  <n> pass
/// 0 fail`, the count left to the literal-oracle tests since rows land with later chunks.
#[fixture]
pub fn stamped_home(home: TestHome, fake_agent_path: PathBuf) -> StampedHome {
    let ran = super::verify::verify(
        home.path(),
        &workspace_path("fixtures/claude"),
        fake::RECORDED_CLI_VERSION,
        &[],
        &[],
        &[],
    );
    let stdout = String::from_utf8_lossy(&ran.stdout);
    let last = stdout.lines().last().unwrap_or_default();
    let summary = last.starts_with(&format!("stamped {}  ", fake::RECORDED_CLI_VERSION))
        && last.ends_with("pass  0 fail");
    assert!(
        ran.code == Some(0) && summary,
        "viola verify did not stamp the home: exit {:?}, summary line matched {summary}",
        ran.code
    );
    StampedHome {
        home,
        fake: fake_agent_path,
        stamped: true,
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
        args.extend(["--cli-version", fake::RECORDED_CLI_VERSION].map(OsString::from));
        args.extend(extra.iter().map(OsString::from));
        let before = Starts::read(&home, name);
        seed_conpty(&home);
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
        let watch = Watch::start("ready");
        let deadline = Instant::now() + WITHIN;
        loop {
            let now = Starts::read(self.home(), &self.name);
            let started = now.wrapper > before.wrapper
                && now.child > before.child
                && now.receipt > before.receipt;
            // Read in the predicate's short-circuit order: an early read of `snapshot.json` holds it
            // open while the wrapper replaces it, which Windows refuses (viola_state::fs).
            let snapshot = started && snapshot_ready(&self.instance_dir());
            let beat = snapshot && beat_fresh(&self.instance_dir());
            if started && snapshot && beat {
                return;
            }
            watch.note(&format!(
                "starts w{} c{} r{} snapshot {snapshot} beat {beat}",
                now.wrapper, now.child, now.receipt
            ));
            if let Some(code) = self.pty.try_wait() {
                panic!("wrapper {} exited before ready: {code}", self.name);
            }
            watch.deadline_check(deadline, &format!("wrapper {} not ready", self.name));
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

    /// Ctrl-C into the terminal, then the wrapper's own exit and its endpoint gone.
    pub fn stop(self) -> Stopped {
        self.stop_keep().0
    }

    /// `stop`, handing the home back for a next start of the same name. An exit code is not the
    /// endpoint gone: on Windows the exiting wrapper's pipe can still take a connect after the
    /// exit is seen (`.claude/docs/gotchas.md`), so a recorded endpoint is waited out too.
    pub fn stop_keep(self) -> (Stopped, StampedHome) {
        let instance_dir = self.instance_dir();
        let Self {
            stamped, mut pty, ..
        } = self;
        pty.write(b"\x03");
        let code = pty.wait_exit(EXIT_WITHIN);
        let endpoint =
            snapshot_data(&instance_dir).and_then(|d| d["endpoint"].as_str().map(str::to_owned));
        drop(pty);
        if let Some(endpoint) = endpoint {
            wait_endpoint_gone(&endpoint, "stop", |_| {});
        }
        (Stopped(Some(code)), stamped)
    }
}

/// The endpoint is gone for a client (the harness `endpoint_gone` rule,
/// `crates/viola-e2e/src/harness/cleanup.rs`): on Windows a viola-client connect finds no pipe; on
/// Unix the socket file no longer exists.
pub fn unconnectable(endpoint: &str) -> bool {
    #[cfg(windows)]
    {
        matches!(
            viola_channel::Client::connect(endpoint, "cli"),
            Err(viola_channel::ChannelError::Connect(e)) if e.kind() == std::io::ErrorKind::NotFound
        )
    }
    #[cfg(unix)]
    {
        !Path::new(endpoint).exists()
    }
}

/// Polls until `endpoint` is unconnectable, handing each reading to `observe` (`true` = still
/// reachable); fails at `WITHIN` with the watch report.
pub fn wait_endpoint_gone(endpoint: &str, label: &str, mut observe: impl FnMut(bool)) {
    let watch = Watch::start(label);
    let deadline = Instant::now() + WITHIN;
    loop {
        let reachable = !unconnectable(endpoint);
        observe(reachable);
        if !reachable {
            return;
        }
        watch.note("endpoint reachable");
        watch.deadline_check(deadline, &format!("{label}: endpoint still reachable"));
        std::thread::yield_now();
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
        let lines = super::ndjson::read_lines(&role);
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
