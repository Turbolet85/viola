use super::*;

#[test]
fn ledger_rows_are_the_twenty_four_measured_behaviours_in_order() {
    let ids: Vec<&str> = LedgerRow::ALL.iter().map(|r| r.id()).collect();
    assert_eq!(
        ids,
        [
            "shim-resolution",
            "spine-hooks",
            "session-start-fields",
            "prompt-verbatim",
            "stop-message",
            "largest-hook-payload",
            "modal-signature",
            "input-box-signature",
            "quiet-period",
            "confirm-window",
            "question-answer",
            "plan-approve-revise",
            "question-notes",
            "dialog-concurrency",
            "long-paste-wrapper",
            "tag-escaping",
            "local-command-clear",
            "paste-newline",
            "statusline-override",
            "statusline-shell",
            "statusline-rate-limits",
            "resume-session",
            "agents-join",
            "local-command-compact",
        ]
    );
    let words: Vec<&str> = LedgerRow::ALL.iter().map(|r| r.words()).collect();
    assert_eq!(
        words,
        [
            "claude resolves to a real executable",
            "spine hooks fire through the plugin dir",
            "SessionStart carries session_id and source",
            "UserPromptSubmit carries the prompt as sent",
            "Stop carries last_assistant_message",
            "every hook payload fits the frame cap",
            "an untrusted start shows a compiled modal literal",
            "a trusted start shows a compiled input-box literal and no modal",
            "the screen settles within the gate's maximum wait",
            "the typed prompt reaches UserPromptSubmit within the window",
            "a question answered through PreToolUse takes effect",
            "a plan revise and approve each take effect",
            "free text and notes reach the question",
            "two parallel questions each raise a dialog",
            "a long paste unwraps to the text as pasted",
            "tag-like text un-escapes to the text as pasted",
            "/clear starts a new session and submits no prompt",
            "an LF inside a paste stays inside the one prompt submitted",
            "a --settings statusLine runs in the status line's place",
            "a statusline command runs through /bin/sh",
            "the statusline payload carries two whole-number rate_limits windows",
            "--resume <id> reopens the session under that id",
            "claude agents --json names the live session by pid and session id",
            "/compact keeps the session id and submits no prompt",
        ]
    );
    assert!(words.iter().all(|w| w.is_ascii()));
}

#[test]
fn relied_rows_are_every_row_in_order() {
    let here: Vec<&str> = LedgerRow::relied().into_iter().map(LedgerRow::id).collect();
    let all: Vec<&str> = LedgerRow::ALL.into_iter().map(LedgerRow::id).collect();
    assert_eq!(here.len(), 24);
    assert_eq!(here, all);
    assert_eq!(here.last(), Some(&"local-command-compact"));
}

#[test]
fn probe_prompt_is_fixed_ascii_without_tags() {
    assert_eq!(
        PROBE_PROMPT,
        "viola verify probe: reply with the single word ok"
    );
    assert!(!PROBE_PROMPT.contains('<') && !PROBE_PROMPT.contains('>'));
}

#[rstest]
#[case::claude(b"2.1.283 (Claude Code)\n", Some("2.1.283"))]
#[case::no_newline(b"2.1.0 (Claude Code)", Some("2.1.0"))]
#[case::crlf(b"10.20.30 (Claude Code)\r\n", Some("10.20.30"))]
#[case::trailing_space(b"2.1.0 (Claude Code)  \nmore", Some("2.1.0"))]
#[case::later_line_ignored(b"2.1.0 (Claude Code)\n\xff\xfe", Some("2.1.0"))]
#[case::viola(b"viola 0.1.0\n", None)]
#[case::garbage(b"garbage (Claude Code)\n", None)]
#[case::two_fields(b"2.1 (Claude Code)\n", None)]
#[case::four_fields(b"2.1.0.1 (Claude Code)\n", None)]
#[case::empty_field(b"2..0 (Claude Code)\n", None)]
#[case::letter_field(b"2.1.x (Claude Code)\n", None)]
#[case::leading_space(b" 2.1.0 (Claude Code)\n", None)]
#[case::prefix(b"v2.1.0 (Claude Code)\n", None)]
#[case::other_product(b"2.1.0 (Other)\n", None)]
#[case::second_line(b"\n2.1.0 (Claude Code)\n", None)]
#[case::empty(b"", None)]
#[case::not_utf8(b"2.1.\xff (Claude Code)\n", None)]
fn parse_version_takes_only_the_documented_first_line(
    #[case] stdout: &[u8],
    #[case] expected: Option<&str>,
) {
    assert_eq!(parse_version(stdout).as_deref(), expected);
}

#[test]
fn capture_plugin_files_register_the_spine_on_the_pinned_path() {
    let files = capture_plugin_files(
        "C:/h/bin/0.1.0-0123456789abcdef/viola.exe",
        "C:/h/ledger/probes/7/captures",
        ProbePlugin::Spine,
    );
    assert_eq!(files[0].0, ".claude-plugin/plugin.json");
    assert_eq!(
        files[0].1,
        "{\n  \"name\": \"viola-verify-probe\",\n  \"version\": \"0.1.0\",\n  \
             \"description\": \"viola verify: records the probe's raw hook payloads\"\n}\n"
    );
    assert_eq!(files[1].0, "hooks/hooks.json");
    assert_eq!(files[1].1, PROBE_HOOKS_JSON);
}

#[test]
fn capture_plugin_files_escape_a_path_as_json() {
    let files = capture_plugin_files("C:/a \"q\"/viola.exe", "C:/c", ProbePlugin::Spine);
    let doc: Value = serde_json::from_str(&files[1].1).expect("json");
    assert_eq!(
        doc["hooks"]["Stop"][0]["hooks"][0]["command"],
        "C:/a \"q\"/viola.exe"
    );
}

#[test]
fn event_names_are_the_cli_pascal_names() {
    let names: Vec<&str> = HookEvent::ALL.into_iter().map(event_name).collect();
    assert_eq!(
        names,
        [
            "SessionStart",
            "UserPromptSubmit",
            "Stop",
            "SessionEnd",
            "Notification",
            "PostToolUse",
            "PostToolUseFailure",
            "PreToolUse",
            "PermissionRequest",
        ]
    );
}

#[rstest]
#[case::first("SessionStart.1.json", Some((HookEvent::SessionStart, 1)))]
#[case::later("PostToolUseFailure.12.json", Some((HookEvent::PostToolUseFailure, 12)))]
#[case::zero("Stop.0.json", None)]
#[case::no_k("Stop.json", None)]
#[case::dialog_event("PreToolUse.1.json", Some((HookEvent::PreToolUse, 1)))]
#[case::unknown_event("Statusline.1.json", None)]
#[case::not_json("Stop.1.txt", None)]
#[case::not_a_number("Stop.x.json", None)]
#[case::kebab("session-start.1.json", None)]
fn parse_capture_file_name_reads_event_and_k(
    #[case] name: &str,
    #[case] expected: Option<(HookEvent, u32)>,
) {
    assert_eq!(parse_capture_file_name(name), expected);
}

#[test]
fn capture_file_name_round_trips() {
    let name = capture_file_name(HookEvent::UserPromptSubmit, 3);
    assert_eq!(name, "UserPromptSubmit.3.json");
    assert_eq!(
        parse_capture_file_name(&name),
        Some((HookEvent::UserPromptSubmit, 3))
    );
}

#[test]
fn capture_read_keeps_the_size_and_a_parsed_payload() {
    let c = Capture::read(HookEvent::Stop, b"{\"a\":1}");
    assert_eq!(c.bytes, 7);
    assert_eq!(c.payload, Some(json!({"a": 1})));
    assert_eq!(Capture::read(HookEvent::Stop, b"not json").payload, None);
}

#[test]
fn check_passes_every_row_on_a_clean_probe() {
    assert_eq!(verdicts(&clean_run()), [true; 24]);
}

#[test]
fn check_shim_resolution_needs_a_real_executable_that_answered() {
    let silent = ProbeRun {
        version_answered: false,
        ..clean_run()
    };
    assert!(!check(LedgerRow::ShimResolution, &probes(&silent)));
}

#[test]
fn check_spine_hooks_needs_the_four_in_order_each_naming_itself() {
    let mut missing = clean_run();
    missing.captures.remove(2);
    assert!(!check(LedgerRow::SpineHooks, &probes(&missing)));

    let mut swapped = clean_run();
    swapped.captures.swap(1, 2);
    assert!(!check(LedgerRow::SpineHooks, &probes(&swapped)));

    let mut doubled = clean_run();
    let again = doubled.captures[3].clone();
    doubled.captures.push(again);
    assert!(!check(LedgerRow::SpineHooks, &probes(&doubled)));

    let mut misnamed = clean_run();
    misnamed.captures[0] = capture(HookEvent::SessionStart, json!({"hook_event_name": "Stop"}));
    assert!(!check(LedgerRow::SpineHooks, &probes(&misnamed)));

    let mut unparsed = clean_run();
    unparsed.captures[3] = Capture::read(HookEvent::SessionEnd, b"not json");
    assert!(!check(LedgerRow::SpineHooks, &probes(&unparsed)));

    let mut not_object = clean_run();
    not_object.captures[3] = capture(HookEvent::SessionEnd, json!(["SessionEnd"]));
    assert!(!check(LedgerRow::SpineHooks, &probes(&not_object)));
}

#[rstest]
#[case::empty_session(json!({"session_id": "", "source": "startup"}))]
#[case::no_session(json!({"source": "startup"}))]
#[case::number_session(json!({"session_id": 1, "source": "startup"}))]
#[case::resume(json!({"session_id": "s", "source": "resume"}))]
#[case::no_source(json!({"session_id": "s"}))]
fn check_session_start_fields_fails_without_both_fields(#[case] payload: Value) {
    let mut run = clean_run();
    run.captures[0] = capture(HookEvent::SessionStart, payload);
    assert!(!check(LedgerRow::SessionStartFields, &probes(&run)));
    run.captures.remove(0);
    assert!(!check(LedgerRow::SessionStartFields, &probes(&run)));
}

#[test]
fn check_prompt_verbatim_needs_the_exact_prompt() {
    let mut run = clean_run();
    run.captures[1] = capture(
        HookEvent::UserPromptSubmit,
        json!({"prompt": format!("{PROBE_PROMPT} ")}),
    );
    assert!(!check(LedgerRow::PromptVerbatim, &probes(&run)));
    run.captures.remove(1);
    assert!(!check(LedgerRow::PromptVerbatim, &probes(&run)));
}

#[test]
fn check_stop_message_needs_a_string() {
    let mut run = clean_run();
    run.captures[2] = capture(HookEvent::Stop, json!({"last_assistant_message": null}));
    assert!(!check(LedgerRow::StopMessage, &probes(&run)));
    run.captures.remove(2);
    assert!(!check(LedgerRow::StopMessage, &probes(&run)));
}

#[test]
fn check_largest_hook_payload_holds_at_the_cap_and_fails_past_it() {
    let cap = usize::try_from(MAX_FRAME).expect("fits");
    let mut run = clean_run();
    run.captures[1].bytes = cap;
    assert!(check(LedgerRow::LargestHookPayload, &probes(&run)));
    run.captures[1].bytes = cap + 1;
    assert!(!check(LedgerRow::LargestHookPayload, &probes(&run)));
}

#[test]
fn check_reads_the_first_capture_of_an_event() {
    let mut run = clean_run();
    run.captures.push(capture(
        HookEvent::Stop,
        json!({"last_assistant_message": 1}),
    ));
    assert!(check(LedgerRow::StopMessage, &probes(&run)));
}

#[test]
fn largest_keeps_the_biggest_per_event_in_arrival_order() {
    let sized = |event, bytes| Capture {
        event,
        bytes,
        payload: None,
    };
    let got = largest(&[
        sized(HookEvent::Stop, 5),
        sized(HookEvent::SessionStart, 9),
        sized(HookEvent::Stop, 7),
        sized(HookEvent::Stop, 6),
    ]);
    assert_eq!(got, [(HookEvent::Stop, 7), (HookEvent::SessionStart, 9)]);
    assert!(largest(&[]).is_empty());
}

#[test]
fn merge_stamp_writes_the_envelope_for_a_first_version() {
    let bytes = merge_stamp(
        None,
        "2.1.0",
        &all_pass(),
        &[(HookEvent::SessionStart, 120), (HookEvent::Stop, 90)],
        &clean_typed(),
        Some(false),
        "2026-09-28T10:00:00.000Z",
    );
    assert_eq!(
        doc(&bytes),
        json!({
            "v": 1,
            "written_at": "2026-09-28T10:00:00.000Z",
            "writer": "verify",
            "data": {"versions": {"2.1.0": {
                "verified_at": "2026-09-28T10:00:00.000Z",
                "rows": {
                    "shim-resolution": "pass", "spine-hooks": "pass",
                    "session-start-fields": "pass", "prompt-verbatim": "pass",
                    "stop-message": "pass", "largest-hook-payload": "pass",
                    "modal-signature": "pass", "input-box-signature": "pass",
                    "quiet-period": "pass", "confirm-window": "pass",
                    "question-answer": "pass", "plan-approve-revise": "pass",
                    "question-notes": "pass", "dialog-concurrency": "pass",
                    "long-paste-wrapper": "pass", "tag-escaping": "pass",
                    "local-command-clear": "pass", "paste-newline": "pass",
                    "statusline-override": "pass", "statusline-shell": "pass",
                    "statusline-rate-limits": "pass", "resume-session": "pass",
                    "agents-join": "pass", "local-command-compact": "pass",
                },
                "measured": {
                    "largest_hook_payload": {"SessionStart": 120, "Stop": 90},
                    "typed_probe": {
                        "ready_settle_ms": 1084, "turn_settle_ms": 314,
                        "prompt_latency_ms": 31, "max_turn_gap_ms": 224,
                        "compact_latency_ms": 18_779,
                        "box_only_guards": 2,
                    },
                    "dialog_probe": {"parallel_both_before_first_post": false},
                },
            }}},
        })
    );
    assert_eq!(verified(&bytes, "2.1.0"), Ok(true));
}

#[test]
fn merge_stamp_keeps_unknown_versions_and_fields_and_replaces_its_own() {
    let existing = json!({
        "v": 1, "written_at": "old", "writer": "verify", "later": true,
        "data": {"note": "kept", "versions": {
            "3.0.0": {"verified_at": "old", "rows": {"future-row": "pass"}},
            "2.1.0": {"verified_at": "old", "rows": {"spine-hooks": "pass"}, "extra": 1},
        }},
    })
    .to_string();
    let mut results = all_pass();
    results[4].1 = false;
    let bytes = merge_stamp(
        Some(existing.as_bytes()),
        "2.1.0",
        &results,
        &[],
        &TypedRun::default(),
        None,
        "new",
    );
    let d = doc(&bytes);
    assert_eq!(d["later"], true);
    assert_eq!(d["written_at"], "new");
    assert_eq!(d["data"]["note"], "kept");
    assert_eq!(d["data"]["versions"]["3.0.0"]["rows"]["future-row"], "pass");
    let own = &d["data"]["versions"]["2.1.0"];
    assert_eq!(own["verified_at"], "new");
    assert_eq!(own["rows"]["stop-message"], "fail");
    assert_eq!(own["rows"]["spine-hooks"], "pass");
    assert!(own.get("extra").is_none());
    assert_eq!(
        own["measured"],
        json!({"largest_hook_payload": {}, "typed_probe": {
                "ready_settle_ms": null, "turn_settle_ms": null,
                "prompt_latency_ms": null, "max_turn_gap_ms": null,
                "compact_latency_ms": null, "box_only_guards": null,
            }, "dialog_probe": {"parallel_both_before_first_post": null}})
    );
    assert_eq!(verified(&bytes, "2.1.0"), Ok(false));
}

#[rstest]
#[case::not_json(b"not json".as_slice())]
#[case::array(b"[1]".as_slice())]
#[case::data_not_object(br#"{"v":1,"data":[1]}"#.as_slice())]
#[case::versions_not_object(br#"{"v":1,"data":{"versions":1},"keep":1}"#.as_slice())]
fn merge_stamp_replaces_bytes_of_the_wrong_shape_whole(#[case] existing: &[u8]) {
    let bytes = merge_stamp(
        Some(existing),
        "2.1.0",
        &all_pass(),
        &[],
        &TypedRun::default(),
        None,
        "t",
    );
    let d = doc(&bytes);
    assert!(d.get("keep").is_none());
    assert_eq!(verified(&bytes, "2.1.0"), Ok(true));
}

#[test]
fn merge_stamp_fills_a_data_object_without_versions() {
    let existing = br#"{"v":1,"data":{"note":1}}"#;
    let bytes = merge_stamp(
        Some(existing),
        "2.1.0",
        &all_pass(),
        &[],
        &TypedRun::default(),
        None,
        "t",
    );
    let d = doc(&bytes);
    assert_eq!(d["data"]["note"], 1);
    assert_eq!(verified(&bytes, "2.1.0"), Ok(true));
}

#[test]
fn verified_needs_every_row_pass_under_that_version() {
    let mut results = all_pass();
    let full = merge_stamp(
        None,
        "2.1.0",
        &results,
        &[],
        &TypedRun::default(),
        None,
        "t",
    );
    assert_eq!(verified(&full, "2.1.0"), Ok(true));
    assert_eq!(verified(&full, "3.0.0"), Ok(false));
    let last = *LedgerRow::relied().last().expect("a relied row");
    results.retain(|(row, _)| *row != last);
    let short = merge_stamp(
        None,
        "2.1.0",
        &results,
        &[],
        &TypedRun::default(),
        None,
        "t",
    );
    assert_eq!(verified(&short, "2.1.0"), Ok(false), "a missing row");
}

/// A stamp written before the three framing rows holds the fourteen older ids alone, one written
/// before the newline row every id but that one, and one written before the resume row the
/// twenty-one rows before it: each `pass`, and the version reads unverified until every row
/// relied on here passes.
#[test]
fn verified_reads_a_fourteen_row_stamp_as_unverified() {
    let rows: Map<String, Value> = [
        "shim-resolution",
        "spine-hooks",
        "session-start-fields",
        "prompt-verbatim",
        "stop-message",
        "largest-hook-payload",
        "modal-signature",
        "input-box-signature",
        "quiet-period",
        "confirm-window",
        "question-answer",
        "plan-approve-revise",
        "question-notes",
        "dialog-concurrency",
    ]
    .into_iter()
    .map(|id| (id.to_owned(), json!("pass")))
    .collect();
    let mut stamp = json!({"v": 1, "data": {"versions": {"2.1.0": {"rows": rows}}}});
    assert_eq!(verified(stamp.to_string().as_bytes(), "2.1.0"), Ok(false));
    for id in ["long-paste-wrapper", "tag-escaping", "local-command-clear"] {
        stamp["data"]["versions"]["2.1.0"]["rows"][id] = json!("pass");
    }
    assert_eq!(
        verified(stamp.to_string().as_bytes(), "2.1.0"),
        Ok(false),
        "every row but the newline one"
    );
    stamp["data"]["versions"]["2.1.0"]["rows"]["paste-newline"] = json!("pass");
    assert_eq!(
        verified(stamp.to_string().as_bytes(), "2.1.0"),
        Ok(false),
        "the eighteen rows before the statusline rows"
    );
    let mut nineteen = stamp.clone();
    nineteen["data"]["versions"]["2.1.0"]["rows"]["resume-session"] = json!("pass");
    assert_eq!(
        verified(nineteen.to_string().as_bytes(), "2.1.0"),
        Ok(false),
        "the eighteen rows and the resume row, before the agents row"
    );
    nineteen["data"]["versions"]["2.1.0"]["rows"]["agents-join"] = json!("pass");
    assert_eq!(
        verified(nineteen.to_string().as_bytes(), "2.1.0"),
        Ok(false),
        "the eighteen rows, the resume row and the agents row, before the compact row"
    );
    nineteen["data"]["versions"]["2.1.0"]["rows"]["local-command-compact"] = json!("pass");
    assert_eq!(
        verified(nineteen.to_string().as_bytes(), "2.1.0"),
        Ok(false),
        "the eighteen rows, the resume row, the agents row and the compact row"
    );
    for id in [
        "statusline-override",
        "statusline-shell",
        "statusline-rate-limits",
    ] {
        stamp["data"]["versions"]["2.1.0"]["rows"][id] = json!("pass");
    }
    assert_eq!(
        verified(stamp.to_string().as_bytes(), "2.1.0"),
        Ok(false),
        "the twenty-one rows before the resume row"
    );
    stamp["data"]["versions"]["2.1.0"]["rows"]["resume-session"] = json!("pass");
    assert_eq!(
        verified(stamp.to_string().as_bytes(), "2.1.0"),
        Ok(false),
        "the twenty-two rows before the agents row"
    );
    stamp["data"]["versions"]["2.1.0"]["rows"]["agents-join"] = json!("pass");
    assert_eq!(
        verified(stamp.to_string().as_bytes(), "2.1.0"),
        Ok(false),
        "the twenty-three rows of an older stamp"
    );
    stamp["data"]["versions"]["2.1.0"]["rows"]["local-command-compact"] = json!("pass");
    assert_eq!(verified(stamp.to_string().as_bytes(), "2.1.0"), Ok(true));
}

/// Every statusline row is relied on, so a statusline row that failed leaves the version
/// unverified.
#[test]
fn verified_reads_a_failed_statusline_row_by_the_relied_set() {
    let results: Vec<(LedgerRow, bool)> = LedgerRow::ALL
        .iter()
        .map(|r| (*r, *r != LedgerRow::StatusLineShell))
        .collect();
    let stamp = merge_stamp(
        None,
        "2.1.0",
        &results,
        &[],
        &TypedRun::default(),
        None,
        "t",
    );
    assert_eq!(verified(&stamp, "2.1.0"), Ok(false));
}

#[rstest]
#[case::not_json(b"not json".as_slice())]
#[case::array(b"[]".as_slice())]
#[case::no_v(br#"{"data":{"versions":{}}}"#.as_slice())]
#[case::v2(br#"{"v":2,"data":{"versions":{}}}"#.as_slice())]
#[case::data_not_object(br#"{"v":1,"data":1}"#.as_slice())]
#[case::versions_not_object(br#"{"v":1,"data":{"versions":[]}}"#.as_slice())]
fn verified_refuses_a_malformed_envelope(#[case] bytes: &[u8]) {
    assert_eq!(verified(bytes, "2.1.0"), Err(AgentError::StampsMalformed));
}

#[test]
fn verified_reads_an_envelope_without_data_as_unverified() {
    assert_eq!(verified(br#"{"v":1}"#, "2.1.0"), Ok(false));
    assert_eq!(
        AgentError::StampsMalformed.to_string(),
        "the capability stamps are malformed"
    );
}
