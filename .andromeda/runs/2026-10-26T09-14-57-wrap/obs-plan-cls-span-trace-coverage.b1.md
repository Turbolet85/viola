# Classified — obs-plan · span-trace-coverage.b1

## Answers
span-trace-coverage#1.1 · no · no span name was added or renamed and the check runs inside the existing verb spans, so this CLIENT list holds
span-trace-coverage#1.2 · no · no SERVER span name changed and the check is client-side, so every channel.request still has its channel.dispatch
span-trace-coverage#1.3 · no · no INTERNAL span name was added or renamed, so this list holds
span-trace-coverage#2.1 · yes · add-to · the six frame verbs verify the server inside their dispatch spans before any frame (F1), and this list omits last.client, which F1 names
span-trace-coverage#2.2 · no · run's start spans and their order are untouched, and run's collision check still reads the snapshot without a server check
span-trace-coverage#2.3 · yes · add-to · the hook's event and dialog paths share one connect that verifies the server before any channel.request, and a SessionEnd whose check fails appends directly (F4)
span-trace-coverage#2.4 · no · viola-channel's new server_pid readers write and log nothing, and the channel.request and channel.dispatch spans and their attributes are unchanged
span-trace-coverage#2.5 · yes · add-to · viola mcp's send, wait, last and answer tools verify the server inside mcp.tool_call after TokioClient::connect (F5), and a mismatch returns instance-unreachable with no frame and no log line
span-trace-coverage#3.1 · no · run's start sequence is untouched, and its collision check still reads the snapshot without verifying the server
span-trace-coverage#3.2 · no · no run start span attribute was added or changed
span-trace-coverage#3.3 · no · run's process-start and process-exit fields and details are unchanged
span-trace-coverage#3.4 · no · run's start cleanup and handle-wait exit lines are unchanged
span-trace-coverage#4.1 · yes · add-to · the CLI send verifies the server inside send.client before channel.request (F1), so the must-trace chain gains that step and a mismatch ends at send.client
span-trace-coverage#4.2 · no · the send's required span attributes are unchanged, since no span or attribute was added and the new server_pid readers write and log nothing
span-trace-coverage#4.3 · yes · add-to · a server-verify mismatch writes no channel-request or channel-response line (F1), an exception the required send log fields must carry
span-trace-coverage#4.4 · yes · stale · exit 21 during connect is no longer always instance-dead, because a mismatch or an unmakeable check exits 21 with detail server-verify-failed
span-trace-coverage#5.1 · yes · add-to · wait.block and last.client verify the server before channel.request (F1), after the client's own exit-13 refusals
span-trace-coverage#5.2 · no · no wait span attribute was added or changed
span-trace-coverage#5.3 · yes · contradict · the sentence that server-verify-failed cannot occur on wait or last is false on Linux and Windows, since the check now runs inside wait.block and last.client
span-trace-coverage#5.4 · yes · add-to · wait.block also closes on the server-verify refusal that runs inside it before any frame (F1), not only on a response or a connection error
span-trace-coverage#6.1 · yes · add-to · the CLI answer verifies the server inside answer.client before channel.request(answer) (F1), and a mismatch sends no answer frame
span-trace-coverage#6.2 · no · the dialog span attributes dialog_kind, pending_conflict, deadline_ms, deadline_hit and decision_emitted are unchanged and the check adds none
span-trace-coverage#6.3 · yes · contradict · a dialog hook's hook-invoked is no longer written only after the hook.dialog response, since a server-verify mismatch writes hook-invoked with corr null and then hook-decision with no response
span-trace-coverage#6.4 · no · the dialog cleanup is unchanged, and a fail-open before dialog_id already gives hook-invoked corr null plus hook-decision with a detail, the shape the mismatch takes
span-trace-coverage#7.1 · yes · add-to · pause.client and release.client verify the server inside their span before channel.request(pause or release) (F1), so the must-trace chain gains that step
