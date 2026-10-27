# Classified — obs · obs-scope-summary.b2

## Answers
obs-scope-summary#8.3 · yes · the R8-stripped CLAUDE* NEVER-log floor binds the claude --version and claude agents --json spawns the chunk bounds, names and counts only on process-start
obs-scope-summary#9.1 · no · a source pointer to the rmcp stack and Workspace, stating no rule for the chunk's code or tests to honour
obs-scope-summary#9.2 · yes · the MCP server is instrumentable, so the chunk's mcp stdin and tool-argument rejects get their codes-only lines
obs-scope-summary#9.3 · no · standing S6 · no print!/eprintln!/dbg! in product code paths; stdout stays reserved on every role
obs-scope-summary#10.1 · no · a source pointer to portable-pty and Project Intent, stating no rule for the chunk's code or tests to honour
obs-scope-summary#10.2 · yes · boundary-only binds the chunk's PTY pump read loop and vt100 feed, which take no per-byte or per-chunk span or event
obs-scope-summary#10.3 · yes · the child is instrumented only at spawn, exit, paste writes and the readiness-gate outcome, which gate.rs's catch_unwind degrade to input-not-ready must still log
obs-scope-summary#11.1 · no · a source pointer to the hook contract and MCP tools, stating no rule for the chunk's code or tests to honour
obs-scope-summary#11.2 · yes · boundary-only binds the chunk's hook stdin bound, its fail-open path and its mcp tool-call rejects, all kept at the handling boundary
obs-scope-summary#11.3 · yes · hook-invoked, hook-decision and MCP tool-call handling are the instrumented points the chunk's fail-open and codes-only rejects write to
obs-scope-summary#12.1 · no · a source pointer to sysinfo, claude agents --json and security Vector 8, stating no rule for the chunk's code or tests to honour
obs-scope-summary#12.2 · yes · boundary-only binds the chunk's claude agents --json read, instrumented at spawn, exit and the parse outcome only
obs-scope-summary#12.3 · yes · an oversize or malformed claude agents --json read gives one parse-rejected line and the value "unknown", with the rows never logged
obs-scope-summary#13.1 · no · a source pointer to the viola hook statusline contract, stating no rule for the chunk's code or tests to honour
obs-scope-summary#13.2 · yes · boundary-only binds the statusline shell-out the chunk bounds under take(MAX_FRAME), instrumented at its boundary only
obs-scope-summary#13.3 · yes · the shell-out logs duration and exit status only, its stdout and command string never logged, which the chunk's passthrough keeps
obs-scope-summary#14.1 · no · a source pointer to the Stack, stating no rule for the chunk's code or tests to honour
obs-scope-summary#14.2 · yes · boundary-only binds the chunk's bounded readers as viola call sites (budget.json's shared lock, the strict-modes checks, the snapshot and stamps reads), not the OS internals
obs-scope-summary#14.3 · yes · the chunk's budget.json shared-lock read and its strict-modes checks are the named call sites, so its no-line failure paths must match the refusal-telemetry trigger
obs-scope-summary#15.1 · no · a source pointer to the ndjson and snapshot stack and the standard contracts, stating no rule for the chunk's code or tests to honour
obs-scope-summary#15.2 · yes · the events.ndjson skip counts, the state-recovered line and the checked budget and stamps reads are the viola-state call sites the chunk changes
obs-scope-summary#15.3 · yes · the per-store scrub rules bind the chunk's events.ndjson, snapshot.json replay and budget.json readers, the replay carrying no endpoint, pid or statusline_command
obs-scope-summary#16.1 · no · a source pointer to the config-management hints and the config data classification, stating no rule for the chunk's code or tests to honour
obs-scope-summary#16.2 · yes · not-instrumentable binds the chunk's closed-type config read, which logs no config value
obs-scope-summary#16.3 · yes · the load-time parse is the only observable, so the five_hour, seven_day and dialog_hold_s reads emit one parse-rejected line per bad key and no other
