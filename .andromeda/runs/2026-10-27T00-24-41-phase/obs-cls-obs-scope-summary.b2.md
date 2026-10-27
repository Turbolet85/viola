# Classified — obs · obs-scope-summary.b2

## Answers
obs-scope-summary#8.3 · no · drift reports and the R8 CLAUDE* never-log floor are not reached: the pin write writes no drift report and logs no CLAUDE* value, and the floor is a re-check every new log line owes
obs-scope-summary#9.1 · no · a source citation (rmcp stack, workspace) for the viola mcp scope, which the chunk does not reach
obs-scope-summary#9.2 · no · the instrumentable label of the viola mcp scope; the MCP surface is not reached (Not needed)
obs-scope-summary#9.3 · no · standing S7 · no print!/eprintln!/dbg! and no telemetry on stdout, a convention the task lists
obs-scope-summary#10.1 · no · a source citation (portable-pty, the unmodified claude child) only; its rule is carried by #10.3
obs-scope-summary#10.2 · yes · the claude child is boundary-only, and the run start's pin-then-spawn scenario reaches its spawn boundary (process-start on claude-child)
obs-scope-summary#10.3 · yes · the claude child's spawn and exit boundary (process-start, sideload_fallback on claude-child) that the run start's pin-then-spawn sequence reaches; the witness waits on hook events and events.ndjson offsets, never on its screen
obs-scope-summary#11.1 · no · a source citation (Hook contract, MCP tools) only; the hook rules are carried by #11.2 and #11.3
obs-scope-summary#11.2 · yes · the hook surface is boundary-only, and the race breaks a hook spawn of the pinned path (receipt ran:false, fail-open exit 0); the MCP half is not reached
obs-scope-summary#11.3 · yes · hook-invoked and hook-decision instrumentation, the lines the race witness reads (ran:false, no session-start) and the fail-open hook keeps; the MCP tool-call half is not reached
obs-scope-summary#12.1 · no · a source citation (sysinfo, claude agents --json, Vector 8) only; the agents read is not reached (Not needed)
obs-scope-summary#12.2 · no · the boundary-only label of the claude agents read, which the chunk neither runs nor changes (Not needed: the claude agents --json parse)
obs-scope-summary#12.3 · no · spawn, exit and parse outcome of claude agents, which the chunk does not run or change (Not needed)
obs-scope-summary#13.1 · no · a source citation (the only shell-out) only; the only-shell-out rule is a re-check every change owes, and the chunk adds no shell-out
obs-scope-summary#13.2 · no · the boundary-only label of the statusline shell-out; the chunk touches no status line or budget gate (Not needed)
obs-scope-summary#13.3 · no · the statusline shell-out duration, exit status and command-string rule; the chunk touches no status line (Not needed)
obs-scope-summary#14.1 · no · a source citation (Stack) only; the call-site rule is carried by #14.3
obs-scope-summary#14.2 · yes · the call sites are boundary-only, and the pin write's rename and lock go through them, where the chunk's new io-error WARN or ERROR is logged
obs-scope-summary#14.3 · yes · viola's rename and lock call sites in the pin write (its io error on rename or open), and the home-tree strict-modes check kept before verify's log (a refused tree writes no line)
obs-scope-summary#15.1 · no · a source citation (ndjson, snapshots, event line, heartbeat, Data Classifications) only; the chunk writes no event line or snapshot
obs-scope-summary#15.2 · yes · instrumentable through the viola-state call sites, which the chunk's new sync WARN or ERROR on the pin write's io error goes through
obs-scope-summary#15.3 · yes · the pinned_bin scrub from external output (no path in a span or line) and its diagnostics-only full chain bind; the replay (E5) and budget.json sub-rules are not reached
obs-scope-summary#16.1 · no · a source citation (Config management, config values) only
obs-scope-summary#16.2 · no · config.json is not reached: the chunk reads or parses no config value
obs-scope-summary#16.3 · no · the load-time config.json parse and its skipped-key counts, which the chunk does not touch
