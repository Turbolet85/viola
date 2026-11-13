use super::*;

/// An absolute command is run with the stdin file's bytes and receipted in the hook receipt's
/// form, without an `event`.
#[test]
fn fake_statusline_receipts_the_run_of_an_absolute_command() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let path = |name: &str| tmp.path().join(name).to_str().expect("utf-8").to_owned();
    let (settings, stdin, receipt) = (path("settings.json"), path("stdin"), path("receipt"));
    let doc = json!({"statusLine": {"type": "command", "command": whoami()}});
    fs::write(&settings, doc.to_string()).expect("settings");
    fs::write(&stdin, b"payload").expect("stdin");
    let list = [
        "--settings",
        &settings,
        "--statusline-stdin",
        &stdin,
        "--receipt",
        &receipt,
    ];
    Agent::new(Opts::parse(&args(&list))).run_statusline();
    let text = fs::read_to_string(&receipt).expect("the receipt");
    let line: Value = serde_json::from_str(text.trim_end()).expect("one receipt line");
    assert_eq!(line["kind"], "statusline");
    assert!(line.get("event").is_none());
    assert_eq!(line["command_absolute"], true);
    assert_eq!(line["ran"], true);
    assert_eq!(line["exit_code"], 0);
    assert_eq!(line["stdin_hex"], hex(b"payload"));
    assert!(line["stdout_hex"].as_str().is_some_and(|h| !h.is_empty()));
    assert!(line["stderr_len"].is_u64());
}

#[test]
fn fake_statusline_replays_the_default_then_the_rate_limits_once_under_framing() {
    let lines = statusline_replayed(&whoami(), true, &["--framing"]);
    let stdins: Vec<&str> = lines
        .iter()
        .filter(|l| l["kind"] == "statusline")
        .map(|l| l["stdin_hex"].as_str().unwrap_or(""))
        .collect();
    assert_eq!(stdins, [hex(b"{\"n\":1}\n"), hex(b"{\"n\":2}\n")]);
    assert!(lines.iter().all(|l| l["ran"] == true));
}

#[test]
fn fake_statusline_replays_nothing_without_framing_or_without_the_variants() {
    assert!(statusline_replayed(&whoami(), true, &[]).is_empty());
    assert!(statusline_replayed(&whoami(), false, &["--framing"]).is_empty());
}

#[test]
fn fake_statusline_replay_of_a_relative_command_runs_nothing() {
    let lines = statusline_replayed("relative hook statusline", true, &["--framing"]);
    assert_eq!(lines.len(), 2);
    for line in lines {
        assert_eq!(
            line,
            json!({"v": 1, "kind": "statusline", "command_absolute": false, "ran": false})
        );
    }
}

#[test]
fn fake_statusline_hands_the_shell_name_word_over_as_bin_sh() {
    let words = |list: &[&str]| list.iter().map(|w| (*w).to_owned()).collect::<Vec<_>>();
    assert_eq!(
        shell_expanded(words(&["hook", "statusline", "--shell", "\"$0\""])),
        words(&["hook", "statusline", "--shell", "/bin/sh"])
    );
    assert_eq!(
        shell_expanded(words(&["$0", "'$0'", "\"$0\"x", "\"$1\""])),
        words(&["$0", "'$0'", "\"$0\"x", "\"$1\""])
    );
    assert_eq!(SHELL_NAME_WORD, "\"$0\"");
}

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
