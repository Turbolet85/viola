# Classified — obs · obs-scope-summary.b3

## Answers
obs-scope-summary#17.1 · no · a source label on the CI/CD Platform entity that states no rule the chunk's code or tests would honour
obs-scope-summary#17.2 · no · the CI/CD platform is not product runtime, and the chunk's CI re-read and release builds emit no telemetry
obs-scope-summary#17.3 · no · fixtures are scrub-required data and no telemetry source, and the chunk records no fixture; its evidence files are not fixtures
obs-scope-summary#18.1 · yes · the tracing facade and JSON logger that the chunk's new viola-state spans and its WARN or ERROR pin-write line go through
obs-scope-summary#18.2 · no · frontend is N/A; the chunk adds no prompt, TUI or pager, and its refusal text is held by 18.5
obs-scope-summary#18.3 · yes · verify's pin logs: process-start and process-exit go to cli-<name>.ndjson only with VIOLA_NAME set, and a refused home-tree check writes no line
obs-scope-summary#18.4 · yes · service.version stays the compile-time env!("CARGO_PKG_VERSION") that any measurement-driven build change must keep
obs-scope-summary#18.5 · yes · fixed refusal and fault strings with no path, pid or chain, and a closed detail code for each new exit-1 cause on the absent-copy write
obs-scope-summary#19.1 · yes · spans on the run start sequence, where the pin step sits, and on viola-state writes, kept sync and Tokio-free
obs-scope-summary#19.2 · no · frontend is N/A; the chunk does not change what the child's terminal shows
obs-scope-summary#19.3 · yes · the pin write's new line goes to the run diagnostics sink and never the terminal, and run writes nothing to the terminal while the child runs
obs-scope-summary#19.4 · yes · the identity the chunk's run-side pin lines inherit, process="run" and the instance from VIOLA_NAME, the detail-<process> file included
obs-scope-summary#19.5 · yes · the pin step sits before the child spawn, so a panic in it must reach the replaced hook and never the child's terminal
obs-scope-summary#20.1 · yes · the hook-invoked and hook-decision lines the witness reads for receipt ran:false, and the catch_unwind panic line that panic = unwind keeps
obs-scope-summary#20.2 · no · frontend is N/A; the chunk adds no hook output
obs-scope-summary#20.3 · yes · the hook fail-open the chunk must keep: exit 0, empty stderr, and a write to hook-<name>.ndjson only
obs-scope-summary#20.4 · yes · a silent no-op on a refused or absent instance, and with no VIOLA_NAME, the hook fail-open the chunk must keep
obs-scope-summary#20.5 · no · the one-write append rule and the hook latency budget are over hook-<name>.ndjson and the hook's timed path, which the chunk neither writes nor times
obs-scope-summary#21.1 · no · channel corr propagation and client or server spans; the chunk adds or changes no channel method, and the corr join is not needed
obs-scope-summary#21.2 · no · frontend is N/A; the chunk has no frontend
obs-scope-summary#21.3 · no · the per-side channel log files; the chunk changes no channel side, and verify's no-instance rule is carried by 18.3
obs-scope-summary#21.4 · no · the peer version on channel frames (params.sender), which the chunk does not touch
obs-scope-summary#21.5 · no · JSON-RPC id uniqueness, -32603 and the self-reported from are channel rules, and no channel method changes
obs-scope-summary#22.1 · no · MCP tool-call spans are the viola mcp surface, which the chunk does not reach, and its Tokio client sits outside viola-state
obs-scope-summary#22.2 · no · frontend is N/A; the chunk has no frontend
