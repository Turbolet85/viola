# Classified — test-plan · e2e-test-strategy.b1

## Answers
e2e-test-strategy#1.1 · yes · add-to · cli `--json` failures stay one document with empty stderr, the catch site now prints the internal-error document under `--json` (F8), and `cross_exit_causes.rs` adds each cause's document and hint pair to the cli surface (F2)
e2e-test-strategy#1.2 · no · the hook surface and hook fail-open stand; `src/cmd/hook.rs` is not in the change and hooks stay exit 0 with empty stderr
e2e-test-strategy#1.3 · no · run passthrough verdicts come from the event log, receipt and exit status, all unchanged; run's exit and human pairs stand and no claim here names a cause or detail
e2e-test-strategy#1.4 · no · the wrapper channel's framing, `sanitised_fault`, endpoint naming and fault rows are untouched and `tests/channel_error_bodies.rs` is not reached
e2e-test-strategy#1.5 · no · `crates/viola-e2e/tests/mcp_tools.rs` is reached, but `MAX_FRAME`, the one oversize `parse-rejected` line, the code-only `isError` bodies and MCP's text are unchanged and the two silent `viola mcp` exits stand
e2e-test-strategy#1.6 · no · names only the cargo-nextest version and the doctest run; no runner, version or Cargo test-binary claim here moved
e2e-test-strategy#2.1 · no · the Path 1 start sequence, its exit 1 refusals and its events and snapshot records are unchanged; no exit 21 or detail claim is made
e2e-test-strategy#2.2 · no · the run home-tree and instance refusals stay exit 1 and open no log and write no line; the reached `cli_instance_state.rs` and `security_negatives_strict_modes.rs` keep their named claims, since no exit moved and no `RefusalReason` was added
e2e-test-strategy#2.3 · no · helpers shared through `tests/support/start.rs` as stated (e2e :38 re-read, stays true), and the ci kill line is named without its count, so the 24 to 25 operand change leaves it true
e2e-test-strategy#2.4 · no · the Path 1 cleanup lives in the unreached `path1_run_start` binary, and the `support/start.rs` extraction moved only start setups
e2e-test-strategy#3.1 · no · the path2 success witness and the MCP send parity with the CLI document hold; the change moved no success body and added no `RefusalReason`
e2e-test-strategy#3.2 · no · the send procedure steps name no exit 21 path and the send refusal bodies they reach are unmoved
e2e-test-strategy#3.3 · no · the named `cli_send.rs` cases assert exits 0, 10 and 13 with their records and bodies, all unmoved since no exit moved and no `RefusalReason` was added
e2e-test-strategy#3.4 · no · the send cleanup command is unchanged
e2e-test-strategy#4.1 · yes · add-to · the vanished-wrapper witnesses gain MCP's endpoint-lost witness `mcp_tools_a_wrapper_killed_under_a_parked_wait_reads_endpoint_lost` (F9), and `chaos_wait_parked_wrapper_killed_exits_21` stays exit 21
e2e-test-strategy#4.2 · no · the wait, last and send procedure steps and the chaos helper's kill are unchanged in use, and the helper is named by role, not by the file's own kill
e2e-test-strategy#4.3 · yes · contradict · step 7 reads `detail` null on the CLI and the MCP body, but a wrapper killed under a parked wait is now `endpoint-lost` in exit 21 and in the MCP body less `v` (F1, F9)
e2e-test-strategy#4.4 · no · harness cleanup of killed wrappers, verified by sysinfo pid and start time, is unchanged and states no cause or detail
e2e-test-strategy#5.1 · no · the dialog witnesses, `answer` successes and the exit 13 and exit 12 refusals are unchanged, and the MCP answer refusal compared here is `unknown-dialog`, unmoved
e2e-test-strategy#5.2 · no · the answer procedure steps reach only the success and exit 10 and 13 paths, all unchanged
e2e-test-strategy#5.3 · no · the `answer` exit 13 `unknown-dialog` body, the exit 10 body and MCP's `is_error:false` are unmoved and no exit 21 claim is made
e2e-test-strategy#5.4 · no · the harness cleanup is unchanged
e2e-test-strategy#6.1 · no · the `detail` null named here belongs to `human-typing` and was re-read and stays (e2e :132), and the wheel and MCP refusal claims rest on unreached tests and an unchanged `refused: human-typing` text
e2e-test-strategy#6.2 · no · the wheel procedure steps reach the send, pause and release exits 10, 13 and 20, all unchanged
e2e-test-strategy#6.3 · yes · add-to · "driver-facing hints never suggest release" gains its self-check `no_hint_names_release` over the table's hints (F3), which must cover the two new driver-facing hints in reached `hints.rs`
