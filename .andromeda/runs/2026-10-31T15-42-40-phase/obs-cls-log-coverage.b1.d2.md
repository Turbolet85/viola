# Classified — obs · log-coverage.b1

## Answers
log-coverage#1.1 · yes · the `sideload_fallback` `strict-modes-failed` value on a refused conpty/ companion is the strict-modes process-start line the no-line carve-out must name
log-coverage#1.2 · yes · the `process-exit` `detail` field is the closed list the chunk extends for exits 21 and 1
log-coverage#1.3 · no · channel-request fields only; the chunk adds no channel-request line and changes none of its fields
log-coverage#1.4 · yes · the channel-response closed `error_code` (-32603) and `detail` lists are the seam the new error layer crosses through `viola_channel::sanitised_fault` with null data
log-coverage#1.5 · no · send-issued/send-confirmed/send-refused fields; send-refused reasons and details are outside the catalogue (exits 10–14)
log-coverage#1.6 · no · dialog-raised/answered fields; the dialog scenario and `hook.dialog` timing are out of scope
log-coverage#1.7 · no · hook-invoked/hook-decision field list only; the hook's closed fail-open codes bind through #2.4 and no hook line or field changes
log-coverage#1.8 · no · panic fields; the chunk adds no panic path and no panic line
log-coverage#1.9 · no · release-from-driver fields; a release refusal line is not an exit-21 or exit-1 cause
log-coverage#1.10 · no · state-recovered fields; no recovery line changes
log-coverage#1.11 · no · parse-rejected fields and parser list; the chunk extends the process-exit `detail` only, not parse-rejected
log-coverage#2.1 · yes · the exit-1 `process-exit` details (already-live, squatted-name, pinned-hash-mismatch, batch-script-child, internal-error, no-session, session-live, cwd-missing) and revive's preflight order, which the chunk closes and re-reads
log-coverage#2.2 · yes · the exit-21 `process-exit` details instance-dead, strict-modes-failed and server-verify-failed with their WARN shape, each of which the chunk gives its own code
log-coverage#2.3 · no · exit 20 wrapper-fault; exit 20's cause and the shell-out are not re-opened
log-coverage#2.4 · yes · the hook edge: the chunk adds no hook cause, so the closed hook fail-open code list stays as it is
log-coverage#2.5 · no · parse-rejected detail list per parser; the chunk adds no parse-rejected value and the schema edit touches process-exit `detail` only
log-coverage#2.6 · no · mcp-stdin oversize parse-rejected line; the chunk's mcp change covers rejected-call lines and result codes, not this line
log-coverage#2.7 · no · RefusalReason details are send-refused reasons, outside the catalogue (exits 10–14)
log-coverage#2.8 · yes · the filesystem refusals: strict-modes-failed on exit 21 is the no-line carve-out the chunk names, and each refused verb (list, plugin install, link/unlink, revive, verify) writes no line or the pair the chunk must match
log-coverage#3.1 · yes · the agents-probe pair written for revive's session-live reading and for list, the eight verify spawn pairs, and the stopped-run rule the verify WATCH test tallies
