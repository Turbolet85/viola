# Classified — obs · obs-scope-summary.b1

## Answers
obs-scope-summary#1.1 · no · a count of telemetry surfaces; the chunk adds lines to the existing cli, hook and mcp surfaces and adds no surface
obs-scope-summary#1.2 · no · a count of instrumentable crates; the chunk adds no crate, and the crates it touches are classified by their own entities
obs-scope-summary#1.3 · no · a count of must-trace paths that names no path obligation; the chunk adds no path
obs-scope-summary#1.4 · yes · names trigger categories the chunk meets: corr propagation on its channel calls, perf budgets on the hook's added read, multi-OS compat for the macOS gap, Founder Direction 4
obs-scope-summary#2.1 · yes · the chunk's chains go to detail-<process>.ndjson under a diagnostics root still open between <home>/diagnostics/ and instances/<name>/diagnostics/
obs-scope-summary#2.2 · yes · the chunk's new mcp rejected-call line goes to mcp.ndjson, the sink mcp always writes, with stderr only when that file cannot be opened
obs-scope-summary#2.3 · no · the chunk adds no event kind (its exit-21 and fail-open lines reuse process-exit and hook-decision), and release-from-driver is the wrapper's refusal line, which the chunk does not reach
obs-scope-summary#2.4 · yes · the chunk writes a dialog hook's hook-invoked only after the hook.dialog response, so its corr timing against dialog_id assignment is the chunk's to settle
obs-scope-summary#3.1 · no · a source citation for the root bin (clap, vt100, thiserror/anyhow) that states no rule
obs-scope-summary#3.2 · yes · the root bin is instrumentable, so the chunk's new verb checks and spans in src/cmd/ must be instrumented
obs-scope-summary#3.3 · no · standing S8 · typed errors carry fixed-message Display; no serde-sourced chain reaches stderr, --json, MCP isError or channel error.data
obs-scope-summary#4.1 · no · a source citation for viola-core that states no rule
obs-scope-summary#4.2 · no · viola-core is not in the chunk's reached files, and the chunk adds no instrumentation there
obs-scope-summary#4.3 · no · viola-core is not reached; the new detail codes live in the reached crates, not in viola-core
obs-scope-summary#5.1 · no · a source citation for viola-pty that states no rule
obs-scope-summary#5.2 · no · viola-pty is not reached; the verification path makes no PTY call
obs-scope-summary#5.3 · no · the PTY seam and its byte stream are not reached; the verification path reads no PTY bytes
obs-scope-summary#6.1 · no · a source citation (the IPC method set); the check sits at connect before any method, and link and unlink are held
obs-scope-summary#6.2 · yes · viola-channel is instrumentable, and the chunk edits its sync client, endpoint and Tokio client
obs-scope-summary#6.3 · yes · viola-channel instrumentation stays sync with Tokio only behind its feature, which binds the chunk's client.rs, endpoint.rs and tokio_client.rs edits
obs-scope-summary#7.1 · no · a source citation for viola-state (ndjson, atomic-write-file, sysinfo) that states no rule
obs-scope-summary#7.2 · yes · viola-state is instrumentable, and the chunk edits liveness.rs (live_endpoint)
obs-scope-summary#7.3 · yes · viola-state instrumentation stays sync with Tokio banned, which binds the chunk's liveness.rs edit
obs-scope-summary#8.1 · no · a source citation for viola-agent-claude that states no rule; the chunk does not reach that crate
obs-scope-summary#8.2 · no · viola-agent-claude is not reached; the chunk only calls its hook payload check, which keeps its own instrumentation
