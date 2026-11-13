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
        let bytes = merge_stamp(None, "2.1.0", &all_pass(), &[], &typed, None, "t");
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
