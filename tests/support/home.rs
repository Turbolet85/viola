//! The root copy of the fixture chain. The `viola_e2e::fixtures` copy lands with its first E2E
//! consumer; both follow one contract (test-plan §3 `run` step 2).

use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime};

use rstest::fixture;
use serde_json::{Value, json};
use sysinfo::{Pid, ProcessStatus, ProcessesToUpdate, System};

use viola_agent_claude::ledger::LedgerRow;
use viola_pty::Size;

use super::fake;
use super::outer_pty::{EXIT_WITHIN, OuterPty};
use super::watch::{WITHIN, Watch};

pub const VIOLA: &str = env!("CARGO_BIN_EXE_viola");

/// What `viola run` sets in its child: a test-started `viola` runs with none of them, or a verb
/// with no `--home` would take the home of the wrapped session the test runner itself runs in.
pub const UNWRAPPED: [&str; 3] = ["VIOLA_NAME", "VIOLA_DIR", "VIOLA_BIN"];

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

/// Removes an owned dir with its owner record last. A removal that stops part-way (an entry it
/// cannot delete) keeps the record, so a later sweep
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
            if let Some(refused) = headroom_refusal(free_bytes(&target), HEADROOM_FLOOR_MIB) {
                panic!("{refused}");
            }
            return;
        }
    }
    fs::create_dir_all(base).expect("e2e-home");
}

/// The least a linked backing must have free when a test home is made under it. The dev host's
/// tmpfs struck EDQUOT on its per-user quota at about 8 GiB free by `df` (24 G used of 32 G, a
/// run's homes in place), and the quota itself is unreadable from the session's shell, so the
/// filesystem's free bytes stand in for it: a home is refused below 9 GiB, one GiB (about ten
/// pinned 101 163 992 B debug `viola` copies) above that reading. Equal to the harness keeper's.
pub const HEADROOM_FLOOR_MIB: u64 = 9 * 1024;

/// Why a backing with `free` bytes is refused under a floor of `floor_mib` MiB, or `None`.
pub fn headroom_refusal(free: u64, floor_mib: u64) -> Option<String> {
    let free_mib = free / (1024 * 1024);
    (free_mib < floor_mib)
        .then(|| format!("e2e-home backing: {free_mib} MiB free, below the {floor_mib} MiB floor"))
}

/// The bytes an unprivileged writer may still take on the filesystem under `path`. The conversions
/// are identities where `fsblkcnt_t` is 64 bits.
#[cfg(unix)]
#[allow(clippy::useless_conversion)]
fn free_bytes(path: &Path) -> u64 {
    use std::os::unix::ffi::OsStrExt as _;
    let path = std::ffi::CString::new(path.as_os_str().as_bytes()).expect("a path with no NUL");
    // SAFETY: statvfs fills a struct owned here from a NUL-terminated path that outlives the call.
    let stat = unsafe {
        let mut stat: libc::statvfs = std::mem::zeroed();
        assert_eq!(
            libc::statvfs(path.as_ptr(), &mut stat),
            0,
            "e2e-home backing: statvfs failed"
        );
        stat
    };
    u64::from(stat.f_bavail) * u64::from(stat.f_frsize)
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
    /// the named carve-out to the test-data rule.
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

    /// A home for `viola verify --record`, at `<scratch>/vhome`: the print probe's cwd sits under
    /// the home, the scrub rewrites only the user's own home to `~`, and fixture hygiene refuses any
    /// surviving `/home/` component as `absolute-path` (testing.md 2026-10-05).
    pub fn for_record() -> Self {
        let mut tmp = Self::new();
        tmp.home = tmp.home.with_file_name("vhome");
        let user_home = std::env::home_dir().expect("a user home");
        assert!(
            !names_a_home_component(&resolved(&tmp.home), &user_home),
            "a record home holds a path component named home"
        );
        tmp
    }

    pub fn path(&self) -> &Path {
        &self.home
    }

    /// The same home, owning nothing: its drop removes nothing, so a second wrapper booted on it
    /// must be stopped before the owning home is dropped.
    fn borrowed(&self) -> Self {
        Self {
            dir: None,
            home: self.home.clone(),
        }
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

/// Makes a directory a test needs before viola runs, the way viola makes its own folders: 0700,
/// its missing parents too (`viola_state::fs::create_private_dir`), so a home, an instance or a
/// socket directory passes the read rule. `what` names it in the failure.
pub fn private_dir(path: &Path, what: &str) {
    viola_state::fs::create_private_dir(path).unwrap_or_else(|e| panic!("{what}: {e}"));
}

/// Grants another user write on `path`, the way the strict-modes witnesses widen a tree: mode 0770
/// for a folder, 0660 for a file.
pub fn widen(path: &Path) {
    use std::os::unix::fs::PermissionsExt as _;
    let mode = if path.is_dir() { 0o770 } else { 0o660 };
    fs::set_permissions(path, fs::Permissions::from_mode(mode)).expect("chmod");
}

/// Grants another user read on the folder `path`: mode 0750.
pub fn grant_read(path: &Path) {
    use std::os::unix::fs::PermissionsExt as _;
    fs::set_permissions(path, fs::Permissions::from_mode(0o750)).expect("chmod");
}

/// Takes back what [`widen`] or [`grant_read`] granted, so the home can be stopped and removed.
pub fn unwiden(path: &Path) {
    use std::os::unix::fs::PermissionsExt as _;
    let mode = if path.is_dir() { 0o700 } else { 0o600 };
    fs::set_permissions(path, fs::Permissions::from_mode(mode)).expect("chmod");
}

/// Whether `path` still holds the widening a witness made: its group or other bits as [`widen`]
/// or [`grant_read`] set them.
pub fn still_widened(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt as _;
    let mode = fs::metadata(path).expect("meta").permissions().mode() & 0o777;
    matches!(mode, 0o770 | 0o660 | 0o750)
}

/// The spelling viola gives a home once it exists: canonical; a home not there yet is spelt as
/// `run` will have created it, its own name under its parent's spelling. A test oracle, never the
/// product's helper.
pub fn resolved(home: &Path) -> PathBuf {
    if !home.exists() {
        let parent = home.parent().expect("a home has a parent");
        return resolved(parent).join(home.file_name().expect("a home has a name"));
    }
    std::fs::canonicalize(home).expect("an existing home")
}

/// A workspace-relative path resolved against the root package's manifest dir.
pub fn workspace_path(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

#[fixture]
pub fn home() -> TestHome {
    TestHome::new()
}

/// A home given to `viola verify --record`: `TestHome::for_record`.
#[fixture]
pub fn record_home() -> TestHome {
    TestHome::for_record()
}

/// Whether `path`, the user's own home prefix removed, holds a component named exactly `home`.
pub fn names_a_home_component(path: &Path, user_home: &Path) -> bool {
    path.strip_prefix(user_home)
        .unwrap_or(path)
        .components()
        .any(|c| c.as_os_str() == "home")
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
    /// This stamped home again, owning nothing: a second wrapper's in the same home.
    pub fn beside(&self) -> Self {
        Self {
            home: self.home.borrowed(),
            fake: self.fake.clone(),
            stamped: self.stamped,
        }
    }

    /// A home for the tests that assert the unverified path: nothing is run and nothing written.
    pub fn unstamped(home: TestHome) -> Self {
        Self {
            home,
            fake: PathBuf::from(fake::FAKE),
            stamped: false,
        }
    }
}

/// A copy of the one stamp taken for this build of `viola` and the fake agent: the stamp is made
/// once per build under its lock file and copied, file by file, into the test's own home, so the
/// stamp's cost leaves every later test's time and a test that tampers its copy reaches no other.
#[fixture]
pub fn stamped_home(home: TestHome, fake_agent_path: PathBuf) -> StampedHome {
    let (stamp, lock) = stamp_dir();
    copy_tree(&stamp, home.path());
    drop(lock);
    StampedHome {
        home,
        fake: fake_agent_path,
        stamped: true,
    }
}

/// `target/e2e-home/viola-stamp-<key16>/home` (or a later generation's), stamped by `viola verify`
/// the first time a test of this build asks, with the exclusive lock on `viola-stamp-<key16>.lock`
/// held: the lookup, the stamp and the caller's copy all run under it. The stamp dir carries no
/// owner record, so the owner sweep never takes it.
pub fn stamp_dir() -> (PathBuf, fs::File) {
    let base = workspace_path("target/e2e-home");
    prepare_home_base(&base);
    stamp_dir_in(&base, &stamp_key(), take_stamp)
}

/// The file a finished stamp holds beside its `home/`, written after the stamper returns.
const STAMPED: &str = "stamped";

/// Whether the stamp dir `stamp` counts: its completion file and its stamps file stand. A cache
/// restore that keeps a tree's directories and none of its files (rust-cache's save) leaves a stamp
/// that does not.
fn stamp_counts(stamp: &Path) -> bool {
    stamp.join(STAMPED).is_file() && stamp.join("home/ledger/stamps.json").is_file()
}

/// [`stamp_dir`]'s core over `base`, the stamp `key` and the `stamper` that stamps a home. The
/// candidates are `viola-stamp-<key>`, then `viola-stamp-<key>-g<n>` for n = 1, 2, …: the first that
/// counts is returned; one that stands and does not count is left as found; with none counting, the
/// stamp is made at the first name where nothing stands.
pub fn stamp_dir_in(base: &Path, key: &str, stamper: impl FnOnce(&Path)) -> (PathBuf, fs::File) {
    let lock = fs::File::create(base.join(format!("viola-stamp-{key}.lock"))).expect("stamp lock");
    lock.lock().expect("stamp lock");
    let mut stamp = base.join(format!("viola-stamp-{key}"));
    let mut generation = 0u32;
    loop {
        if stamp_counts(&stamp) {
            return (stamp.join("home"), lock);
        }
        if fs::symlink_metadata(&stamp).is_err() {
            break;
        }
        generation += 1;
        stamp = base.join(format!("viola-stamp-{key}-g{generation}"));
    }
    // Under the lock no other process stamps this key: a tmp dir standing is a dead one's.
    remove_stamp_dirs(base, &format!("viola-stamp-{key}.tmp-"));
    let tmp = base.join(format!("viola-stamp-{key}.tmp-{}", std::process::id()));
    private_dir(&tmp, "stamp tmp dir");
    stamper(&tmp.join("home"));
    {
        use std::os::unix::fs::OpenOptionsExt as _;
        fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(tmp.join(STAMPED))
            .expect("stamp completion file");
    }
    fs::rename(&tmp, &stamp).expect("stamp into place");
    sweep_other_stamps(base, key);
    (stamp.join("home"), lock)
}

/// Every dir under `base` whose name starts `prefix`, removed.
fn remove_stamp_dirs(base: &Path, prefix: &str) {
    for entry in fs::read_dir(base).into_iter().flatten().flatten() {
        if entry.file_name().to_string_lossy().starts_with(prefix)
            && entry.file_type().is_ok_and(|t| t.is_dir())
        {
            let _ = fs::remove_dir_all(entry.path());
        }
    }
}

/// The stamps of other builds no process holds: each one's lock taken without waiting, its dirs
/// removed. A stamp another process is making or copying keeps its lock and stays.
fn sweep_other_stamps(base: &Path, key: &str) {
    for entry in fs::read_dir(base).into_iter().flatten().flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        let Some(other) = name
            .strip_prefix("viola-stamp-")
            .and_then(|n| n.strip_suffix(".lock"))
        else {
            continue;
        };
        if other == key {
            continue;
        }
        let Ok(file) = fs::File::open(entry.path()) else {
            continue;
        };
        if file.try_lock().is_ok() {
            remove_stamp_dirs(base, &format!("viola-stamp-{other}"));
        }
    }
}

/// The first 16 hex chars of SHA-256 over the built `viola`'s and the fake agent's SHA-256 and
/// the recorded version.
fn stamp_key() -> String {
    use sha2::{Digest as _, Sha256};
    let mut key = Sha256::new();
    for file in [VIOLA, fake::FAKE] {
        key.update(file_sha256(Path::new(file)));
    }
    key.update(fake::RECORDED_CLI_VERSION.as_bytes());
    key.finalize()[..8]
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// A file's SHA-256, read in blocks.
pub fn file_sha256(path: &Path) -> Vec<u8> {
    use sha2::{Digest as _, Sha256};
    use std::io::Read as _;
    let mut file = fs::File::open(path).expect("open for hashing");
    let mut hasher = Sha256::new();
    let mut block = vec![0u8; 1 << 20];
    loop {
        let n = file.read(&mut block).expect("read for hashing");
        if n == 0 {
            break;
        }
        hasher.update(&block[..n]);
    }
    hasher.finalize().to_vec()
}

/// `src`'s tree made again at `dst`: each dir through `private_dir` (0700), each file copied byte
/// for byte with its mode kept, a new inode every time. Anything else in the tree is a fault.
pub fn copy_tree(src: &Path, dst: &Path) {
    private_dir(dst, "stamped home");
    for entry in fs::read_dir(src).expect("stamp dir") {
        let entry = entry.expect("stamp entry");
        let kind = entry.file_type().expect("stamp entry type");
        let to = dst.join(entry.file_name());
        if kind.is_dir() {
            copy_tree(&entry.path(), &to);
        } else if kind.is_file() {
            fs::copy(entry.path(), &to).expect("copy a stamp file");
        } else {
            panic!("the stamp holds an entry that is neither a file nor a dir");
        }
    }
}

/// `stamped` reads only the verb's result: exit 0 and a last line `stamped <recorded>  <n> pass
/// 0 fail`, the count left to the literal-oracle tests since rows land with later chunks.
pub fn take_stamp(home: &Path) {
    let ran = super::verify::verify(
        home,
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
    // The failing rows' ids only: a step line is fixed text, `[NN/NN] <row id> <words>  fail`.
    let failing: Vec<&str> = stdout
        .lines()
        .filter(|l| l.starts_with('[') && l.ends_with("  fail"))
        .filter_map(|l| l.split(' ').nth(1))
        .collect();
    assert!(
        ran.code == Some(0) && summary,
        "viola verify did not stamp the home: exit {:?}, summary line matched {summary}, failing rows {failing:?}",
        ran.code
    );
}

/// The five rows a stamp with no dialog replay fails.
const DIALOG_ROWS: [LedgerRow; 5] = [
    LedgerRow::QuestionAnswer,
    LedgerRow::PlanApproveRevise,
    LedgerRow::QuestionNotes,
    LedgerRow::DialogConcurrency,
    LedgerRow::PermissionRequestQuestion,
];

/// A home whose stamp fails the dialog rows: `viola verify` against the fake agent over the
/// committed set with no dialog replay, so every other row relied on this OS passes and the five
/// dialog rows fail (exit 1). The counts come from the relied set, so a row added to it moves this
/// precondition with it; the oracle stays the literal in `cli_verify`. `stamped` stays false: the
/// version is not verified.
pub fn dialogless_home(home: TestHome) -> StampedHome {
    let ran = super::verify::verify_without_dialogs(
        home.path(),
        &workspace_path("fixtures/claude"),
        fake::RECORDED_CLI_VERSION,
    );
    let stdout = String::from_utf8_lossy(&ran.stdout);
    let last = stdout.lines().last().unwrap_or_default();
    let (failed, passed): (Vec<LedgerRow>, Vec<LedgerRow>) = LedgerRow::relied()
        .into_iter()
        .partition(|row| DIALOG_ROWS.contains(row));
    let expected = format!(
        "stamped {}  {} pass  {} fail",
        fake::RECORDED_CLI_VERSION,
        passed.len(),
        failed.len()
    );
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
    /// The socket root the wrapper was given when the start made one, kept for its life.
    runtime: Option<SocketRoot>,
}

/// The variable `std::env::home_dir` reads.
pub const USER_HOME_VAR: &str = "HOME";

/// The user home a test's `viola run` or `viola revive` is given when its case names none: a path
/// beside `home` that is never made, so the start's trust step reads `config-absent` and the test
/// runner's own `~/.claude.json` is never locked, read or written.
pub fn unmade_user_home(home: &Path) -> PathBuf {
    home.parent()
        .expect("a test home has a parent")
        .join("no-user-home")
}

/// The variable that places a child's per-user socket directory (its `socket_dir` rule).
pub const SOCKET_ROOT_VAR: &str = "XDG_RUNTIME_DIR";

/// A short directory under `/tmp` a test's own children take as their socket root, per child through
/// `Command::env`: a socket path is capped near 108 bytes, and no test ever loosens a directory
/// another process shares.
#[cfg(unix)]
pub struct SocketRoot(tempfile::TempDir);

#[cfg(unix)]
impl SocketRoot {
    pub fn new() -> Self {
        let root = tempfile::Builder::new()
            .prefix("vs-")
            .tempdir_in("/tmp")
            .expect("the socket root");
        Self(root)
    }

    pub fn path(&self) -> &Path {
        self.0.path()
    }

    /// The per-user directory a child under this root binds in.
    pub fn dir(&self) -> PathBuf {
        self.path().join("viola")
    }

    /// The directory made as a start makes it, at 0700.
    pub fn made(self) -> Self {
        private_dir(&self.dir(), "the socket directory");
        self
    }
}

/// Where a boot is typed, and as whom.
#[derive(Clone, Copy, Default)]
struct Host<'a> {
    /// The directory it is typed in: the fake agent's cwd and its trusted root.
    dir: Option<&'a Path>,
    /// The user home the wrapper is given in place of this process's own.
    user_home: Option<&'a Path>,
    /// No `--home` is passed: the wrapper lives in its user's default home.
    default_home: bool,
    /// The socket root the wrapper is given in place of this process's own.
    socket_root: Option<&'a Path>,
    /// The wrapper starts under `umask 000`, through `/bin/sh`.
    open_umask: bool,
    /// Further variables set on the wrapper alone.
    env: &'a [(&'a str, &'a str)],
    /// `viola run`'s own flags, between the name and `--`.
    flags: &'a [&'a str],
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

    /// `boot_untrusted` as the user whose home directory is `user_home` (`HOME` on the wrapper
    /// alone), typed in `dir`, the fake agent's untrusted cwd, with `env` also set on the wrapper.
    /// The case seeds that user's config file and tells the fake agent of it (`--trust-config`).
    pub fn boot_untrusted_as_user(
        stamped: StampedHome,
        name: &str,
        user_home: &Path,
        dir: &Path,
        env: &[(&str, &str)],
        extra: &[&str],
    ) -> Self {
        let host = Host {
            dir: Some(dir),
            user_home: Some(user_home),
            env,
            ..Host::default()
        };
        Self::boot_with(stamped, name, None, extra, Size::DEFAULT, false, host)
    }

    /// `boot` typed in `dir` (this test's own cwd when none), with `flags` given to `viola run`
    /// before its `--` (and `script`, workspace-relative).
    pub fn boot_flagged(
        stamped: StampedHome,
        name: &str,
        dir: Option<&Path>,
        flags: &[&str],
        script: Option<&str>,
        extra: &[&str],
    ) -> Self {
        let host = Host {
            dir,
            flags,
            ..Host::default()
        };
        Self::boot_with(stamped, name, script, extra, Size::DEFAULT, true, host)
    }

    /// A second stamped home on this wrapper's own home, owning nothing.
    pub fn beside(&self) -> StampedHome {
        self.stamped.beside()
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
            ..Host::default()
        };
        Self::boot_with(stamped, name, None, &[], Size::DEFAULT, true, host)
    }

    /// `boot_in` with `root` as the wrapper's socket root, and with `open_umask` under `umask 000`.
    pub fn boot_on_socket_root(
        stamped: StampedHome,
        name: &str,
        dir: &Path,
        root: &Path,
        open_umask: bool,
        extra: &[&str],
    ) -> Self {
        let host = Host {
            dir: Some(dir),
            socket_root: Some(root),
            open_umask,
            ..Host::default()
        };
        Self::boot_with(stamped, name, None, extra, Size::DEFAULT, true, host)
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
        Self::revive_with(stamped, name, dir, recorded, flags, None)
    }

    /// `revive` as the user whose home directory is `user_home` (`HOME` on the wrapper alone).
    pub fn revive_as_user(
        stamped: StampedHome,
        name: &str,
        dir: &Path,
        recorded: &Path,
        flags: &[&str],
        user_home: &Path,
    ) -> Self {
        Self::revive_with(stamped, name, dir, recorded, flags, Some(user_home))
    }

    fn revive_with(
        stamped: StampedHome,
        name: &str,
        dir: &Path,
        recorded: &Path,
        flags: &[&str],
        user_home: Option<&Path>,
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
        let unmade = unmade_user_home(&home);
        let user = user_home
            .unwrap_or(&unmade)
            .to_str()
            .expect("utf-8 user home");
        let pty = OuterPty::spawn_in(
            Path::new(VIOLA),
            &args,
            &[("PATH", &path), (USER_HOME_VAR, user)],
            Size::DEFAULT,
            dir,
        );
        let mut wrapper = Self {
            stamped,
            name: name.to_owned(),
            pty,
            runtime: None,
        };
        wrapper.wait_ready(&before);
        wrapper
    }

    /// `viola adopt <args…>` typed in `dir` under this test's outer terminal, with the fake agent
    /// as `claude` first on the child's `PATH`; ready as a boot is. `adopt` hands its child no
    /// words of the caller's, so the fake agent's own flags (a revived child's, and `extra`) ride
    /// its `fake-args` file, written before the start; `dir` is the child's cwd, so its trusted
    /// root.
    pub fn adopt(
        stamped: StampedHome,
        name: &str,
        dir: &Path,
        args: &[&str],
        extra: &[&str],
    ) -> Self {
        let home = stamped.home.path().to_path_buf();
        let scratch = stamped.home.scratch().to_path_buf();
        let bin = claude_dir(&scratch);
        let mut flags: Vec<String> = revived_agent_flags(&home, name, dir)
            .iter()
            .map(|flag| flag.to_str().expect("utf-8 flag").to_owned())
            .collect();
        flags.extend(extra.iter().map(|flag| (*flag).to_owned()));
        let refs: Vec<&str> = flags.iter().map(String::as_str).collect();
        fake::write_fake_args(&bin, &refs);
        let mut argv: Vec<OsString> = vec!["--home".into(), home.clone().into(), "adopt".into()];
        argv.extend(args.iter().map(OsString::from));
        let path = path_with(&bin);
        let before = Starts::read(&home, name);
        let user = unmade_user_home(&home);
        let user = user.to_str().expect("utf-8 user home");
        let pty = OuterPty::spawn_without(
            Path::new(VIOLA),
            &argv,
            &[("PATH", &path), (USER_HOME_VAR, user)],
            &UNWRAPPED,
            Size::DEFAULT,
            dir,
        );
        let mut wrapper = Self {
            stamped,
            name: name.to_owned(),
            pty,
            runtime: None,
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
        Self::spawn_with(stamped, name, script, extra, size, trusted, host).ready()
    }

    /// `boot_as_user`'s start, not yet waited ready: its own `viola verify` may run first, which
    /// the case waits on by its log lines before [`Starting::ready`].
    /// It runs under a test-owned socket root (`XDG_RUNTIME_DIR`), so the start's own verify
    /// composes Run B's socket address alike under `pre-push`'s `env -i`, where none is set.
    pub fn start_as_user(stamped: StampedHome, name: &str, user_home: &Path) -> Starting {
        let root = SocketRoot::new();
        let host = Host {
            user_home: Some(user_home),
            socket_root: Some(root.path()),
            ..Host::default()
        };
        let mut starting = Self::spawn_with(stamped, name, None, &[], Size::DEFAULT, true, host);
        starting.wrapper.runtime = Some(root);
        starting
    }

    fn spawn_with(
        stamped: StampedHome,
        name: &str,
        script: Option<&str>,
        extra: &[&str],
        size: Size,
        trusted: bool,
        host: Host<'_>,
    ) -> Starting {
        let cwd = host
            .dir
            .map_or_else(|| std::env::current_dir().expect("cwd"), Path::to_path_buf);
        let home = stamped.home.path().to_path_buf();
        let mut args: Vec<OsString> = Vec::new();
        if !host.default_home {
            args.extend(["--home".into(), home.clone().into()]);
        }
        args.extend(["run", name].map(OsString::from));
        args.extend(host.flags.iter().map(OsString::from));
        args.push("--".into());
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
        let unmade = unmade_user_home(&home);
        let user_home = Some(
            host.user_home
                .unwrap_or(&unmade)
                .to_str()
                .expect("utf-8 user home"),
        );
        let socket_root = host
            .socket_root
            .map(|dir| dir.to_str().expect("utf-8 socket root"));
        let env: Vec<(&str, &str)> = user_home
            .iter()
            .map(|dir| (USER_HOME_VAR, *dir))
            .chain(socket_root.iter().map(|dir| (SOCKET_ROOT_VAR, *dir)))
            .chain(host.env.iter().copied())
            .collect();
        let (program, args) = if host.open_umask {
            let mut shell: Vec<OsString> = vec![
                "-c".into(),
                "umask 000; exec \"$0\" \"$@\"".into(),
                VIOLA.into(),
            ];
            shell.extend(args);
            (Path::new("/bin/sh"), shell)
        } else {
            (Path::new(VIOLA), args)
        };
        // A start with no `--home` would take a wrapped session's home from these, the test
        // runner's own when it runs inside one.
        let pty = OuterPty::spawn_without(program, &args, &env, &UNWRAPPED, size, &cwd);
        let wrapper = Self {
            stamped,
            name: name.to_owned(),
            pty,
            runtime: None,
        };
        Starting { wrapper, before }
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
            // Read in the predicate's short-circuit order: `snapshot.json` only once both
            // processes have started.
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
            if Instant::now() >= deadline {
                self.note_stall(&watch);
            }
            watch.deadline_check(deadline, &format!("wrapper {} not ready", self.name));
            std::thread::yield_now();
        }
    }

    /// A wrapper not ready at the deadline: its own last role-file event, then what the OS says it
    /// is doing, appended to the watch report. It reads the process and never signals it.
    fn note_stall(&self, watch: &Watch) {
        let role = self
            .home()
            .join("diagnostics")
            .join(format!("run-{}.ndjson", self.name));
        let lines = super::ndjson::read_lines(&role);
        let last = lines.last().and_then(|l| l["event"].as_str());
        watch.note(&format!("stall last event {}", last.unwrap_or("none")));
        let control = fake::control_path(self.home(), &self.name);
        let Some(pid) = wrapper_pid(&control) else {
            watch.note("stall pid unknown");
            return;
        };
        watch.note(&format!("stall pid {pid}"));
        for line in process_state(pid) {
            watch.note(&line);
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

    /// `stop`, handing the home back for a next start of the same name. An exit code alone does not
    /// read the endpoint gone, so a recorded endpoint is waited out too.
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

/// The fake agent as `claude` in `<scratch>/claude-bin/`, linked where
/// the volume allows it and copied where not. A revive looks its program up by name, so every
/// test that types `viola revive` puts this directory first on its child's `PATH`: the name never
/// reaches the host's own CLI.
pub fn claude_dir(scratch: &Path) -> PathBuf {
    let dir = scratch.join("claude-bin");
    let claude = dir.join("claude");
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
    let mut command = super::tied::viola();
    command
        .arg("--home")
        .arg(home.path())
        .arg("revive")
        .args(args)
        .current_dir(dir)
        .env_remove(UNWRAPPED[0])
        .env_remove(UNWRAPPED[1])
        .env_remove(UNWRAPPED[2])
        .env("PATH", path_with(&claude_dir(home.scratch())))
        .env(USER_HOME_VAR, unmade_user_home(home.path()))
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());
    command
}

/// No process holds `endpoint` any more, whatever its holder's end was. A killed wrapper removes
/// nothing: its socket file stays, and a connect to it is refused.
pub fn holder_gone(endpoint: &str) -> bool {
    use std::io::ErrorKind::{ConnectionRefused, NotFound};
    std::os::unix::net::UnixStream::connect(endpoint)
        .is_err_and(|e| matches!(e.kind(), ConnectionRefused | NotFound))
}

/// The endpoint is gone for a client (the harness `endpoint_gone` rule,
/// `crates/viola-e2e/src/harness/cleanup.rs`): the socket file no longer exists.
pub fn unconnectable(endpoint: &str) -> bool {
    !Path::new(endpoint).exists()
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
/// A wrapper spawned and not yet ready.
pub struct Starting {
    wrapper: Wrapper,
    before: Starts,
}

impl Starting {
    /// Waits ready as a boot does.
    pub fn ready(mut self) -> Wrapper {
        self.wrapper.wait_ready(&self.before);
        self.wrapper
    }

    /// The wrapper's pid: the outer PTY's own child.
    pub fn pid(&self) -> Option<u32> {
        self.wrapper.pty.child_pid()
    }

    /// The wrapper's exit, when it has exited.
    pub fn exited(&mut self) -> Option<u32> {
        self.wrapper.pty.try_wait()
    }

    /// What the outer terminal has shown so far: the wrapper's stderr among it.
    pub fn shown(&self) -> Vec<u8> {
        self.wrapper.pty.shown()
    }

    pub fn home(&self) -> &Path {
        self.wrapper.home()
    }

    pub fn instance_dir(&self) -> PathBuf {
        self.wrapper.instance_dir()
    }

    /// The wrapper's exit within the recorder's bound, what the terminal showed by then, and the
    /// home back.
    pub fn ended(self) -> (u32, Vec<u8>, StampedHome) {
        let Starting {
            wrapper: Wrapper {
                stamped, mut pty, ..
            },
            ..
        } = self;
        let code = pty.wait_exit(EXIT_WITHIN);
        (code, pty.finish(), stamped)
    }
}

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

/// The pid of the `viola` started with `control` among its arguments (every boot passes the fake
/// agent its own control file, which the fake agent's own arguments hold too).
fn wrapper_pid(control: &Path) -> Option<u32> {
    let mut sys = System::new();
    sys.refresh_processes(ProcessesToUpdate::All, true);
    sys.processes()
        .values()
        .find(|p| {
            let viola = p.exe() == Some(Path::new(VIOLA))
                || p.cmd()
                    .first()
                    .is_some_and(|a| Path::new(a) == Path::new(VIOLA));
            viola && p.cmd().iter().any(|a| Path::new(a) == control)
        })
        .map(|p| p.pid().as_u32())
}

/// `/proc/<pid>/wchan` and the state field of `/proc/<pid>/stat`.
fn process_state(pid: u32) -> Vec<String> {
    let proc = Path::new("/proc").join(pid.to_string());
    let wchan = fs::read_to_string(proc.join("wchan")).unwrap_or_default();
    let stat = fs::read_to_string(proc.join("stat")).unwrap_or_default();
    let state = stat
        .rsplit_once(") ")
        .and_then(|(_, rest)| rest.split_whitespace().next())
        .unwrap_or("unknown");
    vec![format!("stall wchan {} state {state}", wchan.trim())]
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

/// A `builder` wrapper bound under `root`, typed in a directory of the test's own over the committed
/// hook fixtures, its SessionStart logged and that hook ended, so nothing of its own start crosses
/// the channel after this returns.
#[cfg(unix)]
pub fn first_life_on(stamped: StampedHome, root: &SocketRoot, open_umask: bool) -> Wrapper {
    let dir = stamped.home.scratch().join("recorded");
    fs::create_dir(&dir).expect("the test's directory");
    let fixtures = workspace_path("fixtures/claude");
    let fixtures = fixtures.to_str().expect("utf-8 path");
    let wrapper = Wrapper::boot_on_socket_root(
        stamped,
        "builder",
        &dir,
        root.path(),
        open_umask,
        &["--fixtures", fixtures],
    );
    super::events::wait_events(&wrapper.instance_dir(), "the session-start record", |l| {
        l.iter().any(|e| e["kind"] == "session-start")
    });
    super::start::wait_session_start_hooks(&wrapper.receipt(), 1);
    wrapper
}

#[fixture]
pub fn booted_wrapper(stamped_home: StampedHome) -> Wrapper {
    Wrapper::boot(stamped_home, "builder", None, &[])
}
