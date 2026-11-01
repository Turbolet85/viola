# Classified — obs · obs-scope-summary.b2

## Answers
obs-scope-summary#8.3 · no · drift reports to diagnostics/ and R8-stripped CLAUDE* values are never logged; the chunk touches neither drift reports nor the R8 strip
obs-scope-summary#9.1 · no · source citation only (arch excerpt, Stack rmcp, Workspace); states no behaviour the chunk's code must honour
obs-scope-summary#9.2 · no · bare instrumentability classification for rmcp and Workspace; states no rule
obs-scope-summary#9.3 · no · standing S7 · No telemetry on stdout and no print or dbg! macros in hook, run or mcp paths
obs-scope-summary#10.1 · no · source citation only (portable-pty hosts the unmodified claude); states no rule
obs-scope-summary#10.2 · no · bare boundary-only classification for the PTY child; states no rule
obs-scope-summary#10.3 · no · spawn/exit detection on the process handle and the paste and readiness boundaries are unchanged; the chunk's wrapper refusal during the child run does not alter child spawn or exit detection
obs-scope-summary#11.1 · no · source citation only (Hook contract, MCP tools); states no rule
obs-scope-summary#11.2 · no · bare boundary-only classification for hook and MCP; states no rule
obs-scope-summary#11.3 · yes · the hook connect's fail-open hook-decision detail and the MCP frame tools' client-side instance-unreachable refusal inside mcp.tool_call are the hook-invoked, hook-decision and MCP tool-call instrumentation this entity names
obs-scope-summary#12.1 · no · source citation only (sysinfo, claude agents --json); the chunk reads no claude agents output
obs-scope-summary#12.2 · no · bare boundary-only classification for the external CLI; states no rule the chunk reaches
obs-scope-summary#12.3 · no · spawn/exit and parse outcome of the external CLI, which the chunk does not reach
obs-scope-summary#13.1 · no · source citation only (viola hook statusline); states no rule
obs-scope-summary#13.2 · no · bare boundary-only classification; the chunk reaches only hook connect and hook.dialog, not the statusline shell-out
obs-scope-summary#13.3 · no · statusline shell-out duration, exit status and command string, which the chunk does not touch
obs-scope-summary#14.1 · no · source citation only (Stack); states no rule
obs-scope-summary#14.2 · no · bare boundary-only classification; states no rule
obs-scope-summary#14.3 · yes · the chunk adds the bind-side socket-directory check, the connect-side peer euid check beside the server's peer-pid read, and the strict-modes home check in board.rs scan; this entity names bind, connect, peer-pid reads and the home strict-modes checks as the call sites to instrument
obs-scope-summary#15.1 · no · source citation only (ndjson, snapshots, event line, heartbeat); states no rule
obs-scope-summary#15.2 · yes · the chunk's board.rs scan home-check edit is a viola-state call site, so the state-store instrumentation this entity places through viola-state binds where the new refusal lines are written
obs-scope-summary#15.3 · yes · endpoint and pinned_bin paths in snapshot.json are scrubbed from external output; the chunk's new lines and spans must carry no endpoint name or socket path, and hook connect fails open on a replay snapshot with no endpoint
obs-scope-summary#16.1 · no · source citation only (config management, config values); states no rule
obs-scope-summary#16.2 · no · not-instrumentable classification for static config; the chunk changes no config value
obs-scope-summary#16.3 · no · load-time config parse logging of skipped-key counts; the chunk adds no config key and no load-time parse
