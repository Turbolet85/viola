//! Security control negatives, Linux and macOS (test-plan §6 Security control negatives and the
//! macOS exit-cause case; security-plan §Authentication & Authorization, IPC access control (Unix)):
//! a per-user socket directory another user may read, or a symlink to a private one, is refused
//! before a wrapper binds and before a client connects, and is left as found. Every child takes a
//! socket root of its own through `Command::env`, so no directory another process shares is ever
//! loosened; a foreign owner is `socket_dir_verdict`'s injected case, one account making none.
#![cfg(unix)]

#[allow(dead_code)]
mod support;

use std::ffi::OsString;
use std::fs;
use std::io::Write as _;
use std::os::unix::fs::PermissionsExt as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use rstest::rstest;
use serde_json::{Value, json};
use support::cli::{Ran, Running, viola};
use support::fake::FAKE;
use support::home::{
    SOCKET_ROOT_VAR, SocketRoot, StampedHome, TestHome, UNWRAPPED, VIOLA, Wrapper, first_life_on,
    snapshot_data,
};
use support::ndjson::read_lines;
use support::start::{child_starts, refused_with};

const DOCUMENT: &str =
    "{\"v\":1,\"error\":\"instance-unreachable\",\"detail\":\"socket-dir-refused\"}\n";
const UNABLE: &str = "unable  builder  instance-unreachable";
/// `send`'s own line, its read-back mirror's form.
const SEND_UNABLE: &str = "[/ ] unable         builder  instance-unreachable";
const HINT: &str = "hint: viola's socket directory must be a directory you own, readable only by \
                    you; viola does not change its permissions";

/// test-plan §10's provisional spine gate, asserted at unit speed on one process.
const SPINE_BOUND: Duration = Duration::from_secs(1);

/// The two ways a directory fails: its mode, or its kind.
#[derive(Clone, Copy, Debug)]
enum Form {
    Open,
    Link,
}

impl Form {
    /// Loosens the 0700 directory `dir`; a link's target is a private directory beside it.
    fn apply(self, dir: &Path) {
        match self {
            Self::Open => {
                fs::set_permissions(dir, fs::Permissions::from_mode(0o755)).expect("chmod");
            }
            Self::Link => {
                let target = target_of(dir);
                fs::rename(dir, &target).expect("moved aside");
                std::os::unix::fs::symlink(&target, dir).expect("symlink");
            }
        }
    }

    /// The loosening reads as it was made.
    fn assert_as_found(self, dir: &Path) {
        let meta = fs::symlink_metadata(dir).expect("the directory");
        match self {
            Self::Open => {
                assert!(meta.is_dir());
                assert_eq!(meta.permissions().mode() & 0o777, 0o755, "the mode moved");
            }
            Self::Link => {
                assert!(meta.file_type().is_symlink(), "the link was replaced");
                let target = fs::metadata(target_of(dir)).expect("the target");
                assert_eq!(
                    target.permissions().mode() & 0o777,
                    0o700,
                    "the target moved"
                );
            }
        }
    }

    /// Back to the 0700 directory the wrapper bound in.
    fn restore(self, dir: &Path) {
        match self {
            Self::Open => {
                fs::set_permissions(dir, fs::Permissions::from_mode(0o700)).expect("chmod back");
            }
            Self::Link => {
                fs::remove_file(dir).expect("the link");
                fs::rename(target_of(dir), dir).expect("moved back");
            }
        }
    }
}

fn target_of(dir: &Path) -> PathBuf {
    dir.with_file_name("real")
}

/// The digit runs of `text`: a pid or a uid shows as one of them, never inside a longer number.
fn digit_runs(text: &str) -> Vec<&str> {
    text.split(|c: char| !c.is_ascii_digit())
        .filter(|run| !run.is_empty())
        .collect()
}

fn own_uid() -> String {
    // SAFETY: `geteuid` takes no arguments, cannot fail and touches no memory.
    unsafe { libc::geteuid() }.to_string()
}

fn wrapper_pid(wrapper: &Wrapper) -> String {
    snapshot_data(&wrapper.instance_dir()).expect("the snapshot")["pid"]
        .as_u64()
        .expect("the pid")
        .to_string()
}

/// No output holds the socket directory, the home, a pid or a uid.
fn assert_names_nothing(what: &str, text: &str, paths: &[&Path], numbers: &[&str]) {
    for path in paths {
        let path = path.to_str().expect("utf-8");
        assert!(!text.contains(path), "{what}: a path");
    }
    let runs = digit_runs(text);
    for number in numbers {
        assert!(!runs.contains(number), "{what}: a pid or uid");
    }
}

fn channel_requests(lines: &[Value]) -> usize {
    lines
        .iter()
        .filter(|l| l["event"] == "channel-request")
        .count()
}

fn role_file(home: &Path, file: &str) -> Vec<Value> {
    read_lines(&home.join("diagnostics").join(file))
}

/// A line with what is unrelated to the refusal taken out: its clock digits say nothing of a uid.
fn without_clocks(line: &Value) -> String {
    let mut line = line.clone();
    if let Some(fields) = line.as_object_mut() {
        fields.remove("timestamp");
        fields.remove("duration_ms");
    }
    line.to_string()
}

/// The verb's words after `--home <home>`, and its stdin. `pause` first: the wheel it takes makes
/// `send`'s control reach the wrapper and refuse at once.
const VERBS: [(&[&str], Option<&str>); 8] = [
    (&["pause", "builder"], None),
    (&["send", "builder"], Some("hello\n")),
    (
        &["answer", "builder", "1"],
        Some("{\"behavior\": \"allow\"}"),
    ),
    (&["wait", "builder", "--timeout-ms", "100"], None),
    (&["last", "builder"], None),
    (&["release", "builder"], None),
    (&["link", "overseer", "builder"], None),
    (&["unlink", "overseer", "builder"], None),
];

fn in_mode<'a>(words: &[&'a str], json: bool) -> Vec<&'a str> {
    let mut args = words.to_vec();
    if json {
        args.push("--json");
    }
    args
}

/// A wrapper bound under its own root, the root's directory made by the start itself.
fn booted(root: &SocketRoot) -> Wrapper {
    first_life_on(StampedHome::unstamped(TestHome::new()), root, false)
}

/// `viola run builder -- <fake>` under `root`, bounded by `Running`.
fn run_on(home: &Path, root: &Path) -> Ran {
    let mut command = Command::new(VIOLA);
    command
        .arg("--home")
        .arg(home)
        .args(["run", "builder", "--", FAKE])
        .env(SOCKET_ROOT_VAR, root)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for name in UNWRAPPED {
        command.env_remove(name);
    }
    Running::over(command.spawn().expect("viola run")).finish()
}

/// Exit 1 with the pair, no child and no bind, the directory as found, the home's log naming the
/// cause and neither the directory nor a uid.
#[rstest]
#[case::open(Form::Open)]
#[case::link(Form::Link)]
fn security_negative_a_start_refuses_the_socket_directory_before_any_bind(#[case] form: Form) {
    let home = TestHome::new();
    let root = SocketRoot::new().made();
    let dir = root.dir();
    form.apply(&dir);
    let ran = run_on(home.path(), root.path());

    assert_eq!(ran.code, Some(1), "{}", ran.stderr);
    assert_eq!(ran.stdout, "");
    assert_eq!(
        ran.stderr,
        format!("unable: the socket directory for builder cannot be trusted\n{HINT}\n")
    );
    assert_eq!(child_starts(home.path()), 0, "a child started");
    assert!(refused_with(home.path(), "socket-dir-refused"));
    form.assert_as_found(&dir);
    assert_eq!(fs::read_dir(&dir).expect("the dir").count(), 0, "a bind");
    let instance = home.path().join("instances").join("builder");
    assert!(!instance.join("snapshot.json").exists(), "a snapshot");

    let lines = role_file(home.path(), "run-builder.ndjson");
    let exit = lines
        .iter()
        .rfind(|l| l["event"] == "process-exit")
        .expect("the exit line");
    let paths = [root.path(), home.path()];
    assert_names_nothing("stderr", &ran.stderr, &paths, &[&own_uid()]);
    assert_names_nothing("line", &without_clocks(exit), &paths, &[&own_uid()]);
}

/// The eight frame verbs in both modes against a live wrapper whose directory was loosened after it
/// bound: exit 21, no frame on either side, the directory as found. Taken back, each same verb
/// reaches the wrapper: the control.
#[rstest]
#[case::open(Form::Open)]
#[case::link(Form::Link)]
fn security_negative_a_frame_verb_refuses_the_socket_directory_before_any_frame(
    #[case] form: Form,
) {
    let root = SocketRoot::new();
    let wrapper = booted(&root);
    let home = wrapper.home().to_path_buf();
    let dir = root.dir();
    let pid = wrapper_pid(&wrapper);
    let served_before = channel_requests(&role_file(&home, "run-builder.ndjson"));
    form.apply(&dir);

    for (words, stdin) in VERBS {
        for json in [true, false] {
            let label = format!("{} json={json}", words[0]);
            let ran = viola(&home, &in_mode(words, json), stdin, None);
            assert_eq!(ran.code, Some(21), "{label}: {}", ran.stderr);
            if json {
                assert_eq!(ran.stdout, DOCUMENT, "{label}");
                assert_eq!(ran.stderr, "", "{label}");
            } else {
                assert_eq!(ran.stdout, "", "{label}");
                let unable = if words[0] == "send" {
                    SEND_UNABLE
                } else {
                    UNABLE
                };
                assert_eq!(ran.stderr.lines().next(), Some(unable), "{label}");
                assert_eq!(ran.stderr.lines().last(), Some(HINT), "{label}");
                assert!(!ran.stderr.contains('\u{1b}'), "{label}: an SGR byte");
            }
            let shown = format!("{}{}", ran.stdout, ran.stderr);
            let paths = [root.path(), home.as_path()];
            assert_names_nothing(&label, &shown, &paths, &[&pid, &own_uid()]);
            form.assert_as_found(&dir);
        }
    }
    let lines = role_file(&home, "cli-builder.ndjson");
    assert_eq!(channel_requests(&lines), 0, "a frame left a client");
    assert_eq!(
        channel_requests(&role_file(&home, "run-builder.ndjson")),
        served_before,
        "a frame reached the wrapper"
    );
    let exits: Vec<&Value> = lines
        .iter()
        .filter(|l| l["event"] == "process-exit")
        .collect();
    assert_eq!(exits.len(), VERBS.len() * 2, "{lines:?}");
    for exit in exits {
        assert_eq!(exit["subject"], "self");
        assert_eq!(exit["exit_code"], 21);
        assert_eq!(exit["detail"], "socket-dir-refused");
        assert_eq!(exit["during"], "connect");
        assert_eq!(exit["level"], "WARN");
        let paths = [root.path(), home.as_path()];
        assert_names_nothing("line", &without_clocks(exit), &paths, &[&pid, &own_uid()]);
    }

    form.restore(&dir);
    for (at, (words, stdin)) in VERBS.iter().enumerate() {
        let ran = viola(&home, words, *stdin, None);
        assert_ne!(ran.code, Some(21), "{}: {}", words[0], ran.stderr);
        assert_eq!(
            channel_requests(&role_file(&home, "cli-builder.ndjson")),
            at + 1,
            "{}: the control sent no frame",
            words[0]
        );
    }
    assert_eq!(wrapper.stop().code(), Some(0));
}

/// `stdin` written from a thread, since a hook may stop reading early.
fn run_hook(
    event: &str,
    instance: &Path,
    stdin: Vec<u8>,
) -> (Option<i32>, Vec<u8>, Vec<u8>, Duration) {
    let started = Instant::now();
    let mut child = Command::new(VIOLA)
        .args(["hook", event])
        .env("VIOLA_NAME", "builder")
        .env("VIOLA_DIR", OsString::from(instance))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("viola hook");
    let mut input = child.stdin.take().expect("stdin");
    let writer = std::thread::spawn(move || {
        let _ = input.write_all(&stdin);
    });
    let mut running = Running::over(child);
    let deadline = started + support::watch::WITHIN;
    while running.still_running() {
        assert!(Instant::now() < deadline, "viola hook never exited");
        std::thread::yield_now();
    }
    let took = started.elapsed();
    let ran = running.finish();
    let _ = writer.join();
    (
        ran.code,
        ran.stdout.into_bytes(),
        ran.stderr.into_bytes(),
        took,
    )
}

/// Both hook tiers fail open: exit 0, empty streams inside the spine bound, no frame, and the one
/// `hook-decision` naming the cause, with the dialog path's `decision_emitted:false`.
#[rstest]
#[case::open(Form::Open)]
#[case::link(Form::Link)]
fn security_negative_a_hook_fails_open_on_the_socket_directory(#[case] form: Form) {
    let root = SocketRoot::new();
    let wrapper = booted(&root);
    let home = wrapper.home().to_path_buf();
    let instance = wrapper.instance_dir();
    let dir = root.dir();
    let served_before = channel_requests(&role_file(&home, "run-builder.ndjson"));
    let hook_lines_before = role_file(&home, "hook-builder.ndjson").len();
    form.apply(&dir);

    let stop = json!({"hook_event_name": "Stop", "last_assistant_message": "x"});
    let permission = json!({"hook_event_name": "PermissionRequest", "tool_name": "Bash",
        "tool_input": {"command": "true"}});
    for (event, payload) in [("stop", stop), ("permission-request", permission)] {
        let (code, stdout, stderr, took) =
            run_hook(event, &instance, payload.to_string().into_bytes());
        assert_eq!(code, Some(0), "{event}");
        assert!(stdout.is_empty(), "{event}: stdout {} bytes", stdout.len());
        assert!(stderr.is_empty(), "{event}: stderr {} bytes", stderr.len());
        assert!(took < SPINE_BOUND, "{event}: took {took:?}");
        form.assert_as_found(&dir);
    }

    assert_eq!(
        channel_requests(&role_file(&home, "run-builder.ndjson")),
        served_before,
        "a frame reached the wrapper"
    );
    let lines = role_file(&home, "hook-builder.ndjson");
    let new = &lines[hook_lines_before..];
    assert_eq!(channel_requests(new), 0, "a frame left the hook");
    let decisions: Vec<&Value> = new
        .iter()
        .filter(|l| l["event"] == "hook-decision")
        .collect();
    assert_eq!(decisions.len(), 2, "{new:?}");
    for decision in &decisions {
        assert_eq!(decision["detail"], "socket-dir-refused");
        assert_eq!(decision["decision_emitted"], false);
        assert_eq!(decision["level"], "WARN");
    }
    let invoked = new
        .iter()
        .find(|l| l["event"] == "hook-invoked" && l["hook_event"] == "permission-request")
        .expect("the dialog's hook-invoked");
    assert!(invoked.get("corr").is_none(), "{invoked}");
    form.restore(&dir);
    assert_eq!(wrapper.stop().code(), Some(0));
}

/// A clean start under `umask 000`: the socket reads back 0600 and the directory it made 0700, the
/// modes set explicitly and never left to the umask.
#[test]
fn security_negative_a_clean_start_under_an_open_umask_keeps_the_socket_private() {
    let root = SocketRoot::new();
    let wrapper = first_life_on(StampedHome::unstamped(TestHome::new()), &root, true);
    let endpoint = snapshot_data(&wrapper.instance_dir()).expect("the snapshot")["endpoint"]
        .as_str()
        .expect("the endpoint")
        .to_owned();
    let dir = root.dir();
    assert_eq!(Path::new(&endpoint).parent(), Some(dir.as_path()));
    let mode = |path: &Path| fs::metadata(path).expect("meta").permissions().mode() & 0o777;
    assert_eq!(mode(Path::new(&endpoint)), 0o600);
    assert_eq!(mode(&dir), 0o700);
    assert_eq!(wrapper.stop().code(), Some(0));
}

/// The oracle reads whole numbers: a uid is found as its own run, never inside a longer number.
#[test]
fn digit_runs_reads_whole_numbers_only() {
    assert_eq!(digit_runs("uid 501, t=15012.5"), ["501", "15012", "5"]);
    assert!(!digit_runs("15012").contains(&"501"));
}
