use super::*;
use rstest::rstest;

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
fn relied_rows_drop_the_three_statusline_rows_where_the_status_line_is_not_wrapped() {
    let ids = |wrapped: bool| -> Vec<&str> {
        LedgerRow::ALL
            .into_iter()
            .filter(|r| r.relied_on(wrapped))
            .map(LedgerRow::id)
            .collect()
    };
    assert_eq!(ids(true).len(), 24);
    assert_eq!(ids(false).len(), 21);
    assert_eq!(ids(false)[17], "paste-newline");
    assert_eq!(ids(false)[18], "resume-session");
    assert_eq!(ids(false)[19], "agents-join");
    assert_eq!(ids(false).last(), Some(&"local-command-compact"));
    assert_eq!(ids(true).last(), Some(&"local-command-compact"));
    assert!(!ids(false).iter().any(|id| id.starts_with("statusline-")));
    let here: Vec<&str> = LedgerRow::relied().into_iter().map(LedgerRow::id).collect();
    assert_eq!(here.len(), if cfg!(unix) { 24 } else { 21 });
    assert_eq!(here, ids(cfg!(unix)));
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

/// The rendered probe plugin, written out as the oracle.
const PROBE_HOOKS_JSON: &str = r#"{
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "C:/h/bin/0.1.0-0123456789abcdef/viola.exe",
            "args": [
              "hook",
              "session-start",
              "--capture",
              "C:/h/ledger/probes/7/captures"
            ],
            "timeout": 5
          }
        ]
      }
    ],
    "UserPromptSubmit": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "C:/h/bin/0.1.0-0123456789abcdef/viola.exe",
            "args": [
              "hook",
              "user-prompt-submit",
              "--capture",
              "C:/h/ledger/probes/7/captures"
            ],
            "timeout": 5
          }
        ]
      }
    ],
    "Stop": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "C:/h/bin/0.1.0-0123456789abcdef/viola.exe",
            "args": [
              "hook",
              "stop",
              "--capture",
              "C:/h/ledger/probes/7/captures"
            ],
            "timeout": 5
          }
        ]
      }
    ],
    "SessionEnd": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "C:/h/bin/0.1.0-0123456789abcdef/viola.exe",
            "args": [
              "hook",
              "session-end",
              "--capture",
              "C:/h/ledger/probes/7/captures"
            ]
          }
        ]
      }
    ]
  }
}
"#;

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

fn capture(event: HookEvent, payload: Value) -> Capture {
    Capture::read(event, payload.to_string().as_bytes())
}

/// The four spine captures a clean probe leaves, in firing order.
fn clean_captures() -> Vec<Capture> {
    vec![
        capture(
            HookEvent::SessionStart,
            json!({"hook_event_name": "SessionStart", "session_id": "s-1", "source": "startup"}),
        ),
        capture(
            HookEvent::UserPromptSubmit,
            json!({"hook_event_name": "UserPromptSubmit", "prompt": PROBE_PROMPT}),
        ),
        capture(
            HookEvent::Stop,
            json!({"hook_event_name": "Stop", "last_assistant_message": "ok"}),
        ),
        capture(
            HookEvent::SessionEnd,
            json!({"hook_event_name": "SessionEnd", "reason": "other"}),
        ),
    ]
}

fn clean_run() -> ProbeRun {
    ProbeRun {
        program_is_script: false,
        version_answered: true,
        captures: clean_captures(),
    }
}

/// A clean trusted and untrusted pair of interactive runs.
fn clean_typed() -> TypedRun {
    TypedRun {
        modal: Some(rows(&["", " ❯ No, exit", "   Yes, I trust this folder"])),
        ready: Some(rows(&["❯ Try it", "  ⏸ manual mode on · ← for agents"])),
        turn: Some(rows(&["● ok", "  ⏸ manual mode on · ← for agents"])),
        long_turn: Some(rows(&["● long", "  ⏸ manual mode on · ← for agents"])),
        ready_settle_ms: Some(1084),
        turn_settle_ms: Some(314),
        prompt_latency_ms: Some(31),
        max_turn_gap_ms: Some(224),
        compact_latency_ms: Some(18_779),
    }
}

fn rows(list: &[&str]) -> Vec<String> {
    list.iter().map(|r| (*r).to_owned()).collect()
}

fn probes(run: &ProbeRun) -> Probes {
    Probes {
        print: run.clone(),
        typed: clean_typed(),
        trusted: clean_trusted(),
        dialogs: clean_dialogs(),
        statusline: clean_statusline(),
        resumed: clean_resumed(),
        agents: Some(clean_agents()),
    }
}

fn verdicts(run: &ProbeRun) -> Vec<bool> {
    LedgerRow::ALL
        .iter()
        .map(|row| check(*row, &probes(run)))
        .collect()
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
    let script = ProbeRun {
        program_is_script: true,
        ..clean_run()
    };
    assert!(!check(LedgerRow::ShimResolution, &probes(&script)));
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

fn all_pass() -> Vec<(LedgerRow, bool)> {
    LedgerRow::ALL.iter().map(|r| (*r, true)).collect()
}

fn doc(bytes: &[u8]) -> Value {
    serde_json::from_slice(bytes).expect("json")
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
                "compact_latency_ms": null,
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
        Ok(!cfg!(unix)),
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

/// Windows relies on no statusline row, so a statusline row that failed there leaves the
/// version verified; on Unix it does not.
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
    assert_eq!(verified(&stamp, "2.1.0"), Ok(!cfg!(unix)));
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

#[rstest]
#[case::backslash_home(r"C:\Users\Plantuser\x", r"C:\Users\Plantuser", "~\\x")]
#[case::slash_spelling(r"C:/Users/Plantuser/x", r"C:\Users\Plantuser", "~/x")]
#[case::unix_home("/home/plantuser/p/q", "/home/plantuser", "~/p/q")]
#[case::twice("/home/plantuser/a:/home/plantuser/b", "/home/plantuser", "~/a:~/b")]
#[case::not_home("/srv/x", "/home/plantuser", "/srv/x")]
fn scrub_rewrites_the_home_in_both_spellings(
    #[case] text: &str,
    #[case] home: &str,
    #[case] expected: &str,
) {
    assert_eq!(scrub(&json!(text), home, "", false), json!(expected));
}

#[test]
fn scrub_folds_the_home_case_only_when_asked() {
    let text = json!(r"c:\users\plantuser\x");
    assert_eq!(scrub(&text, r"C:\Users\Plantuser", "", true), json!("~\\x"));
    assert_eq!(scrub(&text, r"C:\Users\Plantuser", "", false), text);
}

#[rstest]
#[case::projects_dir(
    r"~\.claude\projects\C--Users-plantuser--viola\s.jsonl",
    r"~\.claude\projects\C--Users-<user>--viola\s.jsonl"
)]
#[case::other_case("by PlantUser today", "by <user> today")]
#[case::whole_word_only("plantusers xplantuser plant_user", "plantusers xplantuser plant_user")]
#[case::at_edges("plantuser", "<user>")]
#[case::underscore_joined("plantuser_x", "plantuser_x")]
fn scrub_rewrites_the_user_as_a_whole_word(#[case] text: &str, #[case] expected: &str) {
    assert_eq!(scrub(&json!(text), "", "plantuser", false), json!(expected));
}

#[test]
fn scrub_walks_keys_arrays_and_nested_objects_and_keeps_other_values() {
    let payload = json!({
        "cwd": "/home/plantuser/w",
        "plantuser": [1, true, null, {"deep": "/home/plantuser"}],
        "n": 2.5,
    });
    let got = scrub(&payload, "/home/plantuser", "plantuser", false);
    assert_eq!(
        got,
        json!({"cwd": "~/w", "<user>": [1, true, null, {"deep": "~"}], "n": 2.5})
    );
    assert!(is_clean(&got, "plantuser"));
    assert!(!is_clean(&payload, "plantuser"));
}

#[test]
fn scrub_with_no_home_and_no_user_changes_nothing() {
    let payload = json!({"cwd": "/home/plantuser/w"});
    assert_eq!(scrub(&payload, "", "", true), payload);
}

#[rstest]
#[case::drive_backslash(json!(r"D:\outside\x"))]
#[case::drive_slash(json!("d:/outside/x"))]
#[case::linux_home(json!("see /home/other/x"))]
#[case::macos_users(json!("see /Users/other/x"))]
#[case::windows_users(json!(r"at \Users\other\x"))]
#[case::user_word(json!({"note": "PLANTUSER wrote this"}))]
#[case::in_a_key(json!({"/home/x/y": 1}))]
#[case::in_an_array(json!(["ok", "C:/x"]))]
#[case::bare_drive(json!("C:/"))]
#[case::user_after_a_space(json!("by plantuser"))]
fn is_clean_refuses_a_path_or_the_user(#[case] value: Value) {
    assert!(!is_clean(&value, "plantuser"));
}

#[rstest]
#[case::tilde(json!(r"~\.viola-record\ledger"))]
#[case::drive_without_separator(json!("C:x"))]
#[case::digit_drive(json!("1:/x"))]
#[case::placeholder(json!("<user> ran it"))]
#[case::relative(json!("home/x"))]
#[case::other_values(json!([1, true, null]))]
#[case::partial_words(json!("plantusers xplantuser plant_user"))]
fn is_clean_passes_scrubbed_text(#[case] value: Value) {
    assert!(is_clean(&value, "plantuser"));
}

#[test]
fn is_clean_ignores_an_empty_or_placeholder_user() {
    assert!(is_clean(&json!("plantuser"), ""));
    assert!(is_clean(&json!("<user>"), "<user>"));
}

fn typed_check(row: LedgerRow, typed: TypedRun) -> bool {
    check(
        row,
        &Probes {
            print: clean_run(),
            typed,
            trusted: clean_trusted(),
            dialogs: clean_dialogs(),
            statusline: clean_statusline(),
            resumed: clean_resumed(),
            agents: Some(clean_agents()),
        },
    )
}

#[test]
fn check_typed_rows_fail_without_a_measurement() {
    for row in [
        LedgerRow::ModalSignature,
        LedgerRow::InputBoxSignature,
        LedgerRow::QuietPeriod,
        LedgerRow::ConfirmWindow,
        LedgerRow::LocalCommandCompact,
    ] {
        assert!(!typed_check(row, TypedRun::default()), "{}", row.id());
    }
    let six = verdicts_with(TypedRun::default());
    assert_eq!(
        six,
        [
            true, true, true, true, true, true, false, false, false, false, true, true, true, true,
            true, true, true, true, true, true, true, true, true, false
        ]
    );
}

fn verdicts_with(typed: TypedRun) -> Vec<bool> {
    LedgerRow::ALL
        .iter()
        .map(|row| typed_check(*row, typed.clone()))
        .collect()
}

#[rstest]
#[case::trust("   Yes, I trust this folder", true)]
#[case::imports("   Yes, allow external imports", true)]
#[case::input_box_only("  ← for agents", false)]
#[case::cancel_only(" ❯ No, exit", false)]
fn check_modal_signature_needs_a_modal_literal(#[case] row: &str, #[case] expected: bool) {
    let typed = TypedRun {
        modal: Some(rows(&["", row])),
        ..clean_typed()
    };
    assert_eq!(typed_check(LedgerRow::ModalSignature, typed), expected);
}

#[test]
fn check_input_box_signature_long_turn_holding_the_literal_passes() {
    let long_turn = TypedRun {
        long_turn: Some(rows(&["● long", "  ⏸ manual mode on · ← for agents"])),
        ..clean_typed()
    };
    assert!(typed_check(LedgerRow::InputBoxSignature, long_turn));
}

#[test]
fn check_input_box_signature_long_turn_holding_a_modal_fails() {
    for modal in [
        "   Yes, allow external imports",
        "   Yes, I trust this folder",
    ] {
        let long_turn = TypedRun {
            long_turn: Some(rows(&[modal, "  ← for agents"])),
            ..clean_typed()
        };
        assert!(
            !typed_check(LedgerRow::InputBoxSignature, long_turn),
            "{modal}"
        );
    }
}

#[test]
fn check_input_box_signature_long_turn_of_none_fails() {
    let long_turn = TypedRun {
        long_turn: None,
        ..clean_typed()
    };
    assert!(!typed_check(LedgerRow::InputBoxSignature, long_turn));
    let without = TypedRun {
        long_turn: Some(rows(&["● long"])),
        ..clean_typed()
    };
    assert!(!typed_check(LedgerRow::InputBoxSignature, without));
}

#[test]
fn check_input_box_signature_needs_both_screens_without_a_modal() {
    assert!(typed_check(LedgerRow::InputBoxSignature, clean_typed()));
    let no_turn = TypedRun {
        turn: None,
        ..clean_typed()
    };
    assert!(!typed_check(LedgerRow::InputBoxSignature, no_turn));
    let turn_without = TypedRun {
        turn: Some(rows(&["● ok"])),
        ..clean_typed()
    };
    assert!(!typed_check(LedgerRow::InputBoxSignature, turn_without));
    let ready_without = TypedRun {
        ready: Some(rows(&["❯ Try it"])),
        ..clean_typed()
    };
    assert!(!typed_check(LedgerRow::InputBoxSignature, ready_without));
    for modal in [
        "   Yes, allow external imports",
        "   Yes, I trust this folder",
    ] {
        let ready_modal = TypedRun {
            ready: Some(rows(&[modal, "  ← for agents"])),
            ..clean_typed()
        };
        assert!(
            !typed_check(LedgerRow::InputBoxSignature, ready_modal),
            "{modal}"
        );
        let turn_modal = TypedRun {
            turn: Some(rows(&["  ← for agents", modal])),
            ..clean_typed()
        };
        assert!(
            !typed_check(LedgerRow::InputBoxSignature, turn_modal),
            "{modal}"
        );
    }
}

#[rstest]
#[case::both_at_the_bound(Some(8500), Some(8500), true)]
#[case::ready_past(Some(8501), Some(10), false)]
#[case::turn_past(Some(10), Some(8501), false)]
#[case::ready_missing(None, Some(10), false)]
#[case::turn_missing(Some(10), None, false)]
fn check_quiet_period_bounds_both_settles(
    #[case] ready: Option<u64>,
    #[case] turn: Option<u64>,
    #[case] expected: bool,
) {
    let typed = TypedRun {
        ready_settle_ms: ready,
        turn_settle_ms: turn,
        ..clean_typed()
    };
    assert_eq!(typed_check(LedgerRow::QuietPeriod, typed), expected);
}

#[rstest]
#[case::at_the_window(Some(10_000), true)]
#[case::past_the_window(Some(10_001), false)]
#[case::zero(Some(0), true)]
#[case::missing(None, false)]
fn check_confirm_window_bounds_the_latency(#[case] ms: Option<u64>, #[case] expected: bool) {
    let typed = TypedRun {
        prompt_latency_ms: ms,
        ..clean_typed()
    };
    assert_eq!(typed_check(LedgerRow::ConfirmWindow, typed), expected);
}

#[test]
fn signature_rows_keep_only_literal_rows() {
    let live = rows(&[
        " Accessing workspace:",
        " /home/plantuser/work",
        " ❯ No, exit",
        "   Yes, I trust this folder",
        "  ⏸ manual mode on · ← for agents",
        "   Yes, allow external imports",
    ]);
    assert_eq!(
        signature_rows(&live),
        rows(&[
            "",
            "",
            "",
            "   Yes, I trust this folder",
            "  ⏸ manual mode on · ← for agents",
            "   Yes, allow external imports",
        ])
    );
}

fn clean(live: &[&str]) -> bool {
    let live = rows(live);
    screen_is_clean(
        &live,
        &signature_rows(&live),
        "/home/plantuser",
        "plantuser",
    )
}

#[test]
fn screen_is_clean_passes_a_measured_screen() {
    assert!(clean(&[
        " ▝▝   ▝▝   ~/dev/projects/viola/.viola-verify-7",
        "────────────────────────",
        "  ctx ? · Haiku 4.5",
        "  ⏸ manual mode on · ← for agents",
    ]));
    assert!(clean(&[
        " /home/plantuser/work/a-long-path",
        " ❯ No, exit",
        "   Yes, I trust this folder",
        " Enter to confirm · Esc to cancel",
    ]));
}

#[rstest]
#[case::home_path(&["  ← for agents /home/plantuser/w"])]
#[case::other_home(&["  ← for agents /home/other/w"])]
#[case::username(&["  ← for agents · plantuser"])]
#[case::email(&["  ← for agents · a.b@example.com"])]
#[case::user_split_above(&[" /x/plant", "user ← for agents"])]
#[case::user_split_below(&["  ← for agents plant", "user/x"])]
#[case::user_split_over_box_drawing(&["  ← for agents plant ──", "── user"])]
#[case::email_split_below(&["  ← for agents a.b@", "example.com"])]
fn screen_is_clean_refuses_a_kept_row_or_seam(#[case] live: &[&str]) {
    assert!(!clean(live));
}

#[test]
fn screen_is_clean_reads_no_dropped_row_alone() {
    assert!(clean(&[
        " /home/plantuser/work",
        "",
        "  ← for agents",
        "",
        " a.b@example.com",
    ]));
}

#[test]
fn unclean_names_the_first_check_that_fails() {
    assert_eq!(Unclean::HomePath.code(), "home-path");
    assert_eq!(Unclean::AbsolutePath.code(), "absolute-path");
    assert_eq!(Unclean::Username.code(), "username");
    assert_eq!(Unclean::Email.code(), "email");
    assert_eq!(
        unclean(&json!({"a": "ok", "b": "/home/other/x"}), "plantuser"),
        Some(Unclean::AbsolutePath)
    );
    assert_eq!(
        unclean(&json!(["by plantuser"]), "plantuser"),
        Some(Unclean::Username)
    );
    assert_eq!(
        unclean(&json!({"plantuser": 1}), "plantuser"),
        Some(Unclean::Username)
    );
    assert_eq!(unclean(&json!({"a": "~/x"}), "plantuser"), None);
}

fn fault(live: &[&str]) -> Option<ScreenFault> {
    let live = rows(live);
    screen_fault(&live, &signature_rows(&live), "/srv/plantuser", "plantuser")
}

#[test]
fn screen_fault_names_the_row_the_seam_and_the_check() {
    assert_eq!(fault(&["x", "  ← for agents"]), None);
    assert_eq!(
        fault(&["x", "  ← for agents /srv/plantuser/w"]),
        Some(ScreenFault {
            row: 1,
            seam: false,
            why: Unclean::HomePath
        })
    );
    assert_eq!(
        fault(&["  ← for agents /home/other/w"]),
        Some(ScreenFault {
            row: 0,
            seam: false,
            why: Unclean::AbsolutePath
        })
    );
    assert_eq!(
        fault(&["x", "y", "  ← for agents by plantuser"]),
        Some(ScreenFault {
            row: 2,
            seam: false,
            why: Unclean::Username
        })
    );
    assert_eq!(
        fault(&["  ← for agents a@example.com"]),
        Some(ScreenFault {
            row: 0,
            seam: false,
            why: Unclean::Email
        })
    );
    assert_eq!(
        fault(&["ctx plant", "user ← for agents"]),
        Some(ScreenFault {
            row: 1,
            seam: true,
            why: Unclean::Username
        })
    );
    assert_eq!(
        fault(&["  ← for agents a@", "example.com"]),
        Some(ScreenFault {
            row: 0,
            seam: true,
            why: Unclean::Email
        })
    );
}

#[rstest]
#[case::plain("a@example.com", true)]
#[case::dotted("x.y+z@mail.example.co", true)]
#[case::no_tld("a@localhost", false)]
#[case::numeric_tld("a@1.2", false)]
#[case::one_letter_tld("a@b.c", false)]
#[case::no_local("@example.com", false)]
#[case::no_host("a@.com", false)]
#[case::none("ctx 23% 45k/200k", false)]
#[case::space_before_the_at("see @example.com", false)]
#[case::hyphen_in_the_domain("a@my-host.example", true)]
fn has_email_reads_local_at_domain_dot_tld(#[case] text: &str, #[case] expected: bool) {
    assert_eq!(has_email(text), expected);
}

/// A captured tool event: `tool_name`, an optional `tool_use_id`, and `extra`'s fields.
fn tool(event: HookEvent, name: &str, id: Option<&str>, extra: Value) -> Capture {
    let mut payload = json!({"tool_name": name});
    if let Some(id) = id {
        payload["tool_use_id"] = json!(id);
    }
    if let (Some(payload), Value::Object(extra)) = (payload.as_object_mut(), extra) {
        payload.extend(extra);
    }
    capture(event, payload)
}

fn prompted(prompt: &str) -> Capture {
    capture(HookEvent::UserPromptSubmit, json!({"prompt": prompt}))
}

fn two_questions() -> Value {
    json!({"questions": [
        {"question": "Probe color?", "options": [{"label": "red"}, {"label": "blue"}]},
        {"question": "Probe size?", "options": [{"label": "small"}, {"label": "large"}]},
    ]})
}

fn one_question(text: &str) -> Value {
    json!({"questions": [{"question": text, "options": [{"label": "yes"}, {"label": "no"}]}]})
}

const PRE: HookEvent = HookEvent::PreToolUse;
const PR: HookEvent = HookEvent::PermissionRequest;
const POST: HookEvent = HookEvent::PostToolUse;

/// The dialog and plan runs as the live 2.1.288 probe recorded them, in claim order (structure
/// of `evidence/step0-dialog-shapes.md`).
fn clean_dialogs() -> DialogRuns {
    let stop = capture(HookEvent::Stop, json!({"last_assistant_message": "ok"}));
    let answered = json!({
        "tool_input": two_questions(),
        "tool_response": {
            "questions": two_questions()["questions"],
            "answers": {"Probe color?": "red", "Probe size?": PROBE_FREE_TEXT},
            "annotations": {"Probe color?": {"notes": PROBE_NOTE}},
        },
    });
    let reply = |q: &str| json!({"tool_response": {"answers": {q: "yes"}}});
    DialogRuns {
        questions: vec![
            capture(HookEvent::SessionStart, json!({"source": "startup"})),
            prompted(DIALOG_PROMPT_QUESTIONS),
            tool(
                PRE,
                "AskUserQuestion",
                Some("q1"),
                json!({"tool_input": two_questions()}),
            ),
            tool(POST, "AskUserQuestion", Some("q1"), answered),
            stop.clone(),
            prompted(DIALOG_PROMPT_PARALLEL),
            tool(
                PRE,
                "AskUserQuestion",
                Some("p1"),
                json!({"tool_input": one_question("Probe left?")}),
            ),
            tool(POST, "AskUserQuestion", Some("p1"), reply("Probe left?")),
            tool(
                PRE,
                "AskUserQuestion",
                Some("p2"),
                json!({"tool_input": one_question("Probe right?")}),
            ),
            tool(POST, "AskUserQuestion", Some("p2"), reply("Probe right?")),
            stop.clone(),
            prompted(DIALOG_PROMPT_PERMISSION),
            tool(
                PR,
                "Bash",
                None,
                json!({"tool_input": {"command": "touch viola-probe-permission"}}),
            ),
            tool(POST, "Bash", Some("b1"), json!({})),
            stop,
        ],
        plan: vec![
            prompted(PLAN_PROMPT),
            tool(POST, "Write", Some("w1"), json!({})),
            tool(
                PRE,
                "ExitPlanMode",
                Some("e1"),
                json!({"tool_input": {"plan": "p"}}),
            ),
            tool(
                PR,
                "ExitPlanMode",
                None,
                json!({"tool_input": {"plan": "p"}}),
            ),
            tool(POST, "Write", Some("w2"), json!({})),
            tool(
                PRE,
                "ExitPlanMode",
                Some("e2"),
                json!({"tool_input": {"plan": "p2"}}),
            ),
            tool(POST, "ExitPlanMode", Some("e2"), json!({})),
        ],
    }
}

const DIALOG_ROWS: [LedgerRow; 4] = [
    LedgerRow::QuestionAnswer,
    LedgerRow::PlanApproveRevise,
    LedgerRow::QuestionNotes,
    LedgerRow::DialogConcurrency,
];

fn dialog_check(row: LedgerRow, dialogs: DialogRuns) -> bool {
    check(
        row,
        &Probes {
            print: clean_run(),
            typed: clean_typed(),
            trusted: clean_trusted(),
            dialogs,
            statusline: clean_statusline(),
            resumed: clean_resumed(),
            agents: Some(clean_agents()),
        },
    )
}

#[test]
fn check_dialog_rows_pass_on_the_recorded_shape() {
    for row in DIALOG_ROWS {
        assert!(dialog_check(row, clean_dialogs()), "{}", row.id());
    }
}

/// Absence is never a pass: no run, or runs whose turns raised no dialog, fail all four.
#[test]
fn check_dialog_rows_fail_when_no_dialog_was_raised() {
    let stop = || capture(HookEvent::Stop, json!({}));
    let silent = DialogRuns {
        questions: vec![
            prompted(DIALOG_PROMPT_QUESTIONS),
            stop(),
            prompted(DIALOG_PROMPT_PARALLEL),
            stop(),
            prompted(DIALOG_PROMPT_PERMISSION),
            stop(),
        ],
        plan: vec![prompted(PLAN_PROMPT), stop()],
    };
    for dialogs in [DialogRuns::default(), silent] {
        for row in DIALOG_ROWS {
            assert!(!dialog_check(row, dialogs.clone()), "{}", row.id());
        }
    }
}

/// Run B's wrapped long prompt as 2.1.287 sent it (structure of `evidence/step0-shapes.md`).
fn wrapped(text: &str) -> String {
    format!("\n\n<pasted_content id=\"7ccf\">\n{text}\n</pasted_content id=\"7ccf\">\n")
}

/// The tag-like text as the CLI escapes it: both typed `pasted_content` tags, nothing else.
const TAG_PROMPT: &str = "viola verify probe: the next part is literal sample text and not \
         markup: <\\pasted_content id=\"1\"> sample <\\/pasted_content id=\"1\"> then \
         <task-notification> and that is all. Reply with the single word ok";

fn session_start(source: &str, id: &str) -> Capture {
    capture(
        HookEvent::SessionStart,
        json!({"source": source, "session_id": id}),
    )
}

fn session_end(reason: &str) -> Capture {
    capture(HookEvent::SessionEnd, json!({"reason": reason}))
}

/// The newline text as the CLI sends it: unchanged, its one LF kept (`rule-inner-lf`).
const NEWLINE_PROMPT: &str = "viola verify probe: this paste holds one line break here\nand \
         the next line ends it. Reply with the single word ok";

/// Run B as the live 2.1.287 probe recorded it, in claim order: the first turn, the long, the
/// tag-like and the newline turns, the SessionStart `/compact` fired, the two hooks the local
/// command fired, then the exit's SessionEnd.
fn clean_trusted() -> Vec<Capture> {
    let stop = capture(HookEvent::Stop, json!({"last_assistant_message": "ok"}));
    vec![
        session_start("startup", RUN_B_ID),
        prompted(PROBE_PROMPT),
        stop.clone(),
        prompted(&wrapped(PROBE_LONG_PASTE)),
        stop.clone(),
        prompted(TAG_PROMPT),
        stop.clone(),
        prompted(NEWLINE_PROMPT),
        stop,
        session_start("compact", RUN_B_ID),
        session_end("clear"),
        session_start("clear", "s-2"),
        session_end("prompt_input_exit"),
    ]
}

/// Run B's first session id, of the session-id shape the CLI gives one.
const RUN_B_ID: &str = "3f2b9c1e-7a4d-4e8b-9c0f-1d2e3a4b5c6d";

/// Run E as the live probe records it: the one SessionStart of the resumed session before the
/// kill.
fn clean_resumed() -> Vec<Capture> {
    vec![session_start("resume", RUN_B_ID)]
}

/// Run B's child pid in the agents read.
const RUN_B_PID: u32 = 4242;

fn agents_row(pid: Option<u32>, id: &str, status: &str) -> Value {
    let mut row = json!({
        "cwd": "~/p",
        "kind": "interactive",
        "name": "a title",
        "sessionId": id,
        "startedAt": 1_760_000_000_000_i64,
        "status": status,
    });
    if let Some(pid) = pid {
        row["pid"] = json!(pid);
    }
    row
}

fn agents_of(child_pid: Option<u32>, rows: &[Value]) -> AgentsCapture {
    AgentsCapture {
        child_pid,
        bytes: Value::from(rows.to_vec()).to_string().into_bytes(),
    }
}

/// The read as a host with another live session lists it: Run B's own row and one more.
fn clean_agents() -> AgentsCapture {
    agents_of(
        Some(RUN_B_PID),
        &[
            agents_row(Some(1), "0b7c9f4e-1d2a-4c3b-8e5f-6a7b8c9d0e1f", "busy"),
            agents_row(Some(RUN_B_PID), RUN_B_ID, "idle"),
        ],
    )
}

fn agents_check(agents: Option<AgentsCapture>) -> bool {
    check(
        LedgerRow::AgentsJoin,
        &Probes {
            agents,
            ..probes(&clean_run())
        },
    )
}

#[test]
fn check_agents_join_passes_on_the_recorded_shape() {
    assert!(agents_check(Some(clean_agents())));
    let busy = agents_of(
        Some(RUN_B_PID),
        &[agents_row(Some(RUN_B_PID), RUN_B_ID, "busy")],
    );
    assert!(agents_check(Some(busy)), "busy");
    let mut extra = agents_row(Some(RUN_B_PID), RUN_B_ID, "idle");
    extra["later"] = json!(1);
    assert!(
        agents_check(Some(agents_of(Some(RUN_B_PID), &[extra]))),
        "an extra key"
    );
}

#[test]
fn check_agents_join_fails_on_each_broken_post_condition() {
    let other = agents_row(Some(1), RUN_B_ID, "idle");
    for (label, rows) in [
        ("no row with the child pid", vec![other.clone()]),
        (
            "another session id",
            vec![agents_row(Some(RUN_B_PID), "s-2", "idle")],
        ),
        (
            "an unknown status",
            vec![agents_row(Some(RUN_B_PID), RUN_B_ID, "waiting")],
        ),
        ("no status", {
            let mut row = agents_row(Some(RUN_B_PID), RUN_B_ID, "idle");
            row.as_object_mut().expect("an object").remove("status");
            vec![row]
        }),
        (
            "two rows with the child pid",
            vec![
                agents_row(Some(RUN_B_PID), RUN_B_ID, "idle"),
                agents_row(Some(RUN_B_PID), RUN_B_ID, "busy"),
            ],
        ),
        (
            "a row by id alone",
            vec![agents_row(None, RUN_B_ID, "idle")],
        ),
        ("no row", Vec::new()),
    ] {
        assert!(
            !agents_check(Some(agents_of(Some(RUN_B_PID), &rows))),
            "{label}"
        );
    }
    let after_clear = agents_of(
        Some(RUN_B_PID),
        &[agents_row(Some(RUN_B_PID), "s-2", "idle")],
    );
    let cleared = Probes {
        agents: Some(after_clear),
        ..probes(&clean_run())
    };
    assert!(
        !check(LedgerRow::AgentsJoin, &cleared),
        "the id after the local command is not Run B's first"
    );
}

#[test]
fn check_agents_join_fails_with_no_capture() {
    assert!(!agents_check(None), "no read");
    let no_pid = AgentsCapture {
        child_pid: None,
        ..clean_agents()
    };
    assert!(!agents_check(Some(no_pid)), "no child pid");
    for bytes in [b"".as_slice(), b"not json", br#"{"pid":4242}"#] {
        let unparsed = AgentsCapture {
            child_pid: Some(RUN_B_PID),
            bytes: bytes.to_vec(),
        };
        assert!(!agents_check(Some(unparsed)), "{bytes:?}");
    }
    let no_run_b = Probes {
        trusted: Vec::new(),
        ..probes(&clean_run())
    };
    assert!(!check(LedgerRow::AgentsJoin, &no_run_b), "no Run B start");
}

#[test]
fn agents_variant_keeps_only_run_b_s_own_row() {
    assert_eq!(
        agents_variant(&clean_agents()),
        Some(json!([agents_row(Some(RUN_B_PID), RUN_B_ID, "idle")]))
    );
    let others = agents_of(Some(RUN_B_PID), &[agents_row(Some(1), RUN_B_ID, "idle")]);
    assert_eq!(agents_variant(&others), None);
    let no_pid = AgentsCapture {
        child_pid: None,
        ..clean_agents()
    };
    assert_eq!(agents_variant(&no_pid), None);
    let unparsed = AgentsCapture {
        child_pid: Some(RUN_B_PID),
        bytes: b"not json".to_vec(),
    };
    assert_eq!(agents_variant(&unparsed), None);
    assert_eq!(AGENTS_RECORDED, "Agents.recorded-1.json");
}

fn resume_check(trusted: Vec<Capture>, resumed: Vec<Capture>) -> bool {
    check(
        LedgerRow::ResumeSession,
        &Probes {
            print: clean_run(),
            typed: clean_typed(),
            trusted,
            dialogs: clean_dialogs(),
            statusline: clean_statusline(),
            resumed,
            agents: Some(clean_agents()),
        },
    )
}

#[test]
fn check_resume_session_passes_on_the_recorded_shape() {
    assert!(resume_check(clean_trusted(), clean_resumed()));
    let upper = vec![session_start(
        "resume",
        "3F2B9C1E-7A4D-4E8B-9C0F-1D2E3A4B5C6D",
    )];
    let trusted = with_trusted(|t| t[0] = session_start("startup", &RUN_B_ID.to_uppercase()));
    assert!(resume_check(trusted, upper), "either hex case");
    let ended = vec![
        session_start("resume", RUN_B_ID),
        session_end("prompt_input_exit"),
    ];
    assert!(
        resume_check(clean_trusted(), ended),
        "a SessionEnd after it"
    );
}

#[test]
fn check_resume_session_fails_on_another_source() {
    for source in ["startup", "clear", "compact", ""] {
        let resumed = vec![session_start(source, RUN_B_ID)];
        assert!(!resume_check(clean_trusted(), resumed), "{source}");
    }
    let unnamed = vec![capture(
        HookEvent::SessionStart,
        json!({"session_id": RUN_B_ID}),
    )];
    assert!(!resume_check(clean_trusted(), unnamed), "no source");
    let resume_second = vec![
        session_start("startup", RUN_B_ID),
        session_start("resume", RUN_B_ID),
    ];
    assert!(
        !resume_check(clean_trusted(), resume_second),
        "the first SessionStart decides"
    );
}

#[test]
fn check_resume_session_fails_on_another_id() {
    for id in ["3f2b9c1e-7a4d-4e8b-9c0f-1d2e3a4b5c6e", "s-2", ""] {
        let resumed = vec![session_start("resume", id)];
        assert!(!resume_check(clean_trusted(), resumed), "{id:?}");
    }
    let no_id = vec![capture(
        HookEvent::SessionStart,
        json!({"source": "resume"}),
    )];
    assert!(!resume_check(clean_trusted(), no_id), "no session_id");
    let cleared = with_trusted(|t| t[11] = session_start("clear", RUN_B_ID));
    let after_clear = vec![session_start("resume", "s-2")];
    assert!(
        !resume_check(cleared, after_clear),
        "the id after the local command is not the target"
    );
}

#[test]
fn check_resume_session_fails_with_no_run_e_capture() {
    assert!(!resume_check(clean_trusted(), Vec::new()), "no capture");
    let ended = vec![session_end("prompt_input_exit")];
    assert!(!resume_check(clean_trusted(), ended), "no SessionStart");
}

#[test]
fn check_resume_session_fails_when_run_b_first_id_is_off_the_session_id_shape() {
    for id in [
        "s-1",
        "",
        "3f2b9c1e-7a4d-4e8b-9c0f-1d2e3a4b5c6",
        "3f2b9c1e-7a4d-4e8b-9c0f-1d2e3a4b5c6dd",
        "3f2b9c1e_7a4d-4e8b-9c0f-1d2e3a4b5c6d",
        "3f2b9c1e-7a4d-4e8b-9c0f-1d2e3a4b5c6g",
    ] {
        let trusted = with_trusted(|t| t[0] = session_start("startup", id));
        let resumed = vec![session_start("resume", id)];
        assert!(!resume_check(trusted, resumed), "{id:?}");
    }
    assert!(!resume_check(Vec::new(), clean_resumed()), "no Run B");
    let resume_first = with_trusted(|t| t.insert(0, session_start("startup", "s-0")));
    assert!(
        !resume_check(resume_first, clean_resumed()),
        "Run B's first SessionStart decides the target"
    );
}

#[test]
fn resume_target_is_run_b_first_session_id_of_the_shape() {
    assert_eq!(resume_target(&clean_trusted()), Some(RUN_B_ID));
    assert_eq!(resume_target(&[]), None);
    let off = with_trusted(|t| t[0] = session_start("startup", "s-1"));
    assert_eq!(resume_target(&off), None);
}

#[test]
fn resume_variants_name_run_e_first_session_start_resume_1() {
    let resumed = vec![
        session_end("other"),
        session_start("resume", RUN_B_ID),
        session_start("startup", "s-9"),
    ];
    let variants = resume_variants(&resumed);
    assert_eq!(variants.len(), 1);
    assert_eq!(variants[0].0, "resume-1");
    assert_eq!(variants[0].1, &resumed[1]);
    assert!(resume_variants(&[session_end("other")]).is_empty());
    assert!(resume_variants(&[]).is_empty());
}

const FRAMING_ROWS: [LedgerRow; 5] = [
    LedgerRow::LongPasteWrapper,
    LedgerRow::TagEscaping,
    LedgerRow::LocalCommandClear,
    LedgerRow::PasteNewline,
    LedgerRow::LocalCommandCompact,
];

fn framing_check(row: LedgerRow, trusted: Vec<Capture>) -> bool {
    check(
        row,
        &Probes {
            print: clean_run(),
            typed: clean_typed(),
            trusted,
            dialogs: clean_dialogs(),
            statusline: clean_statusline(),
            resumed: clean_resumed(),
            agents: Some(clean_agents()),
        },
    )
}

fn with_trusted(edit: impl FnOnce(&mut Vec<Capture>)) -> Vec<Capture> {
    let mut trusted = clean_trusted();
    edit(&mut trusted);
    trusted
}

#[test]
fn check_framing_rows_pass_on_the_recorded_shape() {
    for row in FRAMING_ROWS {
        assert!(framing_check(row, clean_trusted()), "{}", row.id());
    }
}

/// Each paste row reads its own capture: no UserPromptSubmit at all fails it, and so does a
/// capture whose prompt does not normalise to the compiled text.
#[test]
fn check_paste_rows_fail_without_a_matching_capture() {
    let silent: Vec<Capture> = clean_trusted()
        .into_iter()
        .filter(|c| c.event != HookEvent::UserPromptSubmit)
        .collect();
    for (row, at, text) in [
        (LedgerRow::LongPasteWrapper, 3, PROBE_LONG_PASTE),
        (LedgerRow::TagEscaping, 5, PROBE_TAG_PASTE),
    ] {
        assert!(!framing_check(row, silent.clone()), "{} silent", row.id());
        assert!(!framing_check(row, Vec::new()), "{} no run", row.id());
        let other = with_trusted(|t| t[at] = prompted(&format!("{text} and more")));
        assert!(!framing_check(row, other), "{} another text", row.id());
        let framed = with_trusted(|t| t[at] = prompted(&format!("\n{text}")));
        assert!(!framing_check(row, framed), "{} a kept newline", row.id());
    }
    let unwrapped = with_trusted(|t| t[3] = prompted(PROBE_LONG_PASTE));
    assert!(framing_check(LedgerRow::LongPasteWrapper, unwrapped));
}

#[test]
fn check_paste_newline_passes_on_the_one_prompt_holding_the_lf() {
    let row = LedgerRow::PasteNewline;
    assert!(framing_check(row, clean_trusted()));
    let alone = vec![prompted(
        "viola verify probe: this paste holds one line break here\nand the next line ends \
             it. Reply with the single word ok",
    )];
    assert!(framing_check(row, alone));
}

/// A CLI that submitted at the LF fires two prompts, one per line, and neither is the text.
#[test]
fn check_paste_newline_fails_when_the_lf_split_the_prompt() {
    let row = LedgerRow::PasteNewline;
    let split = with_trusted(|t| {
        t[7] = prompted("viola verify probe: this paste holds one line break here");
        t.insert(
            8,
            prompted("and the next line ends it. Reply with the single word ok"),
        );
    });
    assert!(!framing_check(row, split));
    let joined = with_trusted(|t| {
        t[7] = prompted(
            "viola verify probe: this paste holds one line break here and the next line \
                 ends it. Reply with the single word ok",
        );
    });
    assert!(!framing_check(row, joined), "the LF dropped");
}

#[test]
fn check_paste_newline_fails_without_a_matching_capture() {
    let row = LedgerRow::PasteNewline;
    let silent: Vec<Capture> = clean_trusted()
        .into_iter()
        .filter(|c| c.event != HookEvent::UserPromptSubmit)
        .collect();
    assert!(!framing_check(row, silent), "silent");
    assert!(!framing_check(row, Vec::new()), "no run");
    let other = with_trusted(|t| t[7] = prompted(&format!("{NEWLINE_PROMPT} and more")));
    assert!(!framing_check(row, other), "another text");
    let crlf = with_trusted(|t| t[7] = prompted(&NEWLINE_PROMPT.replace('\n', "\r\n")));
    assert!(!framing_check(row, crlf), "a CR before the LF");
    let ended = with_trusted(|t| t[7] = prompted(&format!("{NEWLINE_PROMPT}\n")));
    assert!(!framing_check(row, ended), "a kept newline ending");
}

#[test]
fn check_local_command_clear_fails_on_each_broken_post_condition() {
    let row = LedgerRow::LocalCommandClear;
    let no_start = with_trusted(|t| {
        t.remove(11);
    });
    assert!(!framing_check(row, no_start), "no clear SessionStart");
    let other_source = with_trusted(|t| t[11] = session_start("resume", "s-2"));
    assert!(!framing_check(row, other_source), "another source");
    let same_session = with_trusted(|t| t[11] = session_start("clear", RUN_B_ID));
    assert!(!framing_check(row, same_session), "the first session_id");
    let empty_session = with_trusted(|t| t[11] = session_start("clear", ""));
    assert!(!framing_check(row, empty_session), "an empty session_id");
    let no_session = with_trusted(|t| {
        t[11] = capture(HookEvent::SessionStart, json!({"source": "clear"}));
    });
    assert!(!framing_check(row, no_session), "no session_id");
    let submitted = with_trusted(|t| t.insert(10, prompted(PROBE_LOCAL_COMMAND)));
    assert!(!framing_check(row, submitted), "the command as a prompt");
    let before_the_tag_turn = with_trusted(|t| {
        let start = t.remove(11);
        t.insert(5, start);
    });
    assert!(
        !framing_check(row, before_the_tag_turn),
        "a clear before the tag-like turn"
    );
    let no_tag_turn = with_trusted(|t| {
        t.remove(5);
    });
    assert!(!framing_check(row, no_tag_turn), "no tag-like turn");
    assert!(!framing_check(row, Vec::new()), "no run");
}

fn compact_check(trusted: Vec<Capture>, typed: TypedRun) -> bool {
    check(
        LedgerRow::LocalCommandCompact,
        &Probes {
            print: clean_run(),
            typed,
            trusted,
            dialogs: clean_dialogs(),
            statusline: clean_statusline(),
            resumed: clean_resumed(),
            agents: Some(clean_agents()),
        },
    )
}

#[test]
fn check_local_command_compact_passes_on_the_recorded_shape() {
    assert!(compact_check(clean_trusted(), clean_typed()));
    let at_the_bound = TypedRun {
        compact_latency_ms: Some(40_000),
        ..clean_typed()
    };
    assert!(compact_check(clean_trusted(), at_the_bound), "at the bound");
}

#[test]
fn check_local_command_compact_fails_on_each_broken_post_condition() {
    let typed = clean_typed;
    let other_id = with_trusted(|t| t[9] = session_start("compact", "s-2"));
    assert!(!compact_check(other_id, typed()), "another session_id");
    let empty_id = with_trusted(|t| t[9] = session_start("compact", ""));
    assert!(!compact_check(empty_id, typed()), "an empty session_id");
    let no_id = with_trusted(|t| {
        t[9] = capture(HookEvent::SessionStart, json!({"source": "compact"}));
    });
    assert!(!compact_check(no_id, typed()), "no session_id");
    let cleared = with_trusted(|t| t[9] = session_start("clear", RUN_B_ID));
    assert!(!compact_check(cleared, typed()), "source clear");
    let submitted = with_trusted(|t| t.insert(9, prompted(PROBE_COMPACT_COMMAND)));
    assert!(
        !compact_check(submitted, typed()),
        "the command as a prompt"
    );
    let before_the_newline_turn = with_trusted(|t| {
        let start = t.remove(9);
        t.insert(7, start);
    });
    assert!(
        !compact_check(before_the_newline_turn, typed()),
        "a compaction before the newline turn"
    );
    let past = TypedRun {
        compact_latency_ms: Some(40_001),
        ..clean_typed()
    };
    assert!(
        !compact_check(clean_trusted(), past),
        "a latency past the bound"
    );
    let unmeasured = TypedRun {
        compact_latency_ms: None,
        ..clean_typed()
    };
    assert!(!compact_check(clean_trusted(), unmeasured), "no latency");
}

#[test]
fn check_local_command_compact_fails_without_a_capture() {
    let no_start = with_trusted(|t| {
        t.remove(9);
    });
    assert!(
        !compact_check(no_start, clean_typed()),
        "no compact SessionStart"
    );
    let no_newline_turn = with_trusted(|t| {
        t.remove(7);
    });
    assert!(
        !compact_check(no_newline_turn, clean_typed()),
        "no newline turn"
    );
    assert!(!compact_check(Vec::new(), clean_typed()), "no run");
}

/// A status line payload as 2.1.287 hands one over after a turn: both windows, whole numbers.
fn limited(five_hour: Value, seven_day: Value) -> StatusLineCapture {
    let payload = json!({
        "session_id": "s-1",
        "model": {"id": "claude-haiku"},
        "rate_limits": {"five_hour": five_hour, "seven_day": seven_day},
    });
    StatusLineCapture {
        bytes: payload.to_string().into_bytes(),
        shell: ShellVerdict::BinSh,
    }
}

fn window(used: Value, resets_at: Value) -> Value {
    json!({"used_percentage": used, "resets_at": resets_at})
}

/// Run B's status line as the live 2.1.287 probe would file it: a first payload without
/// `rate_limits`, then one with both windows, each run by `/bin/sh`.
fn clean_statusline() -> Vec<StatusLineCapture> {
    let first = json!({"session_id": "s-1", "model": {"id": "claude-haiku"}});
    vec![
        StatusLineCapture {
            bytes: first.to_string().into_bytes(),
            shell: ShellVerdict::BinSh,
        },
        limited(
            window(json!(23), json!(1_760_400_000)),
            window(json!(91), json!(1_760_900_000)),
        ),
    ]
}

fn statusline_check(row: LedgerRow, statusline: Vec<StatusLineCapture>) -> bool {
    check(
        row,
        &Probes {
            print: clean_run(),
            typed: clean_typed(),
            trusted: clean_trusted(),
            dialogs: clean_dialogs(),
            statusline,
            resumed: clean_resumed(),
            agents: Some(clean_agents()),
        },
    )
}

#[test]
fn check_statusline_override_passes_on_a_filed_payload() {
    let row = LedgerRow::StatusLineOverride;
    assert!(statusline_check(row, clean_statusline()));
    let other_shell = vec![StatusLineCapture {
        bytes: b"not json".to_vec(),
        shell: ShellVerdict::Other,
    }];
    assert!(statusline_check(row, other_shell), "any filed payload");
}

/// The override row has no broken payload shape: its broken post-condition is a Run B whose
/// hooks fired while its status line filed nothing.
#[test]
fn check_statusline_override_fails_when_run_b_ran_and_its_status_line_filed_nothing() {
    let row = LedgerRow::StatusLineOverride;
    assert!(framing_check(LedgerRow::LongPasteWrapper, clean_trusted()));
    assert!(!statusline_check(row, Vec::new()));
}

#[test]
fn check_statusline_override_fails_with_no_capture() {
    let none = Probes {
        print: clean_run(),
        typed: clean_typed(),
        trusted: Vec::new(),
        dialogs: DialogRuns::default(),
        statusline: Vec::new(),
        resumed: Vec::new(),
        agents: None,
    };
    assert!(!check(LedgerRow::StatusLineOverride, &none));
}

#[test]
fn check_statusline_shell_passes_when_every_payload_ran_through_bin_sh() {
    assert!(statusline_check(
        LedgerRow::StatusLineShell,
        clean_statusline()
    ));
}

#[test]
fn check_statusline_shell_fails_on_one_other_shell_among_bin_sh() {
    let mut mixed = clean_statusline();
    mixed[1].shell = ShellVerdict::Other;
    assert!(!statusline_check(LedgerRow::StatusLineShell, mixed));
}

#[test]
fn check_statusline_shell_fails_with_no_capture() {
    assert!(!statusline_check(LedgerRow::StatusLineShell, Vec::new()));
}

#[test]
fn check_statusline_rate_limits_passes_on_two_whole_number_windows() {
    let row = LedgerRow::StatusLineRateLimits;
    assert!(statusline_check(row, clean_statusline()));
    let bounds = limited(
        window(json!(0), json!(0)),
        window(json!(100), json!(9_999_999_999_u64)),
    );
    assert!(statusline_check(row, vec![bounds]), "0 and 100");
    let other_shell = StatusLineCapture {
        shell: ShellVerdict::Other,
        ..clean_statusline().remove(1)
    };
    assert!(
        statusline_check(row, vec![other_shell]),
        "the shell is not this row's"
    );
}

#[test]
fn check_statusline_rate_limits_fails_on_each_broken_window() {
    let row = LedgerRow::StatusLineRateLimits;
    let first = clean_statusline().remove(0);
    let good = || window(json!(23), json!(1_760_400_000));
    for (why, five_hour, seven_day) in [
        (
            "a fractional percentage",
            window(json!(23.5), json!(1)),
            good(),
        ),
        (
            "a whole percentage written as a float",
            window(json!(23.0), json!(1)),
            good(),
        ),
        (
            "a percentage past 100",
            good(),
            window(json!(101), json!(1)),
        ),
        ("a negative percentage", good(), window(json!(-1), json!(1))),
        ("a string percentage", window(json!("23"), json!(1)), good()),
        (
            "a string resets_at",
            good(),
            window(json!(23), json!("2025-02-01T16:00:00Z")),
        ),
        ("a negative resets_at", window(json!(23), json!(-1)), good()),
        (
            "a fractional resets_at",
            window(json!(23), json!(1.5)),
            good(),
        ),
        ("no resets_at", json!({"used_percentage": 23}), good()),
        ("no used_percentage", json!({"resets_at": 1}), good()),
        ("the five-hour window missing", Value::Null, good()),
        ("the seven-day window missing", good(), Value::Null),
        ("a window not an object", good(), json!(23)),
    ] {
        let mut limits = json!({"five_hour": five_hour, "seven_day": seven_day});
        if let Some(limits) = limits.as_object_mut() {
            limits.retain(|_, w| !w.is_null());
        }
        let payload = json!({"session_id": "s-1", "rate_limits": limits});
        let broken = StatusLineCapture {
            bytes: payload.to_string().into_bytes(),
            shell: ShellVerdict::BinSh,
        };
        assert!(!statusline_check(row, vec![first.clone(), broken]), "{why}");
    }
    let not_object = StatusLineCapture {
        bytes: br#"[{"rate_limits":{}}]"#.to_vec(),
        shell: ShellVerdict::BinSh,
    };
    assert!(!statusline_check(row, vec![not_object]), "not one object");
    assert!(!statusline_check(row, vec![first]), "no rate_limits");
}

#[test]
fn check_statusline_rate_limits_fails_with_no_capture() {
    assert!(!statusline_check(
        LedgerRow::StatusLineRateLimits,
        Vec::new()
    ));
}

#[test]
fn shell_verdict_reads_only_the_exact_word() {
    assert_eq!(ShellVerdict::BinSh.word(), "bin-sh");
    assert_eq!(ShellVerdict::Other.word(), "other");
    assert_eq!(ShellVerdict::read(b"bin-sh"), ShellVerdict::BinSh);
    for other in [&b"other"[..], b"bin-sh\n", b"BIN-SH", b"", b"/bin/sh"] {
        assert_eq!(ShellVerdict::read(other), ShellVerdict::Other, "{other:?}");
    }
}

#[test]
fn statusline_variants_name_the_first_without_and_the_first_with_rate_limits() {
    let mut list = clean_statusline();
    let second_default = StatusLineCapture {
        bytes: br#"{"session_id":"s-1","n":2}"#.to_vec(),
        shell: ShellVerdict::BinSh,
    };
    list.insert(1, second_default);
    let named = statusline_variants(&list);
    let names: Vec<&str> = named.iter().map(|(n, _)| n.as_str()).collect();
    assert_eq!(
        names,
        ["StatusLine.default-1.json", "StatusLine.rate-limits-1.json"]
    );
    assert_eq!(named[0].1, list[0].bytes.as_slice());
    assert_eq!(named[1].1, list[2].bytes.as_slice());
    assert!(statusline_variants(&[]).is_empty());
    let only_first = vec![clean_statusline().remove(0)];
    let names: Vec<String> = statusline_variants(&only_first)
        .into_iter()
        .map(|(n, _)| n)
        .collect();
    assert_eq!(names, [STATUSLINE_DEFAULT]);
}

#[test]
fn statusline_capture_file_names_round_trip() {
    assert_eq!(statusline_capture_file_name(3), "StatusLine.3.json");
    assert_eq!(statusline_shell_file_name(3), "StatusLine.3.shell");
    assert_eq!(
        parse_statusline_capture_file_name("StatusLine.3.json"),
        Some(3)
    );
    for other in [
        "StatusLine.3.shell",
        "StatusLine.0.json",
        "StatusLine.x.json",
        "StatusLine.default-1.json",
        "StatusLine3.json",
        "Stop.3.json",
        "statusline.3.json",
    ] {
        assert_eq!(parse_statusline_capture_file_name(other), None, "{other}");
    }
    assert_eq!(parse_capture_file_name("StatusLine.3.json"), None);
}

#[test]
fn framing_variants_name_the_three_prompts_and_the_hooks_the_local_command_fired() {
    let trusted = clean_trusted();
    let named: Vec<(&str, &Capture)> = framing_variants(&trusted);
    let at: Vec<(&str, usize)> = named
        .iter()
        .map(|(stem, c)| {
            let index = trusted
                .iter()
                .position(|t| std::ptr::eq(t, *c))
                .expect("a capture of the run");
            (*stem, index)
        })
        .collect();
    assert_eq!(
        at,
        [
            ("paste-1", 3),
            ("paste-2", 5),
            ("paste-3", 7),
            ("compact-1", 9),
            ("clear-1", 10),
            ("clear-1", 11)
        ]
    );
    assert_eq!(trusted[9].event, HookEvent::SessionStart);
    assert_eq!(trusted[10].event, HookEvent::SessionEnd);
    assert_eq!(trusted[11].event, HookEvent::SessionStart);

    let no_end = with_trusted(|t| t[10] = capture(HookEvent::Stop, json!({})));
    let stems: Vec<&str> = framing_variants(&no_end).iter().map(|v| v.0).collect();
    assert_eq!(
        stems,
        ["paste-1", "paste-2", "paste-3", "compact-1", "clear-1"]
    );
    let no_compact = with_trusted(|t| {
        t.remove(9);
    });
    let stems: Vec<&str> = framing_variants(&no_compact).iter().map(|v| v.0).collect();
    assert_eq!(
        stems,
        ["paste-1", "paste-2", "paste-3", "clear-1", "clear-1"]
    );
    let echoed = vec![
        session_start("startup", "s-1"),
        prompted(PROBE_LONG_PASTE),
        prompted(PROBE_TAG_PASTE),
        prompted(PROBE_NEWLINE_PASTE),
        prompted(PROBE_COMPACT_COMMAND),
        prompted(PROBE_LOCAL_COMMAND),
    ];
    let stems: Vec<&str> = framing_variants(&echoed).iter().map(|v| v.0).collect();
    assert_eq!(stems, ["paste-1", "paste-2", "paste-3"]);
    let no_newline_turn = with_trusted(|t| {
        t.remove(7);
    });
    let stems: Vec<&str> = framing_variants(&no_newline_turn)
        .iter()
        .map(|v| v.0)
        .collect();
    assert_eq!(stems, ["paste-1", "paste-2", "clear-1", "clear-1"]);
    assert!(framing_variants(&[]).is_empty());
}

/// The paste texts are the bytes the live probe pasted, rebuilt here from their parts.
#[test]
fn framing_texts_are_fixed_ascii_each_with_its_stem() {
    let filler: Vec<String> = (1..=110).map(|n| format!("filler-{n:04}")).collect();
    let long = format!(
        "viola verify probe: this message is one long synthetic paste and its filler words \
             carry no meaning. Filler follows: {}xxxxxxx End of the synthetic paste. Reply with \
             the single word ok",
        filler.join(" ")
    );
    assert_eq!(PROBE_LONG_PASTE, long);
    assert_eq!(PROBE_LONG_PASTE.len(), 1500);
    assert_eq!(
        PROBE_TAG_PASTE,
        "viola verify probe: the next part is literal sample text and not markup: \
             <pasted_content id=\"1\"> sample </pasted_content id=\"1\"> then <task-notification> \
             and that is all. Reply with the single word ok"
    );
    assert_eq!(PROBE_TAG_PASTE.len(), 200);
    for text in [PROBE_LONG_PASTE, PROBE_TAG_PASTE] {
        assert!(text.is_ascii() && !text.contains('\n') && !text.contains('@'));
        assert!(text.starts_with("viola verify probe: "));
        assert!(text.ends_with("Reply with the single word ok"));
    }
    assert_eq!(
        PROBE_NEWLINE_PASTE,
        "viola verify probe: this paste holds one line break here\nand the next line ends it. \
             Reply with the single word ok"
    );
    assert!(PROBE_NEWLINE_PASTE.is_ascii() && !PROBE_NEWLINE_PASTE.contains(['\r', '@']));
    assert_eq!(PROBE_NEWLINE_PASTE.matches('\n').count(), 1);
    assert!(PROBE_NEWLINE_PASTE.starts_with("viola verify probe: "));
    assert!(PROBE_NEWLINE_PASTE.ends_with("Reply with the single word ok"));
    assert_eq!(framing_stem(PROBE_LONG_PASTE), Some("paste-1"));
    assert_eq!(framing_stem(PROBE_TAG_PASTE), Some("paste-2"));
    assert_eq!(framing_stem(PROBE_NEWLINE_PASTE), Some("paste-3"));
    assert_eq!(framing_stem("/compact"), Some("compact-1"));
    assert_eq!(framing_stem("/clear"), Some("clear-1"));
    for other in [PROBE_PROMPT, "/remote-control", "/compact ", "/clear ", ""] {
        assert_eq!(framing_stem(other), None, "{other}");
    }
    assert_eq!(dialog_stem(PROBE_LONG_PASTE), None);
}

#[test]
fn local_commands_are_the_three_known_ones_and_clear_and_compact_are_probed() {
    assert_eq!(
        LOCAL_COMMANDS,
        [
            ("/clear", Some(PostCondition::NewSession)),
            ("/remote-control", None),
            ("/compact", Some(PostCondition::CompactedSession))
        ]
    );
    assert_eq!(PROBE_LOCAL_COMMAND, "/clear");
    assert_eq!(PROBE_COMPACT_COMMAND, "/compact");
    let probed: Vec<&str> = LOCAL_COMMANDS
        .iter()
        .filter(|(_, post)| post.is_some())
        .map(|(command, _)| *command)
        .collect();
    assert_eq!(probed, ["/clear", "/compact"]);
}

fn with_questions(edit: impl FnOnce(&mut Vec<Capture>)) -> DialogRuns {
    let mut dialogs = clean_dialogs();
    edit(&mut dialogs.questions);
    dialogs
}

fn with_plan(edit: impl FnOnce(&mut Vec<Capture>)) -> DialogRuns {
    let mut dialogs = clean_dialogs();
    edit(&mut dialogs.plan);
    dialogs
}

fn post_response(answers: Value, annotations: Value) -> Capture {
    tool(
        POST,
        "AskUserQuestion",
        Some("q1"),
        json!({"tool_response": {"answers": answers, "annotations": annotations}}),
    )
}

#[test]
fn check_question_answer_needs_the_label_its_own_post_and_no_permission_request() {
    let notes = json!({"Probe color?": {"notes": PROBE_NOTE}});
    let wrong_label = with_questions(|q| {
        q[3] = post_response(
            json!({"Probe color?": "blue", "Probe size?": PROBE_FREE_TEXT}),
            notes,
        );
    });
    assert!(!dialog_check(LedgerRow::QuestionAnswer, wrong_label));
    let other_id = with_questions(|q| {
        q[3] = tool(POST, "AskUserQuestion", Some("q9"), json!({}));
    });
    assert!(!dialog_check(LedgerRow::QuestionAnswer, other_id));
    let raised = with_questions(|q| {
        q.push(tool(PR, "AskUserQuestion", None, json!({})));
    });
    assert!(!dialog_check(LedgerRow::QuestionAnswer, raised));
    let first_option_only = with_questions(|q| {
        q[3] = post_response(json!({"Probe color?": "red"}), json!({}));
    });
    assert!(dialog_check(LedgerRow::QuestionAnswer, first_option_only));
}

#[test]
fn check_question_notes_needs_the_free_text_and_the_note() {
    let notes = json!({"Probe color?": {"notes": PROBE_NOTE}});
    let no_free_text = with_questions(|q| {
        q[3] = post_response(
            json!({"Probe color?": "red", "Probe size?": "small"}),
            notes.clone(),
        );
    });
    assert!(!dialog_check(LedgerRow::QuestionNotes, no_free_text));
    let no_note = with_questions(|q| {
        q[3] = post_response(
            json!({"Probe color?": "red", "Probe size?": PROBE_FREE_TEXT}),
            json!({"Probe color?": {"notes": "other"}}),
        );
    });
    assert!(!dialog_check(LedgerRow::QuestionNotes, no_note));
    let one = with_questions(|q| {
        q[2] = tool(
            PRE,
            "AskUserQuestion",
            Some("q1"),
            json!({"tool_input": one_question("Probe color?")}),
        );
    });
    assert!(!dialog_check(LedgerRow::QuestionNotes, one));
}

#[test]
fn check_plan_approve_revise_reads_the_revise_the_re_plan_and_the_approve() {
    assert!(!dialog_check(
        LedgerRow::PlanApproveRevise,
        with_plan(|p| p.push(tool(PR, "ExitPlanMode", None, json!({}))))
    ));
    let approve_not_taken = with_plan(|p| {
        p[6] = tool(PR, "ExitPlanMode", None, json!({}));
    });
    assert!(!dialog_check(
        LedgerRow::PlanApproveRevise,
        approve_not_taken
    ));
    let no_revise = with_plan(|p| {
        p.remove(3);
    });
    assert!(!dialog_check(LedgerRow::PlanApproveRevise, no_revise));
    let other_call = with_plan(|p| {
        p[6] = tool(POST, "ExitPlanMode", Some("e1"), json!({}));
    });
    assert!(!dialog_check(LedgerRow::PlanApproveRevise, other_call));
    let starts_with_a_request = with_plan(|p| {
        p.swap(2, 3);
    });
    assert!(!dialog_check(
        LedgerRow::PlanApproveRevise,
        starts_with_a_request
    ));
    let re_plan_missing = with_plan(|p| {
        p[5] = tool(POST, "ExitPlanMode", Some("e2"), json!({}));
    });
    assert!(!dialog_check(LedgerRow::PlanApproveRevise, re_plan_missing));
}

#[test]
fn check_dialog_concurrency_needs_two_distinct_calls_each_answered() {
    let one = with_questions(|q| {
        q.drain(8..10);
    });
    assert!(!dialog_check(LedgerRow::DialogConcurrency, one));
    let same_id = with_questions(|q| {
        q[8] = tool(PRE, "AskUserQuestion", Some("p1"), json!({}));
        q[9] = tool(POST, "AskUserQuestion", Some("p1"), json!({}));
    });
    assert!(!dialog_check(LedgerRow::DialogConcurrency, same_id));
    let unanswered = with_questions(|q| {
        q.remove(9);
    });
    assert!(!dialog_check(LedgerRow::DialogConcurrency, unanswered));
    let three = with_questions(|q| {
        q.insert(10, tool(PRE, "AskUserQuestion", Some("p3"), json!({})));
        q.insert(11, tool(POST, "AskUserQuestion", Some("p3"), json!({})));
    });
    assert!(!dialog_check(LedgerRow::DialogConcurrency, three));
    let no_id = with_questions(|q| {
        q[8] = tool(PRE, "AskUserQuestion", None, json!({}));
    });
    assert!(!dialog_check(LedgerRow::DialogConcurrency, no_id));
}

/// The second call's own PostToolUse is what answers it: one carrying a third id does not.
#[test]
fn check_dialog_concurrency_with_a_post_for_a_third_id_is_not_answered() {
    let third_id = with_questions(|q| {
        q[9] = tool(POST, "AskUserQuestion", Some("p9"), json!({}));
    });
    assert!(!dialog_check(LedgerRow::DialogConcurrency, third_id));
}

#[test]
fn parallel_both_before_first_post_reads_the_claim_order() {
    assert_eq!(
        parallel_both_before_first_post(&clean_dialogs().questions),
        Some(false)
    );
    let open_together = with_questions(|q| q.swap(7, 8)).questions;
    assert_eq!(parallel_both_before_first_post(&open_together), Some(true));
    let unanswered = with_questions(|q| {
        q.remove(9);
        q.remove(7);
    })
    .questions;
    assert_eq!(parallel_both_before_first_post(&unanswered), Some(true));
    let one = with_questions(|q| {
        q.drain(8..10);
    })
    .questions;
    assert_eq!(parallel_both_before_first_post(&one), None);
    assert_eq!(parallel_both_before_first_post(&[]), None);
}

#[test]
fn dialog_variants_name_each_call_of_each_turn() {
    let dialogs = clean_dialogs();
    let names: Vec<String> = dialog_variants(&dialogs)
        .iter()
        .map(|(variant, c)| format!("{}.{variant}", event_name(c.event)))
        .collect();
    assert_eq!(
        names,
        [
            "PreToolUse.questions-1",
            "PostToolUse.questions-1",
            "PreToolUse.parallel-1",
            "PostToolUse.parallel-1",
            "PreToolUse.parallel-2",
            "PostToolUse.parallel-2",
            "PermissionRequest.permission-1",
            "PostToolUse.permission-1",
            "PreToolUse.plan-1",
            "PermissionRequest.plan-1",
            "PreToolUse.plan-2",
            "PostToolUse.plan-2",
        ]
    );
    let posts: Vec<Option<&str>> = dialog_variants(&dialogs)
        .iter()
        .filter(|(_, c)| c.event == POST)
        .map(|(_, c)| c.field("tool_use_id"))
        .collect();
    assert_eq!(
        posts,
        [Some("q1"), Some("p1"), Some("p2"), Some("b1"), Some("e2")]
    );
    assert!(dialog_variants(&DialogRuns::default()).is_empty());
}

/// A PostToolUse joins an id-less call of its tool only: beside a call that has an id of its
/// own, one whose id matches no call is not recorded.
#[test]
fn dialog_variants_with_a_post_of_an_unknown_id_beside_an_identified_call_records_no_post() {
    let dialogs = DialogRuns {
        questions: vec![
            prompted(DIALOG_PROMPT_QUESTIONS),
            tool(PRE, "AskUserQuestion", Some("q1"), json!({})),
            tool(POST, "AskUserQuestion", Some("q9"), json!({})),
        ],
        ..DialogRuns::default()
    };
    let names: Vec<String> = dialog_variants(&dialogs)
        .iter()
        .map(|(variant, c)| format!("{}.{variant}", event_name(c.event)))
        .collect();
    assert_eq!(names, ["PreToolUse.questions-1"]);
}

/// A second capture of one event and variant is not recorded: the first stands.
#[test]
fn dialog_variants_keep_the_first_capture_of_a_variant() {
    let mut dialogs = clean_dialogs();
    dialogs.questions.insert(
        4,
        tool(POST, "AskUserQuestion", Some("q1"), json!({"n": 2})),
    );
    let kept: Vec<&Capture> = dialog_variants(&dialogs)
        .into_iter()
        .filter(|(v, c)| v == "questions-1" && c.event == POST)
        .map(|(_, c)| c)
        .collect();
    assert_eq!(kept.len(), 1);
    assert!(
        kept[0]
            .payload
            .as_ref()
            .is_some_and(|p| p.get("n").is_none())
    );
}

#[test]
fn dialog_prompts_are_fixed_ascii_without_tags_each_with_its_stem() {
    for (run, prompt, stem) in DIALOG_TURNS {
        assert!(prompt.is_ascii() && !prompt.contains('<') && !prompt.contains('>'));
        assert!(prompt.starts_with("viola verify probe: "));
        assert_eq!(dialog_stem(prompt), Some(stem));
        assert_eq!(run == DialogRun::Plan, stem == "plan");
    }
    let stems: Vec<&str> = DIALOG_TURNS.iter().map(|t| t.2).collect();
    assert_eq!(stems, ["questions", "parallel", "permission", "plan"]);
    assert!(DIALOG_PROMPT_PERMISSION.contains("touch viola-probe-permission"));
    assert_eq!(dialog_stem(PROBE_PROMPT), None);
    assert_eq!(dialog_stem(""), None);
    for text in [PROBE_FREE_TEXT, PROBE_NOTE, PROBE_REVISE] {
        assert!(text.is_ascii() && !text.contains('<'));
    }
}

#[test]
fn dialog_settings_are_the_ask_rule_and_the_plans_directory() {
    assert_eq!(
        serde_json::from_str::<Value>(DIALOG_SETTINGS).expect("json"),
        json!({"permissions": {"ask": ["Bash(touch viola-probe-permission)"]}})
    );
    assert_eq!(
        serde_json::from_str::<Value>(&plan_settings("C:\\w \"x\"/plans")).expect("json"),
        json!({"plansDirectory": "C:\\w \"x\"/plans"})
    );
}

#[test]
fn probe_answers_are_the_compiled_table() {
    let ids: Vec<&str> = ProbeAnswer::ALL.iter().map(|a| a.id()).collect();
    assert_eq!(
        ids,
        [
            "questions-first-and-free-text",
            "question-first-option",
            "plan-revise",
            "plan-approve",
            "permit-allow"
        ]
    );
    for answer in ProbeAnswer::ALL {
        assert_eq!(ProbeAnswer::from_id(answer.id()), Some(answer));
    }
    assert_eq!(ProbeAnswer::from_id("plan-approve\n"), None);
    let table: Vec<String> = PROBE_ANSWERS
        .iter()
        .map(|(run, event, j, answer)| {
            format!(
                "{run:?} {} {} {}",
                answer_file_name(*event, *j),
                answer.id(),
                j
            )
        })
        .collect();
    assert_eq!(
        table,
        [
            "Questions PreToolUse.1 questions-first-and-free-text 1",
            "Questions PreToolUse.2 question-first-option 2",
            "Questions PreToolUse.3 question-first-option 3",
            "Questions PermissionRequest.1 permit-allow 1",
            "Plan PermissionRequest.1 plan-revise 1",
            "Plan PreToolUse.2 plan-approve 2",
        ]
    );
}

fn body_of(event: HookEvent, payload: &Value, answer: ProbeAnswer) -> Option<Value> {
    probe_body(event, payload.to_string().as_bytes(), answer)
        .map(|b| serde_json::from_str(&b).expect("a JSON body"))
}

#[test]
fn probe_body_builds_each_answer_through_the_product_mapping() {
    let asked = json!({"tool_name": "AskUserQuestion", "tool_input": two_questions()});
    let first = body_of(PRE, &asked, ProbeAnswer::QuestionsFirstAndFreeText).expect("a body");
    let updated = &first["hookSpecificOutput"]["updatedInput"];
    assert_eq!(updated["questions"], two_questions()["questions"]);
    assert_eq!(
        updated["answers"],
        json!({"Probe color?": "red", "Probe size?": PROBE_FREE_TEXT})
    );
    assert_eq!(
        updated["annotations"],
        json!({"Probe color?": {"notes": PROBE_NOTE}})
    );
    let option = body_of(PRE, &asked, ProbeAnswer::QuestionFirstOption).expect("a body");
    let updated = &option["hookSpecificOutput"]["updatedInput"];
    assert_eq!(
        updated["answers"],
        json!({"Probe color?": "red", "Probe size?": "small"})
    );
    assert!(updated.get("annotations").is_none());

    let plan =
        json!({"tool_name": "ExitPlanMode", "tool_input": {"plan": "p", "planFilePath": "f"}});
    let revise = body_of(PR, &plan, ProbeAnswer::PlanRevise).expect("a body");
    assert_eq!(
        revise["hookSpecificOutput"]["decision"],
        json!({"behavior": "deny", "message": PROBE_REVISE})
    );
    let approve = body_of(PRE, &plan, ProbeAnswer::PlanApprove).expect("a body");
    assert_eq!(
        approve["hookSpecificOutput"],
        json!({"hookEventName": "PreToolUse", "permissionDecision": "allow",
                "updatedInput": {"plan": "p", "planFilePath": "f"}})
    );
    let bash = json!({"tool_name": "Bash", "tool_input": {"command": "touch x"}});
    let allow = body_of(PR, &bash, ProbeAnswer::PermitAllow).expect("a body");
    assert_eq!(
        allow["hookSpecificOutput"]["decision"],
        json!({"behavior": "allow"})
    );
}

#[test]
fn probe_body_with_one_question_answers_it_and_notes_it() {
    let asked = json!({"tool_input": one_question("Probe left?")});
    let got = body_of(PRE, &asked, ProbeAnswer::QuestionsFirstAndFreeText).expect("a body");
    let updated = &got["hookSpecificOutput"]["updatedInput"];
    assert_eq!(updated["answers"], json!({"Probe left?": "yes"}));
    assert_eq!(
        updated["annotations"],
        json!({"Probe left?": {"notes": PROBE_NOTE}})
    );
}

#[rstest]
#[case::not_json(PRE, None, ProbeAnswer::QuestionFirstOption)]
#[case::no_tool_input(PRE, Some(json!({"tool_name": "AskUserQuestion"})), ProbeAnswer::QuestionFirstOption)]
#[case::no_questions(PRE, Some(json!({"tool_input": {}})), ProbeAnswer::QuestionFirstOption)]
#[case::question_not_string(PRE, Some(json!({"tool_input": {"questions": [{"question": 1, "options": [{"label": "a"}]}]}})), ProbeAnswer::QuestionFirstOption)]
#[case::no_option(PRE, Some(json!({"tool_input": {"questions": [{"question": "q", "options": []}]}})), ProbeAnswer::QuestionFirstOption)]
#[case::no_first_question(PRE, Some(json!({"tool_input": {"questions": []}})), ProbeAnswer::QuestionsFirstAndFreeText)]
#[case::revise_on_pre_tool_use(PRE, Some(json!({"tool_input": {"plan": "p"}})), ProbeAnswer::PlanRevise)]
#[case::approve_on_permission_request(PR, Some(json!({"tool_input": {"plan": "p"}})), ProbeAnswer::PlanApprove)]
#[case::allow_on_pre_tool_use(PRE, Some(json!({"tool_input": {}})), ProbeAnswer::PermitAllow)]
fn probe_body_is_none_where_the_answer_cannot_map(
    #[case] event: HookEvent,
    #[case] payload: Option<Value>,
    #[case] answer: ProbeAnswer,
) {
    let bytes = payload.map_or_else(|| b"not json".to_vec(), |p| p.to_string().into_bytes());
    assert_eq!(probe_body(event, &bytes, answer), None);
}

#[test]
fn probe_body_with_no_questions_answers_nothing_by_first_option() {
    let asked = json!({"tool_input": {"questions": []}});
    assert_eq!(
        body_of(PRE, &asked, ProbeAnswer::QuestionFirstOption)
            .map(|b| { b["hookSpecificOutput"]["updatedInput"]["answers"].clone() }),
        Some(json!({}))
    );
}

#[test]
fn capture_plugin_files_for_the_dialog_tier_add_the_three_events_and_the_answers() {
    let files = capture_plugin_files(
        "C:/h/viola.exe",
        "C:/h/c",
        ProbePlugin::Dialog {
            answers_dir_fwd: "C:/h/a",
        },
    );
    let doc: Value = serde_json::from_str(&files[1].1).expect("json");
    let events: Vec<&str> = doc["hooks"]
        .as_object()
        .expect("hooks")
        .keys()
        .map(String::as_str)
        .collect();
    assert_eq!(
        events,
        [
            "SessionStart",
            "UserPromptSubmit",
            "Stop",
            "SessionEnd",
            "PreToolUse",
            "PermissionRequest",
            "PostToolUse"
        ]
    );
    let group = |event: &str| doc["hooks"][event][0].clone();
    let args = |event: &str| group(event)["hooks"][0]["args"].clone();
    assert_eq!(
        args("PreToolUse"),
        json!([
            "hook",
            "pre-tool-use",
            "--capture",
            "C:/h/c",
            "--answers",
            "C:/h/a"
        ])
    );
    assert_eq!(
        group("PreToolUse")["matcher"],
        "AskUserQuestion|ExitPlanMode"
    );
    assert_eq!(
        args("PermissionRequest"),
        json!([
            "hook",
            "permission-request",
            "--capture",
            "C:/h/c",
            "--answers",
            "C:/h/a"
        ])
    );
    assert!(group("PermissionRequest").get("matcher").is_none());
    assert_eq!(
        args("PostToolUse"),
        json!(["hook", "post-tool-use", "--capture", "C:/h/c"])
    );
    assert_eq!(args("Stop"), json!(["hook", "stop", "--capture", "C:/h/c"]));
    assert_eq!(group("PostToolUse")["hooks"][0]["timeout"], 5);
    assert!(group("SessionEnd")["hooks"][0].get("timeout").is_none());
    assert_eq!(group("PreToolUse")["hooks"][0]["command"], "C:/h/viola.exe");
    let spine = capture_plugin_files("C:/h/viola.exe", "C:/h/c", ProbePlugin::Spine);
    assert!(!spine[1].1.contains("--answers") && !spine[1].1.contains("PreToolUse"));
}
