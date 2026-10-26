# obs · standing conventions

- S1 · every new log line goes through `obs_event!` with explicit typed fields, never a raw tracing level macro or `event!`, `corr` copied unchanged (never `?corr` / `%corr`) · §11 Obs Anti-Patterns › Logs / .andromeda/obs-plan-detail/obs-anti-patterns.md
  (.andromeda/obs-plan-detail/ci-integration.md:20) ci
- S2 · every new span is `#[instrument(skip_all, name = "...", fields(...))]` with skip_all first, no `err` / `ret`, a static `<area>.<operation>` name (gate G1) · §4 Span / Trace Coverage / §11 › Spans / Traces
  (.andromeda/obs-plan-detail/ci-integration.md:32) ci
- S3 · every field a new line carries is named in the §6 field catalog and passes G4 against `schemas/diag-line.v1.json` (default-deny; a new field or `detail` code joins the catalog and the schema) · §6 Log Coverage / .andromeda/obs-plan-detail/log-coverage.md / §8 Default-deny posture
  (.andromeda/obs-plan-detail/ci-integration.md:23) ci
  (.andromeda/obs-plan-detail/ci-integration.md:40) ci
  (.andromeda/obs-plan-detail/log-coverage.md:8) log-fields
  (.andromeda/obs-plan-detail/telemetry-strategy.md:42) triggers
- S4 · every new integration test's viola home lives under `target/e2e-home/` so G2, G4, the secret scan and the `diag-<os>` upload cover it · §9 CI Integration › Pipeline integration / .andromeda/obs-plan-detail/ci-integration.md
  (.andromeda/obs-plan-detail/ci-integration.md:8) ci
  (.andromeda/obs-plan-detail/ci-integration.md:24) ci
  (.andromeda/obs-plan-detail/ci-integration.md:59) ci
- S5 · no `print!` / `eprintln!` / `dbg!` outside the output modules that own `--json` and human CLI stderr · §11 › Logs
- S6 · every typed error crossing a crate seam has a fixed `Display`, ends in one logged closed code at WARN/ERROR, and its full chain goes only to the instance detail file · §7 Error Capture & Reporting / .andromeda/obs-plan-detail/error-capture-reporting.md
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:15) pii
- S7 · zero unlogged panics and no retry-once policy in any new test (G2 counts home-level role files) · §10 SLO Invariants / .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md
  (.andromeda/obs-plan-detail/ci-integration.md:25) ci
  (.andromeda/obs-plan-detail/ci-integration.md:36) ci
  (.andromeda/obs-plan-detail/metric-coverage.md:30) metrics
  (.andromeda/obs-plan-detail/obs-scope-summary.md:464) triggers
  (.andromeda/obs-plan-detail/telemetry-strategy.md:40) triggers
- S8 · `detail` codes are closed kebab-case and never hold a path, pid or user content · §11 › PII Scrubbing / .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:19) pii
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:21) pii
