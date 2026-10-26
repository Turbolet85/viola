# Classified — test-plan · e2e-test-strategy.b2

## Answers
e2e-test-strategy#6.4 · no · Path 5 cleanup is outside the link verbs' reach, as the report leaves Paths 1–7 untouched
e2e-test-strategy#7.1 · no · Path 6 steps name no reached file or link verb, as the report leaves Paths 1–7 untouched
e2e-test-strategy#7.2 · no · Path 6 signals are budget, send and release results the link verbs do not reach
e2e-test-strategy#7.3 · no · Path 6 as-landed cases sit in budget-gate and harness tests the link verbs do not reach
e2e-test-strategy#7.4 · no · Path 6 cleanup is the harness cleanup, which the link verbs do not reach
e2e-test-strategy#8.1 · no · Path 7 steps use send, answer and verify, which the link verbs do not reach
e2e-test-strategy#8.2 · no · Path 7 signals are send, answer and verify results the link verbs do not reach
e2e-test-strategy#8.3 · no · Path 7 cleanup is the harness cleanup and TempDir drop, which the link verbs do not reach
e2e-test-strategy#9.1 · no · E2 steps are the env strip and fd dump of viola run, which the link verbs do not reach
e2e-test-strategy#9.2 · no · E2 signals on the child's env and fds are not reached by the link verbs
e2e-test-strategy#9.3 · no · E2 cleanup is the TempDir drop, which the link verbs do not reach
e2e-test-strategy#10.1 · yes · contradict · the As-landed note says the verbs are unbuilt, answer -32601 and that no product path writes unlink, all now false with E3 built in tests/cli_links.rs, and F1 and F2 join its cases
e2e-test-strategy#11.1 · yes · contradict · the link-leg parenthetical says no viola link verb exists and the explicit verbs are unbuilt, now false since link and unlink are served
e2e-test-strategy#12.1 · no · its MCP, hook fail-open and seam claims are not reached by the link verbs, and no MCP tool changed
e2e-test-strategy#13.1 · yes · add-to · the link and unlink verbs' --json refusals and result lines print only their own parsed names, which this check now covers
e2e-test-strategy#14.1 · yes · stale · the six-verb server-identity case is now eight verbs, and the link and unlink strict-modes and stopped-instance exit 21 causes join as cases of tests/cli_links.rs, with F4 beside the home strict-modes cause
e2e-test-strategy#15.1 · yes · stale · the server-identity bullet and its control name six verbs, now eight, and F3 (tests/channel_links.rs, 14 cases) builds the raw-channel name-validation bullet
e2e-test-strategy#16.1 · no · the list board is unchanged, and the nextest kill-line filter only gains binary(channel_links) beside this suite's binary
e2e-test-strategy#17.1 · no · the chaos suite's fault seams and parsers are unchanged, per the report's Untouched list
e2e-test-strategy#18.1 · no · the property suite's parsers and normalisers are unchanged, per the report's Untouched list
e2e-test-strategy#19.1 · no · the contract suite's fixtures and fake-agent hook order are unchanged, per the report's Untouched list
