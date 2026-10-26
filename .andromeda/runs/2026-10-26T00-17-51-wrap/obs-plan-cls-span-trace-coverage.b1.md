# Classified — obs-plan · span-trace-coverage.b1

## Answers
span-trace-coverage#1.1 · no · the session is test-only and changed no client span name, so the CLIENT span list stands
span-trace-coverage#1.2 · no · the session is test-only and changed no server span name, so the SERVER span list stands
span-trace-coverage#1.3 · no · the session is test-only and changed no internal span name, so the INTERNAL span list stands
span-trace-coverage#2.1 · no · the session is test-only, no CLI verb code moved, and Cargo.lock is unchanged so clap 4.6.7 stands
span-trace-coverage#2.2 · no · the session is test-only so the run-verb manual spans stand, and Cargo.lock is unchanged so portable-pty 0.8.1 stands
span-trace-coverage#2.3 · no · the session is test-only and no hook or obs writer code moved, so the hook spans and the sink fallback stand
span-trace-coverage#2.4 · no · the session is test-only and no channel log line or field moved, and Cargo.lock is unchanged so interprocess 2.4.4 stands
span-trace-coverage#2.5 · no · the session is test-only and no viola-mcp code moved, and Cargo.lock is unchanged so rmcp 3.4.1 stands
span-trace-coverage#3.1 · no · the session is test-only and the run-start span chain is unmoved, the reached run files changed only a test capture call
span-trace-coverage#3.2 · no · the session is test-only and the run-start span attributes are unmoved, no asserted field changed
span-trace-coverage#3.3 · no · the session is test-only and the run-start log fields are unmoved, no asserted event, field or level changed
span-trace-coverage#3.4 · no · the session is test-only and the run-start cleanup and exit lines are unmoved
span-trace-coverage#4.1 · no · the session is test-only and the confirmed-send span chain is unmoved, the send test support moved only its capture
span-trace-coverage#4.2 · no · the session is test-only and the confirmed-send span attributes are unmoved, the send-refused fix is capture-only
span-trace-coverage#4.3 · no · the session is test-only and the send-refused and send-issued line shapes are unmoved, the case_4 fix is capture-only in test_support.rs
span-trace-coverage#4.4 · no · the session is test-only and the confirm-window cleanup and send exit codes are unmoved
span-trace-coverage#5.1 · no · no wait or last code changed, so the wait and last spans stand
span-trace-coverage#5.2 · no · no wait or last code changed, so the wait span attributes stand
span-trace-coverage#5.3 · no · no wait or last code changed, so the wait and last log fields stand
span-trace-coverage#5.4 · no · no wait or last code changed, so the wait cleanup rule stands
span-trace-coverage#6.1 · no · the session is test-only and the dialog span chain is unmoved, the dialog file moved only its test capture
span-trace-coverage#6.2 · no · the session is test-only and the dialog span attributes are unmoved, no asserted field changed
span-trace-coverage#6.3 · no · the session is test-only and the dialog log fields are unmoved, no asserted event, field or level changed
span-trace-coverage#6.4 · no · the session is test-only and the dialog hold and expiry cleanup are unmoved
span-trace-coverage#7.1 · no · the session is test-only and the wheel-transition and pause or release spans are unmoved
