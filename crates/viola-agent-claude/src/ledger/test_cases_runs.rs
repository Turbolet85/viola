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
