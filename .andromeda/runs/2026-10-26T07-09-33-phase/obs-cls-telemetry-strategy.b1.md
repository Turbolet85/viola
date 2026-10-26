# Classified — obs · telemetry-strategy.b1

## Answers
telemetry-strategy#1.1 · yes · the chunk's client spans for verification (`<area>.<operation>`) sit on the viola-channel client call and viola-mcp tool-call seams this entity declares
telemetry-strategy#1.2 · yes · the new verification boundary call, its mismatch refusal and the hook's fail-open line are boundary calls and refusals this entity logs as the closed event enum
telemetry-strategy#1.3 · yes · the new typed verification error with a fixed-message Display and the exit-21 process-exit detail codes are the typed errors and distinct detail codes this entity sets
telemetry-strategy#1.4 · yes · the hook perf budget the added peer-credential and start-time reads must hold is gated by `gate --require perf`, which this entity runs on hyperfine
telemetry-strategy#1.5 · yes · the SessionEnd direct-append fallback writes events.ndjson, and its hook line must say which outcome was taken without copying the event payload into a process log
telemetry-strategy#2.1 · no · standing S6 · home-level lines stay path-free and content-free; content and chains go only to instances/<name>/diagnostics/detail-<process>.ndjson
telemetry-strategy#3.1 · yes · the client's channel-request and channel-response lines fall under the channel field allow-list this entity sets
telemetry-strategy#3.2 · yes · a send refused before any frame by a failed verification logs codes-only with no sent text, on the send-refused side this entity fixes
telemetry-strategy#3.3 · yes · the hook's fail-open hook-decision line at WARN, written to hook-<name>.ndjson only, falls under the hook sink and its detail codes
telemetry-strategy#3.4 · yes · the MCP instance-unreachable isError code and its codes-only mcp.ndjson line fall under the MCP surface rule
telemetry-strategy#3.5 · yes · the CLI exit-21 message and the new error's Display must be fixed messages with no path, pid or start time (§7)
telemetry-strategy#3.6 · yes · strict-modes-failed is a filesystem-refusal detail code the exit-21 catalog may carry if P4 takes the strict-modes half into the clients
telemetry-strategy#3.7 · no · the chunk spawns no child (Not needed: no client spawns one) and adds no subject value or parse-rejected parser
telemetry-strategy#3.8 · yes · the added sysinfo start-time read must keep viola-channel and viola-state instrumentation sync, with sysinfo and interprocess held OFF in the sink and cargo-deny clean
telemetry-strategy#3.9 · yes · the client's channel calls carry corr and conn across the process join, the cross-surface propagation this entity sets
telemetry-strategy#3.10 · yes · the hook perf budget (spine hooks' and pre-tool-use's max, session-end under 1.0 s) is the §10 budget row the verification's added reads must stay inside
telemetry-strategy#3.11 · yes · the new exit-21 process-exit lines must carry exit_source and during as this entity names them
telemetry-strategy#3.12 · yes · the macOS recorded gap (euid + dir only) and the Windows pipe server pid must share one file sink, line schema and field set on every OS
telemetry-strategy#3.13 · no · the zero-unlogged-panics SLO is panic machinery the scope marks unchanged, a re-check every change owes
telemetry-strategy#3.14 · no · no log file is added, and the events.ndjson appender and its never-rotated rule are unchanged
telemetry-strategy#3.15 · yes · the chunk adds fields and detail codes (server-verify-failed, the hook fail-open detail) that §3's schema and §6's catalog must record verbatim
telemetry-strategy#3.16 · yes · Founder Direction 4 (no path, pid or start time, closed detail codes) is named in the scope, and the new exit-21 and hook-decision codes must enter the §6 catalog
telemetry-strategy#3.17 · no · the chunk reads and parses no installed-CLI output and adds no counter or parse-rejected path
telemetry-strategy#3.18 · no · the chunk adds no screen read, and the vt100 gate outcome in Scenario 2 sits on the wrapper's readiness path the chunk does not reach
