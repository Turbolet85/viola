# Classified — obs · log-coverage.b1

## Answers
log-coverage#1.1 · no · process-start fields: the chunk adds no process-start field and leaves endpoint_kind unchanged; the no-path rule at run start binds through the detail lines, not this row
log-coverage#1.2 · yes · process-exit `detail` and `during:"connect"`: the new client refusals and the run-start socket-directory refusal write process-exit lines with new closed codes
log-coverage#1.3 · yes · channel-request `from` / `from_trust`: a refused connection writes no channel-request, and a verified euid must leave from_trust self-reported
log-coverage#1.4 · yes · channel-response `detail` is closed to the three -32602 faults and absent elsewhere; a refused connection is no request pair, so the new refusal codes must not reach this line
log-coverage#1.5 · yes · send lines carry `from` / `from_trust`: a verified peer euid must not turn the self-reported from into an authenticated identity in any line
log-coverage#1.6 · yes · dialog lines carry `from` / `from_trust`: hook.dialog's connect refusal and the self-reported-from rule reach them
log-coverage#1.7 · yes · hook-invoked / hook-decision: a hook connect refusal is a fail-open whose trace is a hook-decision fail-open detail
log-coverage#1.8 · no · panic fields: the chunk adds no panic path, location or thread field
log-coverage#1.9 · yes · release-from-driver `from` / `from_trust` (self-reported): the wrapper-side euid check must not change that value
log-coverage#1.10 · no · state-recovered: torn-line healing and snapshot replay are not moved (Not needed); read_snapshot_or_replay is only called by the hook connect
log-coverage#1.11 · no · parse-rejected parsers and details: no frame, stdin or state-entry parse path gains a value; board scan's symlink and malformed counts stay as they are
log-coverage#2.1 · yes · process-exit exit 1 list: a refused socket directory at a wrapper's start and the create_private_dir failure replacing internal-error take closed codes this list must record
log-coverage#2.2 · yes · process-exit exit 21 list: the client-side socket-directory refusal and the peer euid mismatch take new closed codes this list must record
log-coverage#2.3 · no · exit 20 wrapper-fault: the chunk adds no exit-20 cause; the bind failure stays on internal-error (exit 1)
log-coverage#2.4 · yes · hook-decision fail-open codes: a hook connect refusal of this chunk reaches hook only as a fail-open, so its new code must be in this list
log-coverage#2.5 · no · parse-rejected per-parser details: no frame, stdin or state-entry parse path gains a new detail; the euid check runs before any frame
log-coverage#2.6 · no · mcp-stdin oversize line: viola_mcp::serve's stdin cap is untouched; the MCP frame tools' refusals are isError codes with no line
log-coverage#2.7 · no · RefusalReason details: connect refusals are process-exit lines, not send-refused lines, and the chunk adds no RefusalReason
log-coverage#2.8 · yes · filesystem refusals: the home check becomes one rule, the list and scan refusal pair and the no-line rule change, and any socket path brought under strict modes must stay off or join the strict-modes telemetry list
log-coverage#3.1 · no · child and shell spawns: the chunk adds no spawn; the connect probe and harness cleanup are not spawns
