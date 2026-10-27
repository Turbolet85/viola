# Classified — obs · obs-scope-summary.b2

## Answers
obs-scope-summary#8.3 · yes · the R8-stripped CLAUDE* never-log floor, which the chunk's Needs name over the claude --version and claude agents --json spawns it bounds (names and counts only on process-start, never values)
obs-scope-summary#9.1 · no · a source citation (arch excerpt, Stack rmcp and Workspace) that states no rule the chunk's code or tests must honour
obs-scope-summary#9.2 · no · an instrumentability label that restricts nothing; the MCP log lines the chunk adds are bound by 11.3 and the chunk's Needs, not by this label
obs-scope-summary#9.3 · no · standing S6 · stdout stays reserved on every role: no print!, eprintln! or dbg! in product code paths
obs-scope-summary#10.1 · no · a source citation (portable-pty, hosts the unmodified claude, Project Intent) that states no rule
obs-scope-summary#10.2 · yes · boundary-only PTY: the size cap on the child's output into the vt100 feed adds no per-chunk span or event in the pump read loop or the feed
obs-scope-summary#10.3 · yes · the boundary rules bind the chunk: the readiness-gate outcome as a logged signal, the paste writes the paste rule covers, and a size cap that must not be read as EOF or exit
obs-scope-summary#11.1 · no · a source citation (Standard Contracts: Hook contract; MCP tools) that states no rule
obs-scope-summary#11.2 · yes · boundary-only hook and MCP: the chunk's fail-open hook line and codes-only MCP line sit at the boundary, never inside the bounded readers
obs-scope-summary#11.3 · yes · names the instrumentation points (hook-invoked, hook-decision, MCP tool-call handling) where the chunk's fail-open detail and codes-only MCP line land; viola sees only the inbound stdin the take() bounds
obs-scope-summary#12.1 · no · a source citation (Stack, sysinfo and claude agents --json; security Vector 8) that states no rule
obs-scope-summary#12.2 · yes · boundary-only claude agents --json: the bounded read and its parse log only the spawn and the parse outcome, never the rows
obs-scope-summary#12.3 · yes · an oversize or malformed read becomes "unknown" with one parse-rejected claude-agents-json line; the rows are untrusted and never logged verbatim
obs-scope-summary#13.1 · no · a source citation (Standard Contracts: viola hook statusline, the only shell-out) that states no rule
obs-scope-summary#13.2 · yes · boundary-only statusline shell-out: the bounded stdout read adds no line inside the shell-out, and its stdout is never logged
obs-scope-summary#13.3 · no · standing S3 · no content, command strings included, in any home-level line; content goes only to the instance's detail-<process>.ndjson
obs-scope-summary#14.1 · no · a source citation (Stack) that states no rule
obs-scope-summary#14.2 · yes · boundary-only OS call sites: the channel frame reads and the locked budget.json read log at their own call site, never the OS internals
obs-scope-summary#14.3 · yes · the strict-modes checks the stamps and budget reads pass through are call sites whose refusal-line rule the trigger owns, so the chunk adds no line there
obs-scope-summary#15.1 · no · a source citation (Stack ndjson and snapshots; Standard Contracts; security Data Classifications) that states no rule
obs-scope-summary#15.2 · no · an instrumentability label with no restriction; the viola-state log rules bind through 15.3 and the chunk's Needs
obs-scope-summary#15.3 · yes · the events.ndjson payload stays out of process-log lines, and a replayed snapshot holds no endpoint, pid, started_at or statusline_command; the bounded snapshot, events and budget reads keep both
obs-scope-summary#16.1 · no · a source citation (Observability Hints: Config management; security Data Classifications) that states no rule
obs-scope-summary#16.2 · yes · not-instrumentable config: the closed-type config read logs no value, only the parse-rejected key lines and no others
obs-scope-summary#16.3 · yes · only the load-time parse is observable, through the skipped-key counts; the chunk's one parse-rejected config-json line per bad key is that parse
