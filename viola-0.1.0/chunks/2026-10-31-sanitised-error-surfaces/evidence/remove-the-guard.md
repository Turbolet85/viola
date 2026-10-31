# Remove-the-guard readings

Read 2026-10-31T09:09:48Z, nextest `ci` profile, in the working tree; each plant restored before the next.
Tree diff sha before and after the plants: `cd8c5e46ac19` -> `cd8c5e46ac19`.

| guard | file | reading | failed cases |
|---|---|---|---|
| the client's refusal screen (root bin) | `src/cmd/client.rs` | red (Summary [   0.014s] 15 tests run: 14 passed, 1 failed, 3332 skipped) | viola::bin/viola cmd::client::tests::reply_of_reads_the_three_shapes::case_09_refusal_of_free_text |
| the code word's shape (viola-core) | `crates/viola-core/src/lib.rs` | red (Summary [   0.011s] 1 test run: 0 passed, 1 failed, 3346 skipped) | viola-core tests::code_word_literal_table |
| the MCP detail screen | `crates/viola-mcp/src/result.rs` | red (Summary [   0.017s] 13 tests run: 11 passed, 2 failed, 3334 skipped) | viola-mcp result::tests::of_reply_maps_each_reply_shape::case_11_detail_of_free_text; viola-mcp::handlers handlers_a_refusal_of_free_text_reads_as_unknown_with_no_detail |
| the shared fault reader's fixed message | `crates/viola-channel/src/lib.rs` | red (Summary [   0.013s] 14 tests run: 12 passed, 2 failed, 3333 skipped) | viola-channel tests::sanitised_fault_carries_nothing_of_the_peer_but_closed_values::case_04_internal; viola-channel tests::sanitised_fault_carries_nothing_of_the_peer_but_closed_values::case_14_no_message_and_array_data |
| the server line's cause word | `crates/viola-channel/src/server.rs` | red (Summary [  11.923s] 3 tests run: 0 passed, 3 failed, 3344 skipped) | viola-channel server::tests::answer_refuses_a_newer_version_with_supported_and_wrapper; viola-channel server::tests::answer_release_from_driver_is_invalid_params_with_its_reason; viola::channel_error_bodies channel_error_bodies_hold_no_path_or_upstream_text |
| the client line's cause word | `crates/viola-channel/src/client.rs` | red (Summary [   0.012s] 3 tests run: 1 passed, 2 failed, 3344 skipped) | viola-channel client::tests::reply_from_logs_the_cause_of_each_invalid_params::case_2_invalid_params; viola-channel client::tests::reply_from_logs_the_cause_of_each_invalid_params::case_3_driver_release |
| the schema's closed detail | `schemas/diag-line.v1.json` | red (Summary [   4.916s] 1 test run: 0 passed, 1 failed, 0 skipped) | viola::channel_error_bodies channel_error_bodies_hold_no_path_or_upstream_text |
| a reader's cap: thresholds | `src/run/budget.rs` | red (Summary [   0.097s] 1 test run: 0 passed, 1 failed, 3346 skipped) | viola::bin/viola run::budget::tests::read_thresholds_keeps_no_key_past_the_frame_cap |
| a reader's cap: dialog hold | `src/run/dialog.rs` | red (Summary [   0.100s] 1 test run: 0 passed, 1 failed, 3346 skipped) | viola::bin/viola run::dialog::tests::read_hold_keeps_no_key_past_the_frame_cap |
| a reader's cap: claude_env_keep | `src/run/env.rs` | red (Summary [   0.096s] 1 test run: 0 passed, 1 failed, 3346 skipped) | viola::bin/viola run::env::tests::env_keep_keeps_no_key_past_the_frame_cap |
| a reader's cap: the status line's stdout | `src/cmd/hook/statusline.rs` | red (Summary [   0.032s] 1 test run: 0 passed, 1 failed, 3346 skipped) | viola::bin/viola cmd::hook::statusline::tests::shell::statusline_shell_out_past_the_frame_cap_keeps_exactly_the_cap |
| a reader's cap: the child pipe drain | `src/run/version_gate.rs` | red (Summary [   0.060s] 1 test run: 0 passed, 1 failed, 3346 skipped) | viola::bin/viola run::version_gate::tests::drain_keeps_cap_bytes_of_an_endless_pipe |
| the census: a writer no row names | `src/cmd/plugin.rs` | red (Summary [   0.187s] 1 test run: 0 passed, 1 failed, 16 skipped) | viola::contract_error_bodies product_error_writers_are_each_named |
| the census: a message that interpolates | `crates/viola-agent-claude/src/lib.rs` | first plant `{0}` on a tuple variant did not build (exit 101, no reading); re-planted `#[error("the command was not found {}", 1)]`, read 2026-10-31T09:10:06Z: red (Summary [   0.081s] 1 test run: 0 passed, 1 failed, 16 skipped) | viola::contract_error_bodies thiserror_messages_interpolate_nothing |

Restored, the union of the filters: exit 0, `Summary [   4.912s] 56 tests run: 56 passed, 3291 skipped`. The re-planted control restored: `1 test run: 1 passed, 16 skipped`. Tree diff sha after it: `cd8c5e46ac19`, unchanged.

The client line's control plants every `-32602` as `unsupported-version`, so its `newer_peer` case stays green by design and the other two read red.
