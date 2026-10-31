# Classified — test-plan · e2e-test-strategy.b2

## Answers
e2e-test-strategy#6.4 · no · Path 5 cleanup is a PTY Ctrl-C, child wait and TempDir drop; no reached file carries it and no step changed.
e2e-test-strategy#7.1 · yes · stale · step 1's statusline pass-through witness lives in reached tests/cli_instance_state.rs, which the report does not show unmoved (the change there is the Frozen guard).
e2e-test-strategy#7.2 · no · exit 11 and exit 0 are unmoved (no exit moved, no RefusalReason added), hooks stay exit 0 with empty stderr, and the no-release hint holds by F3's no_hint_names_release self-check.
e2e-test-strategy#7.3 · no · budget-paused stays exit 11 with its RefusalReason (no exit moved, none added), and answer, wait, last and pause changed only at their no-endpoint arms, which an open gate's live instance never reaches.
e2e-test-strategy#7.4 · no · harness cleanup and TempDir drop; no harness verb or status shape moved.
e2e-test-strategy#8.1 · no · the steps hold: answer's exit 12 is unmoved, verify keeps its no --json (F12) and changed only its refusal-line logging (F11), and answer changed only at its no-endpoint arm.
e2e-test-strategy#8.2 · no · answer's exit 12 document and verify's stamped pass line are unmoved, since verify changed only the code on refusal lines (F11).
e2e-test-strategy#8.3 · no · the unverified-cli refusal is exit 12 with its RefusalReason, and no exit moved and no RefusalReason was added, so its MCP leg in reached mcp_tools.rs still holds.
e2e-test-strategy#8.4 · no · harness cleanup and TempDir drop; no harness verb moved.
e2e-test-strategy#9.1 · no · the R8 strip and identity floor are in viola-agent-claude, not in the change; run changes only the json flag passed to Launch.
e2e-test-strategy#9.2 · no · the env strip and child fd claims are the untouched R8 strip, and run's change does not touch process-start's env fields.
e2e-test-strategy#9.3 · no · TempDir drop; no teardown step moved.
e2e-test-strategy#10.1 · yes · stale · cli_links.rs is reached and its link and unlink record claims are not shown unmoved, while the link verb's no-endpoint arm changed (src/cmd/link.rs).
e2e-test-strategy#11.1 · yes · add-to · F4: a frame verb against a stale instance reads instance-dead because live_endpoint gives none for a stale wrapper, the same none a replayed snapshot returns here.
e2e-test-strategy#12.1 · yes · stale · the MCP schema-violation and liveness witnesses in reached handlers.rs, tools.rs and mcp_tools.rs are not shown unmoved by the report.
e2e-test-strategy#13.1 · yes · add-to · F8: under --json the cli catch site prints an internal-error document on stdout, a new CLI --json body the no-absolute-path, serde-path and Caused-by check must cover.
e2e-test-strategy#14.1 · yes · contradict · the table is built, not "not built" (F2, F3), exit 21's document carries one of five causes in detail (F1, F4, F9, F14), and revive, verify and catch-site documents carry codes (F5, F6, F7, F8, F11, F12, F13).
e2e-test-strategy#15.1 · yes · add-to · F1: the strict-modes and server-verify exit-21 documents now carry their cause in detail; F10: the socket-directory refusal has no code in this chunk.
e2e-test-strategy#16.1 · no · the ci kill line it names gains an operand only, and a site naming that line without a number stays true; the row assertions are in cross_list_rows.rs, not reached.
