use super::*;

#[rstest]
fn verify_a_complete_set_prints_every_relied_step_and_stamps_every_row(
    #[from(home)] home: TestHome,
) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &[], &[]);
    assert_eq!(ran.code, Some(0), "stdout {}", ran.stdout_text());
    assert_eq!(ran.stdout_text(), all_pass_stdout());
    assert!(ran.stderr.is_empty(), "stderr {}", ran.stderr_text());
    assert_plain(&ran.stdout);

    let doc = stamps(home.path());
    assert_eq!(doc["v"], 1);
    assert_eq!(doc["writer"], "verify");
    let entry = &doc["data"]["versions"]["2.1.0"];
    for id in ROW_IDS {
        assert_eq!(entry["rows"][id], "pass", "{id}");
    }
    let stamped_rows = entry["rows"].as_object().expect("rows").len();
    assert_eq!(
        stamped_rows,
        ROW_IDS.len(),
        "only the relied rows are stamped"
    );
    let measured = entry["measured"]["largest_hook_payload"]
        .as_object()
        .expect("measured");
    let events: Vec<&str> = measured.keys().map(String::as_str).collect();
    assert_eq!(
        events,
        ["SessionStart", "UserPromptSubmit", "Stop", "SessionEnd"]
    );
    assert!(measured.values().all(|v| v.as_u64().is_some_and(|b| b > 0)));
    // The fake agent replays the two parallel calls one after the other, as the live CLI ran them.
    assert_eq!(
        entry["measured"]["dialog_probe"]["parallel_both_before_first_post"],
        false
    );
    let text = fs::read_to_string(ledger(home.path()).join("stamps.json")).expect("stamps");
    assert!(!text.contains(CANARY), "payload content reached the stamps");
    assert!(ledger(home.path()).join("stamps.json.lock").is_file());
    assert_eq!(
        probes_left(home.path()),
        0,
        "the probe dir outlived the run"
    );
    assert!(
        !home.path().join("diagnostics").exists(),
        "a process log without an instance"
    );
    {
        use std::os::unix::fs::PermissionsExt as _;
        let meta = fs::metadata(ledger(home.path()).join("stamps.json")).expect("meta");
        assert_eq!(meta.permissions().mode() & 0o777, 0o600);
    }
}

/// A `ledger/` another user may write, on a home a first run stamped: the next run is refused
/// before its log and before any probe — exit 1, no step line or summary on stdout, the home's
/// pair on stderr — and the stamps stay byte for byte, the widening as found.
#[rstest]
fn verify_refuses_a_widened_ledger_before_any_step(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let first = verify(home.path(), &fixtures(&home), "2.1.0", &[], &[], &[]);
    assert_eq!(first.code, Some(0), "stdout {}", first.stdout_text());
    let stamped = fs::read(ledger(home.path()).join("stamps.json")).expect("stamps");
    support::home::widen(&ledger(home.path()));

    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &[], &[]);
    assert_eq!(ran.code, Some(1), "stderr {}", ran.stderr_text());
    assert_eq!(ran.stdout_text(), "");
    assert_eq!(
        ran.stderr_text(),
        concat!(
            "unable: the viola home is not private to you\n",
            "hint: the viola home must be readable only by you; viola does not change its ",
            "permissions\n",
        )
    );
    assert_eq!(
        fs::read(ledger(home.path()).join("stamps.json")).expect("stamps"),
        stamped
    );
    assert!(support::home::still_widened(&ledger(home.path())));
    assert!(!home.path().join("diagnostics").exists(), "a log opened");
    support::home::unwiden(&ledger(home.path()));
}

/// The trusted run's turn would wait out the probe deadline for a Stop that never comes, so the
/// fake agent's trust root is moved off the cwd: its trusted, dialog and plan runs start at the
/// trust dialog and are killed with no key, which fails the three rows a turn measures, the four
/// dialog rows, the three framing rows, the newline row, the resume row (Run B captured no
/// SessionStart, so no Run E starts), the agents row (Run B never reached its turn, so no read is
/// made), the compact row and, on Unix, the three statusline rows (an untrusted start has no status
/// line).
#[rstest]
fn verify_a_set_without_stop_fails_its_rows_and_still_stamps(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", Some("Stop"));
    let elsewhere = home.scratch().to_str().expect("utf-8").to_owned();
    let after = ["--trusted-root", elsewhere.as_str()];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &after, &[]);
    assert_eq!(ran.code, Some(1));
    let stdout = ran.stdout_text();
    let lines: Vec<&str> = stdout.lines().collect();
    let mut expected = steps_failing(|i| ![0, 2, 3, 5, 6].contains(&i));
    expected.push("stamped 2.1.0  5 pass  19 fail".to_owned());
    assert_eq!(lines, expected);
    assert!(ran.stderr.is_empty());
    let rows = &stamps(home.path())["data"]["versions"]["2.1.0"]["rows"];
    assert_eq!(rows["spine-hooks"], "fail");
    assert_eq!(rows["stop-message"], "fail");
    assert_eq!(rows["shim-resolution"], "pass");
    assert_eq!(probes_left(home.path()), 0);
}

#[rstest]
fn verify_an_unreadable_version_refuses_and_writes_no_stamp(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let after = ["--report-version", "garbage"];
    let env = [("VIOLA_NAME", OsString::from("verifier"))];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &after, &env);
    assert_eq!(ran.code, Some(1));
    assert!(ran.stdout.is_empty());
    assert_eq!(
        ran.stderr_text(),
        "unable: the CLI version could not be read\nhint: run claude --version to check the install\n"
    );
    assert!(!ledger(home.path()).join("stamps.json").exists());
    assert_eq!(probes_left(home.path()), 0);
    assert_refused_line(home.path(), "version-unreadable");
}

/// One byte of the pinned copy a first run placed: the second run refuses before any probe, its code
/// on the line.
#[rstest]
fn verify_a_tampered_pinned_copy_refuses_with_its_code(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let first = verify(home.path(), &fixtures(&home), "2.1.0", &[], &[], &[]);
    assert_eq!(first.code, Some(0), "stdout {}", first.stdout_text());
    let key = fs::read_dir(home.path().join("bin"))
        .expect("bin")
        .next()
        .expect("one key")
        .expect("the entry");
    let pinned = key.path().join("viola");
    let mut bytes = fs::read(&pinned).expect("the pinned copy");
    let middle = bytes.len() / 2;
    bytes[middle] ^= 0x01;
    fs::write(&pinned, &bytes).expect("one byte changed");
    let env = [("VIOLA_NAME", OsString::from("verifier"))];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &[], &env);
    assert_eq!(ran.code, Some(1));
    assert!(ran.stdout.is_empty());
    assert_eq!(
        ran.stderr_text(),
        "unable: the pinned viola copy failed its integrity check\n\
         hint: the pinned copy was changed after it was written, so viola will not run it\n"
    );
    assert_refused_line(home.path(), "pinned-hash-mismatch");
}

/// `ledger/stamps.json` widened between two runs passes the tree check, which reads the folder, and
/// is refused at the write: nothing read or written, the home's pair, its code on the line.
#[rstest]
fn verify_window_a_widened_stamps_file_refuses_with_its_code(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let first = verify(home.path(), &fixtures(&home), "2.1.0", &[], &[], &[]);
    assert_eq!(first.code, Some(0), "stdout {}", first.stdout_text());
    let file = ledger(home.path()).join("stamps.json");
    let stamped = fs::read(&file).expect("stamps");
    support::home::widen(&file);
    let env = [("VIOLA_NAME", OsString::from("verifier"))];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &[], &env);
    assert_eq!(ran.code, Some(1), "stderr {}", ran.stderr_text());
    assert_eq!(
        ran.stderr_text(),
        concat!(
            "unable: the viola home is not private to you\n",
            "hint: the viola home must be readable only by you; viola does not change its ",
            "permissions\n",
        )
    );
    assert_eq!(fs::read(&file).expect("stamps"), stamped);
    assert_refused_line(home.path(), "strict-modes-failed");
    support::home::unwiden(&file);
}

#[rstest]
fn verify_a_missing_program_refuses(#[from(home)] home: TestHome) {
    let absent = home.scratch().join("no-such-claude");
    let args: Vec<OsString> = vec![
        "--home".into(),
        home.path().into(),
        "verify".into(),
        "--".into(),
        absent.into(),
    ];
    let ran = viola(&args, &[]);
    assert_eq!(ran.code, Some(1));
    assert!(ran.stdout.is_empty());
    assert_eq!(
        ran.stderr_text(),
        "unable: the claude CLI was not found\nhint: install Claude Code or put it on PATH\n"
    );
    assert!(
        !home.path().exists(),
        "nothing is written for a missing program"
    );
}

/// Every child runs under the R8 strip: no `env` receipt (the print-mode child and the five
/// interactive runs) lists a stripped name, and every hook fired ran and exited 0: the print turn's
/// four, the trusted run's twelve (SessionStart, UserPromptSubmit and Stop, the three paste turns'
/// UserPromptSubmit and Stop each, `/compact`'s SessionStart, then the local command's SessionEnd
/// and SessionStart), the dialog run's fifteen (SessionStart, then per prompt UserPromptSubmit, its
/// dialog events and Stop), the plan run's seven and the resume run's one SessionStart; the
/// untrusted run fires none: thirty-nine.
/// The dialog, plan and resume runs are killed only once the screen settled after their last
/// awaited hook, so every hook is receipted. On Unix the trusted run's
/// status line runs twice too, `viola hook statusline --capture` each time, and exits 0.
#[rstest]
fn verify_runs_the_probe_under_the_identity_strip(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let receipt = home.scratch().join("receipt.ndjson");
    let receipt_arg = receipt.to_str().expect("utf-8").to_owned();
    let env = [(
        "CLAUDE_CODE_MESSAGING_TOKEN",
        OsString::from("canary-token-value-7f3a"),
    )];
    let after = ["--receipt", receipt_arg.as_str()];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &after, &env);
    assert_eq!(ran.code, Some(0));
    let lines = fake::receipt(&receipt);
    let envs = of_kind(&lines, "env");
    assert_eq!(envs.len(), 6);
    for env in envs {
        let names: Vec<&str> = env["names"]
            .as_array()
            .expect("names")
            .iter()
            .filter_map(Value::as_str)
            .collect();
        assert!(
            !names
                .iter()
                .any(|n| n.eq_ignore_ascii_case("CLAUDE_CODE_MESSAGING_TOKEN"))
        );
    }
    let hooks = of_kind(&lines, "hook");
    assert_eq!(hooks.len(), 39);
    assert!(
        hooks
            .iter()
            .all(|h| h["ran"] == true && h["exit_code"] == 0)
    );
    let statusline = of_kind(&lines, "statusline");
    assert_eq!(statusline.len(), 2);
    assert!(
        statusline
            .iter()
            .all(|h| h["ran"] == true && h["exit_code"] == 0 && h["stdout_hex"] == "")
    );
}

/// A home whose parent is a regular file cannot be statted (`NotADirectory`), so the home's check
/// refuses it before anything runs — exit 1, the home's pair. Nothing on stdout.
#[rstest]
fn verify_under_a_regular_file_refuses_the_home(#[from(home)] home: TestHome) {
    let file = home.scratch().join("file");
    fs::write(&file, b"x").expect("a file where the home's parent should be");
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let ran = verify(&file.join("home"), &fixtures(&home), "2.1.0", &[], &[], &[]);
    assert_eq!(ran.code, Some(1));
    assert!(ran.stdout.is_empty());
    assert_eq!(
        ran.stderr_text(),
        concat!(
            "unable: the viola home is not private to you\n",
            "hint: the viola home must be readable only by you; viola does not change its ",
            "permissions\n",
        )
    );
}

/// A home that passes its check but holds a regular file where `ledger/` goes: the probe dir
/// cannot be made, and the `cli` catch site prints its one line and nothing else.
#[rstest]
fn verify_under_a_regular_file_prints_only_the_internal_error_line(#[from(home)] home: TestHome) {
    crate::support::home::private_dir(home.path(), "the home");
    viola_state::fs::replace_private(&home.path().join("ledger"), b"x", 0o600)
        .expect("a file where the ledger goes");
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &[], &[]);
    assert_eq!(ran.code, Some(1));
    assert!(ran.stdout.is_empty());
    assert_eq!(ran.stderr_text(), INTERNAL_ERROR);
}

/// With `VIOLA_NAME` the run logs to `cli-<name>.ndjson`; a failing `--record` write is a dispatch
/// error whose chain lands only in the instance detail file.
#[rstest]
fn verify_dispatch_error_keeps_the_chain_in_the_detail_file(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let blocker = home.scratch().join("blocker");
    fs::write(&blocker, b"x").expect("a file where the record dir should be");
    let record = blocker.join("rec");
    let record_arg = record.to_str().expect("utf-8").to_owned();
    let env = [("VIOLA_NAME", OsString::from("verifier"))];
    let before = ["--record", record_arg.as_str()];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &before, &[], &env);
    assert_eq!(ran.code, Some(1));
    assert_eq!(ran.stderr_text(), INTERNAL_ERROR);
    assert!(ran.stdout_text().ends_with(&format!("{STAMPED_PASS}\n")));

    let role_path = home.path().join("diagnostics").join("cli-verifier.ndjson");
    let role_text = fs::read_to_string(&role_path).expect("role file");
    let role = support::ndjson::complete_lines(role_text.as_bytes());
    let shape: Vec<(&str, &str, &str)> = role
        .iter()
        .map(|l| {
            (
                l["event"].as_str().unwrap_or(""),
                l["subject"].as_str().unwrap_or(""),
                l["detail"].as_str().unwrap_or(""),
            )
        })
        .collect();
    assert_eq!(
        shape,
        [
            ("process-start", "self", ""),
            ("process-start", "version-probe", ""),
            ("process-exit", "version-probe", ""),
            ("process-start", "verify-probe", ""),
            ("process-exit", "verify-probe", ""),
            ("process-start", "verify-pty-probe", ""),
            ("process-exit", "verify-pty-probe", ""),
            ("process-start", "verify-pty-probe", ""),
            ("process-start", "agents-probe", ""),
            ("process-exit", "agents-probe", ""),
            ("process-exit", "verify-pty-probe", ""),
            ("process-start", "verify-pty-probe", ""),
            ("process-exit", "verify-pty-probe", ""),
            ("process-start", "verify-pty-probe", ""),
            ("process-exit", "verify-pty-probe", ""),
            ("process-start", "verify-pty-probe", ""),
            ("process-exit", "verify-pty-probe", ""),
            ("process-exit", "self", "internal-error"),
        ]
    );
    assert!(
        role.iter()
            .all(|l| l["process"] == "cli" && l["instance"] == "verifier")
    );
    assert!(!role_text.contains("chain"));
    let detail = home
        .path()
        .join("instances")
        .join("verifier")
        .join("diagnostics")
        .join("detail-cli.ndjson");
    let detail = support::ndjson::read_lines(&detail);
    assert_eq!(detail.len(), 1);
    assert_eq!(detail[0]["event"], "process-exit");
    assert!(detail[0]["chain"].as_array().is_some_and(|c| !c.is_empty()));
    assert!(!record.exists());
}

/// A failing row is no internal error. The Stop-less set's trusted run starts at the trust dialog
/// (its trust root moved off the cwd), as in `verify_a_set_without_stop_fails_its_rows_and_still_stamps`.
#[rstest]
fn verify_with_an_instance_logs_its_start_and_exit(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", Some("Stop"));
    let env = [("VIOLA_NAME", OsString::from("verifier"))];
    let elsewhere = home.scratch().to_str().expect("utf-8").to_owned();
    let after = ["--trusted-root", elsewhere.as_str()];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &after, &env);
    assert_eq!(ran.code, Some(1));
    assert!(ran.stderr.is_empty(), "a failing row is no internal error");
    let role =
        support::ndjson::read_lines(&home.path().join("diagnostics").join("cli-verifier.ndjson"));
    let exit = role
        .iter()
        .find(|l| l["event"] == "process-exit" && l["subject"] == "self")
        .expect("exit line");
    assert_eq!(exit["exit_code"], 1);
    assert!(exit.get("detail").is_none());
}

/// Each spawn `verify` makes is logged as a pair (obs-plan §6 Child / shell spawns): the `--version`
/// read as `version-probe`, the print-mode probe as `verify-probe`, each of the five interactive
/// runs as `verify-pty-probe`, between its own start and exit, and inside Run B's pair the agents
/// read as `agents-probe`: eight pairs.
#[rstest]
fn verify_with_an_instance_logs_its_eight_spawn_pairs(#[from(home)] home: TestHome) {
    write_spine_set(&fixtures(&home), "2.1.0", None);
    let env = [("VIOLA_NAME", OsString::from("verifier"))];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &[], &[], &env);
    assert_eq!(ran.code, Some(0));
    assert_eq!(ran.stdout_text(), all_pass_stdout(), "stdout is unchanged");
    assert!(ran.stderr.is_empty());
    let role =
        support::ndjson::read_lines(&home.path().join("diagnostics").join("cli-verifier.ndjson"));
    let shape: Vec<(&str, &str)> = role
        .iter()
        .map(|l| {
            (
                l["event"].as_str().unwrap_or(""),
                l["subject"].as_str().unwrap_or(""),
            )
        })
        .collect();
    assert_eq!(
        shape,
        [
            ("process-start", "self"),
            ("process-start", "version-probe"),
            ("process-exit", "version-probe"),
            ("process-start", "verify-probe"),
            ("process-exit", "verify-probe"),
            ("process-start", "verify-pty-probe"),
            ("process-exit", "verify-pty-probe"),
            ("process-start", "verify-pty-probe"),
            ("process-start", "agents-probe"),
            ("process-exit", "agents-probe"),
            ("process-exit", "verify-pty-probe"),
            ("process-start", "verify-pty-probe"),
            ("process-exit", "verify-pty-probe"),
            ("process-start", "verify-pty-probe"),
            ("process-exit", "verify-pty-probe"),
            ("process-start", "verify-pty-probe"),
            ("process-exit", "verify-pty-probe"),
            ("process-exit", "self"),
        ]
    );
    for exit in [&role[2], &role[4], &role[9], &role[10]] {
        assert_eq!(exit["child_exit_status"], 0);
        assert!(exit["duration_ms"].is_u64());
    }
    for killed in [&role[6], &role[12], &role[14], &role[16]] {
        assert!(killed["duration_ms"].is_u64(), "a killed run's exit");
    }
    let validator =
        jsonschema::validator_for(&load_schema(&workspace_path("schemas/diag-line.v1.json")))
            .expect("valid schema");
    for (n, line) in role.iter().enumerate() {
        assert!(validator.is_valid(line), "role line {n} fails the schema");
    }
}

#[rstest]
fn verify_record_writes_the_scrubbed_spine_variants_and_screens(#[from(home)] home: TestHome) {
    let user_home = std::env::home_dir().expect("a user home");
    let user = user_of(&user_home);
    personal_set(&fixtures(&home), &user_home, &user, None);
    let record = home.scratch().join("rec");
    let record_arg = record.to_str().expect("utf-8").to_owned();
    let before = ["--record", record_arg.as_str()];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &before, &[], &[]);
    assert_eq!(ran.code, Some(0), "stderr {}", ran.stderr_text());
    assert!(ran.stderr.is_empty());

    let dir = record.join("2.1.0");
    let mut names: Vec<String> = fs::read_dir(&dir)
        .expect("recorded dir")
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    let expected = vec![
        "Agents.recorded-1.json",
        "PermissionRequest.permission-1.json",
        "PermissionRequest.plan-1.json",
        "PostToolUse.parallel-1.json",
        "PostToolUse.parallel-2.json",
        "PostToolUse.permission-1.json",
        "PostToolUse.plan-2.json",
        "PostToolUse.questions-1.json",
        "PreToolUse.parallel-1.json",
        "PreToolUse.parallel-2.json",
        "PreToolUse.plan-1.json",
        "PreToolUse.plan-2.json",
        "PreToolUse.questions-1.json",
        "Screen.modal.json",
        "Screen.ready.json",
        "Screen.turn.json",
        "SessionEnd.clear-1.json",
        "SessionEnd.default.json",
        "SessionStart.clear-1.json",
        "SessionStart.compact-1.json",
        "SessionStart.default.json",
        "SessionStart.resume-1.json",
        "StatusLine.default-1.json",
        "StatusLine.rate-limits-1.json",
        "Stop.default.json",
        "UserPromptSubmit.default.json",
        "UserPromptSubmit.paste-1.json",
        "UserPromptSubmit.paste-2.json",
        "UserPromptSubmit.paste-3.json",
    ];
    assert_eq!(names, expected);
    let schema = load_schema(&workspace_path("schemas/claude-fixture.v1.json"));
    let statusline = load_schema(&workspace_path("schemas/claude-statusline-fixture.v1.json"));
    let agents_schema = load_schema(&workspace_path("schemas/claude-agents-fixture.v1.json"));
    let agents = fs::read(dir.join("Agents.recorded-1.json")).expect("the agents read");
    assert_eq!(check(&agents, &agents_schema, Some(&user)), Ok(()));
    let agents_text = String::from_utf8(agents).expect("utf-8");
    assert!(agents_text.ends_with("]\n") && agents_text.matches('\n').count() == 1);
    let agents: Value = serde_json::from_str(&agents_text).expect("json");
    let rows = agents.as_array().expect("an array");
    assert_eq!(rows.len(), 1, "only Run B's own row is kept");
    assert!(rows[0]["cwd"].as_str().is_some_and(|c| c.starts_with('~')));
    assert_eq!(rows[0]["kind"], "interactive");
    assert_eq!(rows[0]["status"], "idle");
    assert!(!has_username(&agents_text, &user));
    for name in names
        .iter()
        .filter(|n| !n.starts_with("Screen.") && !n.starts_with("Agents."))
    {
        let bytes = fs::read(dir.join(name)).expect("fixture");
        let schema = if name.starts_with("StatusLine.") {
            &statusline
        } else {
            &schema
        };
        assert_eq!(check(&bytes, schema, Some(&user)), Ok(()), "{name}");
        let text = String::from_utf8(bytes).expect("utf-8");
        assert!(
            text.ends_with("}\n") && text.matches('\n').count() == 1,
            "{name}"
        );
        let doc: Value = serde_json::from_str(&text).expect("json");
        assert!(
            doc["cwd"].as_str().is_some_and(|c| c.starts_with('~')),
            "{name}"
        );
        assert!(!has_username(&text, &user), "{name}");
    }
    let prompt: Value =
        serde_json::from_slice(&fs::read(dir.join("UserPromptSubmit.default.json")).expect("read"))
            .expect("json");
    assert_eq!(
        prompt["prompt"],
        "viola verify probe: reply with the single word ok"
    );
    // The recorded variants keep the prompt and the fields the fake agent replayed, unscrubbed.
    let read = |name: &str| -> Value {
        serde_json::from_slice(&fs::read(dir.join(name)).expect("read")).expect("json")
    };
    assert_eq!(
        read("UserPromptSubmit.paste-1.json")["prompt"],
        support::verify::wrapped(&support::verify::long_paste())
    );
    assert_eq!(
        read("UserPromptSubmit.paste-2.json")["prompt"],
        support::verify::TAG_PROMPT
    );
    assert_eq!(
        read("UserPromptSubmit.paste-3.json")["prompt"],
        "viola verify probe: this paste holds one line break here\nand the next line ends it. \
         Reply with the single word ok"
    );
    assert_eq!(read("SessionEnd.clear-1.json")["reason"], "clear");
    let started = read("SessionStart.clear-1.json");
    assert_eq!(started["source"], "clear");
    assert_eq!(started["session_id"], "s-verify-2");
    // `/compact`'s SessionStart: Run B's first session id kept.
    let compacted = read("SessionStart.compact-1.json");
    assert_eq!(compacted["source"], "compact");
    assert_eq!(
        compacted["session_id"],
        "5e55a0b1-0c0d-4e0f-8a1b-2c3d4e5f6a7b"
    );
    // Run E's SessionStart: the resumed session under Run B's first id, scrubbed like the rest.
    let resumed = read("SessionStart.resume-1.json");
    assert_eq!(resumed["source"], "resume");
    assert_eq!(
        resumed["session_id"],
        "5e55a0b1-0c0d-4e0f-8a1b-2c3d4e5f6a7b"
    );
    {
        assert!(
            read("StatusLine.default-1.json")
                .get("rate_limits")
                .is_none()
        );
        assert_eq!(
            read("StatusLine.rate-limits-1.json")["rate_limits"]["five_hour"]["used_percentage"],
            23
        );
    }
}

#[rstest]
fn verify_record_refuses_a_path_outside_the_home(#[from(home)] home: TestHome) {
    let user_home = std::env::home_dir().expect("a user home");
    let user = user_of(&user_home);
    personal_set(
        &fixtures(&home),
        &user_home,
        &user,
        Some("/home/elsewhere-5c1e/x"),
    );
    let record = home.scratch().join("rec");
    let record_arg = record.to_str().expect("utf-8").to_owned();
    let before = ["--record", record_arg.as_str()];
    let env = [("VIOLA_NAME", OsString::from("verifier"))];
    let ran = verify(home.path(), &fixtures(&home), "2.1.0", &before, &[], &env);
    assert_eq!(ran.code, Some(1));
    assert_refused_line(home.path(), "unclean-fixture");
    assert_eq!(
        ran.stderr_text(),
        "unable: a recorded fixture is not clean: Stop.default.json absolute-path\n\
         hint: record with a viola home under your user home\n",
        "the refusal names the file and the check, never the path"
    );
    assert!(!record.exists(), "a refused recording writes nothing");
    assert!(ran.stdout_text().ends_with(&format!("{STAMPED_PASS}\n")));
    assert_eq!(
        stamps(home.path())["data"]["versions"]["2.1.0"]["rows"]["spine-hooks"],
        "pass"
    );
}
