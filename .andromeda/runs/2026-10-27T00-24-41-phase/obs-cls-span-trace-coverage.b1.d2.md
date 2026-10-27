# Classified — obs · span-trace-coverage.b1

## Answers
span-trace-coverage#1.1 · no · the CLIENT kinds list; the chunk adds or changes no client-side span and pty.spawn keeps its kind after the pin step
span-trace-coverage#1.2 · no · the SERVER kinds list; the chunk adds or changes no server-side span (hook.handle, channel.dispatch and mcp.tool_call are untouched)
span-trace-coverage#1.3 · yes · the INTERNAL kinds bind run.pin_copy (run.*, the span of the start's pin_bytes call) and any state.* span the viola-state pin write adds
span-trace-coverage#2.1 · yes · verify.rs edits its pin_exe call, so the cli.verify dispatch span and its process-start / process-exit into cli-<name>.ndjson (VIOLA_NAME set) bind
span-trace-coverage#2.2 · yes · the run row's manual start-sequence spans bind through run.pin_copy (the pin_bytes call in run.rs) and run.conpty_sideload if pin_companions takes the shared wait
span-trace-coverage#2.3 · yes · the hook fail-open the scope keeps as it is (exit 0, stderr empty, hook-<name>.ndjson only, silent no-op on a refused or absent instance)
span-trace-coverage#2.4 · no · channel.request / channel.dispatch spans and channel lines; no channel method is added or changed and the race witness reads no channel line
span-trace-coverage#2.5 · no · the viola mcp surface (mcp.tool_call, mcp.ndjson, stderr fallback, isError codes) is listed Not needed and no mcp code changes
span-trace-coverage#3.1 · no · the eight frame verbs' server check; no frame client changes and the race reaches no frame
span-trace-coverage#3.2 · no · the captured-span refusal cases of the frame verbs and the hook; no server-check refusal is changed
span-trace-coverage#3.3 · no · the six frame verbs' instance check; no frame verb changes and the pin write sits under bin/, not an instance directory
span-trace-coverage#3.4 · no · the hook's connect and server-verify fail-open; a failed pinned spawn never reaches hook code and the chunk does not touch connect
span-trace-coverage#3.5 · no · the mcp frame tools' server check; no mcp surface changes
span-trace-coverage#4.1 · yes · run.pin_copy and run.conpty_sideload (Windows, if pin_companions changes) are the spans the chunk edits inside this must-trace chain, before pty.spawn
span-trace-coverage#4.2 · yes · run.pin_copy outcome ok|pinned-hash-mismatch must hold, a new absent-copy cause adds a closed value, and no pinned_bin path, endpoint, companion path or hash reaches any span
span-trace-coverage#4.3 · yes · the closed exit-1 detail set on process-exit (already-live, squatted-name, pinned-hash-mismatch, batch-script-child) gains any new absent-copy cause, and the sideload_fallback values stay as they are
span-trace-coverage#4.4 · no · run.start closes at pty.spawn's return and the child's exit lines; the pin-write refusals close before any child exists and take the failure form of the start's log fields
span-trace-coverage#5.1 · no · the confirmed send scenario, listed Not needed (no send path changes) and the boot witness sends no frame
span-trace-coverage#5.2 · no · send's span attributes (channel method send, readiness gate, paste write, confirm window); no send path changes
span-trace-coverage#5.3 · no · send's log fields (channel-request, send-issued, send-confirmed, send-refused, channel-response); no send path changes
span-trace-coverage#5.4 · no · send's confirm-window cleanup and exit codes; no send path changes
span-trace-coverage#6.1 · no · the wait / last readback scenario, listed Not needed with no frame client change
span-trace-coverage#6.2 · no · wait.block and run.wait_dispatch attributes; no wait path changes
span-trace-coverage#6.3 · no · wait / last channel lines and the client's process-exit; no frame client changes
span-trace-coverage#6.4 · no · wait.block's close on the response or a refusal; no frame client changes
