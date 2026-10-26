# Remove-the-guard controls — 2026-10-26-home-and-code-bearing-file-integrity

testing.md 2026-09-25: every new guard test carries its remove-the-guard run. Each guard was neutralised ALONE
in the working tree (HEAD `99df45362ac6` plus this chunk's uncommitted change), its witnesses run by
`cargo nextest run {pkg} --no-fail-fast -E {filter}`, then the file restored from the bytes read before the edit
and read back by SHA-256 (`restored byte-equal`). The script is `$TMPDIR/{scratchpad}/controls.py`; its run
began 2026-10-26T19:35Z and was interrupted by SIGINT at 2026-10-26T20:28:03Z inside G10 (the overseer's
direction, 20:27Z), after the hang recorded below. Every reading is copied verbatim from the script's output.

## The guard on, the control

The whole workspace suite on the same tree, every guard in place: `3220 tests run: 3220 passed, 0 skipped`
(ended before the controls began at 2026-10-26T19:35Z, journal). That is each witness's green; the blocks below are its red.

## Readings, G1–G9 (guard removed: red; file restored byte-equal)

```
== G1 frame verbs' refuse_untrusted · src/cmd/client.rs · anchors 1 · edit landed True · restored byte-equal True · exit 100
   FAIL [   0.008s] (1/7) viola::cli_controls_not_disableable setting_does_not_disable_the_home_check
   FAIL [   5.010s] (2/7) viola::security_negatives_strict_modes frame_verb_refuses_a_widened_instance_before_any_log::case_3_last
   FAIL [   5.010s] (3/7) viola::security_negatives_strict_modes frame_verb_refuses_a_widened_instance_before_any_log::case_6_release
   FAIL [   5.010s] (4/7) viola::security_negatives_strict_modes frame_verb_refuses_a_widened_instance_before_any_log::case_4_answer
   FAIL [   5.011s] (5/7) viola::security_negatives_strict_modes frame_verb_refuses_a_widened_instance_before_any_log::case_5_pause
   FAIL [   5.212s] (6/7) viola::security_negatives_strict_modes frame_verb_refuses_a_widened_instance_before_any_log::case_2_wait
   FAIL [  12.004s] (7/7) viola::security_negatives_strict_modes frame_verb_refuses_a_widened_instance_before_any_log::case_1_send
   Summary [  12.006s] 7 tests run: 0 passed, 7 failed, 1762 skipped
== G2 hook event/dialog instance check · src/cmd/hook.rs · anchors 1 · edit landed True · restored byte-equal True · exit 100
   FAIL [   0.309s] (1/4) viola::hook_fail_open hook_refused_by_the_instance_check_fails_open_and_writes_nothing::case_4_permission
   FAIL [   0.312s] (2/4) viola::hook_fail_open hook_refused_by_the_instance_check_fails_open_and_writes_nothing::case_1_stop
   FAIL [   0.312s] (3/4) viola::hook_fail_open hook_refused_by_the_instance_check_fails_open_and_writes_nothing::case_3_question
   FAIL [   0.312s] (4/4) viola::hook_fail_open hook_refused_by_the_instance_check_fails_open_and_writes_nothing::case_2_session_end
   Summary [   0.313s] 4 tests run: 0 passed, 4 failed, 1765 skipped
== G2b statusline instance check · src/cmd/hook/statusline.rs · anchors 1 · edit landed True · restored byte-equal True · exit 100
   FAIL [   4.886s] (1/1) viola::hook_statusline hook_statusline_a_widened_instance_runs_and_logs_nothing
   Summary [   4.888s] 1 test run: 0 passed, 1 failed, 1768 skipped
== G3 mcp frame check · crates/viola-mcp/src/tools.rs · anchors 1 · edit landed True · restored byte-equal True · exit 100
   FAIL [   0.009s] (1/1) viola-mcp::handlers handlers_frame_tools_refuse_a_widened_instance_before_any_frame
   Summary [   0.010s] 1 test run: 0 passed, 1 failed, 64 skipped
== G4 run home-tree check · src/cmd/run.rs · anchors 1 · edit landed True · restored byte-equal True · exit 100
   FAIL [   0.230s] (1/5) viola::security_negatives_strict_modes a_home_another_user_may_read_refuses_run
   Summary [3059.428s] 5 tests run: 0 passed, 5 failed, 1764 skipped
== G5 verify home check · src/cmd/verify.rs · anchors 1 · edit landed True · restored byte-equal True · exit 100
   FAIL [   0.189s] (1/2) viola::cli_verify verify_under_a_regular_file_refuses_the_home
   FAIL [   9.347s] (2/2) viola::cli_verify verify_refuses_a_widened_ledger_before_any_step
   left: "error: internal error\n"
   Summary [   9.349s] 2 tests run: 0 passed, 2 failed, 1767 skipped
== G6 update_stamps checks · crates/viola-state/src/stamps.rs · anchors 2 · edit landed True · restored byte-equal True · exit 100
   FAIL [   0.004s] (1/1) viola-state stamps::tests::update_stamps_refuses_a_widened_ledger_and_leaves_it
   Summary [   0.004s] 1 test run: 0 passed, 1 failed, 299 skipped
== G7 diagnostics checks · src/obs.rs · anchors 1 · edit landed True · restored byte-equal True · exit 100
   FAIL [   0.005s] (1/4) viola::bin/viola obs::tests::open_role_file_refuses_a_symlinked_role_file
   FAIL [   0.005s] (2/4) viola::bin/viola obs::tests::open_role_file_refuses_a_widened_diagnostics_path_and_leaves_it::case_3_file_group_readable
   FAIL [   0.005s] (3/4) viola::bin/viola obs::tests::open_role_file_refuses_a_widened_diagnostics_path_and_leaves_it::case_2_dir_other_writable
   FAIL [   0.005s] (4/4) viola::bin/viola obs::tests::open_role_file_refuses_a_widened_diagnostics_path_and_leaves_it::case_1_dir_group_readable
   Summary [   0.007s] 4 tests run: 0 passed, 4 failed, 1765 skipped
== G7b detail-file check · src/obs.rs · anchors 1 · edit landed True · restored byte-equal True · exit 100
   FAIL [   0.005s] (1/1) viola::bin/viola obs::tests::write_detail_refuses_a_widened_diagnostics_folder_and_leaves_it
   Summary [   0.007s] 1 test run: 0 passed, 1 failed, 1768 skipped
== G8 unix read rule · crates/viola-state/src/strict.rs · anchors 1 · edit landed True · restored byte-equal True · exit 100
   FAIL [   0.010s] ( 3/33) viola-state strict::tests::check_diagnostics_refuses_a_widened_mode::case_3_file_group_readable
   FAIL [   0.014s] ( 4/33) viola-state strict::tests::check_diagnostics_refuses_a_widened_mode::case_1_dir_group_readable
   FAIL [   0.014s] ( 6/33) viola-state strict::tests::unix_read_verdict_needs_the_write_rule_and_no_foreign_bit::case_3_group_read_dir
   FAIL [   0.014s] (21/33) viola-state strict::tests::unix_read_verdict_needs_the_write_rule_and_no_foreign_bit::case_4_other_read_dir
   FAIL [   0.014s] (23/33) viola-state strict::tests::unix_read_verdict_needs_the_write_rule_and_no_foreign_bit::case_6_other_exec_dir
   FAIL [   0.014s] (25/33) viola-state strict::tests::strict_instance_holds_the_content_folders_to_the_read_rule::case_3_diagnostics_group_readable
   FAIL [   0.014s] (28/33) viola-state strict::tests::unix_read_verdict_needs_the_write_rule_and_no_foreign_bit::case_7_group_read_file
   FAIL [   0.015s] (14/33) viola-state strict::tests::check_home_tree_refuses_each_widened_path::case_01_home_group_readable
   FAIL [   0.015s] (17/33) viola-state strict::tests::check_home_tree_refuses_each_widened_path::case_05_instance_diagnostics_other_readable
   FAIL [   0.015s] (32/33) viola-state strict::tests::strict_instance_holds_the_content_folders_to_the_read_rule::case_1_home_group_readable
   FAIL [   0.016s] (19/33) viola-state strict::tests::check_home_tree_refuses_each_widened_path::case_04_instance_group_readable
   FAIL [   0.024s] (24/33) viola-state strict::tests::strict_instance_holds_the_content_folders_to_the_read_rule::case_2_instance_other_readable
   FAIL [   0.024s] (27/33) viola-state strict::tests::unix_read_verdict_needs_the_write_rule_and_no_foreign_bit::case_5_group_exec_dir
   Summary [   0.025s] 33 tests run: 20 passed, 13 failed, 267 skipped
== G8b windows read rule · crates/viola-state/src/strict.rs · anchors 1 · edit landed True · restored byte-equal True · exit 100
   FAIL [   0.004s] (3/6) viola-state strict::tests::windows_read_verdict_refuses_any_read_right_to_another_sid::case_1_read_data
   FAIL [   0.004s] (4/6) viola-state strict::tests::windows_read_verdict_refuses_any_read_right_to_another_sid::case_4_read_and_execute
   FAIL [   0.004s] (5/6) viola-state strict::tests::windows_read_verdict_refuses_any_read_right_to_another_sid::case_2_generic_read
   FAIL [   0.004s] (6/6) viola-state strict::tests::windows_read_verdict_refuses_any_read_right_to_another_sid::case_3_inherit_only_generic_read
   Summary [   0.005s] 6 tests run: 2 passed, 4 failed, 294 skipped
== G8c diagnostics symlink · crates/viola-state/src/strict.rs · anchors 1 · edit landed True · restored byte-equal True · exit 100
   FAIL [   0.004s] (1/1) viola-state strict::tests::check_diagnostics_refuses_a_symlinked_role_file_and_folder
   Summary [   0.004s] 1 test run: 0 passed, 1 failed, 299 skipped
== G9 live_endpoint replay · crates/viola-state/src/liveness.rs · anchors 1 · edit landed True · restored byte-equal True · exit 100
   FAIL [   0.006s] (1/2) viola-state liveness::tests::live_endpoint_of_a_replayed_snapshot_is_none_with_its_one_line::case_1_cut_short
   FAIL [   0.007s] (2/2) viola-state liveness::tests::live_endpoint_of_a_replayed_snapshot_is_none_with_its_one_line::case_2_unsupported_v
   Summary [   0.007s] 2 tests run: 0 passed, 2 failed, 298 skipped
```

Read with the filter, never a whole-list count: a PASS row under G8 / G8b is a case the read rule does not decide
(a write-rule case, `0o640` on `instances/` or the snapshot, `read_attributes_only`, the inherit-only `GENERIC_ALL`
row refused `Writable` by the write rule) — each read the same with the guard on, by its table.

## G4's reading is not a control yet

G4's one assertion-red row is `a_home_another_user_may_read_refuses_run` (0.230 s). Its other four witnesses,
`run_refuses_a_widened_home_tree_before_its_log::case_{1..4}`, HUNG: with the guard removed `viola run` was not
refused, started a real wrapper around the fake agent with stdin null, and the witness's `Command::output()` never
returned (nextest's default profile has no kill line). They were ended by SIGTERM to the four test processes on
the overseer's word (20:19Z for pids 3046386, 3046387; 20:27Z for 3046388, 3046390), so they read FAIL by kill, not
by an assertion; the eight wrappers and fake agents they started were terminated by pid after their exe and
home path were read (journal). The witness is re-written with a kill deadline and G4 re-fired (below).

## G4, G9, G10, G11, G12 — re-fired with bounded witnesses (2026-10-26T20:30Z)

The two witnesses whose `viola run` starts a real wrapper once its guard is removed —
`run_refuses_a_widened_home_tree_before_its_log` (G4) and
`a_start_over_a_replayed_snapshot_and_a_fresh_beat_refuses_already_live` (G10) — now spawn through
`support::cli::Running`: `finish()` fails the case at `WITHIN` (7 s) and its `Drop` kills the child. With every
guard on they read `7 tests run: 7 passed`. Re-fired by `python3 -u controls.py G4 G9 G10 G11 G12`
(20:30:00Z–20:30:43Z); census after it: `census: none running`.

```
== G4 run home-tree check · src/cmd/run.rs · anchors 1 · edit landed True · restored byte-equal True · exit 100
   FAIL [   0.251s] (1/5) viola::security_negatives_strict_modes a_home_another_user_may_read_refuses_run
   FAIL [   7.255s] (2/5) viola::run_cli run_refuses_a_widened_home_tree_before_its_log::case_3_bin
   FAIL [   7.255s] (3/5) viola::run_cli run_refuses_a_widened_home_tree_before_its_log::case_4_pinned_copy
   FAIL [   7.255s] (4/5) viola::run_cli run_refuses_a_widened_home_tree_before_its_log::case_2_instance_dir
   FAIL [   7.267s] (5/5) viola::run_cli run_refuses_a_widened_home_tree_before_its_log::case_1_home
   Summary [   7.268s] 5 tests run: 0 passed, 5 failed, 1764 skipped
== G9 live_endpoint replay · crates/viola-state/src/liveness.rs · anchors 1 · edit landed True · restored byte-equal True · exit 100
   FAIL [   0.006s] (1/2) viola-state liveness::tests::live_endpoint_of_a_replayed_snapshot_is_none_with_its_one_line::case_2_unsupported_v
   FAIL [   0.007s] (2/2) viola-state liveness::tests::live_endpoint_of_a_replayed_snapshot_is_none_with_its_one_line::case_1_cut_short
   Summary [   0.007s] 2 tests run: 0 passed, 2 failed, 298 skipped
== G10 collision replay beat · src/cmd/run.rs · anchors 1 · edit landed True · restored byte-equal True · exit 100
   FAIL [   0.007s] (3/5) viola::bin/viola cmd::run::tests::collision_check_over_a_replayed_snapshot_reads_the_beat_alone::case_4_unsupported_v_fresh
   FAIL [   0.007s] (4/5) viola::bin/viola cmd::run::tests::collision_check_over_a_replayed_snapshot_reads_the_beat_alone::case_2_cut_short_fresh
   FAIL [  11.906s] (5/5) viola::chaos_snapshot_replay a_start_over_a_replayed_snapshot_and_a_fresh_beat_refuses_already_live
   PASS [   0.006s] (1/5) viola::bin/viola cmd::run::tests::collision_check_over_a_replayed_snapshot_reads_the_beat_alone::case_1_cut_short_stale
   PASS [   0.007s] (2/5) viola::bin/viola cmd::run::tests::collision_check_over_a_replayed_snapshot_reads_the_beat_alone::case_3_unsupported_v_stale
   Summary [  11.908s] 5 tests run: 2 passed, 3 failed, 1764 skipped
== G11 hook connect replay · src/cmd/hook.rs · anchors 1 · edit landed True · restored byte-equal True · exit 100
   FAIL [   0.006s] (1/2) viola::bin/viola cmd::hook::tests::connect_over_a_replayed_snapshot_is_unreachable::case_2_unsupported_v
   FAIL [   0.007s] (2/2) viola::bin/viola cmd::hook::tests::connect_over_a_replayed_snapshot_is_unreachable::case_1_cut_short
   Summary [   0.009s] 2 tests run: 0 passed, 2 failed, 1767 skipped
== G12 statusline serve replay · src/cmd/hook/statusline.rs · anchors 1 · edit landed True · restored byte-equal True · exit 100
   FAIL [   0.006s] (1/2) viola::bin/viola cmd::hook::statusline::tests::statusline_over_a_replayed_snapshot_runs_nothing::case_1_cut_short
   FAIL [   0.006s] (2/2) viola::bin/viola cmd::hook::statusline::tests::statusline_over_a_replayed_snapshot_runs_nothing::case_2_unsupported_v
   Summary [   0.008s] 2 tests run: 0 passed, 2 failed, 1767 skipped
```

G4's four `run_refuses…` cases now read red by the 7 s deadline (`viola never exited`), never by a hang. G10's two
PASS rows are the stale-beat cases, which read free whether the replay arm is there or not (the neutraliser
removes only the fresh beat's refusal); its three fresh-beat witnesses read red.

## Not measurable on this host

The `conpty/` owner/DACL leg (`pin_companions`' `check_present`, plan step 10) is Windows x64 only: its witnesses
(`pin::tests::pin_companions_refuses_a_path_another_user_may_write`, `conpty_sideload`'s
`run_on_a_companion_another_user_may_write_falls_back_to_conhost`) run on CI's `windows-2025` leg alone, and no
remove-the-guard reading of them is taken here.
