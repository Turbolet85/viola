# Classified — test-plan · e2e-test-strategy.b2

## Answers
e2e-test-strategy#6.4 · no · the Path 5 cleanup (Ctrl-C, child wait, TempDir drop) names no reached file and the session changes no PTY teardown
e2e-test-strategy#7.1 · no · the budget and statusline steps stand: no exit moved (send exit 11, hook exit 0), src/cmd/hook.rs is not in the change, and cli_instance_state.rs changed only its Frozen guard and the stale-holder cases
e2e-test-strategy#7.2 · no · the budget-paused refusal is exit 11 and no exit moved, and its MCP body and the driver hint never naming release (no_hint_names_release) stand
e2e-test-strategy#7.3 · no · the closed-gate, WHEEL-cell and budget claims are exit 11 and unchanged readings, and pause's one changed line is an exit-21 no-endpoint arm that the closed-gate equality does not reach
e2e-test-strategy#7.4 · no · the harness cleanup and TempDir drop name no reached file and the session does not touch teardown
e2e-test-strategy#8.1 · no · the send, answer and stamped verify steps stand: no exit moved, and verify's change adds codes to refusals only, leaving step 4's pass untouched
e2e-test-strategy#8.2 · no · send exit 0, answer exit 12 unverified-cli and verify's pass summary line are unmoved, since no exit moved and verify's change is refusal-only
e2e-test-strategy#8.3 · no · the MCP unverified-cli answer is refusal exit 12, and the MCP body the session changed in mcp_tools.rs is the exit-21 body only
e2e-test-strategy#8.4 · no · the harness cleanup and TempDir drop for H3 name no reached file and the session does not touch teardown
e2e-test-strategy#9.1 · no · the R8 CLAUDE* strip lives in viola-agent-claude, which the report keeps out of the change, and run.rs's change routes json false to Launch without touching env
e2e-test-strategy#9.2 · no · the strip, fd and secret-scan claims rest on the R8 strip the report keeps unchanged, and the run.rs change does not reach env or the process-start fields
e2e-test-strategy#9.3 · no · the TempDir drop cleanup names no reached file and the session does not touch teardown
e2e-test-strategy#10.1 · no · link and unlink records are wrapper writes over the channel, nothing in viola-channel moved, and the reached link.rs and cli_links.rs changes are exit-21 documents with no exit moved
e2e-test-strategy#11.1 · no · the replay readers are unchanged (no reader added, without_endpoint opens nothing), live_endpoint still gives none for a replayed snapshot, and hook connect and the statusline are untouched
e2e-test-strategy#12.1 · no · the sweep's MCP schema claims are invalid-params with no exit moved, hook_fail_open stands in the block, and CI green on 8a4c1a2c covers the handlers.rs text-helper fix
e2e-test-strategy#13.1 · yes · add-to · F8: under --json the cli catch site prints the internal-error document with detail null on stdout, witnessed by cli_verbs_print_the_internal_error_document_under_json, and it joins this CLI --json error-body set
e2e-test-strategy#14.1 · yes · contradict · exit 21's --json document now carries one of five causes in detail, no longer null (F1), cross_exit_causes.rs exists with its per-cause and self-check rows (F2, F3, F9), a stale wrapper reads instance-dead (F4), and revive --json prints documents for its listed codes (F5 to F7)
e2e-test-strategy#15.1 · yes · contradict · the frame verbs' strict-modes refusal document reads detail null, now strict-modes-failed (F1), and the Unix socket-directory refusal has no code this chunk (F10)
e2e-test-strategy#16.1 · no · the list row claims are success-output rendering, the list.rs change is the exit-21 cause on list, cross_list_rows.rs is not reached, and no exit moved
