# Classified — obs-plan · log-coverage.b1

## Answers
log-coverage#1.1 · no · the session changes no process-start field or subject, and the mcp start line stays as written
log-coverage#1.2 · no · the mcp oversize cut ends with process-exit 0 as a closed stdin does, so the field list stands
log-coverage#1.3 · no · the MCP stdin is not the channel and the channel's close-on-oversize rule is unchanged, so channel-request stands
log-coverage#1.4 · no · the session adds no channel method or response field, so channel-response stands
log-coverage#1.5 · no · the session touches no send path, so the send-issued, send-confirmed and send-refused fields stand
log-coverage#1.6 · no · no dialog path changed, so the dialog fields stand
log-coverage#1.7 · no · no hook path changed, so the hook-invoked and hook-decision fields stand
log-coverage#1.8 · no · the chaos test asserts terminal passthrough and adds no panic field or location, so the panic row is unmoved
log-coverage#1.9 · no · the session does not touch release-from-driver
log-coverage#1.10 · no · the session changes no state-recovery path, reader or line
log-coverage#1.11 · yes · add-to · F1 adds mcp-stdin to the parse-rejected parser list, 9 values to 10, for the new WARN line
log-coverage#2.1 · yes · stale · the session reaches diag-line.v1.json whole and the report does not show the 13-value process-exit detail closure holds on the changed file
log-coverage#2.2 · no · the session changes no exit-21 path, so the three exit-21 details are unmoved
log-coverage#2.3 · no · the session produces no wrapper-fault, so exit 20 is unmoved
log-coverage#2.4 · no · no hook fail-open path changed, so the seven codes stand
log-coverage#2.5 · yes · add-to · F1 adds mcp-stdin to the per-parser details with its one detail, oversize, for a stdin line past MAX_FRAME
log-coverage#2.6 · yes · stale · the session reaches diag-line.v1.json whole and the report does not show send-refused.detail still an open string on the changed file
log-coverage#2.7 · no · the report's re-read of replace_private_shared adds no event, field or detail code, so the strict-modes, pinned-hash-mismatch and symlink-ignored lines are unchanged
log-coverage#3.1 · yes · stale · the session reaches diag-line.v1.json whole and the report does not show the closed subject enum holds on the changed file
