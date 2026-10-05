//! `viola verify` against the fake agent: a test-written spine fixture set and a bounded run of
//! the `viola` binary with its output. Payloads are synthetic and carry the content canary, which
//! may reach no stamp and no role file.

use std::ffi::OsString;
use std::io::Read;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use serde_json::{Value, json};

use super::fake::{FAKE, write_fixture};
use super::home::{VIOLA, workspace_path};
use super::watch::{WITHIN, Watch};

/// The input-box literal and the two modal literals, as the recorded screens show them: test
/// literals, never the product's `SIGNATURES`.
pub const INPUT_BOX_ROW: &str = "  ⏸ manual mode on · ← for agents";
pub const TRUST_ROW: &str = "   Yes, I trust this folder";

pub const CANARY: &str = "canary-chain-value-5c1e";
pub const SPINE: [&str; 4] = ["SessionStart", "UserPromptSubmit", "Stop", "SessionEnd"];

/// The payload a clean print-mode turn hands `event`'s hook.
pub fn spine_payload(event: &str) -> Value {
    let mut body = json!({"hook_event_name": event, "session_id": "s-verify-1", "note": CANARY});
    match event {
        "SessionStart" => body["source"] = json!("startup"),
        "UserPromptSubmit" => body["prompt"] = json!("replaced by the fake agent"),
        "Stop" => body["last_assistant_message"] = json!("ok"),
        _ => {}
    }
    body
}

/// `<fixtures>/<version>/<Event>.default.json` for every spine event but `skip`, and the three
/// screens a clean pair of interactive runs records.
pub fn write_spine_set(fixtures: &Path, version: &str, skip: Option<&str>) {
    for event in SPINE.into_iter().filter(|e| Some(*e) != skip) {
        write_fixture(fixtures, version, event, "default", &spine_payload(event));
    }
    write_screen_set(fixtures, version);
}

/// 24 rows, every one `""` but `at`'s.
pub fn screen_rows(at: &[(usize, &str)]) -> Vec<String> {
    let mut rows = vec![String::new(); 24];
    for (i, row) in at {
        rows[*i] = (*row).to_owned();
    }
    rows
}

/// `<fixtures>/<version>/Screen.<phase>.json` with `rows`.
pub fn write_screen(fixtures: &Path, version: &str, phase: &str, rows: &[String]) {
    let dir = fixtures.join(version);
    std::fs::create_dir_all(&dir).expect("fixture dir");
    let doc = json!({"screen_phase": phase, "cols": 80, "rows": rows});
    std::fs::write(dir.join(format!("Screen.{phase}.json")), doc.to_string()).expect("screen");
}

/// The trust dialog for `modal`, the input box for `ready` and `turn`.
pub fn write_screen_set(fixtures: &Path, version: &str) {
    write_screen(fixtures, version, "modal", &screen_rows(&[(9, TRUST_ROW)]));
    for phase in ["ready", "turn"] {
        write_screen(
            fixtures,
            version,
            phase,
            &screen_rows(&[(23, INPUT_BOX_ROW)]),
        );
    }
}

pub struct Ran {
    pub code: Option<i32>,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

impl Ran {
    pub fn stdout_text(&self) -> String {
        String::from_utf8(self.stdout.clone()).expect("utf-8 stdout")
    }

    pub fn stderr_text(&self) -> String {
        String::from_utf8(self.stderr.clone()).expect("utf-8 stderr")
    }
}

fn drain(pipe: Option<impl Read + Send + 'static>) -> mpsc::Receiver<Vec<u8>> {
    let (tx, rx) = mpsc::channel();
    if let Some(mut pipe) = pipe {
        std::thread::spawn(move || {
            let mut bytes = Vec::new();
            let _ = pipe.read_to_end(&mut bytes);
            let _ = tx.send(bytes);
        });
    }
    rx
}

/// Waits for `child` below the kill line; a child still running at the deadline is killed and the
/// test fails with the watch report. With no `within`, the wait is the runner's (the
/// `send_window_` precedent: a nextest override is its kill).
fn wait_bounded(child: &mut Child, what: &str, within: Option<Duration>) -> Option<i32> {
    let Some(within) = within else {
        return child.wait().expect("wait").code();
    };
    let watch = Watch::start(what);
    let deadline = Instant::now() + within;
    loop {
        if let Some(status) = child.try_wait().expect("try_wait") {
            return status.code();
        }
        watch.note("running");
        if Instant::now() >= deadline {
            let _ = child.kill();
        }
        watch.deadline_check(deadline, &format!("{what} never exited"));
        std::thread::yield_now();
    }
}

/// `viola <args>` with stdin null, stdout and stderr captured, and this process's own `VIOLA_NAME`
/// / `VIOLA_DIR` removed unless `env` sets them.
pub fn viola(args: &[OsString], env: &[(&str, OsString)]) -> Ran {
    viola_with_stdin(args, env, None)
}

/// `viola` with no test-side bound: the runner's own kill bounds it.
pub fn viola_unbounded(args: &[OsString], env: &[(&str, OsString)]) -> Ran {
    spawn_viola(args, env, None, None)
}

/// `viola`, with `stdin` written from a thread when given (the process may stop reading early).
pub fn viola_with_stdin(
    args: &[OsString],
    env: &[(&str, OsString)],
    stdin: Option<Vec<u8>>,
) -> Ran {
    spawn_viola(args, env, stdin, Some(WITHIN))
}

fn spawn_viola(
    args: &[OsString],
    env: &[(&str, OsString)],
    stdin: Option<Vec<u8>>,
    within: Option<Duration>,
) -> Ran {
    let mut cmd = Command::new(VIOLA);
    cmd.args(args)
        .env_remove("VIOLA_NAME")
        .env_remove("VIOLA_DIR")
        .stdin(if stdin.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for (key, value) in env {
        cmd.env(key, value);
    }
    let mut child = cmd.spawn().expect("viola runs");
    if let (Some(bytes), Some(mut input)) = (stdin, child.stdin.take()) {
        std::thread::spawn(move || {
            let _ = std::io::Write::write_all(&mut input, &bytes);
        });
    }
    let stdout = drain(child.stdout.take());
    let stderr = drain(child.stderr.take());
    let code = wait_bounded(&mut child, "viola", within);
    Ran {
        code,
        stdout: stdout.recv().unwrap_or_default(),
        stderr: stderr.recv().unwrap_or_default(),
    }
}

/// `viola --home <home> verify <before> -- <fake> --cli-version <version> --fixtures <fixtures>
/// --screens --turn-stop --trusted-root <workspace root> <after>`: the fake agent replays the
/// recorded screens, and verify's trusted run starts in the cwd, the workspace root.
pub fn verify(
    home: &Path,
    fixtures: &Path,
    version: &str,
    before: &[&str],
    after: &[&str],
    env: &[(&str, OsString)],
) -> Ran {
    let mut args = verify_args(home, fixtures, version, before);
    args.extend(["--screens", "--turn-stop", "--trusted-root"].map(OsString::from));
    args.push(workspace_path("").into());
    args.extend(after.iter().map(OsString::from));
    viola(&args, env)
}

/// `verify` with no screen flag for the fake agent: both interactive runs wait out the gate's
/// maximum (about 11 s in all), so, like a `send_window_` test, it has no test-side bound and the
/// nextest `verify_window_` override is its kill.
pub fn verify_without_screens(home: &Path, fixtures: &Path, version: &str, before: &[&str]) -> Ran {
    viola_unbounded(&verify_args(home, fixtures, version, before), &[])
}

fn verify_args(home: &Path, fixtures: &Path, version: &str, before: &[&str]) -> Vec<OsString> {
    let mut args: Vec<OsString> = vec!["--home".into(), home.into(), "verify".into()];
    args.extend(before.iter().map(OsString::from));
    args.extend(["--", FAKE, "--cli-version", version, "--fixtures"].map(OsString::from));
    args.push(fixtures.into());
    args
}
