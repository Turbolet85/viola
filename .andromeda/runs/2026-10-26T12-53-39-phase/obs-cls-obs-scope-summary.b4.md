# Classified — obs · obs-scope-summary.b4

## Answers
obs-scope-summary#22.3 · no · the mcp exporter (mcp.ndjson and its stderr fallback); no viola-mcp file is reached
obs-scope-summary#22.4 · no · mcp's service identity (process mcp, from); no viola-mcp file is reached
obs-scope-summary#22.5 · no · the MCP isError codes-only rule; no viola-mcp file is reached
obs-scope-summary#23.1 · yes · the init order (panic hook, home and instance resolution, diagnostics open, explicit writer, process-start) the new verbs' cli process log runs through
obs-scope-summary#24.1 · yes · the tracing stack and sink rules (one file per role, no stdout sink, one write per line, RFC 3339 ms Z) that cli-<name>.ndjson and the since instant follow
obs-scope-summary#25.0 · yes · the events.ndjson line shape the unlink line takes, the closed event enum with its corr rows (channel lines take the JSON-RPC id; process-start and process-exit take null), and the cli process value
obs-scope-summary#26.1 · yes · the home-level cli-<name>.ndjson basename, the instance-level detail file the internal-fault chain goes to, and the 0600-in-0700 rule
obs-scope-summary#27.1 · no · the run start critical path's surfaces; the chunk changes no run start path
obs-scope-summary#27.2 · no · the run start's must-trace spans and product event order; the chunk changes no run start path
obs-scope-summary#27.3 · no · run's start fields and exit-1 causes (already-live and the rest); the chunk changes no run start path
obs-scope-summary#27.4 · no · a source line for critical path 1, the run start the chunk does not change
obs-scope-summary#28.1 · no · the send path's surfaces; the chunk changes no send path
obs-scope-summary#28.2 · no · the send path's must-trace spans and event order; the chunk changes no send path
obs-scope-summary#28.3 · yes · the channel-line corr rule (the JSON-RPC id) that the link and unlink channel-request and channel-response pair takes; the send-only fields do not reach the chunk
obs-scope-summary#28.4 · no · a source line for critical path 2, the send the chunk does not change
obs-scope-summary#29.1 · no · the wait path's surfaces; the chunk changes no wait path
obs-scope-summary#29.2 · no · the wait path's must-trace spans and events; the chunk changes no wait path
obs-scope-summary#29.3 · no · the wait fields (after, timeout_ms, outcome, exit_code mid-wait); the chunk changes no wait path
obs-scope-summary#29.4 · no · a source line for critical path 3, the wait the chunk does not change
obs-scope-summary#30.1 · no · the dialog path's surfaces; the chunk changes no hook or dialog path
obs-scope-summary#30.2 · no · the dialog path's must-trace spans and hook events; the chunk changes no hook or dialog path
obs-scope-summary#30.3 · yes · the server-verify-failed shape (exit 21, during connect, no pid, start time, path or endpoint) the verbs' connect refusal must match; the answer and dialog-hook fields do not reach the chunk
obs-scope-summary#30.4 · no · a source line for critical paths 4 and 7, the dialog answering the chunk does not change
obs-scope-summary#31.1 · no · the wheel path's surfaces (send refusal, pause, release); the chunk changes no wheel path
obs-scope-summary#31.2 · no · the wheel path's spans and events (send-refused, pause, release, release-from-driver); the chunk changes no wheel path
