use super::*;

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
    assert_eq!(scrub(&json!(text), home, ""), json!(expected));
}

#[test]
fn scrub_matches_the_home_case_exactly() {
    let text = json!(r"c:\users\plantuser\x");
    assert_eq!(scrub(&text, r"C:\Users\Plantuser", ""), text);
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
    assert_eq!(scrub(&json!(text), "", "plantuser"), json!(expected));
}

#[test]
fn scrub_walks_keys_arrays_and_nested_objects_and_keeps_other_values() {
    let payload = json!({
        "cwd": "/home/plantuser/w",
        "plantuser": [1, true, null, {"deep": "/home/plantuser"}],
        "n": 2.5,
    });
    let got = scrub(&payload, "/home/plantuser", "plantuser");
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
    assert_eq!(scrub(&payload, "", ""), payload);
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

#[test]
fn check_typed_rows_fail_without_a_measurement() {
    for row in [
        LedgerRow::ModalSignature,
        LedgerRow::InputBoxSignature,
        LedgerRow::QuietPeriod,
        LedgerRow::ConfirmWindow,
        LedgerRow::LocalCommandCompact,
        LedgerRow::LocalCommandEcho,
        LedgerRow::TrustRecord,
        LedgerRow::LocalCommandContext,
        LedgerRow::CrossSessionPrefix,
    ] {
        assert!(!typed_check(row, TypedRun::default()), "{}", row.id());
    }
    let eight = verdicts_with(TypedRun::default());
    assert_eq!(
        eight,
        [
            true, true, true, true, true, true, false, false, false, false, true, true, true, true,
            true, true, true, true, true, true, false, false, false, true, false, true, true, true,
            false
        ]
    );
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

#[test]
fn check_input_box_signature_passes_on_three_box_only_screens() {
    let typed = TypedRun {
        ready: Some(box_only("❯ Try it")),
        turn: Some(box_only("● ok")),
        long_turn: Some(box_only("● long")),
        ..clean_typed()
    };
    assert!(typed_check(LedgerRow::InputBoxSignature, typed));
}

#[rstest]
#[case::box_with_a_modal(box_only("   Yes, I trust this folder"))]
#[case::box_with_the_imports_modal(box_only("   Yes, allow external imports"))]
#[case::neither(rows(&["● ok", "  ctrl+g to edit in Vim · Update available!"]))]
#[case::short_rule(vec![rule(39), "❯ ".to_owned()])]
fn check_input_box_signature_fails_on_a_box_with_a_modal_or_no_box(#[case] screen: Vec<String>) {
    let typed = TypedRun {
        turn: Some(screen),
        ..clean_typed()
    };
    assert!(!typed_check(LedgerRow::InputBoxSignature, typed));
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

#[test]
fn signature_rows_keep_a_box_row_as_its_literal_alone() {
    let live = vec![
        "● ok".to_owned(),
        format!("{}  ", rule(80)),
        "❯ viola verify probe /home/plantuser/w".to_owned(),
        rule(66),
        "  ⏵⏵ auto mode on (shift+tab to cycle)".to_owned(),
        "❯ a transcript row".to_owned(),
        rule(39),
        "❯ under a short rule".to_owned(),
    ];
    assert_eq!(
        signature_rows(&live),
        vec![
            String::new(),
            rule(80),
            "❯".to_owned(),
            rule(66),
            "  ⏵⏵ auto mode on (shift+tab to cycle)".to_owned(),
            String::new(),
            String::new(),
            String::new(),
        ]
    );
}

#[test]
fn screen_fault_reads_a_recorded_box_row_with_no_finding() {
    let live = vec![
        "● ok".to_owned(),
        rule(80),
        "❯ viola verify probe".to_owned(),
        rule(80),
        "  ctrl+g to edit in Vim · Update available!".to_owned(),
    ];
    let kept = signature_rows(&live);
    assert_eq!(kept[2], "❯");
    assert_eq!(
        screen_fault(&live, &kept, "/home/plantuser", "plantuser"),
        None
    );
}

/// A kept rule is box drawing alone, so its seam is its live neighbour's whole text: a prompt row
/// holding the home refuses the record at the rule below it, never keeps the text.
#[test]
fn screen_fault_refuses_a_box_whose_prompt_row_holds_the_home() {
    let live = vec![
        "● ok".to_owned(),
        rule(80),
        "❯ /home/plantuser/w".to_owned(),
        rule(80),
    ];
    let kept = signature_rows(&live);
    assert_eq!(kept[2], "❯");
    assert_eq!(
        screen_fault(&live, &kept, "/home/plantuser", "plantuser"),
        Some(ScreenFault {
            row: 1,
            seam: true,
            why: Unclean::HomePath
        })
    );
}

#[test]
fn merge_stamp_writes_the_box_only_guards() {
    for (count, expected) in [
        (Some(0), json!(0)),
        (Some(3), json!(3)),
        (None, json!(null)),
    ] {
        let typed = TypedRun {
            box_only_guards: count,
            ..clean_typed()
        };
        let bytes = merge_stamp(
            None,
            "2.1.0",
            &all_pass(),
            &[],
            &typed,
            &Readings::default(),
            "t",
        );
        assert_eq!(
            doc(&bytes)["data"]["versions"]["2.1.0"]["measured"]["typed_probe"]["box_only_guards"],
            expected
        );
    }
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

#[test]
fn unclean_refuses_an_email_in_a_string_or_a_key_after_a_path() {
    assert_eq!(
        unclean(&json!({"a": "mail a.b@example.com"}), "plantuser"),
        Some(Unclean::Email)
    );
    assert_eq!(
        unclean(&json!({"a.b@example.com": 1}), "plantuser"),
        Some(Unclean::Email)
    );
    assert_eq!(
        unclean(&json!(["/home/other/a.b@example.com"]), "plantuser"),
        Some(Unclean::AbsolutePath)
    );
    assert!(!is_clean(&json!({"a": "a.b@example.com"}), "plantuser"));
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

#[test]
fn check_dialog_rows_pass_on_the_recorded_shape() {
    for row in DIALOG_ROWS {
        assert!(dialog_check(row, clean_dialogs()), "{}", row.id());
    }
}

/// Absence is never a pass: no run, or runs whose turns raised no dialog, fail all five.
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
            prompted(DIALOG_PROMPT_PERMISSION_QUESTION),
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

/// The trust-record row passes only when the record was written, Run A (its untrusted twin) showed
/// a modal literal, and Run F's first settled screen shows the input box and no modal literal.
#[test]
fn check_trust_record_needs_the_written_record_the_control_modal_and_an_open_box() {
    let trust = |typed: TypedRun| {
        check(
            LedgerRow::TrustRecord,
            &Probes {
                typed,
                ..probes(&clean_run())
            },
        )
    };
    assert!(trust(clean_typed()));
    let modal = rows(&["", " ❯ No, exit", "   Yes, I trust this folder"]);
    let boxed = rows(&["❯ Try it", "  ⏸ manual mode on · ← for agents"]);
    let cases: [(&str, TypedRun); 7] = [
        (
            "the record not written",
            TypedRun {
                trust_write: Some(TrustWord::ConfigAbsent),
                ..clean_typed()
            },
        ),
        (
            "no write at all",
            TypedRun {
                trust_write: None,
                ..clean_typed()
            },
        ),
        (
            "Run A showed no modal: the control cannot fail",
            TypedRun {
                modal: Some(boxed.clone()),
                ..clean_typed()
            },
        ),
        (
            "Run A never settled",
            TypedRun {
                modal: None,
                ..clean_typed()
            },
        ),
        (
            "Run F settled on the trust dialog",
            TypedRun {
                trust_frame: Some(modal.clone()),
                ..clean_typed()
            },
        ),
        (
            "Run F shows the box and a modal",
            TypedRun {
                trust_frame: Some(modal.iter().chain(boxed.iter().skip(1)).cloned().collect()),
                ..clean_typed()
            },
        ),
        (
            "Run F never settled",
            TypedRun {
                trust_frame: None,
                ..clean_typed()
            },
        ),
    ];
    for (label, typed) in cases {
        assert!(!trust(typed), "{label}");
    }
}

/// The browser-tools offer's rows as 2.1.325 draws them, synthetic.
fn browser_tools_screen() -> Vec<String> {
    rows(&[
        " Claude in Chrome extension detected",
        " ❯ No, keep browser tools off",
        "   Yes, use my browser",
    ])
}

/// `modal-signature` asks what an untrusted start shows: the browser-tools offer at Run A is not
/// the trust or imports dialog, so the row fails.
#[test]
fn modal_signature_fails_on_a_browser_tools_screen_at_run_a() {
    let typed = TypedRun {
        modal: Some(browser_tools_screen()),
        ..clean_typed()
    };
    assert!(!typed_check(LedgerRow::ModalSignature, typed));
}

/// Run F opening on the browser-tools offer is a modal standing: the trust record did not open
/// the folder to the box, so the row fails.
#[test]
fn trust_record_fails_when_run_f_opens_on_the_browser_tools_prompt() {
    let boxed = rows(&["❯ Try it", "  ⏸ manual mode on · ← for agents"]);
    let typed = TypedRun {
        trust_frame: Some(browser_tools_screen().into_iter().chain(boxed).collect()),
        ..clean_typed()
    };
    assert!(!typed_check(LedgerRow::TrustRecord, typed));
}

/// The 26th row reads the first Stop capture's `session_crons` as an array, empty included; no
/// element is read.
#[test]
fn checks_stop_session_crons_with_an_array_passes() {
    for crons in [json!([]), json!([{"id": 1}])] {
        let mut run = clean_run();
        run.captures[2] = capture(HookEvent::Stop, json!({"session_crons": crons}));
        assert!(check(LedgerRow::StopSessionCrons, &probes(&run)), "{crons}");
    }
}

#[rstest]
#[case::absent(json!({"last_assistant_message": "ok"}))]
#[case::null(json!({"session_crons": null}))]
#[case::a_string(json!({"session_crons": "[]"}))]
#[case::an_object(json!({"session_crons": {}}))]
fn checks_stop_session_crons_without_an_array_fails(#[case] payload: Value) {
    let mut run = clean_run();
    run.captures[2] = capture(HookEvent::Stop, payload);
    assert!(!check(LedgerRow::StopSessionCrons, &probes(&run)));
    assert_eq!(
        row_word(LedgerRow::StopSessionCrons, false, &clean_typed()),
        "fail"
    );
    run.captures.remove(2);
    assert!(!check(LedgerRow::StopSessionCrons, &probes(&run)));
}

/// The 27th row passes on the last question turn as recorded: a PreToolUse, a PermissionRequest
/// with the same `tool_input`, and the PostToolUse of the call carrying the first label.
#[test]
fn check_permission_request_question_passes_on_the_recorded_shape() {
    let dialogs = clean_dialogs();
    let last = turn(&dialogs.questions, DIALOG_PROMPT_PERMISSION_QUESTION);
    let events: Vec<HookEvent> = last.iter().map(|c| c.event).collect();
    assert_eq!(events, [PRE, PR, POST, HookEvent::Stop]);
    assert!(dialog_check(LedgerRow::PermissionRequestQuestion, dialogs));
    assert_eq!(
        row_word(LedgerRow::PermissionRequestQuestion, true, &clean_typed()),
        "pass"
    );
    assert_eq!(
        row_word(LedgerRow::PermissionRequestQuestion, false, &clean_typed()),
        "fail"
    );
    assert!(!counts(LedgerRow::PermissionRequestQuestion, "absent"));
}

#[rstest]
#[case::no_permission_request("no_permission_request")]
#[case::unequal_tool_input("unequal_tool_input")]
#[case::no_post_tool_use("no_post_tool_use")]
#[case::answers_differ("answers_differ")]
#[case::question_turn_only("question_turn_only")]
fn check_permission_request_question_fails_when(#[case] fault: &str) {
    let at = |q: &[Capture], event: HookEvent| {
        let start = q
            .iter()
            .position(|c| c.field("prompt") == Some(DIALOG_PROMPT_PERMISSION_QUESTION))
            .expect("the last question turn");
        start
            + q[start..]
                .iter()
                .position(|c| c.event == event)
                .expect("an event")
    };
    let dialogs = with_questions(|q| match fault {
        "no_permission_request" => {
            q.remove(at(q, PR));
        }
        "unequal_tool_input" => {
            let i = at(q, PR);
            q[i] = tool(
                PR,
                "AskUserQuestion",
                None,
                json!({"tool_input": one_question("Probe shape?")}),
            );
        }
        "no_post_tool_use" => {
            q.remove(at(q, POST));
        }
        "answers_differ" => {
            let i = at(q, POST);
            q[i] = tool(
                POST,
                "AskUserQuestion",
                Some("qp"),
                json!({"tool_response": {"answers": {"Probe shape?": "square"}}}),
            );
        }
        "question_turn_only" => {
            let moved = q.remove(at(q, PR));
            q.insert(4, moved);
        }
        _ => unreachable!("{fault}"),
    });
    assert!(
        !dialog_check(LedgerRow::PermissionRequestQuestion, dialogs),
        "{fault}"
    );
}

/// The row's check over the clean probes with Run B's env-names document replaced.
fn floor_check(env: Option<Value>, kept: &[String]) -> bool {
    check(
        LedgerRow::IdentityFloor,
        &Probes {
            env: env_reading(env.as_ref(), kept),
            ..probes(&clean_run())
        },
    )
}

fn env_doc(names: &[&str], unshaped: u64) -> Value {
    json!({"v": 1, "claude_names": names, "claude_unshaped": unshaped, "other_count": 12})
}

#[test]
fn identity_floor_passes_on_floor_and_hook_names() {
    let names = [
        "CLAUDECODE",
        "CLAUDE_PID",
        "CLAUDE_PLUGIN_ROOT",
        "CLAUDE_PROJECT_DIR",
        "CLAUDE_ENV_FILE",
    ];
    assert!(floor_check(Some(env_doc(&names, 0)), &[]));
    assert!(
        floor_check(Some(env_doc(&[], 0)), &[]),
        "no CLAUDE name at all"
    );
    let reading = env_reading(Some(&env_doc(&names, 0)), &[]);
    assert_eq!(reading.added, names);
    assert!(reading.unknown.is_empty());
    assert_eq!(reading.other_count, Some(12));
}

#[rstest]
#[case::unread(None)]
#[case::unknown_name(Some(env_doc(&["CLAUDE_PID", "CLAUDE_VIOLA_TEST_NEW_X"], 0)))]
#[case::unshaped_name(Some(env_doc(&["CLAUDE_PID"], 1)))]
fn identity_floor_fails_when(#[case] env: Option<Value>) {
    assert!(!floor_check(env, &[]));
}

#[test]
fn identity_floor_drops_kept_names_before_the_check() {
    let kept = vec!["CLAUDE_VIOLA_TEST_KEPT".to_owned()];
    let doc = env_doc(&["CLAUDE_PID", "CLAUDE_VIOLA_TEST_KEPT"], 0);
    assert!(floor_check(Some(doc.clone()), &kept));
    let reading = env_reading(Some(&doc), &kept);
    assert_eq!(reading.added, ["CLAUDE_PID"]);
    assert_eq!(
        env_variant(&reading),
        Some(json!({"v": 1, "added": ["CLAUDE_PID"]}))
    );
    assert!(!floor_check(Some(doc), &[]), "the same name, not kept");
}

#[rstest]
#[case::names_not_strings(json!({"v": 1, "claude_names": [1], "claude_unshaped": 0, "other_count": 1}))]
#[case::no_other_count(json!({"v": 1, "claude_names": [], "claude_unshaped": 0}))]
#[case::no_unshaped(json!({"v": 1, "claude_names": [], "other_count": 1}))]
fn env_reading_of_another_shape_reads_unread(#[case] doc: Value) {
    let reading = env_reading(Some(&doc), &[]);
    assert_eq!(reading, EnvReading::default());
    assert_eq!(env_variant(&reading), None);
}

/// The cross-session row's check over Run B's captures and a typed run whose cross-session turn
/// is `run`.
fn cross_check(trusted: Vec<Capture>, run: Option<CrossSessionRun>) -> bool {
    let probes = Probes {
        typed: TypedRun {
            cross_session: run,
            ..clean_typed()
        },
        trusted,
        ..probes(&clean_run())
    };
    check(LedgerRow::CrossSessionPrefix, &probes)
}

#[test]
fn cross_session_prefix_passes_on_a_prefixed_arrival() {
    assert!(cross_check(clean_trusted(), Some(clean_cross_session())));
    let plain = with_trusted(|t| {
        t[12] = prompted(&CROSS_ARRIVAL.replacen("<\\cross", "<cross", 1));
    });
    assert!(
        cross_check(plain, Some(clean_cross_session())),
        "the unescaped tag"
    );
    let probes = probes(&clean_run());
    assert_eq!(
        cross_session_reading(&probes.trusted, &probes.typed),
        ("harness", Some("cross-session-escaped"))
    );
}

#[rstest]
#[case::not_run(clean_trusted(), None, "not-run")]
#[case::no_prefix(
    with_trusted(|t| t[12] = prompted(CROSS_SESSION_TEXT)),
    Some(clean_cross_session()),
    "unprefixed"
)]
#[case::no_prompt(
    with_trusted(|t| { t.remove(12); }),
    Some(clean_cross_session()),
    "no-prompt"
)]
#[case::sender_failed(
    clean_trusted(),
    Some(CrossSessionRun { sender: SenderEnd::Failed, ..clean_cross_session() }),
    "sender-failed"
)]
#[case::sender_deadline(
    clean_trusted(),
    Some(CrossSessionRun { sender: SenderEnd::Deadline, ..clean_cross_session() }),
    "sender-deadline"
)]
fn cross_session_prefix_fails_when(
    #[case] trusted: Vec<Capture>,
    #[case] run: Option<CrossSessionRun>,
    #[case] outcome: &str,
) {
    let typed = TypedRun {
        cross_session: run.clone(),
        ..clean_typed()
    };
    assert_eq!(cross_session_reading(&trusted, &typed), (outcome, None));
    assert!(!cross_check(trusted, run));
}

/// The sender's words decide nothing: a send its tool read refused still passes on a prefixed
/// arrival, and a delivered one still fails on none.
#[test]
fn cross_session_prefix_reads_the_arrival_never_the_sender_words() {
    let refused = CrossSessionRun {
        sender_tools: vec![ToolWord::Search],
        send_result: Some(ResultWord::Refused),
        ..clean_cross_session()
    };
    assert!(cross_check(clean_trusted(), Some(refused)));
    let none = with_trusted(|t| {
        t.remove(12);
    });
    assert!(!cross_check(none, Some(clean_cross_session())));
}

#[test]
fn sender_words_read_the_pre_words_once_each_and_the_first_post_result() {
    let docs = [
        json!({"v": 1, "event": "pre", "tool": "search", "result": null}),
        json!({"v": 1, "event": "pre", "tool": "send", "result": null}),
        json!({"v": 1, "event": "pre", "tool": "search", "result": null}),
        json!({"v": 1, "event": "post", "tool": "send", "result": "refused"}),
        json!({"v": 1, "event": "post", "tool": "send", "result": "delivered"}),
        json!({"v": 1, "event": "pre", "tool": "a-new-word", "result": null}),
        json!("not an object"),
    ];
    assert_eq!(
        sender_words(&docs),
        (
            vec![ToolWord::Search, ToolWord::Send],
            Some(ResultWord::Refused)
        )
    );
    assert_eq!(sender_words(&[]), (Vec::new(), None));
}

#[rstest]
#[case::runtime_dir("/run/user/1000".to_owned(), Some("uds:/run/user/1000/cc-socks/4242.sock".to_owned()))]
#[case::trailing_slash("/run/user/1000/".to_owned(), Some("uds:/run/user/1000/cc-socks/4242.sock".to_owned()))]
#[case::relative("run/user/1000".to_owned(), None)]
#[case::a_space("/run/user/a b".to_owned(), None)]
#[case::a_parent_component("/run/user/../x".to_owned(), None)]
#[case::a_control_byte("/run/a\nb".to_owned(), None)]
#[case::at_the_cli_bound(
    format!("/{}", "a".repeat(83)),
    Some(format!("uds:/{}/cc-socks/4242.sock", "a".repeat(83)))
)]
#[case::over_the_cli_bound(format!("/{}", "a".repeat(84)), None)]
fn uds_address_composes_the_cli_socket_name(
    #[case] runtime: String,
    #[case] expected: Option<String>,
) {
    assert_eq!(uds_address(std::ffi::OsStr::new(&runtime), 4242), expected);
}

#[test]
fn sender_prompt_holds_the_address_between_its_compiled_parts() {
    let address = "uds:/run/user/1000/cc-socks/4242.sock";
    let prompt = sender_prompt(address).expect("a socket address");
    assert_eq!(
        prompt,
        format!("{CROSS_SESSION_SENDER_HEAD}{address}{CROSS_SESSION_SENDER_TAIL}")
    );
    assert!(prompt.contains(CROSS_SESSION_TEXT));
    assert!(prompt.is_ascii() && !prompt.contains('<') && !prompt.contains('>'));
    assert_eq!(sender_prompt("3f2b9c1e-7a4d-4e8b-9c0f-1d2e3a4b5c6d"), None);
    assert_eq!(sender_prompt("/run/user/1000/cc-socks/4242.sock"), None);
    assert_eq!(sender_prompt("uds:/run/user/a b/cc-socks/4242.sock"), None);
    assert_eq!(
        CROSS_SESSION_SETTINGS,
        r#"{"crossSessionInbound":"accept"}"#
    );
}
