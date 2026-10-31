# Classified — obs-plan · log-coverage.b1

## Answers
log-coverage#1.1 · yes · stale · src/run/env.rs and src/run/version_gate.rs are reached whole and the report does not show process-start's fields hold
log-coverage#1.2 · yes · stale · src/cmd/client.rs, src/cmd/run.rs and src/cmd/hook/statusline.rs are reached whole and the report does not show process-exit's fields hold
log-coverage#1.3 · yes · stale · crates/viola-channel/src/client.rs is reached whole and the report does not show channel-request's fields hold
log-coverage#1.4 · yes · add-to · F1 closes channel-response's detail to unsupported-version, invalid-params and release-from-driver, absent on every other channel-response line
log-coverage#1.5 · yes · stale · src/cmd/client.rs and src/run/budget.rs are reached whole and the report does not show the send-line fields hold
log-coverage#1.6 · yes · stale · src/run/dialog.rs is reached whole and the report does not show the dialog-line fields hold
log-coverage#1.7 · yes · stale · src/cmd/hook/statusline.rs and src/run/budget.rs are reached whole and the report does not show the hook-line fields hold
log-coverage#1.8 · no · the panic hook sits in src/obs.rs, which the session does not reach
log-coverage#1.9 · no · the report keeps release-from-driver's own logged event and its exit 21 and exit 1 detail codes, so only channel-response's word moves
log-coverage#1.10 · no · state-recovered is written by crates/viola-state, which the session does not reach
log-coverage#1.11 · yes · stale · crates/viola-channel, crates/viola-mcp and src/run/version_gate.rs are reached whole and the report does not show the parse-rejected parser and detail values hold
log-coverage#2.1 · yes · stale · src/cmd/run.rs, src/run/mod.rs and src/cmd/verify.rs are reached whole and the report does not show the exit-1 detail list holds
log-coverage#2.2 · yes · stale · src/cmd/client.rs is reached whole and the report does not show the exit-21 detail list holds
log-coverage#2.3 · yes · stale · src/cmd/run.rs and src/run/mod.rs are reached whole and the report does not show the exit-20 wrapper-fault line holds
log-coverage#2.4 · yes · stale · src/cmd/hook/statusline.rs and src/cmd/client.rs are reached whole and the report does not show the hook-decision fail-open codes hold
log-coverage#2.5 · yes · stale · crates/viola-channel, crates/viola-mcp, src/run/version_gate.rs and crates/viola-agent-claude are reached whole and the report does not show the per-parser details hold
log-coverage#2.6 · no · the report moves only the exit of an oversize mcp line, and F6 measures the same single mcp-stdin parse-rejected, count 1
log-coverage#2.7 · no · the Refusal retirement does not reach RefusalReason, and the schema closes only channel-response's one field value
log-coverage#2.8 · yes · stale · src/cmd/client.rs, src/cmd/verify.rs and src/run/mod.rs are reached whole and the report does not show the refused-check no-line claims hold
log-coverage#3.1 · yes · stale · src/run/version_gate.rs, src/cmd/verify.rs and src/cmd/hook/statusline.rs are reached whole and the report does not show the spawn pairs hold
