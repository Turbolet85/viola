use super::*;

/// A recorded screen keeps only its signature rows: the cwd, the status line and every other row
/// are written `""`.
#[rstest]
fn verify_record_screens_keep_only_signature_rows(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let busy = screen_rows(&[
        (3, "  ~/work/.viola-verify-7"),
        (20, "❯ Try something"),
        (22, "  ctx ? · a model"),
        (23, INPUT_BOX_ROW),
    ]);
    for phase in ["ready", "turn"] {
        write_screen(&fixtures(&home), "2.1.0", phase, &busy);
    }
    let record = home.scratch().join("rec");
    let record_arg = record.to_str().expect("utf-8").to_owned();
    let ran = verify(
        home.path(),
        &fixtures(&home),
        "2.1.0",
        &["--record", &record_arg],
        &[],
        &[],
    );
    assert_eq!(ran.code, Some(0), "stderr {}", ran.stderr_text());
    let validator = jsonschema::validator_for(&load_schema(&workspace_path(
        "schemas/claude-screen.v1.json",
    )))
    .expect("valid schema");
    for (name, phase, kept) in [
        (SCREENS[0], "modal", (9, TRUST_ROW)),
        (SCREENS[1], "ready", (23, INPUT_BOX_ROW)),
        (SCREENS[2], "turn", (23, INPUT_BOX_ROW)),
    ] {
        let text = fs::read_to_string(record.join("2.1.0").join(name)).expect("screen");
        assert!(
            text.ends_with("}\n") && text.matches('\n').count() == 1,
            "{name}"
        );
        let doc: Value = serde_json::from_str(&text).expect("json");
        assert!(validator.is_valid(&doc), "{name}");
        assert_eq!(doc["screen_phase"], phase);
        assert_eq!(doc["cols"], 80);
        assert_eq!(doc["rows"], json!(screen_rows(&[kept])), "{name}");
    }
}

/// One screen whose kept row, or that row's seam with a live neighbour, holds the home, the user,
/// or an email refuses the whole recording: nothing is written, the stamp still is.
#[rstest]
#[case::home_path("home", "row 23 home-path")]
#[case::username("user", "row 23 username")]
#[case::email("email", "row 23 email")]
#[case::username_split_at_a_seam("seam", "row 23 seam username")]
fn verify_record_refuses_a_dirty_kept_row(
    #[from(home)] home: TestHome,
    #[case] dirt: &str,
    #[case] named: &str,
) {
    let user_home = std::env::home_dir().expect("a user home");
    let user = user_of(&user_home);
    let split = user
        .char_indices()
        .nth(user.chars().count() / 2)
        .map_or(0, |(i, _)| i);
    let (head, tail) = user.split_at(split);
    let rows = match dirt {
        "home" => screen_rows(&[(
            23,
            &format!("{INPUT_BOX_ROW} {}", user_home.join("x").display()),
        )]),
        "user" => screen_rows(&[(23, &format!("{INPUT_BOX_ROW} by {user}"))]),
        "email" => screen_rows(&[(23, &format!("{INPUT_BOX_ROW} a.b@example.com"))]),
        _ => screen_rows(&[
            (22, &format!("  ctx {head}")),
            (23, &format!("{tail} {INPUT_BOX_ROW}")),
        ]),
    };
    write_spine_set(&fixtures(&home), "2.1.0", None);
    write_screen(&fixtures(&home), "2.1.0", "ready", &rows);
    let record = home.scratch().join("rec");
    let record_arg = record.to_str().expect("utf-8").to_owned();
    let ran = verify(
        home.path(),
        &fixtures(&home),
        "2.1.0",
        &["--record", &record_arg],
        &[],
        &[],
    );
    assert_eq!(ran.code, Some(1), "{dirt}");
    assert_eq!(
        ran.stderr_text(),
        format!(
            "unable: a recorded fixture is not clean: Screen.ready.json {named}\n\
             hint: record with a viola home under your user home\n"
        )
    );
    assert!(
        !ran.stderr_text().contains(&user),
        "{dirt}: the refusal names no content"
    );
    assert!(
        !record.exists(),
        "{dirt}: a refused recording writes nothing"
    );
    assert!(ran.stdout_text().ends_with(&format!("{STAMPED_PASS}\n")));
}

/// Run A's dir is a fresh `viola-verify-*` dir under the OS temp dir, Run B's is
/// `<cwd>/.viola-verify-<pid>/`, and Run C's, Run D's and Run E's carry `-dialogs`, `-plan` and
/// `-resume`; after a passing run and after a failing one, none is left. The failing run's Run B
/// starts at the trust dialog and captures no SessionStart, so no Run E starts.
#[rstest]
#[case::passing(false)]
#[case::failing(true)]
fn verify_leaves_neither_run_dir_behind(#[from(home)] home: TestHome, #[case] failing: bool) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    // Outside the workspace, the fake agent's trusted root: the untrusted run must stay untrusted.
    let isolated = tempfile::tempdir().expect("an isolated temp dir");
    let tmp = isolated.path().to_path_buf();
    let receipt = home.scratch().join("receipt.ndjson");
    let receipt_arg = receipt.to_str().expect("utf-8").to_owned();
    let elsewhere = home.scratch().to_str().expect("utf-8").to_owned();
    let mut after = vec!["--receipt", receipt_arg.as_str()];
    if failing {
        after.extend(["--trusted-root", elsewhere.as_str()]);
    }
    let env = [
        ("TMPDIR", tmp.clone().into_os_string()),
        ("TMP", tmp.clone().into_os_string()),
        ("TEMP", tmp.clone().into_os_string()),
    ];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &after, &env);
    assert_eq!(
        ran.code,
        Some(if failing { 1 } else { 0 }),
        "stdout {}",
        ran.stdout_text()
    );
    let cwds = cwds(&receipt);
    assert_eq!(cwds.len(), if failing { 5 } else { 6 });
    for cwd in &cwds {
        assert!(!cwd.exists(), "{} outlived the run", cwd.display());
    }
    let untrusted = &cwds[1];
    assert!(
        untrusted
            .file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n.starts_with("viola-verify-"))
    );
    assert_eq!(
        canonical(untrusted.parent().expect("parent")),
        canonical(&tmp)
    );
    let trusted = &cwds[2];
    let name = |p: &Path| {
        p.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_owned()
    };
    assert!(name(trusted).starts_with(".viola-verify-"));
    assert_eq!(name(&cwds[3]), format!("{}-dialogs", name(trusted)));
    assert_eq!(name(&cwds[4]), format!("{}-plan", name(trusted)));
    if !failing {
        assert_eq!(name(&cwds[5]), format!("{}-resume", name(trusted)));
    }
    for run in &cwds[2..] {
        assert_eq!(
            canonical(run.parent().expect("parent")),
            canonical(&std::env::current_dir().expect("cwd"))
        );
    }
    assert_eq!(fs::read_dir(&tmp).expect("temp dir").count(), 0);
}

#[test]
fn verify_help_names_the_trusted_folder_and_the_external_import_blocker() {
    let ran = viola(&["verify".into(), "--help".into()], &[]);
    assert_eq!(ran.code, Some(0));
    let text = ran.stdout_text();
    assert!(text.is_ascii());
    let flat = text.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        flat.contains("Run it from a folder you trust in Claude Code"),
        "{flat}"
    );
    assert!(
        flat.contains("An unapproved external CLAUDE.md import blocks the probe"),
        "{flat}"
    );
    assert!(!flat.contains("/home/") && !flat.contains(":\\"));
}

/// Without `--screens` the fake agent shows nothing: each of the five interactive runs waits out the
/// gate's 8.5 s maximum, and the twelve rows the first four measure fail while the six print-mode
/// rows pass; the resume row passes on Run E's SessionStart, captured before its wait; the agents
/// row fails, as Run B's turn never settles on a literal and no read is made, and so does the compact
/// row, as no framing paste is made; with no `--framing` either, the three statusline rows fail on
/// Unix.
#[rstest]
fn verify_window_without_screens_fails_every_interactive_row(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let ran = verify_without_screens(home.path(), &fixtures(&home), "2.1.0", &[]);
    assert_eq!(ran.code, Some(1));
    let stdout = ran.stdout_text();
    let lines: Vec<&str> = stdout.lines().collect();
    let mut expected = steps_failing(|i| i >= 6 && ROW_IDS[i] != "resume-session");
    expected.push("stamped 2.1.0  7 pass  17 fail".to_owned());
    assert_eq!(lines, expected);
    let rows = &stamps(home.path())["data"]["versions"]["2.1.0"]["rows"];
    for id in ROW_IDS[6..].iter().filter(|id| **id != "resume-session") {
        assert_eq!(rows[*id], "fail", "{id}");
    }
    assert_eq!(rows["resume-session"], "pass");
    assert_eq!(probes_left(home.path()), 0);
}

/// Run C and Run D are killed only once their last Stop hook has returned (the screen settled after
/// it, as Run B does): with the fake agent holding each Stop hook's receipt 100 ms past the hook's
/// exit, a window forced open and never sampled, every Stop of the nine turns is receipted. A kill at
/// the instant the Stop capture appears would hang up that hook mid-exit (a truncated coverage
/// profile) and lose its receipt.
#[rstest]
fn verify_kills_the_dialog_and_plan_runs_only_after_their_last_stop_hook(
    #[from(home)] home: TestHome,
) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let receipt = home.scratch().join("receipt.ndjson");
    let receipt_arg = receipt.to_str().expect("utf-8").to_owned();
    let after = [
        "--receipt",
        receipt_arg.as_str(),
        "--stop-receipt-hold-ms",
        "100",
    ];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &after, &[]);
    assert_eq!(ran.code, Some(0), "stdout {}", ran.stdout_text());
    let lines = fake::receipt(&receipt);
    let stops = of_kind(&lines, "hook")
        .iter()
        .filter(|h| h["event"] == "Stop")
        .count();
    assert_eq!(
        stops, 9,
        "the print turn's, Run B's four, Run C's three and Run D's Stop hooks"
    );
}

/// Without the dialog replay the dialog and plan runs see turns that raise no dialog: the ten spine
/// and screen rows, the three framing rows, the newline row, the resume row, the agents row, the
/// compact row and, on Unix, the three statusline rows pass, the four dialog rows fail, the stamp is
/// written
/// and verify exits 1 with nothing on stderr. The version is then unverified (R2).
#[rstest]
fn verify_without_the_dialog_replay_fails_the_four_dialog_rows(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let ran = support::verify::verify_without_dialogs(home.path(), &fixtures(&home), "2.1.0");
    assert_eq!(ran.code, Some(1));
    assert!(ran.stderr.is_empty(), "stderr {}", ran.stderr_text());
    let mut expected = steps_failing(|i| (10..14).contains(&i));
    expected.push("stamped 2.1.0  20 pass  4 fail".to_owned());
    let stdout = ran.stdout_text();
    assert_eq!(stdout.lines().collect::<Vec<_>>(), expected);
    assert_plain(&ran.stdout);
    let rows = &stamps(home.path())["data"]["versions"]["2.1.0"]["rows"];
    for id in &ROW_IDS[..10] {
        assert_eq!(rows[*id], "pass", "{id}");
    }
    for id in &ROW_IDS[10..14] {
        assert_eq!(rows[*id], "fail", "{id}");
    }
    for id in &ROW_IDS[14..] {
        assert_eq!(rows[*id], "pass", "{id}");
    }
    let entry = &stamps(home.path())["data"]["versions"]["2.1.0"];
    assert!(entry["measured"]["dialog_probe"]["parallel_both_before_first_post"].is_null());
    assert_eq!(probes_left(home.path()), 0);
}

/// Without the framing replay the fake agent echoes each added paste as typed: the three paste rows
/// pass on the echo, and `/compact` and the local command, each echoed as a prompt with no
/// SessionStart, fail their rows;
/// on Unix the status line, replayed only under `--framing`, files nothing and its three rows fail
/// too. Every paste still went into the trusted run, in order, and the run fired no hook for the
/// command but its echo.
#[rstest]
fn verify_without_framing_fails_the_clear_row_and_on_unix_the_statusline_rows(
    #[from(home)] home: TestHome,
) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let receipt = home.scratch().join("receipt.ndjson");
    let receipt_arg = receipt.to_str().expect("utf-8").to_owned();
    let after = ["--receipt", receipt_arg.as_str()];
    let ran = verify_without_framing(home.path(), &fixtures(&home), "2.1.0", &after);
    assert_eq!(ran.code, Some(1), "stdout {}", ran.stdout_text());
    assert!(ran.stderr.is_empty(), "stderr {}", ran.stderr_text());
    let local = |id: &str| id == "local-command-clear" || id == "local-command-compact";
    let mut expected =
        steps_failing(|i| local(ROW_IDS[i]) || STATUSLINE_ROWS.contains(&ROW_IDS[i]));
    expected.push("stamped 2.1.0  19 pass  5 fail".to_owned());
    let stdout = ran.stdout_text();
    assert_eq!(stdout.lines().collect::<Vec<_>>(), expected);
    assert_plain(&ran.stdout);
    let rows = &stamps(home.path())["data"]["versions"]["2.1.0"]["rows"];
    let failed = |id: &str| local(id) || STATUSLINE_ROWS.contains(&id);
    for id in ROW_IDS.iter().filter(|id| !failed(id)) {
        assert_eq!(rows[*id], "pass", "{id}");
    }
    assert_eq!(rows["local-command-clear"], "fail");
    assert_eq!(rows["local-command-compact"], "fail");
    for id in ROW_IDS.iter().filter(|id| STATUSLINE_ROWS.contains(id)) {
        assert_eq!(rows[*id], "fail", "{id}");
    }
    // The trusted run is the third start; its prompts are the probe prompt and the five pastes.
    let typed = trusted_run_typed(&receipt);
    assert_eq!(
        typed,
        [
            "viola verify probe: reply with the single word ok",
            support::verify::long_paste().as_str(),
            support::verify::TAG_PASTE,
            support::verify::NEWLINE_PASTE,
            "/compact",
            "/clear",
        ]
    );
    assert_eq!(probes_left(home.path()), 0);
}

/// The trusted run pastes only into a settled input box with no modal: when the screen after its
/// first turn shows a modal literal beside the input box, none of the five added texts is pasted
/// and their rows fail. The status line ran at the launch and after that turn, so on Unix its three
/// rows pass, and so do the resume and agents rows.
#[rstest]
fn verify_pastes_nothing_more_once_the_turn_screen_shows_a_modal(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let modal = screen_rows(&[(9, TRUST_ROW), (23, INPUT_BOX_ROW)]);
    write_screen(&fixtures(&home), "2.1.0", "turn", &modal);
    let receipt = home.scratch().join("receipt.ndjson");
    let receipt_arg = receipt.to_str().expect("utf-8").to_owned();
    let after = ["--receipt", receipt_arg.as_str()];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &after, &[]);
    assert_eq!(ran.code, Some(1), "stdout {}", ran.stdout_text());
    let rows = &stamps(home.path())["data"]["versions"]["2.1.0"]["rows"];
    for id in &ROW_IDS[14..18] {
        assert_eq!(rows[*id], "fail", "{id}");
    }
    for id in ROW_IDS[18..]
        .iter()
        .filter(|id| **id != "local-command-compact")
    {
        assert_eq!(rows[*id], "pass", "{id}");
    }
    assert_eq!(rows["local-command-compact"], "fail");
    assert_eq!(rows["input-box-signature"], "fail");
    let typed = trusted_run_typed(&receipt);
    assert_eq!(typed, ["viola verify probe: reply with the single word ok"]);
}

/// The guard before the next paste: when the screen after the tag-like turn alone shows a modal
/// literal beside the input box, the two paste rows have passed, none of the newline text,
/// `/compact` and `/clear` is pasted and their three rows fail. The trusted run's settle after that turn ends on any
/// compiled literal, so nothing is waited out.
#[rstest]
fn verify_pastes_no_local_command_once_the_tag_turn_screen_shows_a_modal(
    #[from(home)] home: TestHome,
) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let modal = screen_rows(&[(9, TRUST_ROW), (23, INPUT_BOX_ROW)]);
    write_screen(&fixtures(&home), "2.1.0", "tag-modal", &modal);
    let receipt = home.scratch().join("receipt.ndjson");
    let receipt_arg = receipt.to_str().expect("utf-8").to_owned();
    let after = [
        "--receipt",
        receipt_arg.as_str(),
        "--tag-turn-screen",
        "tag-modal",
    ];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &after, &[]);
    assert_eq!(ran.code, Some(1), "stdout {}", ran.stdout_text());
    let summary = "stamped 2.1.0  21 pass  3 fail\n";
    assert!(
        ran.stdout_text().ends_with(summary),
        "stdout {}",
        ran.stdout_text()
    );
    let rows = &stamps(home.path())["data"]["versions"]["2.1.0"]["rows"];
    assert_eq!(rows["long-paste-wrapper"], "pass");
    assert_eq!(rows["tag-escaping"], "pass");
    assert_eq!(rows["local-command-clear"], "fail");
    assert_eq!(rows["paste-newline"], "fail");
    assert_eq!(rows["local-command-compact"], "fail");
    let typed = trusted_run_typed(&receipt);
    assert_eq!(
        typed,
        [
            "viola verify probe: reply with the single word ok",
            support::verify::long_paste().as_str(),
            support::verify::TAG_PASTE,
        ]
    );
    assert_eq!(probes_left(home.path()), 0);
}

/// claude 2.1.302's stop, reproduced: when the screen after the long paste's turn alone shows no
/// input box the trusted run can type into (here a modal literal beside it, so the settle ends at
/// once on a compiled literal), none of the tag-like text, the newline text, `/compact` and
/// `/clear` is pasted. Their four rows fail, and so does `input-box-signature`, which reads that
/// screen too.
#[rstest]
fn verify_pastes_nothing_more_once_the_long_turn_screen_shows_a_modal(
    #[from(home)] home: TestHome,
) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let modal = screen_rows(&[(9, TRUST_ROW), (23, INPUT_BOX_ROW)]);
    write_screen(&fixtures(&home), "2.1.0", "long-modal", &modal);
    let receipt = home.scratch().join("receipt.ndjson");
    let receipt_arg = receipt.to_str().expect("utf-8").to_owned();
    let after = [
        "--receipt",
        receipt_arg.as_str(),
        "--long-turn-screen",
        "long-modal",
    ];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &after, &[]);
    assert_eq!(ran.code, Some(1), "stdout {}", ran.stdout_text());
    let red = [
        "input-box-signature",
        "tag-escaping",
        "local-command-clear",
        "paste-newline",
        "local-command-compact",
    ];
    let mut expected = steps_failing(|i| red.contains(&ROW_IDS[i]));
    expected.push("stamped 2.1.0  19 pass  5 fail".to_owned());
    let stdout = ran.stdout_text();
    assert_eq!(
        stdout.lines().collect::<Vec<_>>(),
        expected,
        "stdout {stdout}"
    );
    let rows = &stamps(home.path())["data"]["versions"]["2.1.0"]["rows"];
    for id in red {
        assert_eq!(rows[id], "fail", "{id}");
    }
    for id in ROW_IDS.iter().filter(|id| !red.contains(id)) {
        assert_eq!(rows[*id], "pass", "{id}");
    }
    let typed = trusted_run_typed(&receipt);
    assert_eq!(
        typed,
        [
            "viola verify probe: reply with the single word ok",
            support::verify::long_paste().as_str(),
        ]
    );
    assert_eq!(probes_left(home.path()), 0);
}

/// After a long paste the real CLI shows a paste hint in the input-box literal's place (8.0 s from
/// the paste on 2.1.287, measured). With the fake agent holding a cleared screen for 9 s after the
/// long turn's Stop, past the gate's 8.5 s maximum, a window forced open and never sampled, the
/// trusted run waits for the input box to come back, pastes the tag-like text, the newline text,
/// `/compact` and the local command, and every row passes.
#[rstest]
fn verify_window_paste_hint_past_the_gate_maximum_still_stamps(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let receipt = home.scratch().join("receipt.ndjson");
    let receipt_arg = receipt.to_str().expect("utf-8").to_owned();
    let after = ["--receipt", receipt_arg.as_str(), "--paste-hint-ms", "9000"];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &after, &[]);
    assert_eq!(ran.code, Some(0), "stdout {}", ran.stdout_text());
    assert_eq!(ran.stdout_text(), all_pass_stdout());
    assert!(ran.stderr.is_empty(), "stderr {}", ran.stderr_text());
    let typed = trusted_run_typed(&receipt);
    assert_eq!(
        typed,
        [
            "viola verify probe: reply with the single word ok",
            support::verify::long_paste().as_str(),
            support::verify::TAG_PASTE,
            support::verify::NEWLINE_PASTE,
            "/compact",
            "/clear",
        ]
    );
    assert_eq!(probes_left(home.path()), 0);
}

/// A path nested inside a dialog payload's `tool_input` survives the scrub: the whole recording is
/// refused, naming the variant file and the check, never the path.
#[rstest]
fn verify_record_refuses_a_nested_path_in_a_dialog_payload(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let dirty = support::verify::tool_payload(
        "PreToolUse",
        "AskUserQuestion",
        Some("toolu_q1"),
        json!({"tool_input": {"questions": [
            {"question": "Probe color?", "options": [{"label": "red"}], "description": "/home/elsewhere-5c1e/x"},
            {"question": "Probe size?", "options": [{"label": "small"}]},
        ]}}),
    );
    fake::write_fixture(
        &fixtures(&home),
        "2.1.0",
        "PreToolUse",
        "questions-1",
        &dirty,
    );
    let record = home.scratch().join("rec");
    let record_arg = record.to_str().expect("utf-8").to_owned();
    let ran = verify(
        home.path(),
        &fixtures(&home),
        "2.1.0",
        &["--record", &record_arg],
        &[],
        &[],
    );
    assert_eq!(ran.code, Some(1));
    assert_eq!(
        ran.stderr_text(),
        "unable: a recorded fixture is not clean: PreToolUse.questions-1.json absolute-path\n\
         hint: record with a viola home under your user home\n"
    );
    assert!(!record.exists(), "a refused recording writes nothing");
    assert!(ran.stdout_text().ends_with(&format!("{STAMPED_PASS}\n")));
}

#[test]
fn hook_capture_files_the_raw_payload_and_prints_nothing() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let payload = br#"{"hook_event_name":"SessionStart","raw":"kept as sent"}"#;
    let ran = hook_capture(tmp.path(), payload);
    assert_eq!(ran.code, Some(0));
    assert!(ran.stdout.is_empty() && ran.stderr.is_empty());
    assert_eq!(
        fs::read(tmp.path().join("SessionStart.1.json")).expect("capture"),
        payload
    );
    assert_eq!(fs::read_dir(tmp.path()).expect("dir").count(), 1);
}

#[test]
fn hook_capture_into_a_relative_or_missing_dir_writes_nothing() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let relative = Path::new("viola-capture-relative-5c1e");
    let missing = tmp.path().join("missing");
    for dir in [relative, missing.as_path()] {
        let ran = hook_capture(dir, b"{}");
        assert_eq!(ran.code, Some(0));
        assert!(ran.stdout.is_empty() && ran.stderr.is_empty());
        assert!(!dir.exists());
    }
    assert_eq!(fs::read_dir(tmp.path()).expect("dir").count(), 0);
}

/// Three `viola verify` runs the stamped-home fixture makes, each over its own fresh home, released
/// together by one barrier: each must stamp every row, whatever the others load the disk with (the
/// `resume-session` row read an empty Run E capture on a loaded runner, `ci#38994749835`).
#[test]
fn stamped_home_concurrent_verify_runs_each_stamp_the_home() {
    const RUNS: usize = 3;
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(RUNS));
    let runs: Vec<_> = (0..RUNS)
        .map(|_| {
            let barrier = std::sync::Arc::clone(&barrier);
            std::thread::spawn(move || {
                let home = TestHome::new();
                barrier.wait();
                let ran = verify(
                    home.path(),
                    &workspace_path("fixtures/claude"),
                    fake::RECORDED_CLI_VERSION,
                    &[],
                    &[],
                    &[],
                );
                (ran, home)
            })
        })
        .collect();
    let mut failures = Vec::new();
    for (n, run) in runs.into_iter().enumerate() {
        let (ran, _home) = run.join().expect("a verify thread");
        let stdout = ran.stdout_text();
        let last = stdout.lines().last().unwrap_or_default();
        let stamped = last.starts_with(&format!("stamped {}  ", fake::RECORDED_CLI_VERSION))
            && last.ends_with("pass  0 fail");
        if ran.code != Some(0) || !stamped {
            let failing: Vec<&str> = stdout
                .lines()
                .filter(|l| l.starts_with('[') && l.ends_with("  fail"))
                .filter_map(|l| l.split(' ').nth(1))
                .collect();
            failures.push(format!(
                "run {n}: exit {:?}, failing rows {failing:?}",
                ran.code
            ));
        }
    }
    assert!(failures.is_empty(), "{failures:?}");
}
