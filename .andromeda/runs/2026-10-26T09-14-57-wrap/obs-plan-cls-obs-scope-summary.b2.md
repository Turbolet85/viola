# Classified — obs-plan · obs-scope-summary.b2

## Answers
obs-scope-summary#8.3 · no · the diagnostics/ sinks and the ledger read are unchanged (Untouched list) and the new lines carry no CLAUDE* value, so no drift report or R8 claim moves
obs-scope-summary#9.1 · no · the rmcp Stack and Workspace citation still holds on the changed viola-mcp tools.rs, which adds a verify step and no crate or dependency
obs-scope-summary#9.2 · no · instrumentable still holds on the changed tools.rs: the four frame tools keep their mcp.tool_call span and a mismatch takes the same no-line path as unreachable
obs-scope-summary#9.3 · no · the claim still holds on the changed tools.rs: the verify step writes nothing to stdout, which stays reserved for MCP frames
obs-scope-summary#10.1 · no · the portable-pty source citation is unchanged and no PTY file is reached
obs-scope-summary#10.2 · no · boundary-only still holds: no PTY file is reached, and the new process-exit line is the CLI's own exit (subject self), not the child's
obs-scope-summary#10.3 · no · the child spawn, exit and paste boundary is unchanged: no PTY file is reached, and the new process-exit line is the CLI's own exit
obs-scope-summary#11.1 · no · the Hook contract and MCP tools citation still holds on the changed hook.rs and tools.rs: the hook still fails open with exit 0, empty streams and no frame
obs-scope-summary#11.2 · no · boundary-only still holds on the changed hook.rs and tools.rs: the caller remains the only source of the inbound payload
obs-scope-summary#11.3 · no · the claim still holds on the changed hook.rs and tools.rs: viola still sees only the caller's stdin and its own response, and hook-decision and tool-call handling remain the instrumented points
obs-scope-summary#12.1 · no · the sysinfo and claude agents Stack citation still holds on the changed liveness.rs, and the agents read is not reached
obs-scope-summary#12.2 · no · boundary-only still holds on the changed liveness.rs: the server verdict is held in memory and the external CLI boundary is unchanged
obs-scope-summary#12.3 · yes · add-to · the sysinfo liveness check now also carries the server verdict and returns a LiveEndpoint (endpoint, pid, start time) from one snapshot read
obs-scope-summary#13.1 · no · the viola hook statusline contract citation holds: statusline.rs and budget.json are not reached, and the hook.rs change is the event and dialog connect
obs-scope-summary#13.2 · no · boundary-only still holds: the statusline shell-out and its snapshot read at statusline.rs are unchanged
obs-scope-summary#13.3 · no · the shell-out duration and exit-status rule and the never-logged command string still hold, since the statusline arm is not reached
obs-scope-summary#14.1 · no · the Stack citation still holds on the changed channel client.rs, and no dependency was added or bumped
obs-scope-summary#14.2 · no · boundary-only still holds: bind, connect and the other call sites remain instrumented boundaries on the changed client.rs
obs-scope-summary#14.3 · yes · add-to · F8: on macOS the connect call site gets no peer pid, so its server check compares nothing (the recorded gap)
obs-scope-summary#15.1 · no · the ndjson and snapshot Stack and contract citations hold, and the new lines reuse the existing diag-line schema with no event value or schema edit
obs-scope-summary#15.2 · no · instrumentable through the viola-state call sites still holds, since the liveness change adds no store and no line kind
obs-scope-summary#15.3 · no · the per-store scrub rules hold on the changed liveness.rs: LiveEndpoint stays in memory, and the new lines carry no pid, start time, path or endpoint
obs-scope-summary#16.1 · no · the config citation holds: no config key was added and no config parse changed
obs-scope-summary#16.2 · no · not-instrumentable still holds, since no config value is touched
obs-scope-summary#16.3 · no · the load-time parse and its skipped-key logging are untouched, since no config key or value moved
