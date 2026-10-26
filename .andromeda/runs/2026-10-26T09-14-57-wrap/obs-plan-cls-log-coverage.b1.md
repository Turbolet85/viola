# Classified — obs-plan · log-coverage.b1

## Answers
log-coverage#1.1 · no · the session adds no process-start field, subject or spawn, so the row's fields still hold
log-coverage#1.2 · no · the new CLI process-exit line uses only fields this row already lists (subject self, exit_code, detail, during connect), and no field is added
log-coverage#1.3 · no · the reached channel client files add no channel-request field, and a mismatch sends no request, so the row's claim still holds on the changed files
log-coverage#1.4 · no · channel-response fields and error codes are unchanged, and a mismatch sends no request that could draw a response
log-coverage#1.5 · no · send-issued, send-confirmed and send-refused keep their fields, and a mismatch writes process-exit rather than a send-refused line
log-coverage#1.6 · no · dialog-raised and dialog-answered keep their fields, and a mismatch sends neither a hook.dialog nor an answer frame
log-coverage#1.7 · yes · add-to · on a mismatch the dialog path now writes hook-invoked{corr:null} and then hook-decision{corr:null, decision_emitted:false, detail:server-verify-failed} at WARN (F3)
log-coverage#1.8 · no · panic hooks and catch sites are untouched per the report, so the panic row's fields still hold
log-coverage#1.9 · no · release-from-driver logs the wrapper's refusal of a release carrying from, which the session does not reach, and the CLI release check runs before its own frame
log-coverage#1.10 · no · read_snapshot_or_replay and its state-recovered lines are unchanged, no reader moved onto it, and the hook's read stays a plain read_snapshot
log-coverage#1.11 · no · the session adds no parse-rejected parser or detail, and the server check parses no frame or stdin
log-coverage#2.1 · no · no exit-1 detail is added or moved, and run and revive are not among the reached files, so the thirteen-value count still holds
log-coverage#2.2 · yes · add-to · server-verify-failed is now written by this build as the exit-21 process-exit in cli-<name>.ndjson at WARN, during connect (F2)
log-coverage#2.3 · no · wrapper-fault is the exit-20 process-exit, and the report keeps the internal-error exit path and panic hooks unreached
log-coverage#2.4 · yes · add-to · server-verify-failed is now written by this build as the fail-open hook-decision at WARN into hook-<name>.ndjson, after hook-invoked on the dialog path (F3)
log-coverage#2.5 · no · no parse-rejected parser or detail is added, and the server check parses no input that could raise one
log-coverage#2.6 · no · RefusalReason details are unchanged, since server-verify-failed is a process-exit detail and no send-refused line carries it
log-coverage#2.7 · no · no strict-modes-failed line is added because the strict-modes half stands, and verify_server is a server-identity check, so the nine hook events still run no instance check
log-coverage#3.1 · no · the session spawns no child or shell, so the subject list and the spawn pairs are unchanged
