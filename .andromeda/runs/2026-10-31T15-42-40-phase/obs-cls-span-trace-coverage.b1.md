# Classified — obs · span-trace-coverage.b1

## Answers
span-trace-coverage#1.1 · yes · the client dispatch spans of send, answer, pause, release, link, unlink and the wait and last client spans, where the chunk's client-side exit-21 refusals and its endpoint-lost line sit
span-trace-coverage#1.2 · yes · mcp.tool_call is the span the chunk's codes-only isError results and rejected-call lines are produced inside, so they must carry codes only there
span-trace-coverage#1.3 · no · INTERNAL kind list for run, state, hook.decision_emit and pty.paste_write; the chunk adds no INTERNAL span and its pre-log refusals are governed by the start entities
span-trace-coverage#2.1 · yes · the cli short-lived verb log table: process-start and process-exit per verb, the revive --list pair, the revive start arm logging as run, and link and unlink opening no log on a refused check
span-trace-coverage#2.2 · no · the viola run row covers its own start-sequence spans and the child's handle-wait lines, none of which the chunk's refusal or code edits change
span-trace-coverage#2.3 · yes · the hook's refused or absent instance exits 0 with no line, the hook edge the catalogue must leave alone by adding no hook cause
span-trace-coverage#2.4 · yes · the channel-request and channel-response lines carry error_code and detail, which the chunk's -32603 null-data and endpoint-lost codes write through
span-trace-coverage#2.5 · yes · the MCP tool-call span's codes-only send-refused and channel-response lines on rejects, which the chunk's MCP isError codes must match
span-trace-coverage#3.1 · yes · the eight frame verbs' server check and its process-exit server-verify-failed shape at WARN with during connect, which the chunk catalogues as an exit-21 cause
span-trace-coverage#3.2 · yes · the captured-span cases assert that the server-check refusal sits inside the dispatch span, and the chunk edits that server check in client.rs
span-trace-coverage#3.3 · yes · refuse_untrusted runs before any role file on the six frame verbs, and its refusal's --json document with detail null is the contract choice the chunk edits
span-trace-coverage#3.4 · yes · the hook's server check fails open with hook-decision server-verify-failed as its only record, which the chunk must leave as the hook edge adding no cause
span-trace-coverage#3.5 · yes · the -32602 closed-word detail on channel-response and the other faults carrying none, which the chunk's -32603 null-data change and fault_of's reading of data must keep
span-trace-coverage#3.6 · yes · the MCP frame tools' check_instance before live_endpoint and the codes-only isError instance-unreachable with no frame and no line, both in the chunk's tools.rs and result.rs scope
span-trace-coverage#3.7 · yes · CLI reply_of and MCP of_reply screen replies through is_code_word and sanitised_fault, the seam the chunk's new codes cross
span-trace-coverage#4.1 · yes · the run start span chain, with run.collision_check and run.pin_copy placed in it, where the chunk edits the collision check and the start refusals
span-trace-coverage#4.2 · yes · the run.collision_check and run.pin_copy outcome sets, which the chunk's collision-check and pinned-hash refusals must keep
span-trace-coverage#4.3 · yes · the run start's process-exit detail list (already-live, squatted-name, pinned-hash-mismatch, batch-script-child), the revive preflight details and sideload_fallback, all of it the chunk's exit-1 detail work
span-trace-coverage#4.4 · yes · run's own self process-exit line with exit_code and duration_ms, which the chunk's Run internal-error exit line writes
span-trace-coverage#5.1 · yes · the pin copy's outcome set stays ok or pinned-hash-mismatch with no span or line for placement, which the chunk's pinned-hash start refusal must keep
span-trace-coverage#6.1 · yes · the send path's instance check before any role file and the server check before channel.request inside send.client, where the chunk's send causes and client refusals sit
span-trace-coverage#6.2 · no · the send path's span attributes (channel method and conn, readiness outcome, paste text_bytes, confirm window), none of which the chunk's send causes change
span-trace-coverage#6.3 · yes · the send log fields: server-verify-failed process-exit at WARN with no channel-request line, and channel-response detail on -32602 only, which the chunk's send codes must meet
span-trace-coverage#6.4 · yes · send.client's closing process-exit with exit 21 detail instance-dead or server-verify-failed and exit 1 internal-error, the send exit causes the chunk codes
span-trace-coverage#7.1 · yes · wait.block and last.client run the server check before channel.request, in the client.rs check the chunk edits and the wait.rs and last.rs files it reaches
