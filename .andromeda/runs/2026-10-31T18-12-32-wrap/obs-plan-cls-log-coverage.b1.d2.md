# Classified — obs-plan · log-coverage.b1

## Answers
log-coverage#1.1 · no · the process-start fields and subject list are untouched, and the schema moved only the process-exit detail list
log-coverage#1.2 · no · the process-exit field list and the during enum (connect|call) are unchanged, the session moved values and not fields
log-coverage#1.3 · no · channel-request lives in viola-channel, which the change does not reach
log-coverage#1.4 · no · channel-response's closed detail words and the -32603 body did not move, and no channel file is reached
log-coverage#1.5 · no · the send-* fields are unchanged, and the send change moves only the exit-21 --json document, which writes no line
log-coverage#1.6 · no · dialog events run on the hook path, whose code is not in the diff
log-coverage#1.7 · no · src/cmd/hook.rs is not in the diff and the schema kept hook-decision's seven words
log-coverage#1.8 · no · the panic fields are unchanged and no panic path or location moved
log-coverage#1.9 · no · release-from-driver is channel-side, and the release verb's change moves only its exit-21 document, not a log line
log-coverage#1.10 · no · the replay and appender code is not in the diff, and liveness.rs's without_endpoint opens nothing and writes no line
log-coverage#1.11 · no · the parse-rejected parsers and details are unchanged, and the schema moved only the process-exit detail list
log-coverage#2.1 · yes · contradict · a stale holder now logs stale-heartbeat (F3) rather than already-live, so the already-live clause is no longer true for a stale holder; the exit-1 list also gains the new words of F1 and F2
log-coverage#2.2 · yes · add-to · exit 21's process-exit list gains unwrapped-name and endpoint-lost, the closed cause words of F1, F4 and F5
log-coverage#2.3 · no · wrapper-fault on exit 20 is not among the session's changes
log-coverage#2.4 · no · hook.rs is not in the diff, and the hook-decision fail-open words and their arms stand
log-coverage#2.5 · no · the parse-rejected per-parser details are unchanged, and the schema moved only the process-exit detail list
log-coverage#2.6 · no · the mcp-stdin oversize write is unchanged, and tools.rs and result.rs map refusal causes without touching parse-rejected
log-coverage#2.7 · no · no RefusalReason was added and the refusal details are unchanged
log-coverage#2.8 · yes · add-to · F2: verify's process-exit under a parsed VIOLA_NAME now writes strict-modes-failed for a refused check, a new place for the word beside the listed ones
log-coverage#3.1 · no · the closed subject enum and the eight verify spawn pairs are unchanged, and verify's new codes sit on its subject-self refusal line
