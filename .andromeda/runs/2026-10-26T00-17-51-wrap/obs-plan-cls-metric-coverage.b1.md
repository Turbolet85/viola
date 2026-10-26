# Classified — obs-plan · metric-coverage.b1

## Answers
metric-coverage#1.1 · no · the send-refused side and detail fields are unmoved, the send/test_support.rs change is a test-only capture fix, and the torn-line skip as in G2 holds since no gate command changed
metric-coverage#1.2 · no · the reached-file changes are test captures, Windows-only cases and a dev-dependency table, and none moves a duration_ms field or its raw samples
metric-coverage#1.3 · no · the budget.rs obs-line tests only moved their capture call with no asserted field changed, so budget.read_at and the liveness reading still hold on the changed file
metric-coverage#2.1 · no · the hook-decision duration_ms line is unmoved, the statusline.rs change is a capture call only, and the hyperfine perf gate is untouched
metric-coverage#2.2 · no · the channel-response line shape and its duration field are unmoved, and the new Windows pipe cases assert listener admission without changing any channel log line
metric-coverage#2.3 · no · the send-confirmed duration_ms and the readiness-gate wait it includes are unmoved, since send/test_support.rs and gate.rs changed only test captures
metric-coverage#2.4 · no · the send-confirmed and send-refused grouping fields are unmoved and the send/test_support.rs change is a test-only capture fix
metric-coverage#2.5 · no · the channel-response method and outcome fields are unmoved, since no channel log line or shape changed in the reached test file
metric-coverage#2.6 · no · the state-recovered detail is unmoved, the witness emit in src/main.rs is a test-only site, and the torn-append rule holds
metric-coverage#2.7 · no · no reached file holds the list --json skipped field, so the change does not reach it
metric-coverage#2.8 · no · the parse-rejected fields are unmoved on gate.rs and obs.rs, and both red tests fail only under plain cargo test with their product claims unchanged
metric-coverage#2.9 · no · the send-refused refusal values and the channel-response error codes are unmoved, so the -32602 over-count note still holds
metric-coverage#2.10 · no · the process-exit fields are unmoved, since the run-start obs-line tests in src/cmd/run.rs moved only their capture call
metric-coverage#2.11 · no · G2 reads zero panics at this chunk and the src/main.rs change is the test-only helper and witness, so the panic claim holds on the changed file
metric-coverage#3.0 · no · the hook-decision session-end snippet is unaffected, since no hook timing or hook-decision field changed in statusline.rs
