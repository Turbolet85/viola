use super::*;

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

fn echo_check(trusted: Vec<Capture>, echo_latency_ms: Option<u64>) -> bool {
    check(
        LedgerRow::LocalCommandEcho,
        &Probes {
            print: clean_run(),
            typed: TypedRun {
                echo_latency_ms,
                ..clean_typed()
            },
            trusted,
            dialogs: clean_dialogs(),
            resumed: clean_resumed(),
            agents: Some(clean_agents()),
        },
    )
}

/// The echo row passes on a tag read within the send's 10 s window with no prompt carrying the
/// command; a tag past the window, none read, or the command submitted as a prompt fails it.
#[test]
fn check_local_command_echo_needs_a_tag_within_the_window_and_no_prompt() {
    assert!(echo_check(clean_trusted(), Some(1_200)));
    assert!(echo_check(clean_trusted(), Some(10_000)), "at the bound");
    assert!(!echo_check(clean_trusted(), Some(10_001)), "past the bound");
    assert!(!echo_check(clean_trusted(), None), "no tag read");
    let submitted = with_trusted(|t| t.insert(9, prompted(PROBE_ECHO_COMMAND)));
    assert!(
        !echo_check(submitted, Some(1_200)),
        "the command as a prompt"
    );
}
