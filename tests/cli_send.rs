//! Critical Path 2 over the CLI and the fake agent's receipt (test-plan §6 Path 2; architecture
//! [Delivery Confirmation]): `viola send` types the text as one bracketed paste, the wrapper reads
//! it back through the matching `prompt-submitted`, relabelled `driver`, and every outcome is a CL-1
//! record. A send is never presumed delivered: a mute agent, a local command and a second send in
//! flight are each `not-delivered`. The `send_window_` cases wait out the product's 10 s
//! confirmation window by design.

#[allow(dead_code)]
mod support;

use std::io::Write as _;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::time::Instant;

use serde_json::{Value, json};
use support::fake::{self, of_kind};
use support::home::{StampedHome, TestHome, VIOLA, Wrapper, workspace_path};
use support::watch::{WITHIN, Watch};

const CANARY: &str = "canary-chain-value-5c1e";
/// A gated `PostToolUse`, then a gated `Stop`: the turn a confirmed send started, ended on cue.
const PATH3: &str = "fixtures/fake-scripts/path3.json";

/// A wrapper over the fake agent with the committed hook fixtures (and `script`), its SessionStart
/// record landed.
fn boot(script: Option<&str>, extra: &[&str]) -> Wrapper {
    let fixtures = workspace_path("fixtures/claude");
    let mut args = vec!["--fixtures", fixtures.to_str().expect("utf-8 path")];
    args.extend_from_slice(extra);
    let wrapper = Wrapper::boot(
        StampedHome::unstamped(TestHome::new()),
        "builder",
        script,
        &args,
    );
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

/// `events.ndjson`'s length: the cursor the next send is issued at.
fn end_offset(instance_dir: &Path) -> u64 {
    std::fs::metadata(instance_dir.join("events.ndjson")).map_or(0, |m| m.len())
}

/// The complete records from byte `from` on.
fn records_after(instance_dir: &Path, from: u64) -> Vec<Value> {
    let bytes = std::fs::read(instance_dir.join("events.ndjson")).expect("events");
    let from = usize::try_from(from).expect("offset");
    support::ndjson::complete_lines(&bytes[from..])
}

/// `viola --home <home> send <args…>` with its stdin piped and nothing written yet.
fn start_send(home: &Path, args: &[&str]) -> Child {
    Command::new(VIOLA)
        .arg("--home")
        .arg(home)
        .arg("send")
        .args(args)
        .env_remove("VIOLA_NAME")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("viola send")
}

/// Writes `text` to the child's stdin and closes it.
fn feed(child: &mut Child, text: &str) -> std::io::Result<()> {
    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(text.as_bytes())
}

/// `viola --home <home> send <args…>` with `text` on stdin, started, not waited. A child that
/// exits before reading (a usage error) closes the pipe first, so `BrokenPipe` alone is tolerated:
/// the caller's assertions on the exit still decide.
fn spawn_send(home: &Path, args: &[&str], text: &str) -> Child {
    let mut child = start_send(home, args);
    match feed(&mut child, text) {
        Err(e) if e.kind() != std::io::ErrorKind::BrokenPipe => panic!("stdin: {e}"),
        _ => {}
    }
    child
}

struct Sent {
    code: Option<i32>,
    stdout: String,
    stderr: String,
}

fn finish(child: Child) -> Sent {
    let out = child.wait_with_output().expect("viola send exits");
    Sent {
        code: out.status.code(),
        stdout: String::from_utf8(out.stdout).expect("utf-8 stdout"),
        stderr: String::from_utf8(out.stderr).expect("utf-8 stderr"),
    }
}

fn send(home: &Path, args: &[&str], text: &str) -> Sent {
    finish(spawn_send(home, args, text))
}

/// The receipt's prompts once at least `n` landed: the agent receipts a prompt after its hook
/// returns, which can be after the send's reply.
fn prompts_at_least(receipt: &Path, n: usize) -> Vec<Value> {
    fake::wait_for(receipt, "the typed prompts", |l| {
        of_kind(l, "prompt").len() >= n
    });
    prompts(receipt)
}

fn prompts(receipt: &Path) -> Vec<Value> {
    of_kind(&fake::receipt(receipt), "prompt")
        .into_iter()
        .cloned()
        .collect()
}

fn is_ms_utc(ts: &str) -> bool {
    let bytes = ts.as_bytes();
    ts.len() == 24
        && bytes[10] == b'T'
        && ts.ends_with('Z')
        && bytes[19] == b'.'
        && chrono::DateTime::parse_from_rfc3339(ts).is_ok()
}

/// Releases `PATH3`'s two gated steps and waits for its `turn-ended`.
fn end_turn(wrapper: &Wrapper) {
    let ended = events(&wrapper.instance_dir())
        .iter()
        .filter(|e| e["kind"] == "turn-ended")
        .count();
    wrapper.release();
    wrapper.release();
    wait_events(
        &wrapper.instance_dir(),
        "the scripted Stop's turn-ended",
        |l| l.iter().filter(|e| e["kind"] == "turn-ended").count() > ended,
    );
}

#[test]
fn path2_send_confirms_with_cl1_events() {
    let wrapper = boot(Some(PATH3), &[]);
    let dir = wrapper.instance_dir();
    let home = wrapper.home().to_path_buf();
    let text = format!("{CANARY} step four\nline two\n\tindented");
    let l = end_offset(&dir);

    let sent = send(&home, &["builder", "--json"], &text);
    assert_eq!(sent.code, Some(0), "stderr: {}", sent.stderr);
    assert!(sent.stderr.is_empty(), "--json writes nothing on stderr");
    assert_eq!(sent.stdout.lines().count(), 1, "one JSON document");
    let doc: Value = serde_json::from_str(&sent.stdout).expect("one JSON document");
    assert_eq!(doc["v"], 1);
    assert_eq!(doc["ok"]["cursor"], l);
    let submitted_at = doc["ok"]["submitted_at"].as_str().expect("submitted_at");
    assert!(is_ms_utc(submitted_at), "{submitted_at}");

    let records = records_after(&dir, l);
    let kinds: Vec<&Value> = records.iter().map(|r| &r["kind"]).collect();
    assert_eq!(kinds, ["send-issued", "prompt-submitted", "send-confirmed"]);
    assert_eq!(records[0]["data"], json!({"cursor": l}));
    assert_eq!(records[0]["source"], "wrapper");
    assert_eq!(
        records[1]["data"],
        json!({"text": text, "origin": "driver"})
    );
    assert_eq!(records[1]["ts"], submitted_at);
    assert_eq!(records[2]["data"], json!({"cursor": l}));

    let typed = prompts_at_least(&wrapper.receipt(), 1);
    assert_eq!(typed.len(), 1, "exactly one prompt typed");
    assert_eq!(typed[0]["bare_esc"], false);
    assert_eq!(typed[0]["text"], text.as_str());
    assert_eq!(typed[0]["submit"], "fired");

    let role = support::ndjson::read_lines(&home.join("diagnostics").join("run-builder.ndjson"));
    let line = |event: &str| -> Value {
        let found: Vec<&Value> = role.iter().filter(|r| r["event"] == event).collect();
        assert_eq!(found.len(), 1, "one {event} line");
        found[0].clone()
    };
    let (issued, confirmed) = (line("send-issued"), line("send-confirmed"));
    assert_eq!(issued["corr"], l);
    assert_eq!(confirmed["corr"], l);
    assert_eq!(confirmed["confirmed"], true);
    assert_eq!(issued["rpc_id"], confirmed["rpc_id"]);
    assert!(issued["rpc_id"].is_u64());
    assert!(
        issued["conn"]
            .as_str()
            .is_some_and(|c| c.starts_with("cli-"))
    );
    assert_eq!(issued["conn"], confirmed["conn"]);
    assert_eq!(issued["text_bytes"], text.len());
    assert!(issued.get("from").is_none());
    for file in ["run-builder.ndjson", "cli-builder.ndjson"] {
        let bytes = std::fs::read(home.join("diagnostics").join(file)).expect("role file");
        assert!(
            !String::from_utf8_lossy(&bytes).contains(CANARY),
            "{file} holds the sent text"
        );
    }

    end_turn(&wrapper);
    // Human mode with stdout piped: the read-back line alone, no issue line.
    let l2 = end_offset(&dir);
    let human = send(&home, &["builder"], &format!("{CANARY} again"));
    assert_eq!(human.code, Some(0), "stderr: {}", human.stderr);
    assert!(
        human.stderr.is_empty(),
        "no issue line when stdout is piped"
    );
    let lines: Vec<&str> = human.stdout.lines().collect();
    assert_eq!(lines.len(), 1);
    let prefix = "[RB] read back      builder  ";
    assert!(lines[0].starts_with(prefix), "{}", lines[0]);
    assert!(
        lines[0].ends_with(&format!("  cursor {l2}")),
        "{}",
        lines[0]
    );
    let time = &lines[0][prefix.len()..prefix.len() + 13];
    assert!(time.ends_with('Z') && time.as_bytes()[8] == b'.', "{time}");
    wrapper.stop();
}

#[test]
fn send_window_no_prompt_submitted_refuses_and_logs() {
    let wrapper = boot(None, &["--suppress-prompt-submit"]);
    let dir = wrapper.instance_dir();
    let l = end_offset(&dir);
    let sent = send(wrapper.home(), &["builder"], &format!("{CANARY} unheard"));
    assert_eq!(sent.code, Some(13));
    assert!(sent.stdout.is_empty());
    assert_eq!(
        sent.stderr,
        "[/ ] unable         builder  not-delivered  no-prompt-submitted\n\
         hint: builder did not submit the prompt; check it, then send again\n"
    );
    let records = records_after(&dir, l);
    let kinds: Vec<&Value> = records.iter().map(|r| &r["kind"]).collect();
    assert_eq!(kinds, ["send-issued", "send-refused"]);
    assert_eq!(
        records[1]["data"],
        json!({"refusal": "not-delivered", "detail": "no-prompt-submitted", "cursor": l})
    );
    let typed = prompts(&wrapper.receipt());
    assert_eq!(typed.len(), 1);
    assert_eq!(typed[0]["submit"], "suppressed");
    wrapper.stop();
}

/// verification-matrix v1-12: two concurrent sends to one instance, exactly one typed.
#[test]
fn send_window_second_concurrent_send_is_refused_turn_running() {
    let wrapper = boot(None, &["--suppress-prompt-submit"]);
    let dir = wrapper.instance_dir();
    let l = end_offset(&dir);
    let first = spawn_send(wrapper.home(), &["builder"], &format!("{CANARY} first"));
    wait_events(&dir, "the first send-issued", |lines| {
        lines.iter().any(|e| e["kind"] == "send-issued")
    });

    let second = send(wrapper.home(), &["builder"], &format!("{CANARY} second"));
    assert_eq!(second.code, Some(13));
    assert_eq!(
        second.stderr,
        "[/ ] unable         builder  not-delivered  turn-running\n\
         hint: a turn is running; viola wait builder first\n"
    );
    let first = finish(first);
    assert_eq!(first.code, Some(13));
    assert!(
        first
            .stderr
            .starts_with("[/ ] unable         builder  not-delivered  no-prompt-submitted\n"),
        "{}",
        first.stderr
    );

    let records = records_after(&dir, l);
    let kinds: Vec<&Value> = records.iter().map(|r| &r["kind"]).collect();
    assert_eq!(kinds, ["send-issued", "send-refused", "send-refused"]);
    assert_eq!(records[0]["data"]["cursor"], l);
    assert_eq!(
        records[1]["data"],
        json!({"refusal": "not-delivered", "detail": "turn-running"})
    );
    assert_eq!(
        records[2]["data"],
        json!({"refusal": "not-delivered", "detail": "no-prompt-submitted", "cursor": l})
    );
    let typed = prompts(&wrapper.receipt());
    assert_eq!(typed.len(), 1, "exactly one prompt typed");
    assert_eq!(typed[0]["text"], format!("{CANARY} first"));
    wrapper.stop();
}

/// The driver's own confirmed prompt starts a turn: a next `send` is `turn-running` and types
/// nothing until the turn's `turn-ended`, after which the same `send` is accepted.
#[test]
fn send_after_a_confirmed_send_is_turn_running_until_turn_ended() {
    let wrapper = boot(Some(PATH3), &[]);
    let dir = wrapper.instance_dir();
    let home = wrapper.home().to_path_buf();
    let first = send(&home, &["builder", "--json"], &format!("{CANARY} first"));
    assert_eq!(first.code, Some(0), "stderr: {}", first.stderr);

    let l = end_offset(&dir);
    let second = format!("{CANARY} second");
    let refused = send(&home, &["builder"], &second);
    assert_eq!(refused.code, Some(13));
    assert!(refused.stdout.is_empty());
    assert_eq!(
        refused.stderr,
        "[/ ] unable         builder  not-delivered  turn-running\n\
         hint: a turn is running; viola wait builder first\n"
    );
    let records = records_after(&dir, l);
    assert_eq!(records.len(), 1, "{records:?}");
    assert_eq!(records[0]["kind"], "send-refused");
    assert_eq!(
        records[0]["data"],
        json!({"refusal": "not-delivered", "detail": "turn-running"})
    );
    assert_eq!(prompts(&wrapper.receipt()).len(), 1, "nothing typed");

    end_turn(&wrapper);
    let accepted = send(&home, &["builder"], &second);
    assert_eq!(accepted.code, Some(0), "stderr: {}", accepted.stderr);
    let typed = prompts_at_least(&wrapper.receipt(), 2);
    assert_eq!(typed.len(), 2);
    assert_eq!(typed[1]["text"], second.as_str());
    wrapper.stop();
}

/// No local-command list is compiled: a local command is never presumed delivered.
#[test]
fn send_window_local_command_is_not_presumed_delivered() {
    let wrapper = boot(None, &["--local-command-mode"]);
    let sent = send(wrapper.home(), &["builder", "--json"], "/clear");
    assert_eq!(sent.code, Some(13));
    assert_eq!(
        serde_json::from_str::<Value>(&sent.stdout).expect("one JSON document"),
        json!({"v": 1, "refusal": "not-delivered", "detail": "no-prompt-submitted"})
    );
    assert!(sent.stderr.is_empty());
    let typed = prompts(&wrapper.receipt());
    assert_eq!(typed.len(), 1);
    assert_eq!(typed[0]["submit"], "local-command");
    wrapper.stop();
}

#[test]
fn send_leading_slash_argument_is_a_usage_error() {
    let wrapper = boot(None, &[]);
    let dir = wrapper.instance_dir();
    let l = end_offset(&dir);
    for args in [&["/clear"][..], &["builder", "/clear"][..]] {
        let sent = send(wrapper.home(), args, CANARY);
        assert_eq!(sent.code, Some(2), "{args:?}");
        assert!(sent.stdout.is_empty());
    }
    assert_eq!(end_offset(&dir), l, "events.ndjson gained bytes");
    assert!(prompts(&wrapper.receipt()).is_empty(), "a prompt was typed");
    wrapper.stop();
}

/// The window ci#37196414168 lost on ubuntu, forced open: the child has already exited on the usage
/// error when its stdin is written, so the write always meets a closed pipe.
#[test]
fn send_leading_slash_argument_exits_before_its_stdin_is_written() {
    let home = TestHome::new();
    let mut child = start_send(home.path(), &["builder", "/clear"]);
    let watch = Watch::start("exit");
    let deadline = Instant::now() + WITHIN;
    while child.try_wait().expect("try_wait").is_none() {
        watch.deadline_check(deadline, "viola send never exited");
        std::thread::yield_now();
    }
    let write = feed(&mut child, CANARY);
    assert_eq!(
        write.map_err(|e| e.kind()),
        Err(std::io::ErrorKind::BrokenPipe)
    );
    let sent = finish(child);
    assert_eq!(sent.code, Some(2));
    assert!(sent.stdout.is_empty());
}

#[test]
fn send_unreachable_name_exits_21_not_running() {
    let home = TestHome::new();
    let human = send(home.path(), &["builder"], CANARY);
    assert_eq!(human.code, Some(21));
    assert!(human.stdout.is_empty());
    assert_eq!(
        human.stderr,
        "[/ ] unable         builder  instance-unreachable\n\
         hint: builder is not running; viola list shows the live instances\n"
    );
    let json = send(home.path(), &["builder", "--json"], CANARY);
    assert_eq!(json.code, Some(21));
    assert!(json.stderr.is_empty());
    assert_eq!(
        json.stdout,
        "{\"v\":1,\"error\":\"instance-unreachable\",\"detail\":null}\n"
    );
    let role =
        support::ndjson::read_lines(&home.path().join("diagnostics").join("cli-builder.ndjson"));
    let exits: Vec<&Value> = role
        .iter()
        .filter(|l| l["event"] == "process-exit")
        .collect();
    assert_eq!(exits.len(), 2);
    assert!(exits.iter().all(|e| e["exit_code"] == 21
        && e["detail"] == "instance-dead"
        && e["during"] == "connect"));
}
