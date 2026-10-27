# Classified — obs · span-trace-coverage.b1

## Answers
span-trace-coverage#1.1 · no · span-kind list only; the chunk adds and re-kinds no span, so these kinds bind nothing it changes
span-trace-coverage#1.2 · no · span-kind list only; the chunk adds and re-kinds no span, so these kinds bind nothing it changes
span-trace-coverage#1.3 · no · span-kind list only; the chunk adds and re-kinds no span, and run.readiness_gate and pty.paste_write bind by name through their own entries
span-trace-coverage#2.1 · yes · the CLI send client's paste refusals write send-refused{side:"client"} into the verb's log, which opens only once an instance resolves
span-trace-coverage#2.2 · yes · run.readiness_gate, where the vt100 degrade's input-not-ready outcome is read, and the feed panic line sit in run's process beside its state.* snapshot read
span-trace-coverage#2.3 · yes · the bounded hook stdin fails open with its line only in hook-<name>.ndjson, and a refused or absent instance still reads no stdin before its check
span-trace-coverage#2.4 · yes · channel-request and channel-response keep only the listed fields and never record params or result bodies, which the bounded frames on both sides must keep
span-trace-coverage#2.5 · yes · the codes-only send-refused and channel-response on MCP rejects (invalid-params, instance-unreachable) bind the tool-argument bounds under mcp.tool_call
span-trace-coverage#3.1 · yes · the CLI send paste refusal is a client-side refusal that must come before connect and before any frame, so it stays ahead of the server check
span-trace-coverage#3.2 · no · the captured-span oracle proves the server check's refusal place, which the chunk does not change, and no paste or bound witness is stated to use it
span-trace-coverage#3.3 · yes · the instance check before any role file binds names-before-path-joins, with the send text read bounded ahead of it and a refused name writing nothing to diagnostics/
span-trace-coverage#3.4 · yes · the bounded snapshot read in read_snapshot_or_replay keeps replayed and absent snapshots unreachable, and the hook stdin bound sits before any frame
span-trace-coverage#3.5 · yes · check_instance before live_endpoint, with instance-unreachable writing no frame and no line, binds the MCP send, wait, last and answer tools that take the name and argument bounds
span-trace-coverage#4.1 · yes · the bounded stamps read in run.version_gate, the claude --version output and the statusline-source.json and settings.json reads must add no span or line to this ordered chain
span-trace-coverage#4.2 · yes · run.version_gate's cli_verified reads false on an unreadable, oversize, malformed or refused stamps.json, and no env value or pinned path reaches any start attribute
span-trace-coverage#4.3 · yes · the version-probe process-start and process-exit pair and the revive agents-probe pair with parse-rejected{claude-agents-json} on an oversize or malformed read bind the bounded claude reads
span-trace-coverage#4.4 · yes · the bounded PTY output read must not turn reader EOF into a claude-child process-exit, since only handle-wait emits that line and run then emits its self exit
span-trace-coverage#5.1 · no · the concurrent pinned-copy placement sits in bin/, which none of the chunk's bounded readers covers, and place_absent is not reached
span-trace-coverage#6.1 · yes · a control-character or empty-text refusal in the wrapper, made before the first wheel read, ends the chain in channel.dispatch with nothing typed and no pty.paste_write or run.confirm_window
span-trace-coverage#6.2 · yes · run.readiness_gate's outcome and vt100_panicked carry the degrade the chunk re-verifies, and text_bytes is the only typed-text measure the paste rule may record
span-trace-coverage#6.3 · yes · the client and wrapper send-refused lines for control-character and empty-text, one per deciding side and codes only, and the input-not-ready detail bind the paste rule and the degrade
span-trace-coverage#6.4 · yes · channel-response from the Drop guard in channel.dispatch must still log on the paste refusal's early return, and the send verb's self process-exit must still close its span
span-trace-coverage#7.1 · yes · the bounded wait and last frames keep the wait.block and last.client chains, with the client refusals and the name check ahead of the server check and channel.request
span-trace-coverage#7.2 · yes · wait.block's after and timeout_ms are the u64 params the chunk types, and run.wait_dispatch's woken_kind stays inside the six WAIT_WAKE kinds the bounded events reader feeds
span-trace-coverage#7.3 · yes · channel-request and channel-response for wait and last keep only the listed fields and no result body, with after and timeout_ms as u64 only, and MCP instance-unreachable writes no line
