//! Hooks to normalised events (architecture §Conventions Hook → kind map; §Standard Contracts
//! Event `data` per kind): the fake agent fires every registered hook through the plugin `viola run`
//! wrote, each lands as one `source:"hook"` line of its kind in `events.ndjson`, prompts arrive
//! unwrapped and un-escaped with their origin, and a SessionEnd the channel cannot take is
//! appended directly unless another writer holds the log. Fixtures are synthetic, written here.

#[allow(dead_code)]
mod support;

use std::collections::BTreeSet;
use std::ffi::OsString;
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Instant;

use serde_json::{Value, json};
use support::fake::{self, FAKE, of_kind};
use support::home::{StampedHome, TestHome, VIOLA, Wrapper, workspace_path};
use support::hygiene::load_schema;
use support::watch::{WITHIN, Watch};

const CANARY: &str = "canary-chain-value-5c1e";

fn stamped() -> StampedHome {
    StampedHome {
        home: TestHome::new(),
        fake: PathBuf::from(FAKE),
        stamped: false,
    }
}

/// `<scratch>/fixtures/2.1.0/<Event>.<variant>.json` for each entry.
fn fixtures(scratch: &Path, bodies: &[(&str, &str, Value)]) -> PathBuf {
    let dir = scratch.join("fixtures");
    for (event, variant, body) in bodies {
        fake::write_fixture(&dir, "2.1.0", event, variant, body);
    }
    dir
}

/// A fake-agent script firing `(event, variant)` steps in order, ungated.
fn script(scratch: &Path, steps: &[(&str, &str)]) -> PathBuf {
    let steps: Vec<Value> = steps
        .iter()
        .map(|(event, variant)| json!({"event": event, "variant": variant}))
        .collect();
    let path = scratch.join("hooks.script.json");
    fs::write(&path, json!({"v": 1, "steps": steps}).to_string()).expect("script");
    path
}

fn path_str(path: &Path) -> &str {
    path.to_str().expect("utf-8 path")
}

fn events(instance_dir: &Path) -> Vec<Value> {
    support::ndjson::read_lines(&instance_dir.join("events.ndjson"))
}

/// Waits until `events.ndjson` holds `count` complete lines.
fn wait_events(instance_dir: &Path, count: usize) -> Vec<Value> {
    let watch = Watch::start("events");
    let deadline = Instant::now() + WITHIN;
    loop {
        let lines = events(instance_dir);
        if lines.len() >= count {
            return lines;
        }
        watch.note(&format!("events {} of {count}", lines.len()));
        watch.deadline_check(deadline, "timed out waiting for the hook events");
        std::thread::yield_now();
    }
}

fn hook_receipts(receipt: &Path, count: usize) -> Vec<Value> {
    let lines = fake::wait_for(receipt, "hook receipts", |l| {
        of_kind(l, "hook").len() >= count
    });
    of_kind(&lines, "hook").into_iter().cloned().collect()
}

/// Every hook line and detail line the run wrote passes its committed schema (G4).
fn assert_hook_logs_valid(home: &Path) {
    let role = support::ndjson::read_lines(&home.join("diagnostics").join("hook-builder.ndjson"));
    assert!(!role.is_empty(), "no hook role lines");
    let schema = load_schema(&workspace_path("schemas/diag-line.v1.json"));
    let validator = jsonschema::validator_for(&schema).expect("valid schema");
    for (n, line) in role.iter().enumerate() {
        assert!(
            validator.is_valid(line),
            "hook role line {n} fails the schema"
        );
        assert!(
            !line.to_string().contains(CANARY),
            "hook role line {n} holds content"
        );
    }
    let decisions = role.iter().filter(|l| l["event"] == "hook-decision");
    assert!(
        decisions.clone().all(|l| l.get("detail").is_none()),
        "a hook failed open"
    );
}

fn keys(data: &Value) -> BTreeSet<String> {
    data.as_object()
        .expect("data is an object")
        .keys()
        .cloned()
        .collect()
}

/// SessionStart at the agent's start, then every other registered hook from its script: one line
/// per hook, of its kind, `source:"hook"`, with exactly its kind's `data` keys.
#[test]
fn hook_every_registered_event_lands_as_one_line_of_its_kind() {
    let stamped = stamped();
    let scratch = stamped.home.scratch().to_path_buf();
    let fx = fixtures(
        &scratch,
        &[
            (
                "SessionStart",
                "default",
                json!({"hook_event_name": "SessionStart", "session_id": "s-1", "source": "startup", "transcript_path": CANARY}),
            ),
            (
                "UserPromptSubmit",
                "default",
                json!({"hook_event_name": "UserPromptSubmit", "session_id": "s-1", "prompt": CANARY}),
            ),
            (
                "Stop",
                "default",
                json!({"hook_event_name": "Stop", "session_id": "s-1", "last_assistant_message": CANARY}),
            ),
            (
                "SessionEnd",
                "default",
                json!({"hook_event_name": "SessionEnd", "session_id": "s-1", "reason": CANARY}),
            ),
            (
                "Notification",
                "default",
                json!({"hook_event_name": "Notification", "session_id": "s-1", "message": CANARY}),
            ),
            (
                "PostToolUse",
                "default",
                json!({"hook_event_name": "PostToolUse", "session_id": "s-1", "tool_name": "Bash", "tool_input": {"command": CANARY}}),
            ),
            (
                "PostToolUseFailure",
                "default",
                json!({"hook_event_name": "PostToolUseFailure", "session_id": "s-1", "tool_name": "Edit", "error": CANARY}),
            ),
        ],
    );
    let steps = script(
        &scratch,
        &[
            ("UserPromptSubmit", "default"),
            ("Stop", "default"),
            ("SessionEnd", "default"),
            ("Notification", "default"),
            ("PostToolUse", "default"),
            ("PostToolUseFailure", "default"),
        ],
    );
    let wrapper = Wrapper::boot(
        stamped,
        "builder",
        None,
        &["--fixtures", path_str(&fx), "--script", path_str(&steps)],
    );
    let receipts = hook_receipts(&wrapper.receipt(), 7);
    for receipt in &receipts {
        assert_eq!(receipt["command_absolute"], true, "{receipt}");
        assert_eq!(receipt["ran"], true, "{receipt}");
        assert_eq!(receipt["exit_code"], 0, "{receipt}");
        assert_eq!(receipt["stderr_len"], 0, "{receipt}");
        assert_eq!(receipt["stdout_hex"], "", "{receipt}");
    }
    let lines = wait_events(&wrapper.instance_dir(), 9);
    assert_eq!(lines.len(), 9);
    let hooked: Vec<&Value> = lines.iter().filter(|l| l["source"] == "hook").collect();
    assert_eq!(hooked.len(), 7);
    let mut seen: Vec<(String, BTreeSet<String>)> = hooked
        .iter()
        .map(|l| {
            (
                l["kind"].as_str().expect("kind").to_owned(),
                keys(&l["data"]),
            )
        })
        .collect();
    seen.sort();
    let set = |names: &[&str]| {
        names
            .iter()
            .map(|n| (*n).to_owned())
            .collect::<BTreeSet<_>>()
    };
    assert_eq!(
        seen,
        [
            ("activity".to_owned(), set(&[])),
            ("activity".to_owned(), set(&["tool"])),
            ("activity".to_owned(), set(&["tool"])),
            ("prompt-submitted".to_owned(), set(&["origin", "text"])),
            ("session-end".to_owned(), set(&[])),
            (
                "session-start".to_owned(),
                set(&["agent_session_id", "cause"])
            ),
            ("turn-ended".to_owned(), set(&["last_assistant_message"])),
        ]
    );
    for line in &hooked {
        assert_eq!(line["v"], 1);
        assert_eq!(line["instance"], "builder");
        for payload_key in ["session_id", "hook_event_name", "transcript_path"] {
            assert!(line.get(payload_key).is_none(), "{payload_key} at the top");
            assert!(
                line["data"].get(payload_key).is_none(),
                "{payload_key} in data"
            );
        }
    }
    let start = hooked
        .iter()
        .find(|l| l["kind"] == "session-start")
        .expect("start");
    assert_eq!(
        start["data"],
        json!({"cause": "startup", "agent_session_id": "s-1"})
    );
    let ended = hooked
        .iter()
        .find(|l| l["kind"] == "turn-ended")
        .expect("stop");
    assert_eq!(ended["data"], json!({"last_assistant_message": CANARY}));
    let tools: BTreeSet<&str> = hooked
        .iter()
        .filter_map(|l| l["data"]["tool"].as_str())
        .collect();
    assert_eq!(tools, BTreeSet::from(["Bash", "Edit"]));
    let (stopped, stamped) = wrapper.stop_keep();
    assert_eq!(stopped.code(), Some(0));
    assert_hook_logs_valid(stamped.home.path());
}

/// The CLI's own paste pair unwraps; typed tag text only loses its escaping; a harness prefix
/// gives `harness`, and a typed one does not.
#[test]
fn hook_prompts_arrive_normalised_with_their_origin() {
    let cases: [(&str, String, &str, String); 5] = [
        (
            "harness",
            format!("<task-notification>{CANARY}</task-notification>"),
            "harness",
            format!("<task-notification>{CANARY}</task-notification>"),
        ),
        (
            "paste",
            format!(
                "\n\n<pasted_content id=\"2f85\">\nA paste {CANARY}\n</pasted_content id=\"2f85\">\n"
            ),
            "human",
            format!("\n\nA paste {CANARY}\n"),
        ),
        (
            "typed",
            format!("<\\pasted_content id=\"1\">{CANARY}<\\/pasted_content id=\"1\">"),
            "human",
            format!("<pasted_content id=\"1\">{CANARY}</pasted_content id=\"1\">"),
        ),
        (
            "mismatch",
            format!("<pasted_content id=\"1\">\n{CANARY}\n</pasted_content id=\"2\">"),
            "human",
            format!("<pasted_content id=\"1\">\n{CANARY}\n</pasted_content id=\"2\">"),
        ),
        (
            "typedtask",
            format!("<\\task-notification>{CANARY}"),
            "human",
            format!("<task-notification>{CANARY}"),
        ),
    ];
    let stamped = stamped();
    let scratch = stamped.home.scratch().to_path_buf();
    let bodies: Vec<(&str, &str, Value)> = cases
        .iter()
        .map(|(variant, prompt, _, _)| {
            (
                "UserPromptSubmit",
                *variant,
                json!({"hook_event_name": "UserPromptSubmit", "session_id": "s-2", "prompt": prompt}),
            )
        })
        .collect();
    let fx = fixtures(&scratch, &bodies);
    let steps: Vec<(&str, &str)> = cases
        .iter()
        .map(|(variant, ..)| ("UserPromptSubmit", *variant))
        .collect();
    let steps = script(&scratch, &steps);
    let wrapper = Wrapper::boot(
        stamped,
        "builder",
        None,
        &["--fixtures", path_str(&fx), "--script", path_str(&steps)],
    );
    let lines = wait_events(&wrapper.instance_dir(), 2 + cases.len());
    let got: BTreeSet<(String, String)> = lines
        .iter()
        .filter(|l| l["kind"] == "prompt-submitted")
        .map(|l| {
            assert_eq!(l["source"], "hook");
            (
                l["data"]["origin"].as_str().expect("origin").to_owned(),
                l["data"]["text"].as_str().expect("text").to_owned(),
            )
        })
        .collect();
    let want: BTreeSet<(String, String)> = cases
        .iter()
        .map(|(_, _, origin, text)| ((*origin).to_owned(), text.clone()))
        .collect();
    assert_eq!(got, want);
    let (stopped, stamped) = wrapper.stop_keep();
    assert_eq!(stopped.code(), Some(0));
    assert_hook_logs_valid(stamped.home.path());
}

fn session_end(instance_dir: &Path) -> std::process::Output {
    let mut child = Command::new(VIOLA)
        .args(["hook", "session-end"])
        .env("VIOLA_NAME", "builder")
        .env("VIOLA_DIR", OsString::from(instance_dir))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("viola hook");
    let payload = json!({"hook_event_name": "SessionEnd", "reason": CANARY});
    let mut input = child.stdin.take().expect("stdin");
    input
        .write_all(payload.to_string().as_bytes())
        .expect("payload");
    drop(input);
    child.wait_with_output().expect("exit")
}

/// With the wrapper gone, SessionEnd appends its own line; with the log's lock held by another
/// writer, it appends nothing. Both exit 0 silently.
#[test]
fn hook_session_end_appends_directly_only_while_the_log_is_free() {
    let (stopped, stamped) = Wrapper::boot(stamped(), "builder", None, &[]).stop_keep();
    assert_eq!(stopped.code(), Some(0));
    let dir = stamped.home.path().join("instances").join("builder");
    assert_eq!(events(&dir).len(), 2);

    let out = session_end(&dir);
    assert_eq!(out.status.code(), Some(0));
    assert!(out.stdout.is_empty() && out.stderr.is_empty());
    let lines = events(&dir);
    assert_eq!(lines.len(), 3);
    assert_eq!(lines[2]["kind"], "session-end");
    assert_eq!(lines[2]["source"], "hook");
    assert_eq!(lines[2]["data"], json!({}));

    let lock = fs::OpenOptions::new()
        .write(true)
        .open(dir.join("events.ndjson.lock"))
        .expect("lock file");
    lock.lock().expect("held by this test");
    let out = session_end(&dir);
    assert_eq!(out.status.code(), Some(0));
    assert!(out.stdout.is_empty() && out.stderr.is_empty());
    drop(lock);
    assert_eq!(events(&dir).len(), 3, "a line landed past a held lock");
    let role = support::ndjson::read_lines(
        &stamped
            .home
            .path()
            .join("diagnostics")
            .join("hook-builder.ndjson"),
    );
    let details: Vec<&Value> = role
        .iter()
        .filter(|l| l["event"] == "hook-decision")
        .map(|l| &l["detail"])
        .collect();
    assert_eq!(details, [&json!("channel-unreachable"); 2]);
}
