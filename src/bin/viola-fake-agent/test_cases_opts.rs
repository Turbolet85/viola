use super::*;

#[test]
fn opts_parse_reads_values_flags_and_ignores_unknown() {
    let o = Opts::parse(&args(&[
        "--argv-sentinel",
        "--cli-version",
        "3.0.0",
        "--report-version",
        "4.0.0",
        "--script",
        "s.json",
        "--control",
        "c",
        "--receipt",
        "r",
        "--fixtures",
        "f",
        "--plugin-dir",
        "p",
        "--suppress-prompt-submit",
        "--local-command-mode",
        "--inject-harness-turn",
        "--exit-no-eof",
        "--vt100-panic-bytes",
        "--trusted-root",
        "t",
        "--screens",
        "--turn-stop",
        "--dialogs",
        "--framing",
        "--stop-receipt-hold-ms",
        "5000",
        "--paste-hint-ms",
        "11000",
        "--tag-turn-screen",
        "tagged",
        "--long-turn-screen",
        "longed",
        "--resume",
        "c0f0cc23-690b-45dd-bbfb-06d6cfd44942",
        "--fork-session",
        "--settings",
        "st.json",
        "--statusline-stdin",
        "sl",
        "--version",
        "-p",
        "a prompt",
    ]));
    assert_eq!(o.settings.as_deref(), Some(Path::new("st.json")));
    assert_eq!(o.statusline_stdin.as_deref(), Some(Path::new("sl")));
    assert_eq!(o.print.as_deref(), Some("a prompt"));
    assert_eq!(
        Opts::parse(&args(&["--print", "q"])).print.as_deref(),
        Some("q")
    );
    assert_eq!(o.cli_version(), "3.0.0");
    assert_eq!(o.version_answer(), "4.0.0 (Claude Code)");
    assert_eq!(o.script.as_deref(), Some(Path::new("s.json")));
    assert_eq!(o.control.as_deref(), Some(Path::new("c")));
    assert_eq!(o.receipt.as_deref(), Some(Path::new("r")));
    assert_eq!(o.fixtures.as_deref(), Some(Path::new("f")));
    assert_eq!(o.plugin_dir.as_deref(), Some(Path::new("p")));
    assert!(o.version && o.suppress_prompt_submit && o.local_command_mode);
    assert!(o.inject_harness_turn && o.exit_no_eof && !o.hold_stdout);
    assert!(o.vt100_panic_bytes);
    assert_eq!(o.trusted_root.as_deref(), Some(Path::new("t")));
    assert!(o.screens && o.turn_stop && o.dialogs && o.framing);
    assert_eq!(
        o.stop_receipt_hold,
        Some(Duration::from_millis(1000)),
        "capped"
    );
    assert_eq!(o.paste_hint, Some(Duration::from_millis(10_000)), "capped");
    assert_eq!(o.tag_turn_screen.as_deref(), Some("tagged"));
    assert_eq!(o.long_turn_screen.as_deref(), Some("longed"));
    assert_eq!(
        o.resume.as_deref(),
        Some("c0f0cc23-690b-45dd-bbfb-06d6cfd44942")
    );
    assert!(o.fork_session);
    assert!(Opts::parse(&args(&[HOLD_STDOUT])).hold_stdout);
}

#[test]
fn opts_default_to_the_default_cli_version() {
    let o = Opts::parse(&[]);
    assert_eq!(o.cli_version(), DEFAULT_CLI_VERSION);
    assert_eq!(o.version_answer(), "2.1.287 (Claude Code)");
    assert!(!o.version && !o.exit_no_eof && !o.suppress_prompt_submit);
    assert!(!o.vt100_panic_bytes);
    assert!(o.print.is_none());
    assert!(!o.screens && !o.turn_stop && !o.dialogs && o.trusted_root.is_none());
    assert!(!o.framing);
    assert_eq!(o.stop_receipt_hold, None);
    assert_eq!(
        Opts::parse(&args(&["--stop-receipt-hold-ms", "100"])).stop_receipt_hold,
        Some(Duration::from_millis(100))
    );
    assert_eq!(
        Opts::parse(&args(&["--stop-receipt-hold-ms", "x"])).stop_receipt_hold,
        None
    );
    assert_eq!(o.paste_hint, None);
    assert_eq!(
        Opts::parse(&args(&["--paste-hint-ms", "6000"])).paste_hint,
        Some(Duration::from_millis(6000))
    );
    assert_eq!(
        Opts::parse(&args(&["--paste-hint-ms", "x"])).paste_hint,
        None
    );
    assert_eq!(o.tag_turn_screen, None);
    assert_eq!(
        Opts::parse(&args(&["--tag-turn-screen"])).tag_turn_screen,
        None
    );
    assert!(o.resume.is_none() && !o.fork_session);
    assert_eq!(Opts::parse(&args(&["--resume"])).resume, None);
    assert!(o.settings.is_none() && o.statusline_stdin.is_none());
    assert_eq!(Opts::parse(&args(&["--settings"])).settings, None);
    assert!(!o.trusted(), "no root: every cwd is untrusted");
}

#[test]
fn opts_long_turn_screen_takes_the_phase_it_names_alone() {
    let o = Opts::parse(&args(&["--long-turn-screen", "long-modal"]));
    assert_eq!(o.long_turn_screen.as_deref(), Some("long-modal"));
    assert_eq!(o.tag_turn_screen, None);
}

#[test]
fn opts_long_turn_screen_defaults_to_none() {
    assert_eq!(Opts::parse(&[]).long_turn_screen, None);
    assert_eq!(
        Opts::parse(&args(&["--long-turn-screen"])).long_turn_screen,
        None
    );
}

#[test]
fn under_root_walks_up_from_the_cwd() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let deep = tmp.path().join("a").join("b");
    fs::create_dir_all(&deep).expect("dirs");
    assert!(under_root(&deep, tmp.path()));
    assert!(under_root(&deep, &deep));
    assert!(under_root(&deep, &tmp.path().join("a").join("..")));
    assert!(!under_root(tmp.path(), &deep));
    assert!(!under_root(&deep, &tmp.path().join("missing")));
    assert!(!under_root(&tmp.path().join("missing"), tmp.path()));
}

#[test]
fn screen_reads_the_versioned_fixture_rows() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let dir = tmp.path().join("3.0.0");
    fs::create_dir_all(&dir).expect("dir");
    fs::write(
        dir.join("Screen.ready.json"),
        r#"{"screen_phase":"ready","cols":80,"rows":["","a b"]}"#,
    )
    .expect("fixture");
    fs::write(dir.join("Screen.turn.json"), r#"{"rows":[1]}"#).expect("bad");
    let o = Opts::parse(&args(&[
        "--cli-version",
        "3.0.0",
        "--fixtures",
        tmp.path().to_str().expect("utf-8"),
    ]));
    assert_eq!(
        o.screen("ready"),
        Some(vec![String::new(), "a b".to_owned()])
    );
    assert_eq!(o.screen("turn"), None);
    assert_eq!(o.screen("modal"), None);
    assert_eq!(Opts::parse(&[]).screen("ready"), None);
    assert_eq!(
        render_screen(&["x".to_owned(), "y".to_owned()]),
        b"\x1b[2J\x1b[Hx\r\ny"
    );
}

#[test]
fn input_paste_then_cr_is_one_submit() {
    let mut i = Input::new();
    assert_eq!(
        feed(&mut i, b"\x1b[200~a\rb\x1b[201~\r"),
        ["k:0d", "p:610d62"]
    );
}

#[test]
fn input_typed_bytes_are_keys_and_cr_submits_them() {
    let mut i = Input::new();
    assert_eq!(feed(&mut i, b"ab\r"), ["k:61", "k:62", "k:0d", "p:6162"]);
    assert_eq!(feed(&mut i, b"\r"), ["k:0d"]);
    assert_eq!(feed(&mut i, b"\x03"), ["exit"]);
}

#[test]
fn input_escape_sequences_that_are_not_a_paste_are_keys() {
    let mut i = Input::new();
    assert_eq!(feed(&mut i, b"\x1b[I"), ["k:1b", "k:5b", "k:49"]);
    assert_eq!(
        feed(&mut i, b"\x1b[20x"),
        ["k:1b", "k:5b", "k:32", "k:30", "k:78"]
    );
    assert!(feed(&mut i, b"\x1b[200").is_empty());
    assert_eq!(
        feed(&mut i, b"\x1b"),
        ["k:1b", "k:5b", "k:32", "k:30", "k:30"]
    );
    assert!(feed(&mut i, b"[200~x\x1b[201~").is_empty());
    assert_eq!(
        feed(&mut i, b"\r"),
        ["k:0d", "p:1b5b491b5b3230781b5b32303078"]
    );
}

#[test]
fn parse_script_takes_registered_events_with_defaults() {
    let steps = parse_script(
        r#"{"v":1,"steps":[{"event":"Stop"},{"event":"PreToolUse","variant":"ask","gate":true}]}"#,
    )
    .expect("script");
    assert_eq!(
        steps,
        [
            Step {
                event: "Stop".to_owned(),
                variant: "default".to_owned(),
                gate: false,
                harness: false
            },
            Step {
                event: "PreToolUse".to_owned(),
                variant: "ask".to_owned(),
                gate: true,
                harness: false
            },
        ]
    );
    assert_eq!(parse_script(r#"{"v":2,"steps":[]}"#), None);
    assert_eq!(parse_script(r#"{"v":1,"steps":[{"event":"Other"}]}"#), None);
    assert_eq!(parse_script(r#"{"v":1}"#), None);
    assert_eq!(parse_script("x"), None);
}

#[test]
fn hook_commands_take_only_command_entries_for_the_event() {
    let doc = r#"{"hooks":{
            "Stop":[{"hooks":[{"type":"command","command":"/a","args":["x","y"]},{"type":"prompt","command":"/b"}]},
                    {"matcher":"m","hooks":[{"type":"command","command":"/c"}]}],
            "PreToolUse":[{"hooks":[{"type":"command","command":"/d"}]}]}}"#;
    assert_eq!(
        hook_commands(doc, "Stop", None),
        [
            ("/a".to_owned(), vec!["x".to_owned(), "y".to_owned()]),
            ("/c".to_owned(), vec![])
        ]
    );
    assert!(hook_commands(doc, "SessionEnd", None).is_empty());
    assert!(hook_commands("not json", "Stop", None).is_empty());
}

/// A group's matcher is read against a tool-bearing payload's `tool_name`, name by name.
#[test]
fn hook_commands_evaluate_the_matcher_against_the_tool() {
    let doc = r#"{"hooks":{"PreToolUse":[
            {"matcher":"AskUserQuestion|ExitPlanMode","hooks":[{"type":"command","command":"/dialog"}]},
            {"matcher":"","hooks":[{"type":"command","command":"/empty"}]},
            {"hooks":[{"type":"command","command":"/all"}]}]}}"#;
    let commands = |tool| {
        hook_commands(doc, "PreToolUse", tool)
            .into_iter()
            .map(|(c, _)| c)
            .collect::<Vec<_>>()
    };
    assert_eq!(
        commands(Some("ExitPlanMode")),
        ["/dialog", "/empty", "/all"]
    );
    assert_eq!(
        commands(Some("AskUserQuestion")),
        ["/dialog", "/empty", "/all"]
    );
    assert_eq!(commands(Some("Bash")), ["/empty", "/all"]);
    assert_eq!(commands(Some("Exit")), ["/empty", "/all"]);
    assert_eq!(commands(None), ["/dialog", "/empty", "/all"]);
}

#[test]
fn payload_sets_only_the_prompt_field() {
    let fixture = br#"{"prompt":"old","session_id":"s"}"#.to_vec();
    let out: Value =
        serde_json::from_slice(&payload(fixture.clone(), &[("prompt", "new")])).expect("json");
    assert_eq!(out, json!({"prompt": "new", "session_id": "s"}));
    assert_eq!(payload(fixture.clone(), &[]), fixture);
    assert_eq!(payload(b"[1]".to_vec(), &[("prompt", "new")]), b"[1]");
    assert_eq!(
        payload(b"{\"prompt\":\"old\"}\n".to_vec(), &[("prompt", "new")]),
        b"{\"prompt\":\"new\"}\n"
    );
    assert_eq!(
        payload(b"{\"prompt\":\"old\"}".to_vec(), &[("prompt", "new")]),
        b"{\"prompt\":\"new\"}"
    );
}

#[test]
fn fake_resume_sets_the_source_and_the_given_id_on_the_recorded_session_start() {
    let fixture = recorded_session_start();
    let id = "11111111-2222-4333-8444-555555555555";
    let opts = Opts::parse(&args(&["--resume", id]));
    assert_eq!(
        opts.resume_fields(),
        [("source", "resume"), ("session_id", id)]
    );
    let out = payload(fixture.clone(), &opts.resume_fields());
    assert!(fixture.ends_with(b"\n") && out.ends_with(b"\n"));
    assert_eq!(out.iter().filter(|b| **b == b'\n').count(), 1);
    let mut want: Value = serde_json::from_slice(&fixture).expect("the fixture");
    assert_eq!(want["source"], "startup");
    assert_eq!(want["session_id"], "c0f0cc23-690b-45dd-bbfb-06d6cfd44942");
    want["source"] = json!("resume");
    want["session_id"] = json!(id);
    let got: Value = serde_json::from_slice(&out).expect("json");
    assert_eq!(got, want);
    let keys = |v: &Value| {
        v.as_object()
            .expect("object")
            .keys()
            .cloned()
            .collect::<Vec<_>>()
    };
    assert_eq!(
        keys(&got),
        [
            "session_id",
            "transcript_path",
            "cwd",
            "hook_event_name",
            "source"
        ]
    );
}

#[test]
fn fake_resume_with_fork_session_reports_the_compiled_fork_id() {
    let opts = Opts::parse(&args(&[
        "--resume",
        "11111111-2222-4333-8444-555555555555",
        "--fork-session",
    ]));
    assert_eq!(
        opts.resume_fields(),
        [
            ("source", "resume"),
            ("session_id", "0f0e0d0c-0b0a-4908-8706-050403020100")
        ]
    );
    let out: Value =
        serde_json::from_slice(&payload(recorded_session_start(), &opts.resume_fields()))
            .expect("json");
    assert_eq!(out["session_id"], "0f0e0d0c-0b0a-4908-8706-050403020100");
    assert_eq!(out["source"], "resume");
}

/// Under `--resume` the launch fires the set's `resume-1` where it is recorded, else `default`;
/// without `--resume` always `default`.
#[test]
fn fake_resume_launches_on_the_recorded_resume_variant_when_the_set_holds_one() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let set = tmp.path().join("3.0.0");
    fs::create_dir_all(&set).expect("set");
    let variant = |extra: &[&str]| {
        let mut list = vec![
            "--cli-version".to_owned(),
            "3.0.0".to_owned(),
            "--fixtures".to_owned(),
            tmp.path().to_str().expect("utf-8").to_owned(),
        ];
        list.extend(extra.iter().map(|a| (*a).to_owned()));
        Agent::new(Opts::parse(&list)).launch_variant()
    };
    let id = "11111111-2222-4333-8444-555555555555";
    assert_eq!(variant(&["--resume", id]), "default", "no variant recorded");
    fs::write(set.join("SessionStart.resume-1.json"), b"{}\n").expect("variant");
    assert_eq!(variant(&["--resume", id]), "resume-1");
    assert_eq!(variant(&[]), "default", "no --resume");
}

/// Without `--resume` nothing is set, `--fork-session` alone included: the recorded bytes.
#[test]
fn fake_resume_absent_leaves_the_recorded_bytes() {
    let fixture = recorded_session_start();
    for list in [&[][..], &["--fork-session"][..]] {
        let opts = Opts::parse(&args(list));
        assert!(opts.resume_fields().is_empty(), "{list:?}");
        assert_eq!(payload(fixture.clone(), &opts.resume_fields()), fixture);
    }
}

#[test]
fn fake_statusline_words_are_the_command_split_on_ascii_white_space() {
    let words = |doc: &str| statusline_words(doc.as_bytes());
    assert_eq!(
        words(r#"{"statusLine":{"type":"command","command":"/h/bin/k/viola hook  statusline"}}"#),
        Some((
            "/h/bin/k/viola".to_owned(),
            vec!["hook".to_owned(), "statusline".to_owned()]
        ))
    );
    assert_eq!(
        words(r#"{"statusLine":{"command":"relative"}}"#),
        Some(("relative".to_owned(), vec![]))
    );
    assert_eq!(words(r#"{"statusLine":{"command":"  "}}"#), None);
    assert_eq!(words(r#"{"statusLine":{"command":7}}"#), None);
    assert_eq!(words(r#"{"model":"m"}"#), None);
    assert_eq!(words("not json"), None);
}

#[test]
fn fake_statusline_echo_exit_is_the_code_after_the_marker_path() {
    assert_eq!(statusline_echo_exit(&args(&["m"])), 0);
    assert_eq!(statusline_echo_exit(&args(&["m", "--exit", "3"])), 3);
    assert_eq!(statusline_echo_exit(&args(&["m", "--exit"])), 0);
    assert_eq!(statusline_echo_exit(&args(&["m", "--exit", "x"])), 0);
    assert_eq!(statusline_echo_exit(&args(&["m", "--other", "3"])), 0);
    assert_eq!(statusline_echo_exit(&args(&["--exit", "3"])), 0);
    assert_eq!(statusline_echo_exit(&[]), 0);
    assert_eq!(STATUSLINE_ECHO, "statusline-echo");
    assert_eq!(STATUSLINE_ECHO_OUTPUT, b"viola-fake-statusline\n");
}

/// Without both options, or with a stdin file that cannot be read, nothing runs and nothing is
/// receipted; a command that is not absolute is receipted and never run.
#[test]
fn fake_statusline_runs_only_with_both_options_and_only_an_absolute_command() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let path = |name: &str| tmp.path().join(name).to_str().expect("utf-8").to_owned();
    let (settings, stdin, receipt) = (path("settings.json"), path("stdin"), path("receipt"));
    fs::write(&settings, r#"{"statusLine":{"command":"relative arg"}}"#).expect("settings");
    fs::write(&stdin, b"payload").expect("stdin");
    let run = |list: &[&str]| {
        let mut list = list.to_vec();
        list.extend(["--receipt", &receipt]);
        Agent::new(Opts::parse(&args(&list))).run_statusline();
        fs::read_to_string(&receipt).unwrap_or_default()
    };
    assert_eq!(run(&["--settings", &settings]), "");
    assert_eq!(run(&["--statusline-stdin", &stdin]), "");
    let missing = path("missing");
    assert_eq!(
        run(&["--settings", &settings, "--statusline-stdin", &missing]),
        ""
    );
    assert_eq!(
        run(&["--settings", &missing, "--statusline-stdin", &stdin]),
        ""
    );
    let line: Value = serde_json::from_str(
        run(&["--settings", &settings, "--statusline-stdin", &stdin]).trim_end(),
    )
    .expect("one receipt line");
    assert_eq!(
        line,
        json!({"v": 1, "kind": "statusline", "command_absolute": false, "ran": false})
    );
}
