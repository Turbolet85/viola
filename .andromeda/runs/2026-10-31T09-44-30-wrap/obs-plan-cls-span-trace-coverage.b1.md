# Classified — obs-plan · span-trace-coverage.b1

## Answers
span-trace-coverage#1.1 · no · the CLIENT span roster is untouched: the report's Untouched list says no span was added, and no CLIENT span is moved or renamed
span-trace-coverage#1.2 · no · the SERVER span roster is untouched: no span was added, and mcp.tool_call and channel.dispatch are not moved by the change
span-trace-coverage#1.3 · no · the INTERNAL span roster is untouched: no span was added and no run.* or state.* span is moved by the change
span-trace-coverage#2.1 · no · the cli row's instance-check and process-log rules are unmoved: the no-line-of-their-own rule for link and unlink is unmoved and no cli span changed
span-trace-coverage#2.2 · no · the cli run row names no channel field list and no error body; its spans and process lines are unmoved
span-trace-coverage#2.3 · no · the hook row's instance check, fail-open and hook-decision lines are unmoved; no hook span or line changed
span-trace-coverage#2.4 · yes · add-to · the channel-request / channel-response field list omits detail, which channel-response now carries from ProtocolError::cause (F2)
span-trace-coverage#2.5 · no · the mcp rejects stay codes-only: the report shows MCP rejects codes-only and the isError codes holding, now witnessed
span-trace-coverage#3.1 · no · the server-check order and its process-exit 21 are unmoved: the change moves no ServerVerdict, verify or connect step
span-trace-coverage#3.2 · no · the refused_inside captured-span witnesses are unmoved: no span test or helper was changed by the session
span-trace-coverage#3.3 · no · the instance check and its refusal lines are unmoved: refuse_untrusted order and the no-line rule hold
span-trace-coverage#3.4 · no · the hook server check and its hook-decision fail-open are unmoved: no hook line or detail value changed
span-trace-coverage#3.5 · no · the mcp check's codes-only isError instance-unreachable holds, now witnessed; no frame or line is added
span-trace-coverage#4.1 · no · the run start-sequence span chain and product order are unmoved: no span is added or reordered
span-trace-coverage#4.2 · yes · stale · the run.pin_copy row names resolve_program at span-trace-coverage.md:44, which now returns Result<PathBuf, AgentError>
span-trace-coverage#4.3 · yes · stale · schemas/diag-line.v1.json is reached and its pty_backend and sideload_fallback reading is not shown holding on the changed schema
span-trace-coverage#4.4 · no · the cleanup order and process-exit lines of the run start are unmoved
span-trace-coverage#5.1 · no · the concurrent first start under run.pin_copy is unmoved: place_absent and the outcome set stay ok or pinned-hash-mismatch
span-trace-coverage#6.1 · no · the confirmed send span chain is unmoved: the send spans and channel.request / channel.dispatch order hold
span-trace-coverage#6.2 · no · the send span attributes are unmoved: no span attribute value or name changed
span-trace-coverage#6.3 · yes · add-to · the send scenario's channel-response record must carry the closed detail word on -32602 lines (F2: written from ProtocolError::cause, fault_of reads data)
span-trace-coverage#6.4 · no · the send cleanup and Drop-guard emission hold; the channel-response Drop guard still logs on every exit
span-trace-coverage#7.1 · no · the wait and last span chain is unmoved: wait.block, channel.request and run.wait_dispatch order hold
span-trace-coverage#7.2 · no · the wait and last span attributes are unmoved: woken_kind and timed_out values hold
span-trace-coverage#7.3 · yes · stale · the wait and last channel-response reading omits the detail word that -32602 now carries on these paths
