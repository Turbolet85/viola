# Red-before-green — the server check neutralised, then restored

Taken at implement P2 (2026-10-26, Linux dev host, on the chunk's working tree over HEAD 6bdfdda8).

**The neutralising edit** (`crates/viola-state/src/liveness.rs`, `server_verdict`): the guard
`if !peer_pid_reported {` read as `if !peer_pid_reported || recorded_pid > 0 {`, so every verdict is
`Matches` — the build with no server check. Re-read before the run (`recorded_pid > 0` present at :123)
and after the restore (`grep -c 'recorded_pid > 0'` = 0).

**The run:** `cargo nextest run --workspace --features fake-agent -E 'binary(security_negatives_server_identity) | binary(hook_fail_open) | binary(handlers) | test(/server_verdict_rows|verify_server_reads|ask_writes_no_frame|handle_appends_only_a_session_end/)' --no-fail-fast`, exit 100:

    Summary [   0.274s] 72 tests run: 47 passed, 25 failed, 2976 skipped

The 25 red (each its case's own assertion):
- `viola::security_negatives_server_identity security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper` — all 12 rows (send, wait, last, answer, pause, release × json, human)
- `viola::hook_fail_open hook_fails_open_silently_within_the_spine_bound::case_12_impostor`
- `viola::hook_fail_open hook_dialog_tier_fails_open_silently::case_5_question_impostor`, `::case_6_permission_impostor`
- `viola-mcp::handlers handlers_frame_tools_write_no_frame_to_a_server_that_is_not_the_recorded_wrapper`
- `viola::bin/viola cmd::hook::tests::ask_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper::case_1_another_start_time`, `::case_2_started_at_not_a_time`
- `viola::bin/viola cmd::hook::tests::handle_appends_only_a_session_end_the_channel_did_not_take::case_2_server_unverified`
- `viola-state liveness::tests::server_verdict_rows::case_2_another_pid`, `case_3_another_start_time`, `case_4_started_at_not_rfc3339`, `case_5_no_observed_pid`, `case_6_the_observed_process_gone`
- `viola-state liveness::tests::verify_server_reads_the_observed_start_time_from_the_os`

Green under the neutralised check, as they must be (the controls and the rows a no-check build also
satisfies): the six `security_negative_cli_verb_reaches_the_recorded_wrapper` controls, every
pre-existing `hook_fail_open` and `handlers` case, the `server_verdict_rows` match and no-peer-pid rows,
`handle_appends_only_a_session_end_the_channel_did_not_take::case_1_unreachable`.

**Restored:** the same selection green on the restored tree before the block run (the gate run's
entries 8–10 and 7 are the record).

## Second reading — the captured-span cases (the operator's must-trace check)

Added after block run 1: one `request_refuses_an_unverified_server_inside_its_dispatch_span` case in each of
`src/cmd/{send,wait,last,answer,pause,release}.rs` and `cmd::hook::tests::hook_refuses_an_unverified_server_inside_hook_handle`.
The same neutralising edit, re-read before (`grep -c 'recorded_pid > 0'` = 1) and after the restore (= 0):

    cargo nextest run --workspace --features fake-agent --lib --bins -E 'test(/request_refuses_an_unverified_server|hook_refuses_an_unverified_server/)' --no-fail-fast
    Summary [   0.011s] 7 tests run: 0 passed, 7 failed, 2438 skipped

All seven red under the neutralised check; green on the restored tree (11 run with the hook unit cases, 11 passed).
