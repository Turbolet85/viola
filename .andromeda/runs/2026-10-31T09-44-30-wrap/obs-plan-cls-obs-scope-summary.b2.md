# Classified — obs-plan · obs-scope-summary.b2

## Answers
obs-scope-summary#8.3 · no · the logger stack and diagnostics/ paths are unreached and no env var or config key moved, so the drift and R8 CLAUDE* logging claims hold
obs-scope-summary#9.1 · no · Cargo.toml is reached but no dependency is added or bumped and no manifest edge is added, so the rmcp and workspace source still holds
obs-scope-summary#9.2 · no · instrumentability is unmoved and no manifest edge is added
obs-scope-summary#9.3 · no · the measured oversize cut leaves both streams empty, so stdout stays reserved for MCP frames and no telemetry reaches it
obs-scope-summary#10.1 · no · portable-pty is neither added nor bumped and Cargo.lock is unchanged
obs-scope-summary#10.2 · no · instrumentability is unmoved
obs-scope-summary#10.3 · no · the change re-types resolve_program and folds Refusal into AgentError with no span or event added, so the spawn, exit, paste and readiness boundary holds and crates/viola-pty is unreached
obs-scope-summary#11.1 · no · the arch excerpt is unedited and the MCP tool rule stays as it was, codes only with the wire null
obs-scope-summary#11.2 · no · instrumentability is unmoved
obs-scope-summary#11.3 · no · the measured oversize cut is one parse-rejected with count 1 and both streams empty, and no event is added to tool-call handling
obs-scope-summary#12.1 · no · sysinfo is neither added nor bumped, so the Stack source holds
obs-scope-summary#12.2 · no · instrumentability is unmoved
obs-scope-summary#12.3 · yes · stale · viola-agent-claude is reached whole and folds Refusal into AgentError, and the report does not show that a failed claude agents parse still reads unknown with a counter
obs-scope-summary#13.1 · no · the arch excerpt is unedited and the report names no new shell-out, so statusline stays the only one
obs-scope-summary#13.2 · no · instrumentability is unmoved
obs-scope-summary#13.3 · yes · stale · src/cmd/hook/statusline.rs is reached whole, and the report does not show its shell-out still logs only duration and status with the command string kept out of logs
obs-scope-summary#14.1 · no · the Stack source is unedited and no dependency is added or bumped
obs-scope-summary#14.2 · no · instrumentability is unmoved
obs-scope-summary#14.3 · yes · stale · src/cmd/client.rs and the viola-channel server and client are reached whole, and the report does not show the server-verify-failed line or the check's call sites hold there
obs-scope-summary#15.1 · no · events.ndjson and the process-log writer sit in viola-state and the unreached logger, and the diag-line schema gains only a closed three-word detail, so no payload reaches a process-log line
obs-scope-summary#15.2 · no · instrumentability is unmoved and its call sites sit in the unreached viola-state
obs-scope-summary#15.3 · yes · stale · src/run/budget.rs is reached whole, and the report does not show that budget.json metadata may still be logged as structured fields there
obs-scope-summary#16.1 · no · the config source is unedited and no config key or env var moved
obs-scope-summary#16.2 · no · instrumentability is unmoved
obs-scope-summary#16.3 · no · the load-time parse is logged in the unreached viola-state and src/main.rs, and no config key moved
