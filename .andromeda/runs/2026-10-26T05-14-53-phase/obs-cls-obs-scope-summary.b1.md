# Classified — obs · obs-scope-summary.b1

## Answers
obs-scope-summary#1.1 · no · a count of telemetry surfaces and roles; the chunk adds or removes no surface, role or tier (Not needed: tier justification)
obs-scope-summary#1.2 · no · a count of instrumentable crates; the chunk adds no crate and changes no crate's instrumentability (Not needed: tier justification)
obs-scope-summary#1.3 · no · a count of must-trace paths and edge flows; no path's spans or events change (Not needed: must-trace paths)
obs-scope-summary#1.4 · no · a count of the 18 telemetry triggers; the chunk changes no trigger and this count names no trigger the Needs carry (Not needed: tier justification)
obs-scope-summary#2.1 · no · a pending diagnostics-root reconciliation; the chunk touches no diagnostics path (Not needed: pending reconciliation items)
obs-scope-summary#2.2 · no · a landed mcp log-destination rule; the chunk changes no log path, and the tokio bump moves mcp's runtime, not where mcp logs
obs-scope-summary#2.3 · no · a pending event-enum extension; the Needs keep the closed event enum untouched, so this chunk makes no extension
obs-scope-summary#2.4 · no · a pending hook-invoked corr-timing reconciliation; no hook code changes (Not needed: pending reconciliation items)
obs-scope-summary#3.1 · no · the root bin's stack (clap, vt100, thiserror, anyhow) moves nowhere (Cargo.lock edits only the tokio entries), and the root bin is not a sync crate
obs-scope-summary#3.2 · no · the root bin's instrumentable verdict; the chunk adds no instrumentation and the Needs name none for the root bin
obs-scope-summary#3.3 · no · anyhow as the error-capture boundary; error capture is Not needed, the scope names no change to it
obs-scope-summary#4.1 · no · viola-core's provenance line; it names no dependency and the chunk edits no viola-core source
obs-scope-summary#4.2 · no · viola-core's not-instrumentable verdict; the chunk adds no instrumentation or I/O there
obs-scope-summary#4.3 · no · a pure type crate supplying values the Needs keep untouched; the chunk edits no viola-core source and the entity states no rule the chunk must act on
obs-scope-summary#5.1 · no · viola-pty's stack line (portable-pty) is not a telemetry dependency; its RUSTSEC-2017-0008 ignore stays unchanged in deny.toml, which this line does not state
obs-scope-summary#5.2 · no · viola-pty's instrumentable verdict; the Needs name no viola-pty instrumentation and the chunk reaches no PTY code
obs-scope-summary#5.3 · no · the PTY seam's reason (boundary-only OS APIs, byte stream never logged); the chunk reaches no PTY code or log line
obs-scope-summary#6.1 · yes · names interprocess 2.4.4, whose tokio backend's AsyncFd call P3 measures under the patched tokio, so the measured version is this line's value
obs-scope-summary#6.2 · yes · viola-channel's instrumentable verdict; the Needs keep viola-channel's instrumentation sync with no Tokio in its graph, which the bump must keep true
obs-scope-summary#6.3 · yes · Tokio is forbidden in viola-channel's graph without its tokio feature, so its instrumentation must be sync; the bump must keep the feature-off graph Tokio-free (channel-tokio-free.sh)
obs-scope-summary#7.1 · yes · names viola-state's telemetry dependencies (ndjson, atomic-write-file, sysinfo 0.39.6), which the Need keeps Tokio-free in the sync crates, C-free and cargo-deny-clean; deny-sync re-measures their graph
obs-scope-summary#7.2 · yes · viola-state's instrumentable verdict; the Needs keep viola-state's instrumentation sync with no Tokio in its graph, which the bump must keep true
obs-scope-summary#7.3 · yes · Tokio is banned in viola-state, so its instrumentation must be sync; the per-sync-crate tokio ban (deny-sync.toml) is re-run over its graph
obs-scope-summary#8.1 · yes · names serde_path_to_error (drift reports), an obs dependency the Need keeps Tokio-free in the sync crates, C-free and cargo-deny-clean; deny-sync re-measures its graph
obs-scope-summary#8.2 · no · the section 8 crate's instrumentable verdict; the Needs do not name it and no source there is reached
