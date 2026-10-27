# Classified — obs-plan · span-trace-coverage.b1

## Answers
span-trace-coverage#1.1 · no · the CLIENT span names are unchanged: the session adds no client span, and F4's refusals add no span name
span-trace-coverage#1.2 · no · the SERVER span names are unchanged; F1 moves state.snapshot_recover under hook.handle but adds no SERVER span
span-trace-coverage#1.3 · no · state.* already names state.snapshot_recover as an INTERNAL span, so F1's new placement is carried by that glob
span-trace-coverage#2.1 · yes · add-to · the six frame verbs' refused instance check (F4, F13) opens no process log and writes no line, so this no-line exemption list gains send, wait, last, answer, pause and release
span-trace-coverage#2.2 · no · its manual spans are named by kind (run.*, state.*), and F1 only moves an existing state span within the start sequence without adding a kind
span-trace-coverage#2.3 · yes · contradict · F5: a refused or absent instance now exits 0 at once with no hook-invoked or hook-decision line, so "written except the hidden capture arms" no longer holds
span-trace-coverage#2.4 · no · viola-channel is not reached, and its channel.request and channel.dispatch spans and fields are unchanged
span-trace-coverage#2.5 · yes · contradict · F7: a viola mcp frame tool's refused check_instance returns instance-unreachable with no frame and no line, so "codes-only send-refused / channel-response on rejects" no longer covers it
span-trace-coverage#3.1 · no · F4 puts refuse_untrusted first among each verb's client-side refusals, so the server check still follows them and precedes any frame as the row states
span-trace-coverage#3.2 · yes · stale · the named test cases and the refused_inside helper sit in the six verb modules and client.rs, both reached whole, and nothing in the report shows their claim still holds
span-trace-coverage#3.3 · yes · add-to · F1 puts state.snapshot_recover under hook.handle at this connect's read, which the row still names read_snapshot, and F12 makes a replayed snapshot fail open as channel-unreachable
span-trace-coverage#3.4 · no · F7 runs check_instance before live_endpoint, ahead of TokioClient::connect; the server check still follows connect and its no-frame, no-line result is unchanged
span-trace-coverage#4.1 · yes · add-to · F1 puts state.snapshot_recover under run.start before run.collision_check in this chain, and F8's home-tree check before open_wrapper_log is a start step with no span and no line
span-trace-coverage#4.2 · yes · add-to · F3 gives run.collision_check over a replayed snapshot a heartbeat-only outcome (beat_age at or under STALE_AFTER already-live, else free), and the sideload outcome list gains strict-modes-failed (F11)
span-trace-coverage#4.3 · yes · add-to · F2 makes revive's preflight one replay with one state-recovered line, F8's tree refusal exits 1 with no log and no detail, and the closed sideload_fallback list gains strict-modes-failed (F11)
span-trace-coverage#4.4 · no · the handle-wait and run exit closes come only after pty.spawn, and the session's refusals end before run.start opens
span-trace-coverage#5.1 · yes · add-to · F1 puts state.snapshot_recover under hook.handle on the hook path, so the hook-process chain hook.handle › channel.request(hook.event) gains that span
span-trace-coverage#5.2 · no · the send attributes (channel, readiness gate, pty.paste_write text_bytes, confirm window) are unchanged, and no send-path attribute is added
span-trace-coverage#5.3 · no · its validation and server-verification refusals keep their lines and it claims no line for an instance refusal, so no stated claim changes
span-trace-coverage#5.4 · yes · contradict · F4: a refused instance check exits 21 with the instance pair and no line, so the exit-21 claim (instance-dead or server-verify-failed, each with a process-exit line) no longer covers every exit 21
span-trace-coverage#6.1 · no · the wait and last chain is unchanged: the refused instance check precedes the liveness pre-check, and the server check still follows the client refusals and precedes channel.request
span-trace-coverage#6.2 · no · the wait.block and run.wait_dispatch attributes are unchanged
span-trace-coverage#6.3 · yes · stale · the sentence that strict-modes-failed cannot occur, owed to Home and code-bearing file integrity, is stale: that entry closed, and the check now runs before the frame and exits 21 with the instance pair and no line
span-trace-coverage#6.4 · no · its closing triggers (response, connection error, server refusal) are unchanged, and the refused instance check ends before any span opens
span-trace-coverage#7.1 · yes · add-to · F1 puts state.snapshot_recover under hook.handle on the hook path, so the dialog chain hook.handle › channel.request(hook.dialog) gains that span
