# obs · standing conventions

- S1 · Every integration-test home lives under `target/e2e-home/`, so G2, G4, the secret scan and the `diag-<os>` upload cover it · §9 CI Integration in the body
  (.andromeda/obs-plan-detail/ci-integration.md:24) ci
- S2 · Every log line goes through `obs_event!` with explicit typed fields, never a raw tracing macro · §11 Logs in the body
  (.andromeda/obs-plan-detail/ci-integration.md:20) ci
- S3 · A logged field or value is named in the §6 catalog and conforms to its schema under default-deny, field names snake_case and values kebab-case · §6 Log Coverage and §8 PII Scrubbing in the body, .andromeda/obs-plan-detail/log-coverage.md
- S4 · Every span is `#[instrument(skip_all, fields(...))]` or `info_span!` with an allow-list, without `err` or `ret` · §4 Span / Trace Coverage and §11 Spans / Traces in the body
  (.andromeda/obs-plan-detail/ci-integration.md:32) ci
  (.andromeda/obs-plan-detail/span-trace-coverage.md:201) paths
  (.andromeda/obs-plan-detail/telemetry-strategy.md:8) strategy
- S5 · No telemetry on stdout on any role, no print or `dbg!` macros in `hook`, `run` or `mcp` code paths outside the output modules · §11 Logs in the body
  (.andromeda/obs-plan-detail/error-capture-reporting.md:32) errors
  (.andromeda/obs-plan-detail/obs-anti-patterns.md:8) logs
  (.andromeda/obs-plan-detail/obs-scope-summary.md:78) scope-entities
- S6 · A test run leaves zero `event:"panic"` lines in home-level role files and uses no retry-once policy · §10 SLO Invariants and §11 SLO in the body
  (.andromeda/obs-plan-detail/ci-integration.md:25) ci
  (.andromeda/obs-plan-detail/ci-integration.md:36) ci
  (.andromeda/obs-plan-detail/metric-coverage.md:30) metrics
  (.andromeda/obs-plan-detail/telemetry-strategy.md:40) triggers
- S7 · Test verdicts set an exit code and key on hook events and log offsets, with no sleep-based log wait · §10 Standard+ invariants and §11 Universal in the body
  (.andromeda/obs-plan-detail/obs-scope-summary.md:506) triggers
- S8 · Inputs written to scanned or uploaded files are synthetic, because detail-file uploads are admissible only on that basis · §8 PII Scrubbing in the body, .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
