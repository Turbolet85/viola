# Classified — obs · obs-scope-summary.b2

## Answers
obs-scope-summary#8.3 · no · drift reports and R8-stripped CLAUDE* values, both Not needed; the chunk's new lines carry closed codes only and touch neither
obs-scope-summary#9.1 · no · a source citation (arch Stack, rmcp, Workspace), no rule of its own
obs-scope-summary#9.2 · no · an instrumentability verdict for rmcp; the stdout rule it leads to is #9.3
obs-scope-summary#9.3 · yes · stdout reserved for MCP frames alone; the chunk's viola mcp stdout and its codes through result.rs and tools.rs must keep it
obs-scope-summary#10.1 · no · a source citation (portable-pty hosting claude), no rule of its own
obs-scope-summary#10.2 · no · a boundary-only verdict for the PTY-hosted claude; the PTY pump is Not needed and no exit-cause code arises there
obs-scope-summary#10.3 · no · spawn/exit on the process handle and paste/readiness telemetry belong to the hosted claude boundary; the chunk's process-exit lines are viola's own exits and no spawn is touched
obs-scope-summary#11.1 · no · a source citation (hook contract, MCP tools), no rule of its own
obs-scope-summary#11.2 · no · a boundary-only verdict for the hook; the hook edge adds no cause, so nothing for the chunk to act on
obs-scope-summary#11.3 · yes · MCP tool-call handling, where the chunk's isError results and rejected-call lines carry codes only; the hook-invoked/hook-decision half is outside
obs-scope-summary#12.1 · no · a source citation (sysinfo and claude agents --json), no rule of its own
obs-scope-summary#12.2 · no · a boundary-only verdict for the external CLI; claude agents parsing is Not needed
obs-scope-summary#12.3 · no · the claude agents parse-failure fallback; no cause here is exit 21 or exit 1 (Not needed)
obs-scope-summary#13.1 · no · a source citation (viola hook statusline), no rule of its own
obs-scope-summary#13.2 · no · a boundary-only verdict for the statusline shell-out; the statusline is Not needed
obs-scope-summary#13.3 · no · the statusline shell-out duration, exit status and command-string rule; the chunk touches no statusline code or command logging
obs-scope-summary#14.1 · no · a source citation (Stack), no rule of its own
obs-scope-summary#14.2 · no · a boundary-only verdict; the rule it leads to is #14.3
obs-scope-summary#14.3 · yes · viola's call sites: the server check's peer-pid and start-time reads (server-verify-failed in client.rs) and the strict-modes checks check_instance, check_home_tree and check_diagnostics, which the chunk's refused_document and refusal lines reach
obs-scope-summary#15.1 · no · a source citation (ndjson, snapshots, contracts), no rule of its own
obs-scope-summary#15.2 · no · an instrumentability verdict through viola-state; no store the chunk touches changes
obs-scope-summary#15.3 · yes · per-store scrub rules: the new process-exit lines carry no endpoint, pinned_bin or pid path, and run's collision check reads the heartbeat alone over a replay-rebuilt snapshot
obs-scope-summary#16.1 · no · a source citation (config management), no rule of its own
obs-scope-summary#16.2 · no · a not-instrumentable verdict for static config; the chunk edits no config path
obs-scope-summary#16.3 · no · config parse-time skipped-key logging in viola-state; the chunk does not reach config parsing
