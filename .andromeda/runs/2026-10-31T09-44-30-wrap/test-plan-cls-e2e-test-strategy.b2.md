# Classified — test-plan · e2e-test-strategy.b2

## Answers
e2e-test-strategy#6.4 · no · its cleanup (outer-PTY Ctrl-C, child wait, TempDir drop) lives in no reached file, and the report reads test-home teardown unchanged
e2e-test-strategy#7.1 · yes · stale · the statusline reading, the 90 and 89 threshold and the send and release steps it drives sit in reached src/cmd/hook/statusline.rs, src/run/budget.rs and src/cmd/client.rs, and no report reading clears them
e2e-test-strategy#7.2 · yes · stale · the send and release refusal documents and the budget.json and marker readings it asserts sit in reached src/cmd/client.rs, src/cmd/hook/statusline.rs and src/run/budget.rs, reached whole
e2e-test-strategy#7.3 · yes · stale · the MCP budget-paused document it equates with the CLI's sits in reached crates/viola-e2e/tests/mcp_tools.rs and crates/viola-mcp/src/result.rs, and both surfaces now share one screen (cross-surface parity)
e2e-test-strategy#7.4 · no · its harness cleanup is the §3 harness contract, which the report reads unmoved with no verb or status shape changed, and no reached file holds it
e2e-test-strategy#8.1 · yes · stale · the unstamped gate, the answer path and the verify run these steps drive sit in reached src/run/version_gate.rs, src/run/dialog.rs and src/cmd/verify.rs
e2e-test-strategy#8.2 · yes · stale · the exit-12 answer document, the MCP refused text and the verify summary line it asserts sit in reached src/cmd/client.rs, crates/viola-mcp/src/result.rs and src/cmd/verify.rs
e2e-test-strategy#8.3 · yes · stale · crates/viola-e2e/tests/mcp_tools.rs, which holds this MCP leg and its unstamped boot, is reached whole and gains three cases
e2e-test-strategy#8.4 · no · harness cleanup and the TempDir drop for H3 are teardown the report reads unchanged under the untouched §3 harness contract, and no reached file holds them
e2e-test-strategy#9.1 · yes · stale · the identity-floor names and the CLAUDE* strip this scenario drives sit in reached src/run/env.rs, and src/cmd/run.rs is reached whole
e2e-test-strategy#9.2 · yes · stale · the stripped-count, kept-name and child-fd readings it asserts sit in reached src/run/env.rs and src/cmd/run.rs, and no report reading clears them
e2e-test-strategy#9.3 · no · its TempDir drop is test-home teardown, which the report reads unchanged, and no reached file holds it
e2e-test-strategy#10.1 · yes · stale · the viola link and unlink verbs it describes sit in reached src/cmd/client.rs, reached whole, and no report reading clears them
e2e-test-strategy#11.1 · no · the report's Untouched reading holds the E5 snapshot replay and viola-state's readers unchanged with no crate-state path changed, so its replay claims still hold
e2e-test-strategy#12.1 · yes · add-to · F6 adds mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line beside this MAX_FRAME case and mcp_tools_a_wrong_shaped_call_answers_no_caller_text beside the schema-violating MCP calls
e2e-test-strategy#13.1 · yes · contradict · the after-every-E2E-test error-body check was measured never built and is replaced by the contract enumeration, the events.rs check and the three surface witnesses, and -32603 now holds only at the unit layer
e2e-test-strategy#14.1 · yes · add-to · F4 adds cli_error_documents_hold_no_path_or_upstream_text, one document, a literal exit and empty stderr across the dead, refused and live CLI rows, beside this matrix's per-case --json assertions
e2e-test-strategy#15.1 · yes · stale · the CLI verb cases sit in reached src/cmd/client.rs and the raw-channel and listener cases in reached crates/viola-channel/src/client.rs and server.rs, reached whole, and only the kill-line line reads no change
e2e-test-strategy#16.1 · no · the list row-integrity suite and src/human/mirror.rs escaping are reported untouched with no human-mode layout moved, and the nextest 20 s kill line at this line reads no change
e2e-test-strategy#17.1 · yes · stale · the wait endpoint-vanish exit 21 and the send input-not-ready exit 13 sit in reached src/cmd/client.rs, reached whole, and only the .cmd item reads unchanged
