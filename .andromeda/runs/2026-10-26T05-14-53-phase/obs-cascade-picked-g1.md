# obs · picked accounting · group 1 of 2

## Picked
obs-scope-summary#6.1 · carried · names interprocess 2.4.4, the dependency whose tokio backend's `AsyncFd` use P3 measures and which the lock change must leave unmoved
obs-scope-summary#6.2 · not in scope · viola-channel's instrumentability verdict; the chunk adds or changes no instrumentation of the crate
obs-scope-summary#6.3 · carried · viola-channel's Tokio-only-behind-its-feature rule is what `scripts/channel-tokio-free.sh`, re-read and re-run by the chunk, witnesses
obs-scope-summary#7.1 · not in scope · provenance of viola-state's entry; ndjson, atomic-write-file and sysinfo have no tokio edge and the chunk moves no dependency but tokio and its required companions
obs-scope-summary#7.2 · not in scope · viola-state's instrumentability verdict; the chunk adds or changes no instrumentation of the crate
obs-scope-summary#7.3 · carried · viola-state's Tokio ban is the `deny-sync.toml` sole-root ban the chunk re-runs over `scripts/sync-crates.txt`
obs-scope-summary#8.1 · not in scope · provenance of viola-agent-claude's entry naming serde_path_to_error drift reports; it states no Tokio rule, version or ban the chunk reads
obs-scope-summary#9.1 · not in scope · a provenance line naming no rmcp version, feature or rule; the rmcp range and feature set the chunk re-reads are not stated by it
obs-scope-summary#9.3 · carried · `viola-mcp` is the Tokio crate where a forced `AsyncFd` edit would land, and such an edit must send no telemetry to stdout
obs-scope-summary#11.1 · not in scope · provenance of the hook / MCP-client boundary entity; no hook contract or MCP tool changes
obs-scope-summary#15.1 · not in scope · provenance of the persistent-stores entity; no store, envelope or heartbeat is touched
obs-scope-summary#15.2 · not in scope · the stores' instrumentability through `viola-state` call sites; no call site changes
obs-scope-summary#19.1 · carried · sync instrumentation with no Tokio rests on the `cargo deny` ban the chunk re-runs
obs-scope-summary#23.1 · carried · the `cargo deny` Tokio-in-sync-crates and C-crate bans it names are the bans the chunk's `cargo deny check` and sole-root runs must keep passing
obs-scope-summary#24.1 · carried · tracing-subscriber 0.3.23 above the RUSTSEC-2025-0055 floor is read by the `cargo deny` advisories check the chunk re-runs, which must show no other advisory
obs-scope-summary#40.1 · carried · the Vector 9 required signal the chunk's needs name: Tokio-free sync crates, no C-building crate, `cargo deny` passing
telemetry-strategy#1.2 · not in scope · the structured-logs signal row; the chunk adds or changes no log line, role file or format
telemetry-strategy#3.8 · carried · maps the Vector 9 trigger the chunk serves to the `cargo deny` gates it re-runs
telemetry-strategy#3.15 · not in scope · maps Founder Directions 1/2/5 to the log schema and §6; the chunk adds or changes no schema field or log line
telemetry-strategy#3.16 · not in scope · maps Founder Direction 4 to the `detail` code catalog; no code is added or changed
span-trace-coverage#2.4 · carried · states interprocess 2.4.4 emits no tracing, the version the lock change must leave unmoved under viola-channel's spans
error-capture-reporting#4.1 · not in scope · the vt100 feed-thread panic capture in `run`; no `run` or feed code changes
pii-scrubbing-compliance#1.12 · not in scope · the low data class of protocol codes and refusal details; no code or refusal is added or changed
pii-scrubbing-compliance#2.1 · carried · its admissibility rule for the `supply-chain` artifact rules over the job the chunk re-reads and must see green
