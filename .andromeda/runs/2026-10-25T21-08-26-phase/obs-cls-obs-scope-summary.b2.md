# Classified — obs · obs-scope-summary.b2

## Answers
obs-scope-summary#8.3 · yes · R8-stripped CLAUDE* values are never logged anywhere, and the run start's span and lines carry no env value (Needs)
obs-scope-summary#9.1 · no · a citation only (rmcp, Workspace); the MCP surface is Not needed
obs-scope-summary#9.2 · no · the instrumentability label of the MCP surface, which the chunk does not touch
obs-scope-summary#9.3 · yes · no telemetry to stdout: the CARRY capture sees its line without printing to stdout or stderr (Needs)
obs-scope-summary#10.1 · no · a citation only (portable-pty); the PTY seam and the child spawn are Not needed
obs-scope-summary#10.2 · no · the boundary-only label of the third-party child, which the chunk does not spawn
obs-scope-summary#10.3 · no · child spawn and exit, paste writes and the readiness outcome; the exit-1 and the budget refusal fire before any child spawn or paste
obs-scope-summary#11.1 · no · a citation only (hook contract, MCP tools); the hook roles and the MCP surface are Not needed
obs-scope-summary#11.2 · no · the boundary-only label of hook and MCP tool-call handling, which the chunk does not touch
obs-scope-summary#11.3 · no · hook-invoked and hook-decision instrumentation and MCP tool-call handling; the chunk reaches neither
obs-scope-summary#12.1 · no · a citation only (sysinfo, claude agents --json); the chunk reads no agents row
obs-scope-summary#12.2 · no · the boundary-only label of the external CLI, which the chunk does not spawn or parse
obs-scope-summary#12.3 · no · the spawn, exit and parse outcome of claude agents, which the chunk does not run
obs-scope-summary#13.1 · no · a citation only (viola hook statusline); the shell-out is Not needed
obs-scope-summary#13.2 · no · the boundary-only label of the statusline shell-out, which is Not needed
obs-scope-summary#13.3 · no · the shell-out is Not needed and the scope names no statusline command for the start trace, so the never-log rule is a re-check every change owes
obs-scope-summary#14.1 · no · a citation only (Stack); the bind call-site rule it grounds is #14.3
obs-scope-summary#14.2 · yes · boundary-only: the listener bind, pipe mode and first-instance refusal are instrumented at viola's own call site
obs-scope-summary#14.3 · yes · the named-pipe bind and its first-instance and remote-client refusals are instrumented at viola's bind call site on the viola seam, the interprocess target OFF
obs-scope-summary#15.1 · no · a citation only (ndjson, snapshots, event line, heartbeat); the rules it grounds are #15.2, #15.3 and the Needs' diag-line schema
obs-scope-summary#15.2 · yes · the ndjson store is instrumented only through viola-state's call sites, and the chunk's new lines (send-refused, process-exit, listener event) are written into it
obs-scope-summary#15.3 · yes · the endpoint and pinned_bin absolute paths never reach external output (the squatted refusal and start lines carry no path or pid), and events.ndjson payload is never copied into process-log lines
obs-scope-summary#16.1 · no · a citation only (config management, config values)
obs-scope-summary#16.2 · no · the not-instrumentable label of static config, which the chunk does not change
obs-scope-summary#16.3 · no · the config load-time parse and its skipped-key counts are existing logging that the chunk does not change
