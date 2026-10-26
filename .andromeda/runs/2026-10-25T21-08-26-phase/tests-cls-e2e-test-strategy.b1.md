# Classified — tests · e2e-test-strategy.b1

## Answers
e2e-test-strategy#1.1 · yes · the cli driver (assert_cmd and predicates, trycmd for human layouts) that reads the squatted-name exit-1 `--json` document and `hint:` line the chunk re-asserts
e2e-test-strategy#1.2 · yes · the rule that `.env_clear()` must re-add `LLVM_PROFILE_FILE` is the anti-pattern the new listener tests must keep
e2e-test-strategy#1.3 · yes · the outer-PTY `viola run` driver (exit only via child.wait(), exit status as verdict) that tests/path1_run_start.rs runs its wrapper under, beside the squatted exit-1 leg the chunk re-verifies
e2e-test-strategy#1.4 · yes · the wrapper-channel driver (viola-channel client, interprocess raw stream, endpoint read only from snapshot.json) carries the squatter-first and remote-client negatives
e2e-test-strategy#1.5 · no · the MCP stdio driver serves viola-mcp, which the chunk does not touch, and the MCP surface is not needed
e2e-test-strategy#1.6 · yes · the library-only driver (cargo-nextest, cargo test --doc) runs the viola-channel unit tests the chunk edits in win.rs and server.rs
e2e-test-strategy#2.1 · yes · step 4 squats the endpoint through the interprocess server API before run, the squatted leg the chunk re-verifies, and step 1's readiness wait is the witness the wait_ready stall report carries
e2e-test-strategy#2.2 · yes · the squatted endpoint exits 1, the refusal the chunk re-asserts against tests/path1_run_start.rs
e2e-test-strategy#2.3 · yes · the one ordered flow in tests/path1_run_start.rs holds the squatted leg, and its start-refusal helpers are shared with tests/cli_instance_state.rs
e2e-test-strategy#2.4 · yes · the Drop guard over wrappers, the TempDir removal of H and H2 and the dropped squatter are the lifecycle the new listener tests keep
e2e-test-strategy#3.1 · no · the Path 2 send surfaces (cli_send.rs, MCP send, tui receipt) are not reached by the chunk, and its same-user channel send is a regression check every change owes
e2e-test-strategy#3.2 · no · the Path 2 send steps are not reached by the chunk, whose only send-side edit is a root unit test of the budget refusal
e2e-test-strategy#3.3 · no · the Path 2 send records and layouts (mute, local, empty text, human mode) are not the budget-paused refusal the chunk re-asserts
e2e-test-strategy#3.4 · no · the Path 2 cleanup is the harness agent-run cleanup, which the chunk does not use
e2e-test-strategy#4.1 · no · the Path 3 wait and last surfaces are not reached by the chunk
e2e-test-strategy#4.2 · no · the Path 3 wait and last steps (chaos kill, MCP wait and last) are not reached by the chunk
e2e-test-strategy#4.3 · no · the Path 3 wait verification (turn-ended, timed_out, exit 21) is not reached by the chunk
e2e-test-strategy#4.4 · no · the Path 3 cleanup checks killed wrappers by sysinfo, a check the chunk does not run
e2e-test-strategy#5.1 · no · the Path 4 dialog surfaces (hook and answer) are not reached by the chunk
e2e-test-strategy#5.2 · no · the Path 4 dialog steps (PreToolUse, PermissionRequest, answer by id) are not reached by the chunk
e2e-test-strategy#5.3 · no · the Path 4 dialog verification (hook stdout, unknown-dialog exit 13) is not reached by the chunk
e2e-test-strategy#5.4 · no · the Path 4 cleanup is the harness cleanup, which the chunk does not use
e2e-test-strategy#6.1 · no · the Path 5 wheel surfaces (outer-PTY keystrokes, pause, release) are not reached by the chunk
e2e-test-strategy#6.2 · no · the Path 5 wheel steps (human keys, pause, release, harness turn) are not reached by the chunk
e2e-test-strategy#6.3 · yes · a refused send writes one send-refused record with no cursor and no send-issued, the refusal shape case_4 in src/run/send/budget.rs re-asserts
