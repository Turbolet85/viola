//! The sideloaded ConPTY on Windows x64 (security-plan §Data Protection, Code-bearing artefacts and
//! §Input Validation, Child executable resolution; obs-plan §6): on an intact home `viola run`
//! hosts its child in the pinned `OpenConsole.exe`; a tampered companion is left as found and the
//! child runs on System32's `conhost.exe`; a `conpty.dll` planted in the working directory or on
//! `PATH` never runs; and no home-level line names a sideload path or hash. The host is read through
//! `sysinfo` as the console host process whose parent is the wrapper.
#![cfg(all(windows, target_arch = "x86_64"))]

#[allow(dead_code)]
mod support;

use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use rstest::rstest;
use serde_json::Value;
use support::fake::{self, of_kind};
use support::home::{
    StampedHome, VIOLA, snapshot_data, stamped_home, wait_endpoint_gone, workspace_path,
};
use support::outer_pty::{EXIT_WITHIN, OuterPty};
use support::watch::{WITHIN, Watch};
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System, UpdateKind};
use viola_pty::Size;

/// viola's own human and diagnostic literals (the `tui_passthrough` set): none may reach the
/// terminal.
const VIOLA_LITERALS: [&str; 5] = ["unable:", "hint:", "error:", "\"event\":", "\"process\":"];

/// What no home-level line may hold: a sideload path, either host, or either pinned SHA-256.
const NEVER_LOGGED: [&str; 6] = [
    "conpty/",
    "conpty\\\\",
    "OpenConsole.exe",
    "conhost.exe",
    "39fba2713e2495117b1591ae8c32a3b904bea7aa66069cf7815e2844c76d75d8",
    "b7fd936c2668b87b9ecf7b3366dc6568afc1c6f981874cba3e955a1c35cf8160",
];

fn vendored(name: &str) -> PathBuf {
    workspace_path("vendor/conpty/1.24.260710001/x64").join(name)
}

fn system32(name: &str) -> PathBuf {
    PathBuf::from(std::env::var_os("SystemRoot").expect("SystemRoot"))
        .join("System32")
        .join(name)
}

fn same_file(a: &Path, b: &Path) -> bool {
    match (a.canonicalize(), b.canonicalize()) {
        (Ok(a), Ok(b)) => a.as_os_str().eq_ignore_ascii_case(b.as_os_str()),
        _ => false,
    }
}

/// `<home>/bin/<key>/`: `viola verify` pinned the exe there when it stamped the home.
fn pinned_dir(home: &Path) -> PathBuf {
    let dirs: Vec<PathBuf> = fs::read_dir(home.join("bin"))
        .expect("bin dir")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    assert_eq!(dirs.len(), 1, "{dirs:?}");
    dirs[0].clone()
}

/// Writes the vendored `name` with one byte flipped where `viola run` would pin it.
fn tamper_pinned(home: &Path, name: &str) -> (PathBuf, Vec<u8>) {
    let mut bytes = fs::read(vendored(name)).expect("vendored");
    bytes[1000] ^= 1;
    let dir = pinned_dir(home).join("conpty");
    fs::create_dir_all(&dir).expect("conpty dir");
    let path = dir.join(name);
    fs::write(&path, &bytes).expect("tampered companion");
    (path, bytes)
}

/// A started `viola run <name> -- <fake agent>` under an outer PTY.
struct Run {
    pty: OuterPty,
    home: PathBuf,
    name: String,
}

fn launch(stamped: &StampedHome, name: &str, cwd: Option<&Path>, path_dir: Option<&Path>) -> Run {
    let home = stamped.home.path().to_path_buf();
    let mut args: Vec<OsString> = vec!["--home".into(), home.clone().into()];
    args.extend(["run", name, "--"].map(OsString::from));
    args.push(stamped.fake.clone().into());
    args.push("--control".into());
    args.push(fake::control_path(&home, name).into());
    args.push("--receipt".into());
    args.push(fake::receipt_path(&home, name).into());
    args.extend(["--cli-version", fake::RECORDED_CLI_VERSION].map(OsString::from));
    let path = path_dir.map(|dir| {
        let inherited = std::env::var_os("PATH").unwrap_or_default();
        let dirs = std::iter::once(dir.to_path_buf()).chain(std::env::split_paths(&inherited));
        std::env::join_paths(dirs)
            .expect("PATH")
            .into_string()
            .expect("utf-8 PATH")
    });
    let env: Vec<(&str, &str)> = path.iter().map(|p| ("PATH", p.as_str())).collect();
    let cwd = cwd.map_or_else(|| std::env::current_dir().expect("cwd"), Path::to_path_buf);
    let pty = OuterPty::spawn_in(Path::new(VIOLA), &args, &env, Size::DEFAULT, &cwd);
    Run {
        pty,
        home,
        name: name.to_owned(),
    }
}

impl Run {
    fn instance_dir(&self) -> PathBuf {
        self.home.join("instances").join(&self.name)
    }

    /// The fake agent's start receipt, then the wrapper's `process-start{subject:"claude-child"}`
    /// line (written after the snapshot records the child); the snapshot's wrapper pid.
    fn wait_started(&mut self) -> u32 {
        fake::wait_for(&fake::receipt_path(&self.home, &self.name), "start", |l| {
            !of_kind(l, "start").is_empty()
        });
        let watch = Watch::start("child-start");
        let deadline = Instant::now() + WITHIN;
        loop {
            if self.try_child_start().is_some()
                && let Some(pid) = snapshot_data(&self.instance_dir())
                    .filter(|d| d["child_pid"].is_u64())
                    .and_then(|d| d["pid"].as_u64())
            {
                return u32::try_from(pid).expect("pid");
            }
            assert!(self.pty.try_wait().is_none(), "the wrapper exited");
            watch.deadline_check(deadline, "no claude-child process-start");
            std::thread::yield_now();
        }
    }

    /// Ctrl-C, then the run ended as a run does: exit 0 and its endpoint gone, no
    /// `process-exit{exit_code:1}` and no panic line. The outer stream.
    fn stop_clean(mut self) -> Vec<u8> {
        let endpoint = snapshot_data(&self.instance_dir())
            .and_then(|d| d["endpoint"].as_str().map(str::to_owned));
        self.pty.write(b"\x03");
        assert_eq!(self.pty.wait_exit(EXIT_WITHIN), 0);
        if let Some(endpoint) = endpoint {
            wait_endpoint_gone(&endpoint, "stop", |_| {});
        }
        let lines = self.role_lines();
        assert!(
            !lines
                .iter()
                .any(|l| l["event"] == "process-exit" && l["exit_code"] == 1),
            "a process-exit with exit code 1"
        );
        assert!(!lines.iter().any(|l| l["event"] == "panic"));
        self.pty.finish()
    }

    fn role_lines(&self) -> Vec<Value> {
        support::ndjson::read_lines(
            &self
                .home
                .join("diagnostics")
                .join(format!("run-{}.ndjson", self.name)),
        )
    }

    fn try_child_start(&self) -> Option<Value> {
        self.role_lines()
            .into_iter()
            .rfind(|l| l["event"] == "process-start" && l["subject"] == "claude-child")
    }

    /// The wrapper's `process-start{subject:"claude-child"}` line.
    fn child_start(&self) -> Value {
        self.try_child_start().expect("claude-child process-start")
    }
}

fn processes() -> System {
    let mut sys = System::new();
    sys.refresh_processes_specifics(
        ProcessesToUpdate::All,
        true,
        ProcessRefreshKind::nothing().with_exe(UpdateKind::Always),
    );
    sys
}

/// The console host processes whose parent is `wrapper`.
fn hosts(wrapper: u32) -> Vec<PathBuf> {
    let sys = processes();
    let parent = Pid::from_u32(wrapper);
    sys.processes()
        .values()
        .filter(|p| p.parent() == Some(parent))
        .filter_map(|p| p.exe().map(Path::to_path_buf))
        .filter(|exe| {
            exe.file_name().is_some_and(|n| {
                n.eq_ignore_ascii_case("OpenConsole.exe") || n.eq_ignore_ascii_case("conhost.exe")
            })
        })
        .collect()
}

fn assert_hosted_by(wrapper: u32, host: &Path) {
    let found = hosts(wrapper);
    assert!(
        found.len() == 1 && same_file(&found[0], host),
        "hosts {found:?}, expected {}",
        host.display()
    );
}

fn holds(haystack: &[u8], needle: &str) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle.as_bytes())
}

/// No home-level diagnostics line holds a sideload path, a host name or a pinned hash.
fn assert_nothing_logged(home: &Path) {
    for entry in fs::read_dir(home.join("diagnostics"))
        .expect("diagnostics")
        .flatten()
    {
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "ndjson") {
            let text = fs::read_to_string(&path).expect("role file");
            for needle in NEVER_LOGGED {
                assert!(!text.contains(needle), "{needle:?} in {}", path.display());
            }
        }
    }
}

#[rstest]
fn run_hosts_the_child_in_the_pinned_open_console(stamped_home: StampedHome) {
    let mut run = launch(&stamped_home, "builder", None, None);
    let wrapper = run.wait_started();
    let pinned = pinned_dir(stamped_home.home.path()).join("conpty");
    assert_hosted_by(wrapper, &pinned.join("OpenConsole.exe"));
    let start = run.child_start();
    assert_eq!(start["pty_backend"], "conpty-sideload");
    assert!(start.get("sideload_fallback").is_none(), "{start}");
    for name in ["OpenConsole.exe", "conpty.dll"] {
        assert_eq!(
            fs::read(pinned.join(name)).expect("pinned companion"),
            fs::read(vendored(name)).expect("vendored")
        );
    }
    run.stop_clean();
    assert_nothing_logged(stamped_home.home.path());
}

#[rstest]
fn run_on_a_tampered_open_console_falls_back_to_conhost_and_says_nothing(
    stamped_home: StampedHome,
) {
    let (tampered, bytes) = tamper_pinned(stamped_home.home.path(), "OpenConsole.exe");
    let mut run = launch(&stamped_home, "builder", None, None);
    let wrapper = run.wait_started();
    assert_hosted_by(wrapper, &system32("conhost.exe"));
    let start = run.child_start();
    assert_eq!(start["pty_backend"], "conpty");
    assert_eq!(start["sideload_fallback"], "hash-mismatch");
    let stream = run.stop_clean();
    assert_eq!(fs::read(&tampered).expect("left as found"), bytes);
    for literal in VIOLA_LITERALS {
        assert!(!holds(&stream, literal), "viola wrote {literal:?}");
    }
    assert_nothing_logged(stamped_home.home.path());
}

#[rstest]
fn run_on_a_tampered_conpty_dll_records_the_fallback(stamped_home: StampedHome) {
    let (tampered, bytes) = tamper_pinned(stamped_home.home.path(), "conpty.dll");
    let mut run = launch(&stamped_home, "builder", None, None);
    let wrapper = run.wait_started();
    assert_hosted_by(wrapper, &system32("conhost.exe"));
    let start = run.child_start();
    assert_eq!(start["pty_backend"], "conpty");
    assert_eq!(start["sideload_fallback"], "hash-mismatch");
    run.stop_clean();
    assert_eq!(fs::read(&tampered).expect("left as found"), bytes);
    assert_nothing_logged(stamped_home.home.path());
}

/// Real copies of both files in the working directory and on `PATH`, the pinned `conpty.dll`
/// tampered so nothing is pre-loaded: the bare-name load under the search restriction finds
/// neither copy.
#[rstest]
fn run_never_runs_a_planted_conpty(stamped_home: StampedHome) {
    let scratch = stamped_home.home.scratch().to_path_buf();
    let planted: Vec<PathBuf> = ["plant-cwd", "plant-path"]
        .iter()
        .map(|d| {
            let dir = scratch.join(d);
            fs::create_dir_all(&dir).expect("plant dir");
            for name in ["conpty.dll", "OpenConsole.exe"] {
                fs::copy(vendored(name), dir.join(name)).expect("plant");
            }
            dir
        })
        .collect();
    tamper_pinned(stamped_home.home.path(), "conpty.dll");
    let mut run = launch(
        &stamped_home,
        "builder",
        Some(&planted[0]),
        Some(&planted[1]),
    );
    let wrapper = run.wait_started();
    assert_hosted_by(wrapper, &system32("conhost.exe"));
    let from_plants: Vec<PathBuf> = processes()
        .processes()
        .values()
        .filter_map(|p| p.exe().map(Path::to_path_buf))
        .filter(|exe| {
            planted
                .iter()
                .any(|dir| exe.parent().is_some_and(|p| same_file(p, dir)))
        })
        .collect();
    assert!(from_plants.is_empty(), "{from_plants:?}");
    assert_eq!(run.child_start()["sideload_fallback"], "hash-mismatch");
    run.stop_clean();
    assert_nothing_logged(stamped_home.home.path());
}

/// Two starts of one version on one home write and verify the same companions at once.
#[rstest]
fn two_concurrent_runs_both_load_the_sideload(#[from(stamped_home)] stamped: StampedHome) {
    let mut first = launch(&stamped, "builder", None, None);
    let mut second = launch(&stamped, "second", None, None);
    let wrappers = [first.wait_started(), second.wait_started()];
    let host = pinned_dir(stamped.home.path()).join("conpty/OpenConsole.exe");
    for (run, wrapper) in [&first, &second].into_iter().zip(wrappers) {
        let start = run.child_start();
        assert_eq!(start["pty_backend"], "conpty-sideload", "{start}");
        assert_hosted_by(wrapper, &host);
    }
    for run in [first, second] {
        run.stop_clean();
    }
    assert_nothing_logged(stamped.home.path());
}
