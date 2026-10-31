# Classified — obs · obs-scope-summary.b2

## Answers
obs-scope-summary#8.3 · no · R8-stripped values and drift reports are not reached (Not needed: R8 env-strip counts, drift reports); the chunk's new lines carry closed codes only.
obs-scope-summary#9.1 · no · a source citation (Stack rmcp, Workspace) that states no rule.
obs-scope-summary#9.2 · no · an instrumentability label with no rule the chunk's code must meet.
obs-scope-summary#9.3 · yes · `viola mcp` stdout holds MCP frames alone; the table asserts the refused home check and MCP results write nothing to stdout.
obs-scope-summary#10.1 · no · source citation for portable-pty.
obs-scope-summary#10.2 · no · boundary label for the PTY child.
obs-scope-summary#10.3 · no · PTY spawn/exit, paste-write and readiness-gate telemetry are not reached (Not needed: no exit-cause code arises there).
obs-scope-summary#11.1 · no · source citation for the hook and MCP contracts.
obs-scope-summary#11.2 · no · boundary label.
obs-scope-summary#11.3 · yes · MCP tool-call handling binds: `isError` results and rejected-call lines carry codes only; the stdin oversize cut and hook-decision are not reached.
obs-scope-summary#12.1 · no · source citation for sysinfo and `claude agents --json`.
obs-scope-summary#12.2 · no · boundary label.
obs-scope-summary#12.3 · no · the `claude agents --json` parse outcome is not an exit-21 or exit-1 cause (Not needed).
obs-scope-summary#13.1 · no · source citation for the statusline shell-out.
obs-scope-summary#13.2 · no · boundary label.
obs-scope-summary#13.3 · no · the statusline shell-out is not re-opened (Not needed: exit 20 unchanged).
obs-scope-summary#14.1 · no · source citation (Stack).
obs-scope-summary#14.2 · no · boundary label.
obs-scope-summary#14.3 · yes · the server check's peer-pid and start-time reads (`server-verify-failed`) and the strict-modes `check_home_tree` refusals of `wait`, `last` and `verify` are edited call sites.
obs-scope-summary#15.1 · no · source citation for ndjson, snapshots and heartbeat.
obs-scope-summary#15.2 · no · instrumentability label; no viola-state call site changes.
obs-scope-summary#15.3 · yes · the no-path/no-pid rule for external output and `run`'s collision check reading the heartbeat alone under replay bind the edited `run.rs` collision check and the new codes.
obs-scope-summary#16.1 · no · source citation for config values.
obs-scope-summary#16.2 · no · not-instrumentable label.
obs-scope-summary#16.3 · no · config parse at load is not reached by the chunk.
