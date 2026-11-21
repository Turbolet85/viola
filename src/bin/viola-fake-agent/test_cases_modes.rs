use super::*;

#[test]
fn hex_is_lowercase_two_digits_per_byte() {
    assert_eq!(hex(&[0x00, 0x0a, 0xff]), "000aff");
}

#[test]
fn replay_dialogs_replays_a_call_holding_only_a_pre_tool_use() {
    assert_eq!(replayed(&["PreToolUse"]), ["PreToolUse"]);
}

#[test]
fn replay_dialogs_replays_a_call_holding_only_a_permission_request() {
    assert_eq!(replayed(&["PermissionRequest"]), ["PermissionRequest"]);
}

#[test]
fn replay_dialogs_replays_nothing_for_a_stem_with_no_call() {
    assert!(replayed(&[]).is_empty());
}

#[test]
fn run_hook_after_close_hooks_runs_nothing() {
    let agent = Agent::new(Opts::parse(&[]));
    assert!(
        agent.run_hook("Notification", &whoami(), &[], b""),
        "the program ran and printed"
    );
    agent.close_hooks();
    assert!(!agent.run_hook("Notification", &whoami(), &[], b""));
}

#[test]
fn run_hook_of_a_stop_with_the_receipt_hold_takes_at_least_the_hold() {
    let agent = Agent::new(Opts::parse(&args(&["--stop-receipt-hold-ms", "1000"])));
    let hold = agent.opts.stop_receipt_hold.expect("the hold");
    let started = Instant::now();
    assert!(agent.run_hook("Stop", &whoami(), &[], b""));
    assert!(started.elapsed() >= hold, "{:?}", started.elapsed());
}

#[test]
fn submit_of_the_long_paste_with_a_paste_hint_takes_at_least_the_hint() {
    let agent = Agent::new(Opts::parse(&args(&[
        "--framing",
        "--turn-stop",
        "--paste-hint-ms",
        "300",
    ])));
    let hint = agent.opts.paste_hint.expect("the hint");
    let started = Instant::now();
    agent.submit(PROBE_LONG_PASTE.as_bytes(), "human");
    assert!(started.elapsed() >= hint, "{:?}", started.elapsed());
}

#[test]
fn agents_output_of_each_mode_is_its_bytes() {
    let oversize = agents_output(Some("oversize"), None);
    assert_eq!(oversize.len() as u64, MAX_FRAME + 1);
    assert!(oversize.iter().all(|b| *b == b' '));
    let malformed = agents_output(Some("malformed"), None);
    assert!(serde_json::from_slice::<Value>(&malformed).is_err());
    assert_eq!(agents_output(None, None), b"[]\n");
    assert_eq!(agents_output(Some("later-word"), None), b"[]\n");
    assert_eq!(AGENTS, "agents");
}

#[test]
fn agents_output_recorded_lists_the_planted_rows_unchanged_then_the_live_sessions() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let planted = r#"[{"pid":7,"sessionId":"s-1","status":"idle","later":1}]"#;
    fs::write(tmp.path().join(AGENTS_ROWS), format!("{planted}\n")).expect("rows");
    assert_eq!(
        agents_output(Some("recorded"), Some(tmp.path())),
        format!("{planted}\n").into_bytes()
    );
    let live = Registry::open(tmp.path(), "s-live", "~/w").expect("this process registers");
    live.set_status("busy");
    let listed: Value =
        serde_json::from_slice(&agents_output(None, Some(tmp.path()))).expect("json");
    assert_eq!(listed[1]["sessionId"], "s-live");
    assert_eq!(listed[1]["status"], "busy");
    assert_eq!(listed[1]["pid"], std::process::id());
    assert_eq!(listed[1]["name"], SESSION_NAME);
    assert_eq!(listed[1]["cwd"], "~/w");
    assert_eq!(listed.as_array().map(Vec::len), Some(2));
    live.remove();
    assert_eq!(
        agents_output(None, Some(tmp.path())),
        format!("{planted}\n").into_bytes()
    );
}

/// The registry row's id and cwd come from the launch SessionStart as recorded, `~` for a
/// recording without a cwd; with no launch fixture there is no row.
/// Under `--framing`, `/compact` fires its recorded SessionStart with the last fired
/// SessionStart's `session_id` (the launch one, then a replayed clear's) and nothing else
/// changed; a set without the variant fires nothing for it.
#[test]
fn fake_compact_replay_carries_the_last_fired_session_id() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let set = tmp.path().join("fixtures").join("3.0.0");
    fs::create_dir_all(&set).expect("set");
    let write = |name: &str, body: &str| fs::write(set.join(name), body).expect("fixture");
    write(
        "SessionStart.default.json",
        r#"{"session_id":"s-launch","source":"startup"}"#,
    );
    write("SessionEnd.clear-1.json", r#"{"reason":"clear"}"#);
    write(
        "SessionStart.clear-1.json",
        r#"{"session_id":"s-clear","source":"clear"}"#,
    );
    let hooks = tmp.path().join("plugin").join("hooks");
    fs::create_dir_all(&hooks).expect("hooks dir");
    let group = json!([{"hooks": [{"type": "command", "command": whoami()}]}]);
    let doc = json!({"hooks": {"SessionStart": group, "SessionEnd": group}});
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
        "--framing".to_owned(),
    ];
    let agent = Agent::new(Opts::parse(&list));
    agent.fire("SessionStart", "default", &[]);
    agent.submit(b"/compact", "driver");
    write(
        "SessionStart.compact-1.json",
        "{\"session_id\":\"s-recorded\",\"source\":\"compact\",\"cwd\":\"~/rec\"}\n",
    );
    agent.submit(b"/compact", "driver");
    agent.submit(b"/clear", "driver");
    agent.submit(b"/compact", "driver");
    let lines: Vec<Value> = fs::read_to_string(&receipt)
        .expect("receipt")
        .lines()
        .filter_map(|l| serde_json::from_str::<Value>(l).ok())
        .collect();
    let submits: Vec<&str> = lines
        .iter()
        .filter(|l| l["kind"] == "prompt")
        .map(|l| l["submit"].as_str().unwrap_or(""))
        .collect();
    assert_eq!(submits, ["no-fixture", "fired", "fired", "fired"]);
    let starts: Vec<Value> = lines
        .iter()
        .filter(|l| l["kind"] == "hook" && l["event"] == "SessionStart")
        .map(|l| {
            let stdin = l["stdin_hex"].as_str().expect("stdin_hex");
            let bytes: Vec<u8> = (0..stdin.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&stdin[i..i + 2], 16).expect("hex"))
                .collect();
            serde_json::from_slice(&bytes).expect("json")
        })
        .collect();
    assert_eq!(
        starts,
        [
            json!({"session_id": "s-launch", "source": "startup"}),
            json!({"session_id": "s-launch", "source": "compact", "cwd": "~/rec"}),
            json!({"session_id": "s-clear", "source": "clear"}),
            json!({"session_id": "s-clear", "source": "compact", "cwd": "~/rec"}),
        ]
    );
    assert!(
        !lines
            .iter()
            .any(|l| l["kind"] == "hook" && l["event"] == "UserPromptSubmit"),
        "no UserPromptSubmit"
    );
}

#[test]
fn launch_session_replays_the_recorded_id_and_cwd() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let set = tmp.path().join("9.9.9");
    fs::create_dir_all(&set).expect("set");
    let root = tmp.path().to_str().expect("utf-8").to_owned();
    let agent = |extra: &[&str]| {
        let mut a = vec!["--fixtures", root.as_str(), "--cli-version", "9.9.9"];
        a.extend_from_slice(extra);
        Agent::new(Opts::parse(&args(&a)))
    };
    assert_eq!(agent(&[]).launch_session(), None);
    fs::write(
        set.join("SessionStart.default.json"),
        br#"{"session_id":"s-1","cwd":"~/rec"}"#,
    )
    .expect("launch");
    assert_eq!(
        agent(&[]).launch_session(),
        Some(("s-1".to_owned(), "~/rec".to_owned()))
    );
    let resumed = agent(&["--resume", "s-9"]).launch_session();
    assert_eq!(resumed, Some(("s-9".to_owned(), "~/rec".to_owned())));
    fs::write(
        set.join("SessionStart.default.json"),
        br#"{"session_id":"s-1"}"#,
    )
    .expect("launch");
    assert_eq!(
        agent(&[]).launch_session(),
        Some(("s-1".to_owned(), "~".to_owned()))
    );
}

/// A row whose process is gone, or alive under another start time, is left out; this
/// process's own row with its own start time is listed.
#[test]
fn live_sessions_skip_a_dead_or_reused_pid() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let sessions = tmp.path().join(SESSIONS);
    fs::create_dir_all(&sessions).expect("sessions");
    let mut child = Command::new(std::env::current_exe().expect("exe"))
        .arg("--list")
        .stdout(Stdio::null())
        .spawn()
        .expect("child");
    let dead = child.id();
    child.wait().expect("reaped");
    let own = std::process::id();
    let started = process_start_time(own).expect("own start") * 1000;
    for (file, pid, at) in [
        ("1.json", dead, started),
        ("2.json", own, 1000),
        ("3.json", own, started),
    ] {
        let row = json!({"pid": pid, "sessionId": file, "startedAt": at, "status": "idle"});
        fs::write(sessions.join(file), row.to_string()).expect("row");
    }
    fs::write(sessions.join("4.json"), b"not json").expect("torn");
    fs::write(sessions.join("5.json.next"), b"{}").expect("a sibling");
    let ids: Vec<Value> = live_sessions(tmp.path())
        .into_iter()
        .map(|r| r["sessionId"].clone())
        .collect();
    assert_eq!(ids, [json!("3.json")]);
}

#[test]
fn hyprctl_output_replays_the_recorded_fixture_for_its_argv_and_each_mode_its_bytes() {
    use super::hyprctl::{HYPRCTL, hyprctl_output};
    let tmp = tempfile::tempdir().expect("tempdir");
    let set = tmp.path().join("hyprctl-set");
    fs::create_dir_all(&set).expect("set");
    let clients = json!({"v": 1, "hyprland": "0.56.2", "request": "clients",
                         "args": ["-j", "clients"], "exit": 0, "stdout": "[]\n"});
    let absent = json!({"v": 1, "hyprland": "0.56.2", "request": "version",
                        "args": ["-j", "version"], "exit": 3, "stdout": "viola fixture text"});
    fs::write(set.join("Clients.recorded-1.json"), clients.to_string()).expect("clients");
    fs::write(set.join("Version.absent-1.json"), absent.to_string()).expect("absent");
    let dir = Some(tmp.path());
    let clients_argv = args(&["-j", "clients"]);
    assert_eq!(
        hyprctl_output(None, dir, &clients_argv),
        (b"[]\n".to_vec(), 0)
    );
    assert_eq!(
        hyprctl_output(Some("recorded"), dir, &clients_argv),
        (b"[]\n".to_vec(), 0)
    );
    assert_eq!(
        hyprctl_output(None, dir, &args(&["-j", "version"])),
        (Vec::new(), 1)
    );
    assert_eq!(
        hyprctl_output(None, dir, &args(&["clients"])),
        (Vec::new(), 1)
    );
    assert_eq!(
        hyprctl_output(Some("absent"), dir, &clients_argv),
        (b"viola fixture text".to_vec(), 3)
    );
    let (oversize, code) = hyprctl_output(Some("oversize"), dir, &clients_argv);
    assert_eq!((oversize.len() as u64, code), (MAX_FRAME + 1, 0));
    let (malformed, code) = hyprctl_output(Some("malformed"), dir, &clients_argv);
    assert!(serde_json::from_slice::<Value>(&malformed).is_err());
    assert_eq!(code, 0);
    assert_eq!(hyprctl_output(None, None, &clients_argv), (Vec::new(), 1));
    assert_eq!(HYPRCTL, "hyprctl");
}

/// A fixtures root holding `<version>/<name>` for each of `files`, and the agent's arguments over
/// it: `--cli-version 3.0.0 --fixtures <root> --receipt <tmp>/receipt.ndjson` and `extra`.
fn over_set(tmp: &Path, files: &[(&str, &str)], extra: &[&str]) -> Vec<String> {
    let set = tmp.join("fixtures").join("3.0.0");
    fs::create_dir_all(&set).expect("set");
    for (name, body) in files {
        fs::write(set.join(name), body).expect("fixture");
    }
    let utf8 = |p: PathBuf| p.to_str().expect("utf-8").to_owned();
    let mut list = vec![
        "--cli-version".to_owned(),
        "3.0.0".to_owned(),
        "--fixtures".to_owned(),
        utf8(tmp.join("fixtures")),
        "--receipt".to_owned(),
        utf8(tmp.join("receipt.ndjson")),
    ];
    list.extend(args(extra));
    list
}

/// The receipt's lines of `kind`.
fn receipted(tmp: &Path, kind: &str) -> Vec<Value> {
    fs::read_to_string(tmp.join("receipt.ndjson"))
        .unwrap_or_default()
        .lines()
        .filter_map(|l| serde_json::from_str::<Value>(l).ok())
        .filter(|l| l["kind"] == kind)
        .collect()
}

#[test]
fn session_start_hook_carries_the_recorded_env_names() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let env =
        "{\"v\":1,\"added\":[\"CLAUDE_PROJECT_DIR\",\"CLAUDE_VIOLA_TEST_NEW_X\",\"not-a-name\"]}\n";
    let list = over_set(tmp.path(), &[(ENV_RECORDED, env)], &[]);
    let agent = Agent::new(Opts::parse(&list));
    assert_eq!(
        agent.recorded_env_names(),
        ["CLAUDE_PROJECT_DIR", "CLAUDE_VIOLA_TEST_NEW_X"]
    );
    let printenv = "/usr/bin/printenv".to_owned();
    assert!(agent.run_hook(
        "SessionStart",
        &printenv,
        &args(&["CLAUDE_VIOLA_TEST_NEW_X"]),
        b""
    ));
    assert!(!agent.run_hook("Stop", &printenv, &args(&["CLAUDE_VIOLA_TEST_NEW_X"]), b""));
    let hooks = receipted(tmp.path(), "hook");
    assert_eq!(hooks[0]["stdout_hex"], hex(b"fake-agent\n"));
    assert_eq!(hooks[1]["stdout_hex"], "");
    let bare = Agent::new(Opts::parse(&over_set(
        tempfile::tempdir().expect("t").path(),
        &[],
        &[],
    )));
    assert!(bare.recorded_env_names().is_empty(), "no variant, no name");
}
