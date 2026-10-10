//! The root copy of the fixture chain. The `viola_e2e::fixtures` copy lands with its first E2E
//! consumer; both follow one contract (test-plan §3 `run` step 2).

use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime};

use rstest::fixture;
use serde_json::{Value, json};
use sysinfo::{Pid, ProcessStatus, ProcessesToUpdate, System};

use viola_pty::Size;

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
        if remove_owned(&dir) {
            removed += 1;
        }
    }
    removed
}

/// Removes an owned dir with its owner record last. A removal that stops part-way (a file another
/// process still holds open, which Windows refuses to delete) keeps the record, so a later sweep
/// can reclaim the rest once that owner is gone; an ownerless remnant could never be reclaimed.
/// Whether the dir is gone.
pub fn remove_owned(dir: &Path) -> bool {
    let mut whole = true;
    for entry in fs::read_dir(dir).into_iter().flatten().flatten() {
        if entry.file_name() == OWNER {
            continue;
        }
        let path = entry.path();
        let removed = if entry.file_type().is_ok_and(|t| t.is_dir()) {
            fs::remove_dir_all(&path)
        } else {
            fs::remove_file(&path)
        };
        whole &= removed.is_ok();
    }
    whole && fs::remove_file(dir.join(OWNER)).is_ok() && fs::remove_dir(dir).is_ok()
}

/// Records this test process as the owner of `dir`.
pub fn write_owner(dir: &Path) {
    let pid = std::process::id();
    let started_at = process_start(pid).expect("own start time");
    let record = json!({"pid": pid, "started_at": started_at});
    fs::write(dir.join(OWNER), record.to_string()).expect("owner record");
}

/// Prepares the base every test home is created under. A plain base is created as a directory. On
/// Unix a base that is a link is a host's own backing (test-plan §5 Setup / teardown lifecycle): its
/// target must be absolute, is made again with mode 0700 when it is gone (a reboot clears a tmpfs
/// one), and is refused unless it is a real directory with no group or other bit. Nothing is
/// removed and no `home` is created.
pub fn prepare_home_base(base: &Path) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::{DirBuilderExt as _, PermissionsExt as _};
        if fs::symlink_metadata(base).is_ok_and(|m| m.file_type().is_symlink()) {
            let target = fs::read_link(base).expect("e2e-home link");
            assert!(
                target.is_absolute(),
                "e2e-home backing: the link's target is relative"
            );
            // A target that is there answers already-exists and stays as it is: the read below is
            // the verdict either way.
            let _ = fs::DirBuilder::new().mode(0o700).create(&target);
            let meta = fs::symlink_metadata(&target).expect("e2e-home backing");
            assert!(
                meta.is_dir(),
                "e2e-home backing: the link's target is not a real directory"
            );
            assert!(
                meta.permissions().mode() & 0o077 == 0,
                "e2e-home backing: the link's target is not owner-only"
            );
            return;
        }
    }
    fs::create_dir_all(base).expect("e2e-home");
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
        prepare_home_base(&base);
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

    /// A home under the system temp dir, NOT under `target/e2e-home`: outside the CI scans (G2
    /// zero-panics, G4 schema conformance, the secret scan) by construction. Only the forced
    /// feed-panic chaos test takes it, because its contained panic writes a G2-counted line; it is
    /// the second named carve-out to the test-data rule, beside `seed_conpty`.
    pub fn outside_scan() -> Self {
        let dir = tempfile::Builder::new()
            .prefix("viola-chaos-")
            .tempdir()
            .expect("tempdir");
        write_owner(dir.path());
        let home = dir.path().join("home");
        Self {
            dir: Some(dir),
            home,
        }
    }

    /// A home that is a scratch user's default home, `<user home>/.viola`: `scratch()` is then that
    /// user's home directory, which the test creates. A wrapper given that user home and no
    /// `--home` lives here (`Wrapper::boot_as_user`).
    pub fn default_of_user() -> Self {
        let mut tmp = Self::new();
        tmp.home = tmp.home.with_file_name("user").join(".viola");
        tmp
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
        let Some(dir) = self.dir.take() else {
            return;
        };
        let path = dir.keep();
        if keep {
            // A kept home is no longer owned: no later sweep may take it.
            let _ = fs::remove_file(path.join(OWNER));
        } else {
            remove_owned(&path);
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

/// Plants `<home>/statusline-source.json` naming `command` as the user's statusline command, in
/// the settings shape. Only in a home viola already created (a stamped home, or one a wrapper
/// booted in): a home made here would not carry viola's own modes.
pub fn plant_statusline_source(home: &Path, command: &str) -> PathBuf {
    assert!(home.is_dir(), "the home is not one viola created");
    let path = home.join("statusline-source.json");
    let doc = json!({"statusLine": {"type": "command", "command": command}});
    fs::write(&path, doc.to_string()).expect("statusline source");
    path
}

/// The file the test user's statusline command appends to, under the home's `fake/`.
pub fn statusline_marker(home: &Path) -> PathBuf {
    home.join("fake").join("statusline.marker")
}

/// What the fake agent's `statusline-echo` mode prints: a test literal, never the agent's own
/// constant.
pub const STATUSLINE_ECHO_OUTPUT: &str = "viola-fake-statusline\n";

/// The test user's statusline command for this OS: the fake agent's `statusline-echo` mode appending
/// to `marker`, then `extra`. Both paths are written with forward slashes and must hold only
/// characters the override rule leaves unquoted (ASCII letters, digits and `_ - . / :`), so a shell
/// and the fake agent's own split read the same words.
pub fn statusline_echo_command(marker: &Path, extra: &[&str]) -> String {
    let forward = |path: &Path| path.to_str().expect("utf-8 path").replace('\\', "/");
    let mut words = vec![
        forward(Path::new(fake::FAKE)),
        "statusline-echo".to_owned(),
        forward(marker),
    ];
    for path in [&words[0], &words[2]] {
        assert!(
            path.chars()
                .all(|c| c.is_ascii_alphanumeric() || "_-./:".contains(c)),
            "a test statusline path holds a character a shell could misread"
        );
    }
    words.extend(extra.iter().map(|word| (*word).to_owned()));
    words.join(" ")
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

/// A home whose stamp fails the dialog rows: `viola verify` against the fake agent over the
/// committed set with no dialog replay, so the spine, screen and framing rows pass and the four
/// dialog rows fail (`13 pass  4 fail`, exit 1). `stamped` stays false: the version is not verified.
pub fn dialogless_home(home: TestHome) -> StampedHome {
    let ran = super::verify::verify_without_dialogs(
        home.path(),
        &workspace_path("fixtures/claude"),
        fake::RECORDED_CLI_VERSION,
    );
    let stdout = String::from_utf8_lossy(&ran.stdout);
    let last = stdout.lines().last().unwrap_or_default();
    let expected = format!("stamped {}  13 pass  4 fail", fake::RECORDED_CLI_VERSION);
    assert!(
        ran.code == Some(1) && last == expected,
        "viola verify did not stamp the dialog rows failed: exit {:?}, last line {last:?}",
        ran.code
    );
    StampedHome {
        home,
        fake: PathBuf::from(fake::FAKE),
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

/// The variable `std::env::home_dir` reads on this OS.
const USER_HOME_VAR: &str = if cfg!(windows) { "USERPROFILE" } else { "HOME" };

/// Where a boot is typed, and as whom.
#[derive(Clone, Copy, Default)]
struct Host<'a> {
    /// The directory it is typed in: the fake agent's cwd and its trusted root.
    dir: Option<&'a Path>,
    /// The user home the wrapper is given in place of this process's own.
    user_home: Option<&'a Path>,
    /// No `--home` is passed: the wrapper lives in its user's default home.
    default_home: bool,
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
        Self::boot_sized(stamped, name, script, extra, Size::DEFAULT)
    }

    /// `boot` under an outer terminal of `size`.
    pub fn boot_sized(
        stamped: StampedHome,
        name: &str,
        script: Option<&str>,
        extra: &[&str],
        size: Size,
    ) -> Self {
        Self::boot_with(stamped, name, script, extra, size, true, Host::default())
    }

    /// `boot` with the fake agent's cwd untrusted: it shows the recorded trust dialog and fires no
    /// hook (the full gate's witness).
    pub fn boot_untrusted(
        stamped: StampedHome,
        name: &str,
        script: Option<&str>,
        extra: &[&str],
    ) -> Self {
        let host = Host::default();
        Self::boot_with(stamped, name, script, extra, Size::DEFAULT, false, host)
    }

    /// `boot` typed in `dir`, which is the fake agent's cwd and its trusted root.
    pub fn boot_in(stamped: StampedHome, name: &str, dir: &Path, extra: &[&str]) -> Self {
        let host = Host {
            dir: Some(dir),
            ..Host::default()
        };
        Self::boot_with(stamped, name, None, extra, Size::DEFAULT, true, host)
    }

    /// `boot` as the user whose home directory is `user_home`: that one variable is set on the
    /// wrapper alone, and nothing else of this process's user reaches it differently. With
    /// `default_home` no `--home` is passed, so the wrapper lives in that user's default home,
    /// which `stamped`'s home must be (`TestHome::default_of_user`).
    pub fn boot_as_user(
        stamped: StampedHome,
        name: &str,
        user_home: &Path,
        default_home: bool,
    ) -> Self {
        assert!(
            !default_home || stamped.home.path() == user_home.join(".viola"),
            "the test home is not this user's default home"
        );
        let host = Host {
            dir: None,
            user_home: Some(user_home),
            default_home,
        };
        Self::boot_with(stamped, name, None, &[], Size::DEFAULT, true, host)
    }

    /// `viola revive <name> <flags…> -- <the fake agent's flags>` typed in `dir`, with the fake
    /// agent as `claude` first on the child's `PATH`; ready as a boot is. `recorded` is the
    /// directory the first life recorded: the revived child's cwd, so its trusted root.
    pub fn revive(
        stamped: StampedHome,
        name: &str,
        dir: &Path,
        recorded: &Path,
        flags: &[&str],
    ) -> Self {
        let home = stamped.home.path().to_path_buf();
        let scratch = stamped.home.scratch().to_path_buf();
        let mut args: Vec<OsString> = vec!["--home".into(), home.clone().into()];
        args.extend(["revive", name].map(OsString::from));
        args.extend(flags.iter().map(OsString::from));
        args.push("--".into());
        args.extend(revived_agent_flags(&home, name, recorded));
        let path = path_with(&claude_dir(&scratch));
        let before = Starts::read(&home, name);
        let pty = OuterPty::spawn_in(
            Path::new(VIOLA),
            &args,
            &[("PATH", &path)],
            Size::DEFAULT,
            dir,
        );
        let mut wrapper = Self {
            stamped,
            name: name.to_owned(),
            pty,
        };
        wrapper.wait_ready(&before);
        wrapper
    }

    /// Every boot replays the recorded screens (`--screens`); the fake agent's cwd is trusted
    /// unless `trusted` is false. It is this test's own cwd, or `dir` when one is given.
    fn boot_with(
        stamped: StampedHome,
        name: &str,
        script: Option<&str>,
        extra: &[&str],
        size: Size,
        trusted: bool,
        host: Host<'_>,
    ) -> Self {
        let cwd = host
            .dir
            .map_or_else(|| std::env::current_dir().expect("cwd"), Path::to_path_buf);
        let home = stamped.home.path().to_path_buf();
        let mut args: Vec<OsString> = Vec::new();
        if !host.default_home {
            args.extend(["--home".into(), home.clone().into()]);
        }
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
        args.push("--screens".into());
        if trusted {
            args.push("--trusted-root".into());
            args.push(cwd.clone().into());
        }
        args.extend(extra.iter().map(OsString::from));
        let before = Starts::read(&home, name);
        seed_conpty(&home);
        let user_home = host
            .user_home
            .map(|dir| dir.to_str().expect("utf-8 user home"));
        let env: Vec<(&str, &str)> = user_home.iter().map(|dir| (USER_HOME_VAR, *dir)).collect();
        let pty = OuterPty::spawn_in(Path::new(VIOLA), &args, &env, size, &cwd);
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

    /// Resizes the host terminal the wrapper runs in.
    pub fn resize(&mut self, size: Size) {
        self.pty.resize(size);
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

    /// What the outer terminal has shown so far.
    pub fn shown(&self) -> Vec<u8> {
        self.pty.shown()
    }

    /// Kills the wrapper this test booted: no Ctrl-C and no clean exit. Its own exit first, then
    /// the child it recorded gone by pid and start time, then its endpoint answering no one. The
    /// home comes back for a revive.
    pub fn kill(self) -> StampedHome {
        let data = snapshot_data(&self.instance_dir()).expect("the snapshot");
        let child_pid = data["child_pid"]
            .as_u64()
            .and_then(|pid| u32::try_from(pid).ok())
            .expect("the recorded child");
        let child_started = process_start(child_pid).expect("the child runs");
        let endpoint = data["endpoint"].as_str().expect("the endpoint").to_owned();
        let Self {
            stamped, mut pty, ..
        } = self;
        pty.kill();
        pty.wait_exit(EXIT_WITHIN);
        let watch = Watch::start("child");
        let deadline = Instant::now() + WITHIN;
        while process_start(child_pid) == Some(child_started) {
            watch.note("child running");
            watch.deadline_check(deadline, "the child outlived its killed wrapper");
            std::thread::yield_now();
        }
        wait_endpoint(&endpoint, "kill", holder_gone, |_| {});
        stamped
    }
}

/// The fake agent as `claude` (`claude.exe` on Windows) in `<scratch>/claude-bin/`, linked where
/// the volume allows it and copied where not. A revive looks its program up by name, so every
/// test that types `viola revive` puts this directory first on its child's `PATH`: the name never
/// reaches the host's own CLI.
pub fn claude_dir(scratch: &Path) -> PathBuf {
    let dir = scratch.join("claude-bin");
    let claude = dir.join(format!("claude{}", std::env::consts::EXE_SUFFIX));
    if !claude.is_file() {
        fs::create_dir_all(&dir).expect("claude dir");
        if fs::hard_link(fake::FAKE, &claude).is_err() {
            fs::copy(fake::FAKE, &claude).expect("claude copy");
        }
    }
    dir
}

/// This process's `PATH` with `dir` first, for one child (`Command::env`, never `set_var`).
pub fn path_with(dir: &Path) -> String {
    let inherited = std::env::var_os("PATH").unwrap_or_default();
    let dirs = std::iter::once(dir.to_path_buf()).chain(std::env::split_paths(&inherited));
    std::env::join_paths(dirs)
        .expect("PATH")
        .into_string()
        .expect("utf-8 PATH")
}

/// The fake agent's own flags for a revived child, typed after `--`: a revive replays no launch,
/// so nothing else carries them. The control file is a fresh one (the first life's releases are
/// not replayed); the receipt is the first life's, so a second `start` line is a second start.
pub fn revived_agent_flags(home: &Path, name: &str, trusted_root: &Path) -> Vec<OsString> {
    let control = home.join("fake").join(format!("{name}.revived.control"));
    let mut flags: Vec<OsString> = vec!["--control".into(), control.into()];
    flags.push("--receipt".into());
    flags.push(fake::receipt_path(home, name).into());
    flags.push("--fixtures".into());
    flags.push(workspace_path("fixtures/claude").into());
    flags.extend(["--cli-version", fake::RECORDED_CLI_VERSION, "--screens"].map(OsString::from));
    flags.push("--trusted-root".into());
    flags.push(trusted_root.into());
    flags
}

/// `viola --home <home> revive <args…>` outside any terminal, typed in `dir`, both streams piped
/// and the fake agent as `claude` first on its `PATH`. The caller starts it and reads its exit.
pub fn revive_command(home: &TestHome, dir: &Path, args: &[&str]) -> std::process::Command {
    let mut command = std::process::Command::new(VIOLA);
    command
        .arg("--home")
        .arg(home.path())
        .arg("revive")
        .args(args)
        .current_dir(dir)
        .env("PATH", path_with(&claude_dir(home.scratch())))
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());
    command
}

/// No process holds `endpoint` any more, whatever its holder's end was. A killed wrapper removes
/// nothing: on Unix its socket file stays, and a connect to it is refused; on Windows the pipe
/// goes with the process, as after a clean stop.
pub fn holder_gone(endpoint: &str) -> bool {
    #[cfg(windows)]
    {
        unconnectable(endpoint)
    }
    #[cfg(unix)]
    {
        use std::io::ErrorKind::{ConnectionRefused, NotFound};
        std::os::unix::net::UnixStream::connect(endpoint)
            .is_err_and(|e| matches!(e.kind(), ConnectionRefused | NotFound))
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
pub fn wait_endpoint_gone(endpoint: &str, label: &str, observe: impl FnMut(bool)) {
    wait_endpoint(endpoint, label, unconnectable, observe);
}

/// `wait_endpoint_gone` under the rule `gone` gives for an endpoint no client can reach.
fn wait_endpoint(
    endpoint: &str,
    label: &str,
    gone: fn(&str) -> bool,
    mut observe: impl FnMut(bool),
) {
    let watch = Watch::start(label);
    let deadline = Instant::now() + WITHIN;
    loop {
        let reachable = !gone(endpoint);
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
