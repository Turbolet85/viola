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

/// `<fixtures>/<version>/<Event>.default.json` for every spine event but `skip`, the three screens a
/// clean pair of interactive runs records, and the dialog set.
pub fn write_spine_set(fixtures: &Path, version: &str, skip: Option<&str>) {
    for event in SPINE.into_iter().filter(|e| Some(*e) != skip) {
        write_fixture(fixtures, version, event, "default", &spine_payload(event));
    }
    write_screen_set(fixtures, version);
    write_dialog_set(fixtures, version);
}

/// The answers the probe's compiled answers put into the questions' PostToolUse: test literals,
/// never the product's constants.
pub const FREE_TEXT: &str = "viola probe free text";
pub const NOTE: &str = "viola probe note";

fn two_questions() -> Value {
    json!({"questions": [
        {"question": "Probe color?", "options": [{"label": "red"}, {"label": "blue"}]},
        {"question": "Probe size?", "options": [{"label": "small"}, {"label": "large"}]},
    ]})
}

fn one_question(text: &str) -> Value {
    json!({"questions": [{"question": text, "options": [{"label": "yes"}, {"label": "no"}]}]})
}

/// One synthetic tool payload: the event, tool and id, and `extra`'s fields.
pub fn tool_payload(event: &str, tool: &str, id: Option<&str>, extra: Value) -> Value {
    let mut body = json!({"hook_event_name": event, "tool_name": tool, "session_id": "s-verify-1",
        "note": CANARY});
    if let Some(id) = id {
        body["tool_use_id"] = json!(id);
    }
    if let (Some(body), Value::Object(extra)) = (body.as_object_mut(), extra) {
        body.extend(extra);
    }
    body
}

/// The twelve dialog variants a clean dialog run and plan run record, the shapes of the live
/// 2.1.288 probe: `questions-1`, `parallel-1`, `parallel-2`, `permission-1`, `plan-1`, `plan-2`.
pub fn dialog_set() -> Vec<(&'static str, &'static str, Value)> {
    let ask = "AskUserQuestion";
    let plan = "ExitPlanMode";
    let answered = json!({"tool_input": two_questions(), "tool_response": {
        "questions": two_questions()["questions"],
        "answers": {"Probe color?": "red", "Probe size?": FREE_TEXT},
        "annotations": {"Probe color?": {"notes": NOTE}},
    }});
    let reply =
        |q: &str| json!({"tool_input": one_question(q), "tool_response": {"answers": {q: "yes"}}});
    let bash = json!({"tool_input": {"command": "touch viola-probe-permission"}});
    vec![
        (
            "PreToolUse",
            "questions-1",
            tool_payload(
                "PreToolUse",
                ask,
                Some("toolu_q1"),
                json!({"tool_input": two_questions()}),
            ),
        ),
        (
            "PostToolUse",
            "questions-1",
            tool_payload("PostToolUse", ask, Some("toolu_q1"), answered),
        ),
        (
            "PreToolUse",
            "parallel-1",
            tool_payload(
                "PreToolUse",
                ask,
                Some("toolu_p1"),
                json!({"tool_input": one_question("Probe left?")}),
            ),
        ),
        (
            "PostToolUse",
            "parallel-1",
            tool_payload("PostToolUse", ask, Some("toolu_p1"), reply("Probe left?")),
        ),
        (
            "PreToolUse",
            "parallel-2",
            tool_payload(
                "PreToolUse",
                ask,
                Some("toolu_p2"),
                json!({"tool_input": one_question("Probe right?")}),
            ),
        ),
        (
            "PostToolUse",
            "parallel-2",
            tool_payload("PostToolUse", ask, Some("toolu_p2"), reply("Probe right?")),
        ),
        (
            "PermissionRequest",
            "permission-1",
            tool_payload("PermissionRequest", "Bash", None, bash.clone()),
        ),
        (
            "PostToolUse",
            "permission-1",
            tool_payload("PostToolUse", "Bash", Some("toolu_b1"), bash),
        ),
        (
            "PreToolUse",
            "plan-1",
            tool_payload(
                "PreToolUse",
                plan,
                Some("toolu_e1"),
                json!({"tool_input": {"plan": "step one"}}),
            ),
        ),
        (
            "PermissionRequest",
            "plan-1",
            tool_payload(
                "PermissionRequest",
                plan,
                None,
                json!({"tool_input": {"plan": "step one"}}),
            ),
        ),
        (
            "PreToolUse",
            "plan-2",
            tool_payload(
                "PreToolUse",
                plan,
                Some("toolu_e2"),
                json!({"tool_input": {"plan": "steps one and two"}}),
            ),
        ),
        (
            "PostToolUse",
            "plan-2",
            tool_payload(
                "PostToolUse",
                plan,
                Some("toolu_e2"),
                json!({"tool_response": {"plan": "steps one and two"}}),
            ),
        ),
    ]
}

/// `<fixtures>/<version>/<Event>.<variant>.json` for every [`dialog_set`] entry.
pub fn write_dialog_set(fixtures: &Path, version: &str) {
    for (event, variant, body) in dialog_set() {
        write_fixture(fixtures, version, event, variant, &body);
    }
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
/// --screens --turn-stop --dialogs --trusted-root <workspace root> <after>`: the fake agent replays
/// the recorded screens and dialogs, and verify's trusted runs start under the cwd, the workspace
/// root.
pub fn verify(
    home: &Path,
    fixtures: &Path,
    version: &str,
    before: &[&str],
    after: &[&str],
    env: &[(&str, OsString)],
) -> Ran {
    verify_with(home, fixtures, version, before, after, env, true)
}

/// `verify` with no dialog replay: the dialog and plan runs see turns that raise no dialog.
pub fn verify_without_dialogs(home: &Path, fixtures: &Path, version: &str) -> Ran {
    verify_with(home, fixtures, version, &[], &[], &[], false)
}

fn verify_with(
    home: &Path,
    fixtures: &Path,
    version: &str,
    before: &[&str],
    after: &[&str],
    env: &[(&str, OsString)],
    dialogs: bool,
) -> Ran {
    let mut args = verify_args(home, fixtures, version, before);
    args.extend(["--screens", "--turn-stop"].map(OsString::from));
    if dialogs {
        args.push("--dialogs".into());
    }
    args.push("--trusted-root".into());
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
