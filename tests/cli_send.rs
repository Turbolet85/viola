//! Critical Path 2 over the CLI and the fake agent's receipt (test-plan §6 Path 2; architecture
//! [Delivery Confirmation]): `viola send` types the text as one bracketed paste, the wrapper reads
//! it back through the matching `prompt-submitted`, relabelled `driver`, and every outcome is a CL-1
//! record. A send is never presumed delivered: a mute agent, a slash text off the compiled list and
//! a second send in flight are each `not-delivered`. A listed local command is confirmed by its
//! measured post-condition or answered `unconfirmable`. The `send_window_` cases wait out the
//! product's 10 s confirmation window by design.

#[allow(dead_code)]
mod support;

use std::io::Write as _;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::time::Instant;

use rstest::rstest;
use serde_json::{Value, json};
use support::fake::{self, of_kind};
use support::home::{StampedHome, TestHome, VIOLA, Wrapper, stamped_home, workspace_path};
use support::watch::{WITHIN, Watch};

const CANARY: &str = "canary-chain-value-5c1e";
/// A gated `PostToolUse`, then a gated `Stop`: the turn a confirmed send started, ended on cue.
const PATH3: &str = "fixtures/fake-scripts/path3.json";

/// A wrapper over the fake agent with the committed hook fixtures (and `script`), its SessionStart
/// record landed.
fn boot(script: Option<&str>, extra: &[&str]) -> Wrapper {
    boot_on(StampedHome::unstamped(TestHome::new()), script, extra)
}

/// `boot` on `stamped`: a home `viola verify` stamped makes the fake agent's version a verified CLI.
fn boot_on(stamped: StampedHome, script: Option<&str>, extra: &[&str]) -> Wrapper {
    let fixtures = workspace_path("fixtures/claude");
    let mut args = vec!["--fixtures", fixtures.to_str().expect("utf-8 path")];
    args.extend_from_slice(extra);
    let wrapper = Wrapper::boot(stamped, "builder", script, &args);
    wait_events(&wrapper.instance_dir(), "the session-start record", |l| {
        l.iter().any(|e| e["kind"] == "session-start")
    });
    wrapper
}

/// The recorded `paste-1` variant's bytes, and the long text it wraps. The frame is a test literal:
/// two newlines, the open tag, a newline; then a newline, the close tag repeating the id, a newline.
fn recorded_long_paste() -> (Vec<u8>, String) {
    let recorded = std::fs::read(workspace_path(
        "fixtures/claude/2.1.287/UserPromptSubmit.paste-1.json",
    ))
    .expect("the recorded paste-1 variant");
    let payload: Value = serde_json::from_slice(&recorded).expect("the variant is JSON");
    let prompt = payload["prompt"].as_str().expect("a recorded prompt");
    let (id, body) = prompt
        .strip_prefix("\n\n<pasted_content id=\"")
        .and_then(|rest| rest.split_once("\">\n"))
        .expect("the recorded prompt opens with the frame");
    let text = body
        .strip_suffix(&format!("\n</pasted_content id=\"{id}\">\n"))
        .expect("the recorded prompt closes with the frame");
    assert_eq!(text.len(), 1500, "the long text as pasted");
    assert!(!text.contains('\n'));
    let text = text.to_owned();
    (recorded, text)
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
    // The first prompt's receipt lands after its hook returns, which can be after the send's reply.
    assert_eq!(
        prompts_at_least(&wrapper.receipt(), 1).len(),
        1,
        "nothing typed"
    );

    end_turn(&wrapper);
    let accepted = send(&home, &["builder"], &second);
    assert_eq!(accepted.code, Some(0), "stderr: {}", accepted.stderr);
    let typed = prompts_at_least(&wrapper.receipt(), 2);
    assert_eq!(typed.len(), 2);
    assert_eq!(typed[1]["text"], second.as_str());
    wrapper.stop();
}

/// A listed local command is never presumed delivered and never refused. On a CLI version with no
/// passing stamp nothing is measured for it, so `/clear` and `/remote-control` are each typed once
/// and answered `unconfirmable` at once: exit 0 and the one `ok` document under `--json`, recorded
/// and logged as `send-confirmed` with `confirmed:false`, the command's text in no role log line.
/// In human mode the same outcome is the open box on stdout.
#[test]
fn send_window_local_command_is_not_presumed_delivered() {
    let wrapper = boot(None, &["--local-command-mode"]);
    let dir = wrapper.instance_dir();
    let home = wrapper.home().to_path_buf();
    let mut cursors = Vec::new();
    for (n, command) in ["/clear", "/remote-control"].into_iter().enumerate() {
        let l = end_offset(&dir);
        let sent = send(&home, &["builder", "--json"], command);
        assert_eq!(sent.code, Some(0), "{command} stdout: {}", sent.stdout);
        assert!(sent.stderr.is_empty(), "--json writes nothing on stderr");
        assert_eq!(sent.stdout.lines().count(), 1, "one JSON document");
        assert_eq!(
            serde_json::from_str::<Value>(&sent.stdout).expect("one JSON document"),
            json!({"v": 1, "ok": {"confirmed": false, "detail": "unconfirmable", "cursor": l}}),
            "{command}"
        );
        let records = records_after(&dir, l);
        let kinds: Vec<&Value> = records.iter().map(|r| &r["kind"]).collect();
        assert_eq!(kinds, ["send-issued", "send-confirmed"], "{command}");
        assert_eq!(records[1]["data"], json!({"cursor": l, "confirmed": false}));
        let typed = prompts_at_least(&wrapper.receipt(), n + 1);
        assert_eq!(typed.len(), n + 1, "{command} typed once");
        assert_eq!(typed[n]["text"], command);
        assert_eq!(typed[n]["submit"], "local-command");
        cursors.push(l);
    }

    cursors.push(end_offset(&dir));
    let human = send(&home, &["builder"], "/remote-control");
    assert_eq!(human.code, Some(0), "stderr: {}", human.stderr);
    assert_eq!(
        human.stdout,
        "[  ] unconfirmable  builder  local command, no measured post-condition\n"
    );
    assert!(human.stderr.is_empty(), "{}", human.stderr);

    let role_file = home.join("diagnostics").join("run-builder.ndjson");
    let role = support::ndjson::read_lines(&role_file);
    for cursor in cursors {
        let of = |event: &str| -> Vec<&Value> {
            role.iter()
                .filter(|r| r["event"] == event && r["corr"] == cursor)
                .collect()
        };
        assert_eq!(of("send-issued").len(), 1, "cursor {cursor}");
        let confirmed = of("send-confirmed");
        assert_eq!(confirmed.len(), 1, "cursor {cursor}");
        assert_eq!(confirmed[0]["confirmed"], false);
    }
    assert!(!role.iter().any(|r| r["event"] == "send-refused"));
    let logged = std::fs::read(&role_file).expect("role file");
    let logged = String::from_utf8_lossy(&logged);
    for command in ["/clear", "/remote-control"] {
        assert!(
            !logged.contains(command),
            "the role log holds the sent text"
        );
    }
    wrapper.stop();
}

/// `/clear` on a verified CLI is confirmed by its measured post-condition (test-plan §6 Path 2).
/// With `--framing` the fake agent answers it with the recorded SessionEnd and SessionStart and no
/// prompt; the new session's id differs from the boot's, and the send reads back like any other.
#[rstest]
fn send_clear_on_a_verified_cli_is_confirmed_by_its_new_session(stamped_home: StampedHome) {
    let wrapper = boot_on(stamped_home, None, &["--framing"]);
    let dir = wrapper.instance_dir();
    let l = end_offset(&dir);
    let sent = send(wrapper.home(), &["builder"], "/clear");
    assert_eq!(sent.code, Some(0), "stderr: {}", sent.stderr);
    assert!(sent.stderr.is_empty(), "{}", sent.stderr);
    let lines: Vec<&str> = sent.stdout.lines().collect();
    assert_eq!(lines.len(), 1, "{}", sent.stdout);
    assert!(
        lines[0].starts_with("[RB] read back      builder  "),
        "{}",
        lines[0]
    );
    assert!(lines[0].ends_with(&format!("  cursor {l}")), "{}", lines[0]);

    let records = records_after(&dir, l);
    let kinds: Vec<&Value> = records.iter().map(|r| &r["kind"]).collect();
    assert_eq!(
        kinds,
        [
            "send-issued",
            "session-end",
            "session-start",
            "send-confirmed"
        ]
    );
    assert_eq!(
        records[2]["data"],
        json!({"cause": "clear", "agent_session_id": "d5d38bc2-fcb5-44d3-b336-6a62b90b7c39"})
    );
    assert_eq!(records[3]["data"], json!({"cursor": l}));
    let typed = prompts_at_least(&wrapper.receipt(), 1);
    assert_eq!(typed.len(), 1, "typed once");
    assert_eq!(typed[0]["text"], "/clear");
    wrapper.stop();
}

/// The decision reads the compiled list, never a leading slash: a slash text that is not on it is
/// an ordinary send, and with no prompt reported it is `not-delivered` once the window expires.
#[test]
fn send_window_slash_text_off_the_list_is_not_delivered() {
    let wrapper = boot(None, &["--local-command-mode"]);
    let sent = send(
        wrapper.home(),
        &["builder", "--json"],
        "/not-a-local-command",
    );
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

/// Git Bash rewrites a leading-slash argument into a path under its install before viola sees it
/// (`/clear` arrives as `C:/Program Files/Git/clear`). In either argument position the rewritten
/// path draws the warning ahead of the usage error. No wrapper runs: the argument never reaches one.
#[test]
fn send_rewritten_path_argument_draws_the_warning() {
    let home = TestHome::new();
    let rewritten = "C:/Program Files/Git/clear";
    for args in [&[rewritten][..], &["builder", rewritten][..]] {
        let sent = send(home.path(), args, CANARY);
        assert_eq!(sent.code, Some(2), "{args:?}");
        assert!(sent.stdout.is_empty(), "{args:?}");
        assert_eq!(
            sent.stderr.lines().next(),
            Some("warning: argument looks like a Git Bash rewritten path"),
            "{args:?}"
        );
    }
}

/// The readiness-gate reading, measured end to end. After a long paste the real CLI shows a paste
/// hint where the input-box literal was (6.5 s after the turn's Stop on 2.1.287, measured). `hint`
/// forces that window open with the fake agent's capped hold, and `no_hint` is the same sequence
/// without it: under the hint a verified wrapper refuses the next send `input-not-ready` and types
/// nothing, and without it the send is delivered. The hold only has to outlast the second send's
/// gate verdict (the screen quiet for 300 ms), and the whole case stays under the nextest `mutants`
/// kill.
#[rstest]
#[case::hint(&["--paste-hint-ms", "3000"], false)]
#[case::no_hint(&[], true)]
fn send_under_the_paste_hint_on_a_verified_cli(
    stamped_home: StampedHome,
    #[case] hold: &[&str],
    #[case] delivered: bool,
) {
    let mut extra = vec!["--framing", "--turn-stop"];
    extra.extend_from_slice(hold);
    let wrapper = boot_on(stamped_home, None, &extra);
    let dir = wrapper.instance_dir();
    let home = wrapper.home().to_path_buf();
    let receipt = wrapper.receipt();
    let (_, long) = recorded_long_paste();
    let first = send(&home, &["builder", "--json"], &long);
    assert_eq!(first.code, Some(0), "stdout: {}", first.stdout);
    wait_events(&dir, "the long turn's turn-ended", |l| {
        l.iter().any(|e| e["kind"] == "turn-ended")
    });
    // The agent draws its next screen once the Stop hook has returned, which its receipt line marks.
    fake::wait_for(&receipt, "the Stop hook's receipt", |l| {
        of_kind(l, "hook").iter().any(|h| h["event"] == "Stop")
    });

    let l = end_offset(&dir);
    let second = format!("{CANARY} after the long paste");
    let sent = send(&home, &["builder", "--json"], &second);
    let doc: Value = serde_json::from_str(&sent.stdout).expect("one JSON document");
    if delivered {
        assert_eq!(sent.code, Some(0), "stdout: {}", sent.stdout);
        assert_eq!(doc["ok"]["cursor"], l);
        let typed = prompts_at_least(&receipt, 2);
        assert_eq!(typed.len(), 2);
        assert_eq!(typed[1]["text"], second.as_str());
        wrapper.stop();
    } else {
        assert_eq!(sent.code, Some(13), "stdout: {}", sent.stdout);
        assert_eq!(
            doc,
            json!({"v": 1, "refusal": "not-delivered", "detail": "input-not-ready"})
        );
        let records = records_after(&dir, l);
        assert_eq!(records.len(), 1, "{records:?}");
        assert_eq!(records[0]["kind"], "send-refused");
        assert_eq!(
            records[0]["data"],
            json!({"refusal": "not-delivered", "detail": "input-not-ready"})
        );
        // The agent reads its stdin again only after the hold, so the stop returns after it.
        let (_, kept) = wrapper.stop_keep();
        let typed = prompts(&receipt);
        assert_eq!(typed.len(), 1, "nothing more was typed");
        assert_eq!(typed[0]["text"], long.as_str());
        drop(kept);
    }
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

/// The full readiness gate on a verified CLI: a stamped wrapper whose agent shows the recorded
/// trust dialog (no trusted root, so no hook fires) refuses a send `input-not-ready` and types
/// nothing. With no SessionStart, only the boot's own readiness is awaited.
#[rstest]
fn send_on_a_verified_cli_refuses_while_the_trust_dialog_is_up(stamped_home: StampedHome) {
    let fixtures = workspace_path("fixtures/claude");
    let fixtures = fixtures.to_str().expect("utf-8 path");
    let wrapper = Wrapper::boot_untrusted(stamped_home, "builder", None, &["--fixtures", fixtures]);
    let sent = send(wrapper.home(), &["builder", "--json"], CANARY);
    assert_eq!(sent.code, Some(13));
    assert_eq!(
        serde_json::from_str::<Value>(&sent.stdout).expect("one JSON document"),
        json!({"v": 1, "refusal": "not-delivered", "detail": "input-not-ready"})
    );
    let events = events(&wrapper.instance_dir());
    assert!(!events.iter().any(|e| e["kind"] == "send-issued"));
    assert!(!events.iter().any(|e| e["kind"] == "session-start"));
    let refused: Vec<&Value> = events
        .iter()
        .filter(|e| e["kind"] == "send-refused")
        .collect();
    assert_eq!(refused.len(), 1);
    assert_eq!(
        refused[0]["data"],
        json!({"refusal": "not-delivered", "detail": "input-not-ready"})
    );
    let receipt = fake::receipt(&wrapper.receipt());
    assert!(of_kind(&receipt, "prompt").is_empty());
    assert!(of_kind(&receipt, "key").is_empty(), "nothing was typed");
    assert!(of_kind(&receipt, "hook").is_empty(), "no hook before trust");
    wrapper.stop();
}

/// The CLI wraps a long paste in its own pair and frames it with newlines (measured on 2.1.287, the
/// recorded `paste-1` variant). With `--framing` the fake agent answers the sent long text with that
/// recorded prompt, its bytes unchanged, and the send is still confirmed: the wrapper's normalised
/// text is the text as sent (test-plan §6 Path 2, step 3).
#[test]
fn send_long_text_wrapped_by_the_cli_is_confirmed() {
    let (recorded, text) = recorded_long_paste();
    let text = text.as_str();

    let wrapper = boot(None, &["--framing"]);
    let dir = wrapper.instance_dir();
    let l = end_offset(&dir);
    let sent = send(wrapper.home(), &["builder", "--json"], text);
    assert_eq!(sent.code, Some(0), "stdout: {}", sent.stdout);
    assert!(sent.stderr.is_empty(), "--json writes nothing on stderr");
    let doc: Value = serde_json::from_str(&sent.stdout).expect("one JSON document");
    assert_eq!(doc["v"], 1);
    assert_eq!(doc["ok"]["cursor"], l);

    let records = wait_events(&dir, "the send's three records", |_| {
        records_after(&dir, l).len() >= 3
    });
    assert!(!records.is_empty());
    let records = records_after(&dir, l);
    let kinds: Vec<&Value> = records.iter().map(|r| &r["kind"]).collect();
    assert_eq!(kinds, ["send-issued", "prompt-submitted", "send-confirmed"]);
    assert_eq!(
        records[1]["data"],
        json!({"text": text, "origin": "driver"})
    );
    assert_eq!(records[2]["data"], json!({"cursor": l}));

    let typed = prompts_at_least(&wrapper.receipt(), 1);
    assert_eq!(typed.len(), 1, "exactly one prompt typed");
    assert_eq!(typed[0]["text"], text);
    assert_eq!(typed[0]["submit"], "fired");
    let receipt = fake::receipt(&wrapper.receipt());
    let offered: Vec<Vec<u8>> = of_kind(&receipt, "hook")
        .into_iter()
        .filter(|h| h["event"] == "UserPromptSubmit")
        .filter_map(|h| h["stdin_hex"].as_str().map(fake::unhex))
        .collect();
    assert_eq!(
        offered,
        [recorded],
        "the hook was offered the recorded wrapped prompt, not an echo"
    );
    wrapper.stop();
}
