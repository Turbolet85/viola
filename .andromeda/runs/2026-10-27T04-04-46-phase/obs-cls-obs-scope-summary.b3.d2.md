# Classified — obs · obs-scope-summary.b3

## Answers
obs-scope-summary#17.1 · no · the CI/CD platform is a scope source label, and the chunk adds no CI job or upload (Not needed)
obs-scope-summary#17.2 · no · CI/CD is not-instrumentable, and the chunk touches no CI surface
obs-scope-summary#17.3 · no · fixtures are scrub-required recorded data and no telemetry source; the chunk names no fixture and its witnesses are unit-level
obs-scope-summary#18.1 · yes · the per-process panic hook's one line binds, since a forced vt100 panic in a witness fires it inside the G2 exemption (Needs, zero-unlogged-panics); the span mechanism is S1's
obs-scope-summary#18.2 · no · frontend N/A, and the chunk adds no prompt, TUI, pager or output channel
obs-scope-summary#18.3 · yes · send-refused and the refused-check rule bind the cli log target: a CLI verb's line goes to cli-<name>.ndjson only after its instance check passes, and a refused check writes no line
obs-scope-summary#18.4 · yes · the instance is ViolaName from an argument or VIOLA_NAME, never the working directory, the names-before-path-joins rule the chunk's ViolaName::try_new checks enforce; service.name and version are unchanged
obs-scope-summary#18.5 · yes · refusal and fault lines stay fixed codes-only strings with no path, pid or upstream text, and the instance comes from VIOLA_NAME or an argument (Needs on codes-only refusals; the Direction 4 codes are owned by Exit-cause code catalogue)
obs-scope-summary#19.1 · yes · vt100 readiness-gate outcomes must be logged (input-not-ready, Needs, gate.rs re-read), and no span or event may sit per chunk in the PTY pump read loop or the feed
obs-scope-summary#19.2 · yes · the terminal carries the child's screen, which is the passthrough the chunk re-verifies when the screen model panics (screen.rs re-read); the N/A frontend hook binds nothing
obs-scope-summary#19.3 · yes · run's one parse-rejected{parser:"vt100-feed"} WARN line goes to diagnostics/run-<name>.ndjson, and run writes nothing to the terminal while the child runs
obs-scope-summary#19.4 · no · each new log line keeps its own process and instance fields, a record every change of its kind owes; service identity is unchanged (Not needed)
obs-scope-summary#19.5 · yes · the panic hook is replaced before the child spawns, so a forced vt100 panic in the feed thread prints nothing to the child's terminal and fires the one panic line the witness counts
obs-scope-summary#20.1 · yes · the oversize or malformed hook stdin is a fail-open hook-decision line (Needs), and a caught panic is one panic line; the span mechanism is S1's
obs-scope-summary#20.2 · no · frontend N/A, and the hook has no frontend surface
obs-scope-summary#20.3 · yes · the fail-open hook-decision line goes to hook-<name>.ndjson, and on oversize or malformed stdin stderr stays empty with no body (Needs); hook statusline prints nothing on oversize stdin
obs-scope-summary#20.4 · yes · the instance check runs before any stdin read and before any line, so the chunk's take must sit after check_instance, and the --capture arms stay ahead of it
obs-scope-summary#20.5 · yes · an unresolved instance writes no log and no stderr: a name the ViolaName check refuses, or a VIOLA_DIR naming no instance, writes nothing to hook-<name>.ndjson (Needs, names before path joins); the append and latency notes bind no further
obs-scope-summary#21.1 · no · standing S1 · every new log line goes through obs_event! under #[instrument(skip_all, fields(..))]
obs-scope-summary#21.2 · no · frontend N/A, and the channel has no frontend surface
obs-scope-summary#21.3 · yes · the over-size and invalid-frame WARN lines (-32602, -32600) on channel frames of both sides go to that process's own log, run-, hook-, mcp or cli-<name> (Needs)
obs-scope-summary#21.4 · no · each side logs its own process, a per-line record every change owes; channel-request and channel-response record no params, so the peer version in params.sender is never logged
obs-scope-summary#21.5 · yes · -32603 is exactly "internal error" with data null, and from is logged as unauthenticated, both Needs on channel frames of both sides; the JSON-RPC id gap is Not needed
obs-scope-summary#22.1 · no · standing S1 · every new log line goes through obs_event! under #[instrument(skip_all, fields(..))]
obs-scope-summary#22.2 · no · frontend N/A, and the MCP surface is stdio with no frontend
