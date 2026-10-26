# Classified — obs-plan · obs-scope-summary.b2

## Answers
obs-scope-summary#8.3 · no · no drift report and no R8-stripped CLAUDE value is among the session's changes
obs-scope-summary#9.1 · no · the rmcp stack and the workspace are unchanged, and link and unlink are not MCP tools
obs-scope-summary#9.2 · no · the MCP surface is unchanged, since link and unlink are not MCP tools
obs-scope-summary#9.3 · no · the stdout rule for MCP frames stands, since the new verbs add no telemetry and no MCP frame
obs-scope-summary#10.1 · no · the portable-pty stack and the hosted claude child are not changed, since no viola-pty file is reached
obs-scope-summary#10.2 · no · the claude child's boundary is unchanged, since the session spawns and pastes nothing
obs-scope-summary#10.3 · no · the claude child's spawn, exit and paste instrumentation is unchanged, and the new process lines are the viola CLI's own in cli-<driven>
obs-scope-summary#11.1 · no · the Hook contract and MCP tools are unchanged, since link and unlink are CLI verbs and channel methods only
obs-scope-summary#11.2 · no · the caller boundary is unchanged, since no hook or MCP file is reached
obs-scope-summary#11.3 · no · hook-invoked, hook-decision and MCP tool-call handling are unchanged, since no hook or MCP file is reached
obs-scope-summary#12.1 · no · no claude agents --json read is among the session's changes, and src/run/agents.rs is not reached
obs-scope-summary#12.2 · no · the external CLI boundary is unchanged, since no agents read is added
obs-scope-summary#12.3 · no · the agents spawn, exit and parse outcome are unchanged, since src/run/agents.rs is not reached
obs-scope-summary#13.1 · no · the hook statusline shell-out is untouched, since no statusline or budget code is reached
obs-scope-summary#13.2 · no · the shell-out boundary is unchanged, since no statusline or budget code is reached
obs-scope-summary#13.3 · no · the statusline command string and its shell-out logging are unchanged, since no statusline code is reached
obs-scope-summary#14.1 · no · the IPC stack is unchanged, since link and unlink reuse the existing connect and server check
obs-scope-summary#14.2 · no · the IPC boundary is unchanged, since link and unlink are clients of the existing connect and server check
obs-scope-summary#14.3 · no · the CLI still logs a failed server check as server-verify-failed, and link and unlink add callers of the same connect rather than a new call-site kind
obs-scope-summary#15.1 · no · the event line's closed kinds and the snapshot envelope are unchanged, since unlink was already in the closed set
obs-scope-summary#15.2 · no · the unlink line is one more write of a kind already in the closed set, so events.ndjson stays instrumentable through viola-state
obs-scope-summary#15.3 · no · the scrub rules hold, since the channel pair carries no driver or driven name and the unlink names stay in events.ndjson, the audit stream by design
obs-scope-summary#16.1 · no · no config key or config parse is among the session's changes
obs-scope-summary#16.2 · no · the static configuration stays not-instrumentable, since the session adds no config key
obs-scope-summary#16.3 · no · the load-time parse and its skipped-key logging are unchanged, since no config key is among the session's changes
