# Classified — obs · span-trace-coverage.b1

## Answers
span-trace-coverage#1.1 · no · the chunk adds no CLIENT-kind span, since its pin step is a local INTERNAL write and pty.spawn is unchanged
span-trace-coverage#1.2 · no · the chunk adds no SERVER-kind span, and hook.handle, channel.dispatch and mcp.tool_call are not reached by the absent-copy write
span-trace-coverage#1.3 · yes · the pin step's run.pin_copy and any state.* span the viola-state absent-copy instrumentation adds must be INTERNAL under run.* and state.*
span-trace-coverage#2.1 · yes · verify's pin logs process-start and process-exit to cli-<name>.ndjson only when an instance resolves, a refused home tree writes no line, and the full chain goes to detail-cli only
span-trace-coverage#2.2 · yes · the viola run start sequence's manual spans hold run.pin_copy and any absent-copy state.* span the fix adds
span-trace-coverage#2.3 · yes · the hook fail-open (exit 0, empty stderr, hook-<name>.ndjson only, silent no-op on a refused or absent instance) is the surface the race breaks and must stay as it is
span-trace-coverage#2.4 · no · the chunk adds or changes no channel method, channel.request or channel.dispatch span, or channel-request and channel-response line
span-trace-coverage#2.5 · no · the chunk changes no viola mcp tool, mcp.tool_call span or mcp.ndjson line, the mcp surface being excluded
span-trace-coverage#3.1 · no · the frame verbs' connect-time server check is a frame-client rule and the chunk changes no frame client
span-trace-coverage#3.2 · no · the captured-span cases pin the frame verbs' and the hook's server-check refusal, which the chunk does not touch
span-trace-coverage#3.3 · no · the six frame verbs' instance check is a frame-client rule, and the chunk's verify check is the home-tree check instead
span-trace-coverage#3.4 · no · the hook's connect and server-verify path is unchanged, and the race fails the pinned spawn before any connect
span-trace-coverage#3.5 · no · the viola-mcp frame tools' server check is unchanged and the chunk changes no mcp tool
span-trace-coverage#4.1 · yes · the chunk's pin step is the run.pin_copy span of this chain, between run.collision_check and run.conpty_sideload
span-trace-coverage#4.2 · yes · run.pin_copy's outcome set takes any new absent-copy cause, and the pinned_bin path stays out of every span
span-trace-coverage#4.3 · yes · the closed run detail list takes any new absent-copy exit-1 detail, and the sideload_fallback values bind if the Windows arm changes
span-trace-coverage#4.4 · yes · run.start's close and run's own process-exit with subject self are where a new absent-copy exit-1 path lands, while the handle-wait child exit is untouched
span-trace-coverage#5.1 · no · the chunk changes no send path, and the Confirmed send (CL-1) scenario is excluded
span-trace-coverage#5.2 · no · the send's channel, readiness-gate, paste and confirm-window attributes sit on the CL-1 path the chunk does not change
span-trace-coverage#5.3 · no · the send-issued, send-confirmed and send-refused lines are CL-1 lines the chunk does not reach
span-trace-coverage#5.4 · no · the confirm-window cleanup and the send.client exit codes are CL-1 behaviour the chunk does not change
span-trace-coverage#6.1 · no · wait.block and last.client belong to the wait/last readback scenario, which the chunk does not change
span-trace-coverage#6.2 · no · the wait.block and run.wait_dispatch attributes belong to the wait/last readback scenario, which the chunk does not change
span-trace-coverage#6.3 · no · the wait and last channel lines belong to the wait/last readback scenario, which the chunk does not change
span-trace-coverage#6.4 · no · the wait.block cleanup belongs to the wait/last readback scenario, which the chunk does not change
