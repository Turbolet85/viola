# Classified — obs-plan · log-coverage.b1

## Answers
log-coverage#1.1 · no · the run-start tests moved only their capture call with no asserted event, field or level changed, and no process-log line moved (session header)
log-coverage#1.2 · no · no process-log line or log field moved (session header), and the run-start and statusline tests moved only their capture call
log-coverage#1.3 · no · no channel log line, conn field or channel-request shape changed, and the new Windows cases only assert pipe listener admission
log-coverage#1.4 · no · no channel log line, conn field or channel-response shape changed, and the new Windows cases only assert pipe listener admission
log-coverage#1.5 · no · the send-refused product line and its fields are unmoved, and the case_4 fix changes only the test capture in src/run/send/test_support.rs
log-coverage#1.6 · no · the dialog tests moved only their capture call with no asserted event, field or level changed
log-coverage#1.7 · no · the statusline tests moved only their capture call and no product file changed (session header)
log-coverage#1.8 · no · no product file changed and no process-log line moved (session header), so the panic line's fields are unmoved
log-coverage#1.9 · no · no product file changed and no log field or process-log line moved (session header), so the release line's fields are unmoved
log-coverage#1.10 · no · the witness's emit_witness_line is a test-only site and not a product emitter, and the state-recovered fields and torn-append rule are unmoved
log-coverage#1.11 · no · the vt100-feed and config-json product claims are unmoved, and their red under plain cargo test is test-runner only
log-coverage#2.1 · no · no process-exit detail or schema changed (Untouched), and no process-log line moved (session header)
log-coverage#2.2 · no · no product file changed and no process-log line or log field moved (session header)
log-coverage#2.3 · no · no product file changed and no process-log line or log field moved (session header)
log-coverage#2.4 · no · the statusline tests moved only their capture call and no product file changed, so the statusline fail-open lines are unmoved
log-coverage#2.5 · no · the vt100-feed and config-json product claims are unmoved, and their red under plain cargo test is test-runner only
log-coverage#2.6 · no · the send-refused product line and the budget gate's asserted lines are unmoved, and the case_4 fix changes only the test capture
log-coverage#2.7 · no · no product file changed (session header) and plugin install's code moved in no way (Untouched), so the refused-check lines hold
log-coverage#3.1 · no · no product file changed (session header), verify's edge flow is unmoved (Untouched), and the statusline tests moved only their capture call
