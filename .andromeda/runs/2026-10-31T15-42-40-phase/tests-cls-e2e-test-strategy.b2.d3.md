# Classified — tests · e2e-test-strategy.b2

## Answers
e2e-test-strategy#6.4 · no · Path 5 human-wheel refusal teardown; Path 5 wheel flows are Not needed and the chunk does not reach its child or TempDir
e2e-test-strategy#7.1 · yes · step 6 (chmod the home to 0770, then pipe statusline stdin into the hook) is the widened-home hook refusal the no-line rule and the hook's exit-0 contract cover; the other Path 6 budget steps are Not needed
e2e-test-strategy#7.2 · yes · step 6 outcome binds: a hook refused by strict-modes exits 0 with empty stdout and runs no statusline command, under the hook contract (exit 0, no body) and the no-line rule; the budget clauses are Not needed
e2e-test-strategy#7.3 · no · as-landed Path 6 budget tests, the list budget cell and the budget-paused MCP form; budget-paused is exit 11, under Not needed
e2e-test-strategy#7.4 · no · Path 6 harness cleanup; Path 6 flows are Not needed
e2e-test-strategy#8.1 · no · Path 7 version-gate boots and a fresh-home verify; the unverified-cli flow is Not needed and no refusal here is an exit 1 or 21 cause
e2e-test-strategy#8.2 · yes · the hook clause binds: the question hook's stdout is empty with exit 0 within the spine bound, the hook contract the scope names (exit 0, no body); the exit-12 answer and verify lines are Not needed
e2e-test-strategy#8.3 · no · Path 7 MCP leg booting an unstamped pair; the version gate is Not needed and the leg states no exit 1 or 21 cause
e2e-test-strategy#8.4 · no · Path 7 harness cleanup and H3 TempDir drop; the version-gate flow is Not needed
e2e-test-strategy#9.1 · no · E2 env-strip run with the CLAUDE* names; Scenario E2's environment handling is Not needed
e2e-test-strategy#9.2 · no · standing S12 · every new or changed process-log line holds no stripped-variable value
e2e-test-strategy#9.3 · no · E2 TempDir cleanup; E2 is Not needed
e2e-test-strategy#10.1 · no · E3 link and unlink records and their send-side lines; the chunk changes no link record, and the link verbs' exit 21 rows are carried by #14.1
e2e-test-strategy#11.1 · yes · the replay readers bind: revive over a replayed snapshot is refused cwd-missing, run's collision check reads the beat alone (already-live or a free name), hook connect reads the instance unreachable, and list reads through the replay
e2e-test-strategy#12.1 · yes · the hook fail-open matrix holds the hook bound (exit 0, empty streams, no frame, no line), the impostor rows' hook-decision detail server-verify-failed, and the --home strict-modes refusals
e2e-test-strategy#13.1 · yes · the error-body rules bind every new code and the -32603 body: CLI --json, MCP isError and channel error.data hold no absolute path, serde path or Caused by, and internal-error stays "internal error" with null data
e2e-test-strategy#14.1 · yes · the exit-cause matrix itself: the exit 21 and exit 1 causes, each asserting exit code, --json document and last hint line, and the shared-hint failure the new cross_exit_causes.rs table must hold
e2e-test-strategy#15.1 · yes · the security controls the chunk reaches: the Unix socket-dir refusal (exit 1 and 21), the server-identity negative (exit 21 document, hint and process-exit line), strict-modes before the first open, the single-writer exit 21 entries and the --home refusal (exit 1)
e2e-test-strategy#16.1 · no · list board row integrity (headers, column offsets, escaped names); the board layout is Not needed and only list's refusal joins this chunk
