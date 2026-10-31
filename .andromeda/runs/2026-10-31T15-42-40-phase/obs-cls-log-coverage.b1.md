# Classified — obs · log-coverage.b1

## Answers
log-coverage#1.1 · no · process-start fields; the chunk adds no process-start line or field, its new codes ride process-exit only
log-coverage#1.2 · yes · process-exit fields; the new exit-21 and exit-1 detail values and the `during` field of server-verify-failed land on this line
log-coverage#1.3 · no · channel-request fields; the new causes are process-exit lines (during:call), no channel-request field changes
log-coverage#1.4 · yes · channel-response closed `detail` and `error_code`; the -32603 fault that sanitised_fault passes must stay in the closed error_code list, and detail stays absent outside its three values
log-coverage#1.5 · no · send-issued/send-confirmed/send-refused fields; send-refused reasons belong to exits 10–14, outside this catalogue, and the chunk adds no send line
log-coverage#1.6 · no · dialog-raised/dialog-answered fields; the scope excludes the dialog scenario and its lines
log-coverage#1.7 · yes · hook-invoked/hook-decision `detail`; the scope's hook edge makes hook-decision detail the hook's only cause record, so the chunk must add no hook code
log-coverage#1.8 · no · panic fields; the chunk adds no panic line and no panic path
log-coverage#1.9 · no · release-from-driver fields; a D-01 driver line that the chunk's release refusals (exit 21, no line) do not touch
log-coverage#1.10 · no · state-recovered fields; torn-line healing and snapshot replay are excluded by the scope
log-coverage#1.11 · yes · parse-rejected parser list; the ledger-stamps strict-modes-failed line is the only strict-modes line the carve-out compares against
log-coverage#2.1 · yes · process-exit exit 1 closed details; run's start refusals, revive's preflight codes, internal-error and session-live all land on this list
log-coverage#2.2 · yes · process-exit exit 21 closed details; instance-unreachable's log forms, server-verify-failed and the new exit-21 codes sit here
log-coverage#2.3 · no · process-exit exit 20 wrapper-fault; exit 20 and its cause are excluded and no new exit code is added
log-coverage#2.4 · yes · hook-decision fail-open codes; the hook edge is named in scope, so the catalogue adds no hook cause and no hook arm gains strict-modes-failed
log-coverage#2.5 · yes · parse-rejected details per parser, including ledger-stamps strict-modes-failed, the carve-out's one comparator line
log-coverage#2.6 · no · parse-rejected mcp-stdin oversize line; a framing line holding no byte, not one of the rejected-call lines the mcp scope covers
log-coverage#2.7 · no · send-refused refusal details; exits 10–14, outside the catalogue
log-coverage#2.8 · yes · filesystem refusals: strict-modes-failed on exit 21 and the exit-1 refusals that write no line, the carve-out the chunk's contract choice rests on
log-coverage#3.1 · yes · agents-probe pair for revive's session-live reading, which witnesses whether session-live is landed; that re-read is the chunk's own work
