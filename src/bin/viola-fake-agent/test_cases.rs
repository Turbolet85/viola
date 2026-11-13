use super::agents::{agents_output, live_sessions, planted_rows};
use super::statusline::statusline_echo_exit;
use super::*;
use std::time::Instant;

fn args(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| (*s).to_owned()).collect()
}

/// Feeds bytes and renders the actions as `k:<hex>` / `p:<hex>` / `exit`.
fn feed(input: &mut Input, bytes: &[u8]) -> Vec<String> {
    bytes
        .iter()
        .flat_map(|b| input.feed(*b))
        .map(|a| match a {
            Action::Key(b) => format!("k:{}", hex(&[b])),
            Action::Submit(p) => format!("p:{}", hex(&p)),
            Action::Exit => "exit".to_owned(),
        })
        .collect()
}

/// The recorded default SessionStart of the default set, as committed.
fn recorded_session_start() -> Vec<u8> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join("claude")
        .join(DEFAULT_CLI_VERSION)
        .join("SessionStart.default.json");
    fs::read(path).expect("the recorded SessionStart")
}

/// A set holding the two statusline variants and a settings file naming `command`, then the
/// receipt lines the launch's replay and two turns under `--framing --turn-stop` leave, with the
/// extra arguments given.
fn statusline_replayed(command: &str, variants: bool, extra: &[&str]) -> Vec<Value> {
    let tmp = tempfile::tempdir().expect("tempdir");
    let set = tmp.path().join("fixtures").join("3.0.0");
    fs::create_dir_all(&set).expect("set");
    if variants {
        fs::write(set.join("StatusLine.default-1.json"), b"{\"n\":1}\n").expect("default");
        fs::write(set.join("StatusLine.rate-limits-1.json"), b"{\"n\":2}\n").expect("limits");
    }
    let settings = tmp.path().join("settings.json");
    let doc = json!({"statusLine": {"type": "command", "command": command}});
    fs::write(&settings, doc.to_string()).expect("settings");
    let receipt = tmp.path().join("receipt.ndjson");
    let utf8 = |p: PathBuf| p.to_str().expect("utf-8").to_owned();
    let mut list: Vec<String> = [
        "--cli-version".to_owned(),
        "3.0.0".to_owned(),
        "--fixtures".to_owned(),
        utf8(tmp.path().join("fixtures")),
        "--settings".to_owned(),
        utf8(settings),
        "--receipt".to_owned(),
        utf8(receipt.clone()),
        "--turn-stop".to_owned(),
    ]
    .into();
    list.extend(extra.iter().map(|a| (*a).to_owned()));
    let agent = Agent::new(Opts::parse(&list));
    agent.replay_statusline(STATUSLINE_DEFAULT);
    agent.end_turn(None);
    agent.end_turn(None);
    fs::read_to_string(&receipt)
        .unwrap_or_default()
        .lines()
        .filter_map(|l| serde_json::from_str::<Value>(l).ok())
        .collect()
}

/// An absolute host program that prints one line and exits by itself.
fn whoami() -> String {
    let path = PathBuf::from("/usr/bin/whoami");
    path.to_str().expect("utf-8 path").to_owned()
}

/// The `event` of each `hook` receipt line `replay_dialogs("probe")` writes over a set whose
/// call `probe-1` holds only `events`. Every hook command is relative: receipted, never run.
fn replayed(events: &[&str]) -> Vec<String> {
    let tmp = tempfile::tempdir().expect("tempdir");
    let set = tmp.path().join("fixtures").join("3.0.0");
    fs::create_dir_all(&set).expect("set");
    for event in events {
        fs::write(set.join(format!("{event}.probe-1.json")), b"{}").expect("fixture");
    }
    let hooks = tmp.path().join("plugin").join("hooks");
    fs::create_dir_all(&hooks).expect("hooks dir");
    let group = json!([{"hooks": [{"type": "command", "command": "relative"}]}]);
    let doc = json!({"hooks": {
        "PreToolUse": group, "PermissionRequest": group, "PostToolUse": group,
    }});
    fs::write(hooks.join("hooks.json"), doc.to_string()).expect("hooks.json");
    let receipt = tmp.path().join("receipt.ndjson");
    let utf8 = |p: PathBuf| p.to_str().expect("utf-8").to_owned();
    let list = [
        "--cli-version".to_owned(),
        "3.0.0".to_owned(),
        "--fixtures".to_owned(),
        utf8(tmp.path().join("fixtures")),
        "--plugin-dir".to_owned(),
        utf8(tmp.path().join("plugin")),
        "--receipt".to_owned(),
        utf8(receipt.clone()),
    ];
    Agent::new(Opts::parse(&list)).replay_dialogs("probe");
    fs::read_to_string(&receipt)
        .expect("receipt")
        .lines()
        .filter_map(|l| serde_json::from_str::<Value>(l).ok())
        .filter(|l| l["kind"] == "hook")
        .map(|l| l["event"].as_str().unwrap_or("").to_owned())
        .collect()
}

#[rstest::rstest]
#[case::not_json("not json")]
#[case::an_object(r#"{"pid":7}"#)]
#[case::empty("")]
fn planted_rows_of_a_malformed_rows_file_are_none(#[case] text: &str) {
    let tmp = tempfile::tempdir().expect("tempdir");
    fs::write(tmp.path().join(AGENTS_ROWS), text).expect("rows");
    assert!(planted_rows(tmp.path()).is_empty());
    assert!(planted_rows(&tmp.path().join("absent")).is_empty());
}

#[path = "test_cases_opts.rs"]
mod opts;

#[path = "test_cases_modes.rs"]
mod modes;
