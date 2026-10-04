//! Critical Path 4, its cli + wrapper channel half (test-plan §6 Path 4; architecture §Standard
//! Contracts `hook.dialog` / `answer`): a `question` and a `plan` raised through the dialog-tier
//! hooks are each logged once with a wrapper-assigned `dialog_id`, wake a parked `viola wait`, and
//! are answered by id with `viola answer`; the hook prints the decision body the answer maps to
//! (S3 / S7 / S8). The PermissionRequest that repeats a PreToolUse is the same dialog: it logs
//! nothing and carries only a plan's revise. A second concurrent dialog, an unknown id and an
//! unverified CLI are each left to the human. The dialog payloads are the relayed 2.1.287 captures
//! (`fixtures/claude/2.1.287/RELAYED.md`); the canary rides only the answers and the driver's text.
//! The `permission` kind's end-to-end case is owed to the live test (working-route `:84`).

#[allow(dead_code)]
mod support;

use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::Instant;

use rstest::rstest;
use serde_json::{Value, json};
use support::fake::{self, of_kind};
use support::home::{StampedHome, TestHome, VIOLA, Wrapper, stamped_home, workspace_path};
use support::hygiene::load_schema;
use support::watch::{WITHIN, Watch};

const CANARY: &str = "canary-chain-value-5c1e";
const PATH4: &str = "fixtures/fake-scripts/path4.json";

fn fixtures_arg() -> String {
    workspace_path("fixtures/claude")
        .to_str()
        .expect("utf-8 path")
        .to_owned()
}

/// A relayed 2.1.287 fixture, as the fake agent replays it.
fn relayed(name: &str) -> Value {
    let path = workspace_path("fixtures/claude")
        .join(fake::RECORDED_CLI_VERSION)
        .join(name);
    serde_json::from_slice(&std::fs::read(path).expect("relayed fixture")).expect("json")
}

/// A wrapper over the Path 4 script, its SessionStart record landed.
fn boot(stamped: StampedHome) -> Wrapper {
    let fx = fixtures_arg();
    let wrapper = Wrapper::boot(stamped, "builder", Some(PATH4), &["--fixtures", &fx]);
    wait_events(&wrapper.instance_dir(), "the session-start record", |l| {
        l.iter().any(|e| e["kind"] == "session-start")
    });
    wrapper
}

fn events(instance_dir: &Path) -> Vec<Value> {
    support::ndjson::read_lines(&instance_dir.join("events.ndjson"))
}

fn wait_events(instance_dir: &Path, what: &str, pred: impl Fn(&[Value]) -> bool) -> Vec<Value> {
    let watch = Watch::start("events");
    let deadline = Instant::now() + WITHIN;
    loop {
        let lines = events(instance_dir);
        if pred(&lines) {
            return lines;
        }
        watch.note(&format!("events {}", lines.len()));
        watch.deadline_check(deadline, &format!("timed out waiting for {what}"));
        std::thread::yield_now();
    }
}

fn of_event_kind<'a>(lines: &'a [Value], kind: &str) -> Vec<&'a Value> {
    lines.iter().filter(|l| l["kind"] == kind).collect()
}

/// The offset after each complete `events.ndjson` line.
fn line_end_of(instance_dir: &Path, dialog_id: u64) -> u64 {
    let bytes = std::fs::read(instance_dir.join("events.ndjson")).expect("events");
    let mut end = 0u64;
    for line in bytes.split_inclusive(|b| *b == b'\n') {
        end += u64::try_from(line.len()).expect("len");
        let value: Value = serde_json::from_slice(line).expect("one JSON object");
        if value["data"]["dialog_id"] == dialog_id {
            return end;
        }
    }
    panic!("no line for dialog {dialog_id}");
}

/// The fake agent's hook receipts once at least `n` landed.
fn hooks_at_least(receipt: &Path, n: usize) -> Vec<Value> {
    let lines = fake::wait_for(receipt, "the hook receipts", |l| {
        of_kind(l, "hook").len() >= n
    });
    of_kind(&lines, "hook").into_iter().cloned().collect()
}

fn stdout_of(hook: &Value) -> String {
    let hex = hook["stdout_hex"].as_str().expect("stdout_hex");
    String::from_utf8(fake::unhex(hex)).expect("utf-8 stdout")
}

/// A hook run that exited 0 with nothing on stderr; its stdout.
fn clean_stdout(hook: &Value) -> String {
    assert_eq!(hook["ran"], true, "{hook}");
    assert_eq!(hook["exit_code"], 0, "{hook}");
    assert_eq!(hook["stderr_len"], 0, "{hook}");
    stdout_of(hook)
}

/// A `viola` the test started; killed on drop if it is still running.
struct Running(Option<Child>);

struct Ran {
    code: Option<i32>,
    stdout: String,
    stderr: String,
}

impl Running {
    fn finish(mut self) -> Ran {
        let watch = Watch::start("viola");
        let deadline = Instant::now() + WITHIN;
        let child = self.0.as_mut().expect("child");
        while child.try_wait().expect("try_wait").is_none() {
            watch.note("running");
            watch.deadline_check(deadline, "viola never exited");
            std::thread::yield_now();
        }
        let out = self
            .0
            .take()
            .expect("child")
            .wait_with_output()
            .expect("output");
        Ran {
            code: out.status.code(),
            stdout: String::from_utf8(out.stdout).expect("utf-8 stdout"),
            stderr: String::from_utf8(out.stderr).expect("utf-8 stderr"),
        }
    }
}

impl Drop for Running {
    fn drop(&mut self) {
        if let Some(mut child) = self.0.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

fn spawn(home: &Path, args: &[&str], stdin: Option<&str>, from: Option<&str>) -> Running {
    let mut command = Command::new(VIOLA);
    command
        .arg("--home")
        .arg(home)
        .args(args)
        .env_remove("VIOLA_NAME")
        .stdin(if stdin.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(from) = from {
        command.env("VIOLA_NAME", from);
    }
    let mut child = command.spawn().expect("viola");
    if let Some(text) = stdin {
        let mut pipe = child.stdin.take().expect("stdin");
        pipe.write_all(text.as_bytes()).expect("stdin");
    }
    Running(Some(child))
}

/// `viola answer builder <id>` with `response` on stdin, the sender `overseer`.
fn answer(home: &Path, dialog_id: u64, response: &Value) -> Ran {
    let id = dialog_id.to_string();
    spawn(
        home,
        &["answer", "builder", &id],
        Some(&response.to_string()),
        Some("overseer"),
    )
    .finish()
}

fn diagnostics(home: &Path, file: &str) -> Vec<Value> {
    support::ndjson::read_lines(&home.join("diagnostics").join(file))
}

/// Waits until the wrapper logged `count` `channel-request`s for `method`.
fn wait_request_logged(home: &Path, method: &str, count: usize) {
    let watch = Watch::start("request");
    let deadline = Instant::now() + WITHIN;
    loop {
        let n = diagnostics(home, "run-builder.ndjson")
            .iter()
            .filter(|l| l["event"] == "channel-request" && l["method"] == method)
            .count();
        if n >= count {
            return;
        }
        watch.note(&format!("{method} requests {n}"));
        watch.deadline_check(
            deadline,
            &format!("no {method} request reached the wrapper"),
        );
        std::thread::yield_now();
    }
}

/// No home-level diagnostics line carries the canary, and every role line passes the diag-line
/// schema (obs-plan §9 G4).
fn assert_logs_clean(home: &Path) {
    let schema = load_schema(&workspace_path("schemas/diag-line.v1.json"));
    let validator = jsonschema::validator_for(&schema).expect("valid schema");
    for entry in std::fs::read_dir(home.join("diagnostics")).expect("diagnostics") {
        let path = entry.expect("entry").path();
        let text = std::fs::read_to_string(&path).expect("role file");
        assert!(!text.contains(CANARY), "{path:?} holds content");
        let file = path
            .file_name()
            .expect("name")
            .to_string_lossy()
            .into_owned();
        for (n, line) in diagnostics(home, &file).iter().enumerate() {
            assert!(validator.is_valid(line), "{file} line {n} fails the schema");
        }
    }
}

/// Raises the next gated step and waits for its dialog line: its `dialog_id`.
fn raise(wrapper: &Wrapper, kind: &str, nth: usize) -> u64 {
    wrapper.release();
    let lines = wait_events(&wrapper.instance_dir(), "the dialog line", |l| {
        of_event_kind(l, kind).len() >= nth
    });
    of_event_kind(&lines, kind)[nth - 1]["data"]["dialog_id"]
        .as_u64()
        .expect("a dialog_id")
}

/// test-plan §6 Path 4: question (S3 + S8 annotations), the question's PermissionRequest repeat,
/// plan revise via the PermissionRequest repeat (S7), plan approve via PreToolUse, unknown dialog;
/// the `v1-30` dialog-kind wait witness for `question` and `plan`.
#[rstest]
fn path4_dialogs_are_logged_once_woken_and_answered_by_id(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home);
    let home = wrapper.home().to_path_buf();
    let dir = wrapper.instance_dir();
    let receipt = wrapper.receipt();

    // A parked `wait` wakes on the question, its line uncoloured.
    let parked = spawn(&home, &["wait", "builder"], None, None);
    wait_request_logged(&home, "wait", 1);
    let question_id = raise(&wrapper, "question", 1);
    let woken = parked.finish();
    assert_eq!(woken.code, Some(0), "stderr: {}", woken.stderr);
    let end = line_end_of(&dir, question_id);
    assert_eq!(
        woken.stdout,
        format!("question  builder  dialog {question_id}  cursor {end}\n")
    );
    assert!(!woken.stdout.contains('\x1b'));

    // A `wait` without `--after` while the dialog is pending returns it at once.
    let pending = spawn(&home, &["wait", "builder", "--json"], None, None).finish();
    assert_eq!(pending.code, Some(0), "stderr: {}", pending.stderr);
    let doc: Value = serde_json::from_str(&pending.stdout).expect("one JSON document");
    assert_eq!(doc["ok"]["event"]["kind"], "question");
    assert_eq!(doc["ok"]["event"]["data"]["dialog_id"], question_id);
    assert_eq!(doc["ok"]["cursor"], end);

    // The question is answered with free text and a note (S3, S8).
    let tool_input = relayed("PreToolUse.ask-user-question.json")["tool_input"].clone();
    let asked = tool_input["questions"][0]["question"]
        .as_str()
        .expect("a question")
        .to_owned();
    let answers = json!({&asked: format!("{CANARY} a triangle, rounded")});
    let annotations = json!({&asked: {"notes": format!("{CANARY} keep it small")}});
    let reply = answer(
        &home,
        question_id,
        &json!({"answers": answers, "annotations": annotations}),
    );
    assert_eq!(reply.code, Some(0), "stderr: {}", reply.stderr);
    assert_eq!(
        reply.stdout,
        format!("answered  builder  dialog {question_id}\n")
    );
    assert!(reply.stderr.is_empty());
    let hooks = hooks_at_least(&receipt, 2);
    assert_eq!(hooks[1]["event"], "PreToolUse");
    let body: Value = serde_json::from_str(&clean_stdout(&hooks[1])).expect("one JSON body");
    let mut updated = tool_input.clone();
    updated["answers"] = answers;
    updated["annotations"] = annotations;
    assert_eq!(
        body,
        json!({"hookSpecificOutput": {"hookEventName": "PreToolUse",
            "permissionDecision": "allow", "updatedInput": updated}})
    );

    // Its PermissionRequest repeat is the same dialog: nothing printed, nothing logged.
    wrapper.release();
    let hooks = hooks_at_least(&receipt, 3);
    assert_eq!(hooks[2]["event"], "PermissionRequest");
    assert_eq!(clean_stdout(&hooks[2]), "");
    assert_eq!(of_event_kind(&events(&dir), "question").len(), 1);

    // A plan answered `revise`: no PreToolUse decision; the repeat carries `deny` + `message` (S7).
    let parked = spawn(&home, &["wait", "builder"], None, None);
    wait_request_logged(&home, "wait", 3);
    let revised_id = raise(&wrapper, "plan", 1);
    let woken = parked.finish();
    let end = line_end_of(&dir, revised_id);
    assert_eq!(
        woken.stdout,
        format!("plan  builder  dialog {revised_id}  cursor {end}\n")
    );
    let message = format!("{CANARY} rename the file first");
    let reply = answer(
        &home,
        revised_id,
        &json!({"behavior": "revise", "message": message}),
    );
    assert_eq!(reply.code, Some(0), "stderr: {}", reply.stderr);
    let hooks = hooks_at_least(&receipt, 4);
    assert_eq!(hooks[3]["event"], "PreToolUse");
    assert_eq!(clean_stdout(&hooks[3]), "");
    wrapper.release();
    let hooks = hooks_at_least(&receipt, 5);
    assert_eq!(hooks[4]["event"], "PermissionRequest");
    assert_eq!(
        clean_stdout(&hooks[4]),
        format!(
            "{{\"hookSpecificOutput\":{{\"hookEventName\":\"PermissionRequest\",\
             \"decision\":{{\"behavior\":\"deny\",\"message\":\"{message}\"}}}}}}"
        )
    );
    assert_eq!(of_event_kind(&events(&dir), "plan").len(), 1);

    // A plan approved through PreToolUse: `allow` alone.
    let approved_id = raise(&wrapper, "plan", 2);
    let reply = answer(&home, approved_id, &json!({"behavior": "approve"}));
    assert_eq!(reply.code, Some(0), "stderr: {}", reply.stderr);
    let hooks = hooks_at_least(&receipt, 6);
    assert_eq!(hooks[5]["event"], "PreToolUse");
    assert_eq!(
        clean_stdout(&hooks[5]),
        "{\"hookSpecificOutput\":{\"hookEventName\":\"PreToolUse\",\"permissionDecision\":\"allow\"}}"
    );

    // Ids rise; an id that is not pending is refused.
    assert!(question_id < revised_id && revised_id < approved_id);
    let unknown = spawn(
        &home,
        &["answer", "builder", "999999", "--json"],
        Some("{\"behavior\": \"allow\"}"),
        None,
    )
    .finish();
    assert_eq!(unknown.code, Some(13));
    assert_eq!(
        unknown.stdout,
        "{\"v\":1,\"refusal\":\"not-delivered\",\"detail\":\"unknown-dialog\"}\n"
    );
    assert!(unknown.stderr.is_empty());

    // Each dialog event once, from the hook, its data the dialog's (never the raw payload).
    let lines = events(&dir);
    let dialogs: Vec<&Value> = lines
        .iter()
        .filter(|l| matches!(l["kind"].as_str(), Some("question" | "permission" | "plan")))
        .collect();
    assert_eq!(dialogs.len(), 3);
    for line in &dialogs {
        assert_eq!(line["source"], "hook");
        assert!(line["data"].get("tool_input").is_none());
        assert!(line["data"].get("session_id").is_none());
    }
    assert!(!lines.iter().any(|l| l["kind"] == "activity"));

    // The logs: one `dialog-raised` per dialog, one `dialog-answered` per answer, the hook's pair
    // joined on the same `corr`.
    let run = diagnostics(&home, "run-builder.ndjson");
    let raised: Vec<&Value> = run
        .iter()
        .filter(|l| l["event"] == "dialog-raised")
        .collect();
    let ids: Vec<u64> = raised.iter().filter_map(|l| l["corr"].as_u64()).collect();
    assert_eq!(ids, [question_id, revised_id, approved_id]);
    let answered: Vec<&Value> = run
        .iter()
        .filter(|l| l["event"] == "dialog-answered")
        .collect();
    assert_eq!(answered.len(), 3);
    for line in &answered {
        assert_eq!(line["from"], "overseer");
        assert_eq!(line["from_trust"], "self-reported");
    }
    let hook_lines = diagnostics(&home, "hook-builder.ndjson");
    for id in [question_id, revised_id, approved_id] {
        for event in ["hook-invoked", "hook-decision"] {
            assert!(
                hook_lines
                    .iter()
                    .any(|l| l["event"] == event && l["corr"] == id),
                "no {event} for dialog {id}"
            );
        }
    }
    let emitted: Vec<&Value> = hook_lines
        .iter()
        .filter(|l| l["event"] == "hook-decision" && l["decision_emitted"] == true)
        .collect();
    assert_eq!(emitted.len(), 3, "question, the revise repeat, approve");
    assert_logs_clean(&home);
    wrapper.stop();
}

/// The hook's own environment, as `viola run` set it in the child.
fn hook_env(wrapper: &Wrapper) -> [(&'static str, PathBuf); 1] {
    [("VIOLA_DIR", wrapper.instance_dir())]
}

/// `viola hook <event>` run by the test, as the child would run it, with `payload` on stdin.
fn run_hook(wrapper: &Wrapper, event: &str, payload: &Value) -> Ran {
    let mut command = Command::new(VIOLA);
    command
        .args(["hook", event])
        .env("VIOLA_NAME", "builder")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for (key, value) in hook_env(wrapper) {
        command.env(key, value);
    }
    let mut child = command.spawn().expect("viola hook");
    let mut pipe = child.stdin.take().expect("stdin");
    pipe.write_all(payload.to_string().as_bytes())
        .expect("stdin");
    drop(pipe);
    Running(Some(child)).finish()
}

/// While one dialog is held, a second is logged with its own id and left to the human at once:
/// its hook prints nothing.
#[rstest]
fn path4_second_concurrent_dialog_is_left_to_the_human(stamped_home: StampedHome) {
    let wrapper = boot(stamped_home);
    let home = wrapper.home().to_path_buf();
    let first = raise(&wrapper, "question", 1);
    let second = run_hook(
        &wrapper,
        "pre-tool-use",
        &relayed("PreToolUse.exit-plan-mode.json"),
    );
    assert_eq!(second.code, Some(0));
    assert!(second.stdout.is_empty());
    assert!(second.stderr.is_empty());
    let lines = events(&wrapper.instance_dir());
    let plan = of_event_kind(&lines, "plan");
    assert_eq!(plan.len(), 1);
    assert_eq!(plan[0]["data"]["dialog_id"], first + 1);
    let late = spawn(
        &home,
        &["answer", "builder", &(first + 1).to_string(), "--json"],
        Some("{\"behavior\": \"approve\"}"),
        None,
    )
    .finish();
    assert_eq!(late.code, Some(13));
    // The held one still takes its answer.
    let tool_input = relayed("PreToolUse.ask-user-question.json")["tool_input"].clone();
    let asked = tool_input["questions"][0]["question"].as_str().expect("q");
    let reply = answer(&home, first, &json!({"answers": {asked: CANARY}}));
    assert_eq!(reply.code, Some(0), "stderr: {}", reply.stderr);
    let hooks = hooks_at_least(&wrapper.receipt(), 2);
    assert!(!clean_stdout(&hooks[1]).is_empty());
    assert_logs_clean(&home);
    wrapper.stop();
}

/// test-plan §6 Path 7 on the dialog tier: an unverified CLI logs the dialog, the hook prints
/// nothing within the spine deadline, and `answer` is refused `unverified-cli` (exit 12). No body
/// is ever emitted without a stamp.
#[test]
fn path4_unstamped_dialogs_are_left_to_the_human_and_answers_refused() {
    let wrapper = boot(StampedHome::unstamped(TestHome::new()));
    let home = wrapper.home().to_path_buf();
    let receipt = wrapper.receipt();
    let started = Instant::now();
    let id = raise(&wrapper, "question", 1);
    let hooks = hooks_at_least(&receipt, 2);
    assert!(started.elapsed() < WITHIN);
    assert_eq!(clean_stdout(&hooks[1]), "");
    let refused = spawn(
        &home,
        &["answer", "builder", &id.to_string(), "--json"],
        Some(&json!({"answers": {"q": CANARY}}).to_string()),
        None,
    )
    .finish();
    assert_eq!(refused.code, Some(12));
    assert_eq!(
        refused.stdout,
        "{\"v\":1,\"refusal\":\"unverified-cli\",\"detail\":null}\n"
    );
    let human = spawn(
        &home,
        &["answer", "builder", &id.to_string()],
        Some("{\"behavior\": \"allow\"}"),
        None,
    )
    .finish();
    assert_eq!(human.code, Some(12));
    assert_eq!(
        human.stderr,
        "unable  builder  unverified-cli\nhint: run viola verify for this CLI version\n"
    );
    for _ in 0..4 {
        wrapper.release();
    }
    let hooks = hooks_at_least(&receipt, 6);
    for hook in &hooks {
        assert_eq!(stdout_of(hook), "", "a body without a stamp: {hook}");
    }
    assert_logs_clean(&home);
    wrapper.stop();
}

/// The client checks the free text before any frame: ESC in a message is refused at exit 13, its
/// hint the last stderr line, and no `answer` request reaches the wrapper.
#[test]
fn answer_control_character_is_refused_before_any_frame() {
    let wrapper = boot(StampedHome::unstamped(TestHome::new()));
    let home = wrapper.home().to_path_buf();
    let ran = spawn(
        &home,
        &["answer", "builder", "1"],
        Some(&json!({"behavior": "deny", "message": format!("{CANARY}\u{1b}[2J")}).to_string()),
        None,
    )
    .finish();
    assert_eq!(ran.code, Some(13));
    assert!(ran.stdout.is_empty());
    assert_eq!(
        ran.stderr,
        "unable  builder  not-delivered  control-character\n\
         hint: the text contains a control character (only LF, CR, TAB are allowed)\n"
    );
    assert!(
        !diagnostics(&home, "run-builder.ndjson")
            .iter()
            .any(|l| l["event"] == "channel-request" && l["method"] == "answer")
    );
    wrapper.stop();
}

/// A response that is not one of the three shapes is a usage error, before anything is sent.
#[test]
fn answer_unusable_response_is_exit_2() {
    let home = TestHome::new();
    let ran = spawn(
        home.path(),
        &["answer", "builder", "1"],
        Some("approve"),
        None,
    )
    .finish();
    assert_eq!(ran.code, Some(2));
    assert!(ran.stdout.is_empty());
    assert_eq!(
        ran.stderr,
        "error: the response must be one JSON object of an answer's shape, at most 16 MiB of UTF-8\n"
    );
}
