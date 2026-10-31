# Classified — obs-plan · obs-scope-summary.b2

## Answers
obs-scope-summary#8.3 · no · drift reports and R8-stripped CLAUDE values are not in the session's diff or touches
obs-scope-summary#9.1 · no · Cargo.toml is reached but no dependency was added or bumped, so the rmcp stack and workspace it cites still hold
obs-scope-summary#9.2 · no · the instrumentable label still holds, MCP still records by isError alone
obs-scope-summary#9.3 · yes · add-to · viola-mcp now maps each refused frame to a cause in the isError detail, with no line and stdout still reserved for MCP frames
obs-scope-summary#10.1 · no · the viola-pty and viola-agent-claude files are not in the diff
obs-scope-summary#10.2 · no · the boundary-only label is unchanged and no file of the pty entity is reached
obs-scope-summary#10.3 · no · spawn and exit on the process handle are unchanged, and the pty entity is untouched
obs-scope-summary#11.1 · no · the hook contract and MCP tools citation are unchanged
obs-scope-summary#11.2 · no · the boundary-only label is unchanged
obs-scope-summary#11.3 · no · MCP tool-call instrumentation and the stdin cut are unchanged, and the isError body gains only a detail
obs-scope-summary#12.1 · no · the sysinfo and claude agents citation is not touched and no file of it is reached
obs-scope-summary#12.2 · no · the boundary-only label is unchanged and no file of the entity is reached
obs-scope-summary#12.3 · no · the external CLI parse outcome is not touched
obs-scope-summary#13.1 · no · the statusline shell-out lives in src/cmd/hook.rs, which is not in the diff
obs-scope-summary#13.2 · no · the boundary-only label is unchanged and the hook is untouched
obs-scope-summary#13.3 · no · the shell-out duration and exit status instrumentation is unchanged and the hook is untouched
obs-scope-summary#14.1 · no · the arch Stack citation is unchanged
obs-scope-summary#14.2 · no · the boundary-only label is unchanged
obs-scope-summary#14.3 · yes · add-to · viola-core now supplies UnreachableCause, the closed set of exit 21's five words (F11), and the bind and server-check carve-out with no code for the socket directory and no macOS server check (F12)
obs-scope-summary#15.1 · no · the source citation is unchanged
obs-scope-summary#15.2 · yes · add-to · viola-state gains without_endpoint, a reader that opens nothing and writes no line, which the call-site instrumentation statement must now account for
obs-scope-summary#15.3 · no · live_endpoint still gives none over a snapshot rebuilt without an endpoint, and collision_check still reads the heartbeat alone
obs-scope-summary#16.1 · no · the config values citation is unchanged
obs-scope-summary#16.2 · no · the not-instrumentable label is unchanged
obs-scope-summary#16.3 · no · the config load-time parse and skipped-key logging are not in the session's touches
