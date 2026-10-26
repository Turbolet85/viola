# Classified — obs-plan · telemetry-strategy.b1

## Answers
telemetry-strategy#1.1 · no · the tracing 0.1.44 pin and its lock entry hold on the changed files (the lock delta is the tokio entry alone and Cargo.toml changed only its tokio pin at line 242), so the span declarations and the deferred bridge are not reached
telemetry-strategy#1.2 · no · the tracing-subscriber 0.3.23 pin held on the changed Cargo.toml (no obs pin moved) and no sink, event or field changed, so the JSON-per-line log row still holds
telemetry-strategy#1.3 · no · the thiserror 2.0.20 pin held and the profile panic = unwind lines did not move (Cargo.toml changed at line 242 only), and the §7 task-boundary containment reads unmoved on tokio 1.53.3, so the panic-hook and catch_unwind claims still hold
telemetry-strategy#1.4 · no · the lock delta is the tokio entry alone, so no exporter or in-process instrument crate entered, and hyperfine 1.20.0 is a CI tool in a workflow the session did not change
telemetry-strategy#1.5 · no · the events.ndjson audit stream, its path and its no-rotation reading are untouched: no event, log path or retention gate changed
telemetry-strategy#2.1 · no · the panic hook runs at each panic itself and no source changed, and the §7 task-boundary containment of mcp panics reads unmoved on tokio 1.53.3, so every panic still yields one record
telemetry-strategy#3.1 · no · the channel surface row, the §6 channel field allow-list and the §8 table are untouched (no field, event or scrubbing layer changed), and viola-channel's Tokio-client containment was re-measured unmoved
telemetry-strategy#3.2 · no · the paste-text coverage (§4 Scenario 2 send-refused side, §8) is untouched: no span, code path or scrubbing layer changed
telemetry-strategy#3.3 · no · the hook-stdin coverage (§3 hook sink, §4 Scenario 4, §6 hook-decision codes) is untouched: no code path, sink or detail code changed
telemetry-strategy#3.4 · no · the mcp surface row and the §6 allow-list are untouched (no span, field or event changed), and the obs claims name no tokio version, so the tokio patch under viola-mcp moves none of the rows it points to
telemetry-strategy#3.5 · no · the CLI/env coverage (§4 Scenario 1 env_stripped, §7 fixed messages, §8) is untouched: no env read, message or scrubbing layer changed
telemetry-strategy#3.6 · no · the filesystem coverage (§3 log file location, §6 state-recovered and skipped with the detail catalog, §9 secret-scan-before-upload) is untouched: no log path, catalog code or CI step changed
telemetry-strategy#3.7 · no · the child-spawn coverage (§4 Scenario 1 subject values, §6 parse-rejected) is untouched: no spawn code changed and the portable-pty =0.8.1 pin held
telemetry-strategy#3.8 · no · the supply-chain claim still holds on the changed lock: cargo deny exits 0, the four sole-root deny-sync bans exit 0, channel-tokio-free reads ok, and Cargo.toml changed only its tokio pin, so the feature set is unmoved
telemetry-strategy#3.9 · no · the trace-propagation mechanism (corr and conn, §3 propagation, §4 Scenarios 2 and 4) is untouched: no span, channel code path or wire field changed
telemetry-strategy#3.10 · no · the budgets (§5 and the §10 budget table) are untouched: no hook or gate changed, and the hyperfine CI pin sits in a workflow the session did not change
telemetry-strategy#3.11 · no · the chaos-instrumentation rows (§6 state-recovered and process-exit exit_source and during, §10) are untouched: no event, field or gate changed
telemetry-strategy#3.12 · no · the exporter-compat rows (§3 log file location and concurrent-append check, §4 Scenario 1 endpoint_kind and pty_backend, §9 per-OS artifacts) are untouched: no CI artifact, workflow or home-permission code changed
telemetry-strategy#3.13 · no · the zero-unlogged-panics budget rests on §7 and the §10 zero counts, which the report reads unmoved: no hook or gate changed and the mcp task containment is unmoved on tokio 1.53.3
telemetry-strategy#3.14 · no · the audit-log retention row (§3 no rotation, §10) is untouched: no log file, rotation or gate changed
telemetry-strategy#3.15 · no · the founder-direction row (§3 schema verbatim, §6) is untouched: no schema, including diag-line and diag-detail, and no field changed
telemetry-strategy#3.16 · no · the Direction 4 row (§6 detail code catalog, §12 D-01) is untouched: no detail code changed and the session made no obs decision
telemetry-strategy#3.17 · no · the installed-CLI row (§5 counters, §6 parse-rejected and skipped) is untouched: no counter, parse site or event changed
telemetry-strategy#3.18 · no · the no-screen-scraping row (§4 Scenario 2 vt100 gate outcome, §11) is untouched: no span, scenario or dependency changed
