//! A `viola run` start as root tests read it (test-plan §6 Path 1): the role file's lines, a second
//! start refused outside any terminal, the endpoint a start computes and the plugin dir it handed
//! the fake agent; and the starts `viola revive`'s tests stand on: a first life, stopped, the agents
//! list a revive reads, a revive typed outside any terminal, and a wrapper killed from outside.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::Instant;

use serde_json::Value;
use viola_core::ViolaName;

use super::cli::{Ran, Running};
use super::events::wait_events;
use super::fake::{self, FAKE, of_kind};
use super::home::{
    StampedHome, TestHome, VIOLA, Wrapper, claude_dir, revive_command, workspace_path,
};
use super::piped::Piped;
use super::watch::{WITHIN, Watch};

/// The session id the committed 2.1.287 `SessionStart.default` payload carries.
pub const FIXTURE_SESSION: &str = "c0f0cc23-690b-45dd-bbfb-06d6cfd44942";

/// The complete lines of `builder`'s run role file.
pub fn role_lines(home: &Path) -> Vec<Value> {
    super::ndjson::read_lines(&home.join("diagnostics").join("run-builder.ndjson"))
}

/// How many children `builder`'s starts have spawned.
pub fn child_starts(home: &Path) -> usize {
    role_lines(home)
        .iter()
        .filter(|l| l["event"] == "process-start" && l["subject"] == "claude-child")
        .count()
}

/// A second `viola run builder` outside any terminal. A start that is (wrongly) not refused gets
/// Ctrl-C at once, so the call still ends.
pub fn run_refused(home: &Path) -> Output {
    let piped = Piped::spawn(
        Command::new(VIOLA)
            .arg("--home")
            .arg(home)
            .args(["run", "builder", "--", FAKE]),
    );
    piped.write(b"\x03");
    piped.finish()
}

/// `builder`'s role file holds the refused start's `process-exit` with `detail`.
pub fn refused_with(home: &Path, detail: &str) -> bool {
    role_lines(home).iter().any(|l| {
        l["event"] == "process-exit"
            && l["subject"] == "self"
            && l["level"] == "ERROR"
            && l["exit_code"] == 1
            && l["detail"] == detail
    })
}

/// The endpoint `viola run builder` computes for `home` (its golden vectors are viola-channel's),
/// over the home's canonical spelling, which `run` hashes once the home exists.
pub fn endpoint_of(home: &Path) -> String {
    let home = super::home::resolved(home);
    let name = ViolaName::try_new("builder".to_owned()).expect("valid");
    viola_channel::endpoint_path(&name, &home).expect("endpoint")
}

/// A `builder` wrapper typed in `dir` over the committed hook fixtures, its SessionStart logged
/// and that hook ended.
pub fn first_life(stamped: StampedHome, dir: &Path) -> Wrapper {
    let fixtures = workspace_path("fixtures/claude");
    let fixtures = fixtures.to_str().expect("utf-8 path");
    let wrapper = Wrapper::boot_in(stamped, "builder", dir, &["--fixtures", fixtures]);
    wait_events(&wrapper.instance_dir(), "the session-start record", |l| {
        l.iter().any(|e| e["kind"] == "session-start")
    });
    wait_session_start_hooks(&wrapper.receipt(), 1);
    wrapper
}

/// The receipt's `hook` lines for SessionStart number `count`: each such hook has ended.
pub fn wait_session_start_hooks(receipt: &Path, count: usize) {
    fake::wait_for(receipt, "the SessionStart hook's receipt", |lines| {
        of_kind(lines, "hook")
            .iter()
            .filter(|hook| hook["event"] == "SessionStart")
            .count()
            == count
    });
}

/// A directory the test made inside its own scratch.
pub fn made_dir(home: &TestHome, name: &str) -> PathBuf {
    let dir = home.scratch().join(name);
    fs::create_dir(&dir).expect("the test's directory");
    dir
}

/// A first life typed in a directory of the test's own, stopped: the home, the directory and how
/// many role-file lines the first life left.
pub fn stopped_life(stamped: StampedHome) -> (StampedHome, PathBuf, usize) {
    let dir = made_dir(&stamped.home, "recorded");
    let (stopped, stamped) = first_life(stamped, &dir).stop_keep();
    assert_eq!(stopped.code(), Some(0));
    let before = role_lines(stamped.home.path()).len();
    (stamped, dir, before)
}

/// Removes `dir` once nothing holds it. A directory a process still has as its own cannot be
/// removed on Windows, and a wrapper's console host can outlive the wrapper by a moment.
pub fn remove_dir_once_free(dir: &Path) {
    let watch = Watch::start("remove");
    let deadline = Instant::now() + WITHIN;
    while fs::remove_dir_all(dir).is_err() {
        watch.note("the directory is held");
        watch.deadline_check(deadline, "the recorded directory could not be removed");
        std::thread::yield_now();
    }
}

/// The `claude` a revive finds answers `agents --json` with `mode` when given, else with `rows`.
pub fn plant_agents(home: &TestHome, mode: Option<&str>, rows: &str) {
    let dir = claude_dir(home.scratch());
    if let Some(mode) = mode {
        fs::write(dir.join("agents-mode"), mode).expect("the agents mode");
    }
    fs::write(dir.join("agents-rows.json"), rows).expect("the agents rows");
}

/// One live session listed under `id`, in the measured row shape.
pub fn listed(id: &str) -> String {
    serde_json::json!([{
        "cwd": "~/elsewhere", "kind": "interactive", "name": "another session",
        "pid": 4_294_967_295_u32, "sessionId": id, "startedAt": 1_760_000_000_000_u64,
        "status": "idle",
    }])
    .to_string()
}

/// `viola revive <args…>` outside any terminal, typed in `dir`; its exit and both streams.
pub fn revive(home: &TestHome, dir: &Path, args: &[&str]) -> Ran {
    let child = revive_command(home, dir, args).spawn().expect("viola");
    Running::over(child).finish()
}

/// A wrapper process the test started, ended with no chance to clean up.
pub fn kill_wrapper(pid: u32) {
    #[cfg(windows)]
    {
        use windows_sys::Win32::Foundation::CloseHandle;
        use windows_sys::Win32::System::Threading::{
            OpenProcess, PROCESS_TERMINATE, TerminateProcess,
        };
        // SAFETY: plain values cross; the handle is closed before it goes out of scope.
        unsafe {
            let handle = OpenProcess(PROCESS_TERMINATE, 0, pid);
            assert!(!handle.is_null(), "open the wrapper");
            assert_ne!(TerminateProcess(handle, 1), 0, "terminate the wrapper");
            CloseHandle(handle);
        }
    }
    #[cfg(unix)]
    {
        let killed = Command::new("kill")
            .args(["-KILL", &pid.to_string()])
            .status()
            .expect("kill");
        assert!(killed.success(), "kill -KILL");
    }
}

/// The `--plugin-dir` the wrapper's last start handed the fake agent.
pub fn last_plugin_dir(wrapper: &Wrapper) -> PathBuf {
    let receipt = fake::receipt(&wrapper.receipt());
    let start = of_kind(&receipt, "start")
        .last()
        .copied()
        .cloned()
        .expect("start receipt");
    PathBuf::from(start["plugin_dir"].as_str().expect("plugin_dir"))
}
