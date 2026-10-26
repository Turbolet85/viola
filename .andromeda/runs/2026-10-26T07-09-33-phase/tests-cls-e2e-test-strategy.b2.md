# Classified — tests · e2e-test-strategy.b2

## Answers
e2e-test-strategy#6.4 · no · Path 5 outer-PTY cleanup (Ctrl-C, child.wait, TempDir drop); the chunk names no Path 5 case and edits no PTY teardown
e2e-test-strategy#7.1 · no · Path 6 budget-governor steps, dropped by Not needed (the gate is not touched); its send and release run on a real wrapper that the pre-frame check accepts
e2e-test-strategy#7.2 · no · Path 6 budget verification signals and the step-8 budget-paused hint, dropped by Not needed; the statusline snapshot read stays held to the replay ruling
e2e-test-strategy#7.3 · no · the as-landed Path 6 budget, harness-release and MCP budget-paused cases, dropped by Not needed; the closed-gate rule is about the budget gate, which the chunk does not touch
e2e-test-strategy#7.4 · no · Path 6 cleanup (harness cleanup), a teardown of the Not needed budget scenario
e2e-test-strategy#8.1 · no · Path 7 version-gate setup (--unstamped, --report-version), dropped by Not needed; its send and answer run on a real wrapper that the new check accepts
e2e-test-strategy#8.2 · yes · the PreToolUse hook.dialog path must keep empty stdout, exit 0 and a return within SPINE_DEADLINE, which the verification added before its frame must stay inside (CONNECT_DEADLINE below SPINE_DEADLINE); the answer-12 leg is Not needed
e2e-test-strategy#8.3 · no · Path 7 cleanup (harness cleanup and H3 TempDir drop), a teardown of the Not needed version-gate scenario
e2e-test-strategy#9.1 · no · E2 R8 env strip of run's child (the 11 identity-floor names and an unknown CLAUDE-prefixed name); the chunk edits no spawn path or child env
e2e-test-strategy#9.2 · no · E2 child receipt, fd-table and env-count rules, which no reached spawn path changes; its secret scan is the S10 check every E2E test runs
e2e-test-strategy#9.3 · no · E2 cleanup (TempDir drop); the chunk reaches no E2 scenario step
e2e-test-strategy#10.1 · no · E3 link/unlink and the implicit-link cases, dropped by Not needed ("Explicit session links" stays held; nothing here serves them)
e2e-test-strategy#11.1 · yes · E5's replay leg for the five snapshot readers is owed to this chunk; steps 1–3 bind only if the replay ruling lands, with the wheel:"human" and rebuilt-snapshot signals restated then
e2e-test-strategy#12.1 · yes · the hook fail-open matrix in hook_fail_open.rs gains the verification-failure case (exit 0, empty streams, no frame or event line), and the MCP no-frame-on-schema-violation tests bind the tools.rs pre-frame check
e2e-test-strategy#13.1 · yes · the scope names the error-sanitization obligation for the new unreachable and fail-open outcomes (codes and fixed messages only, no pid, path or start time in --json, isError or error.data), which this scan checks
e2e-test-strategy#14.1 · yes · cross_exit_causes.rs gains the server-verification mismatch as an exit-21 cause (its --json document and last hint: line name it, no shared hint, no release hint), and the impostor-listener case binds
e2e-test-strategy#15.1 · yes · the security_negatives suite takes the server-verification negative beside the Windows SQOS identification case the chunk builds on, and the single-writers item binds the reached clients not to write snapshot.json or stamps.json
e2e-test-strategy#16.1 · no · viola list board layout, row integrity and human-mode SGR, dropped by Not needed; no reached file is src/cmd/list.rs
e2e-test-strategy#17.1 · yes · the endpoint-vanishes-during-wait case must still read unreachable through the new check, and the SIGSTOP case's liveness:stale for a matching pid and start time binds the liveness classifier the chunk edits
e2e-test-strategy#18.1 · no · the property suite's paste, framing, hook, statusline, agents and vt100 cases, dropped by Not needed (no parser surface is added)
e2e-test-strategy#19.1 · no · the contract suite's fake-agent hook sequence, fixtures and content hash; the chunk changes no fixture, receipt, answer annotation or hook exit code (fail-open stays 0)
