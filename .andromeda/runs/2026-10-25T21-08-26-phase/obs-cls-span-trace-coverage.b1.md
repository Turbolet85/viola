# Classified — obs · span-trace-coverage.b1

## Answers
span-trace-coverage#1.1 · no · client verb spans, client-side channel.request, statusline.shell_out and pty.spawn; the chunk changes no client path and pty.spawn lies past channel.bind, beyond the start range
span-trace-coverage#1.2 · no · the SERVER spans channel.dispatch, mcp.tool_call, hook.handle and hook.statusline; a remote-client rejection precedes any request and the chunk edits no dispatch, MCP or hook path
span-trace-coverage#1.3 · yes · run.* is INTERNAL and run.start and run.collision_check, which the squatted-name refusal edit runs under, are in the start range; state.*, hook.decision_emit and pty.paste_write lie outside it
span-trace-coverage#2.1 · no · the short-lived CLI verbs' dispatch spans and cli-<name>.ndjson lines; the chunk changes no verb, and the squatted refusal is viola run, which logs to run-<name>.ndjson
span-trace-coverage#2.2 · yes · the viola run start sequence's manual spans (the PTY seam and readiness gate excluded) and the seam capture of a bind failure, which the squatted-name refusal and the bind-failure capture run through
span-trace-coverage#2.3 · no · the hook roles' spans and hook-invoked and hook-decision lines; the chunk changes no hook path and the scope excludes them
span-trace-coverage#2.4 · yes · the viola-channel listener's sync instrumentation, the channel-request and channel-response lines that never carry params or result bodies, and the bind failure captured at the seam
span-trace-coverage#2.5 · no · the viola-mcp surface (mcp.tool_call and codes-only lines); the scope excludes the MCP tools and their Tokio client
span-trace-coverage#3.1 · yes · the must-trace viola run start chain up to channel bind, which the scope names: the squatted-name refusal sits at run.collision_check and the named-pipe bind at channel.bind
span-trace-coverage#3.2 · yes · run.collision_check outcome squatted-name, channel.bind endpoint_kind named-pipe, and no endpoint name, pinned path or env value in any span
span-trace-coverage#3.3 · yes · the squatted-name failure line process-exit{subject:"self", exit_code:1, detail} and the process-start endpoint_kind, both named in the scope
span-trace-coverage#3.4 · no · the cleanup after pty.spawn and the child's handle-wait exit, beyond the start range the scope names; the squatted-name failure line is covered by span-trace-coverage#3.3
span-trace-coverage#4.1 · no · the confirmed-send chain through readiness gate, paste and confirm window, none of which the chunk reaches; the budget-paused refusal is not a confirmed send
span-trace-coverage#4.2 · no · the send span attributes (readiness outcome, text_bytes, window_ms); the chunk's refusal sets none of them and CARRY case_4 reads the log line, not span attributes
span-trace-coverage#4.3 · yes · the wrapper send-refused line shape (no cursor, corr as the end offset, conn and rpc_id, no send-issued), which CARRY case_4 reads
span-trace-coverage#4.4 · no · the confirm-window close and the send.client exit of a confirmed send; neither the budget-paused refusal nor the chunk's edits reach them
span-trace-coverage#5.1 · no · the wait and last must-trace chain; the chunk changes no wait or last path
span-trace-coverage#5.2 · no · the wait.block and run.wait_dispatch attributes; the chunk changes no wait or last path
span-trace-coverage#5.3 · no · the wait and last channel lines and the wait outcome; the chunk changes no wait or last path
span-trace-coverage#5.4 · no · the wait.block close and the disconnect log of wait or last; the chunk changes neither path
span-trace-coverage#6.1 · no · the dialog-to-answer must-trace chain; the chunk changes no dialog or answer path
span-trace-coverage#6.2 · no · the run.dialog_register and run.dialog_await attributes; the chunk touches no dialog path
span-trace-coverage#6.3 · no · the dialog-raised, dialog-answered and hook dialog lines; the chunk touches no dialog path
span-trace-coverage#6.4 · no · the dialog hold's expiry line and its close; the chunk touches no dialog path
span-trace-coverage#7.1 · no · the wheel-takeover chain (run.wheel_transition, pause and release); the chunk changes no wheel path, and its refused send is a budget refusal, not a wheel one
