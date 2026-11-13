use super::*;

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
