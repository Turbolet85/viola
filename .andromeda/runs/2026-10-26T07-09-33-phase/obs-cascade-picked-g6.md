# obs · picked accounting · group 6 of 7

## Picked
metric-coverage#1.1 · carried · the counter over `send-refused` grouped by `side` and `detail` that reads a client-side line the plan may write for a send refused by a failed verification
metric-coverage#1.2 · carried · the raw `duration_ms` samples the hook-latency reading the chunk re-measures is taken from
metric-coverage#2.1 · carried · the hook-latency gate (hyperfine `max`) the added peer-credential and start-time read must stay inside
metric-coverage#2.2 · not in scope · the channel latency reads `channel-response.duration_ms` of made calls; the chunk adds no `channel-response` field or value and no metric
metric-coverage#2.4 · carried · the send-outcome counter over `send-refused{refusal,detail,side}` that a send refused before any frame by a failed verification reaches
metric-coverage#2.5 · not in scope · the wait-outcome counter reads `channel-response{method:"wait"}.outcome` of a made call; a refused verification writes no frame and adds no outcome value
metric-coverage#2.6 · carried · the recovery counter over `state-recovered.detail`, which the clients held on `read_snapshot` must not feed
metric-coverage#2.10 · not in scope · counts child and probe exits; a client spawns no child and the chunk's exit line is `subject:"self"`
metric-coverage#3.0 · carried · the report-only `session-end` quantile reading of the hook latency the chunk re-measures under 1.0 s
log-coverage#1.2 · carried · the `process-exit` field set the exit-21 verification line is written with (`detail`, `during`)
log-coverage#1.3 · carried · the client's `channel-request` line, never written for a frame a refused verification never sends
log-coverage#1.4 · carried · the client's `channel-response` line on every call that is made after a passed verification
log-coverage#1.5 · carried · the send lines' fields (`text_bytes`, `side`) a client-side send refusal keeps, never the sent text
log-coverage#1.7 · carried · the `hook-decision` fields the hook's fail-open-on-mismatch line carries (`detail`, `duration_ms`)
log-coverage#1.10 · carried · `state-recovered` written only by `read_snapshot_or_replay`, which the five re-read readers do not call under the founder's hold
log-coverage#2.1 · carried · the schema's closed `process-exit` `detail` enum and its stated count, which a new exit-21 code must sit inside
log-coverage#2.2 · carried · the exit-21 `detail` catalog holding `server-verify-failed`, the CLI verbs' mismatch outcome
log-coverage#2.4 · carried · the `hook-decision` fail-open codes holding `server-verify-failed` and `strict-modes-failed`, the hook's mismatch outcome
log-coverage#2.6 · not in scope · the `RefusalReason` details; a failed verification is an exit-21, fail-open or `instance-unreachable` outcome and adds no refusal detail
log-coverage#2.7 · carried · the strict-modes telemetry rule that P4's fork over the strict-modes half changes on `process-exit` exit 21 and `hook-decision`
error-capture-reporting#2.1 · not in scope · the panic hook and its catch sites are unchanged and the scope names none of them
