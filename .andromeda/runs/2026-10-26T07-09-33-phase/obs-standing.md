# obs · standing conventions

- S1 · every new log line goes through `obs_event!` with explicit typed fields; no raw `tracing` level macro or `event!` outside `viola_core::obs` · §11 Obs Anti-Patterns › Logs (.andromeda/obs-plan-detail/obs-anti-patterns.md)
- S2 · every new span is `#[instrument(skip_all, name = ..., fields(...))]` with an explicit allow-list, never `err` / `ret` · §4 Span / Trace Coverage
  (.andromeda/obs-plan-detail/ci-integration.md:32) ci
  (.andromeda/obs-plan-detail/obs-scope-summary.md:156) scope-surfaces
- S3 · every new field or `detail` value is named in the §6 catalog and admitted by `schemas/diag-line.v1.json` (default-deny, G4) · .andromeda/obs-plan-detail/log-coverage.md
  (.andromeda/obs-plan-detail/ci-integration.md:40) ci
- S4 · `detail` codes are closed kebab-case and carry no path or pid · §6 Log Coverage (.andromeda/obs-plan-detail/log-coverage.md)
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:21) pii
- S5 · log levels follow the §6 mapping (exit 21 and fail-open `hook-decision` at WARN, refusals at INFO) · §6 Log Coverage
- S6 · home-level lines stay path-free and content-free; content and chains go only to `instances/<name>/diagnostics/detail-<process>.ndjson` · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
  (.andromeda/obs-plan-detail/log-coverage.md:17) log-fields
  (.andromeda/obs-plan-detail/obs-scope-summary.md:115) scope-entities
  (.andromeda/obs-plan-detail/obs-scope-summary.md:284) logs
  (.andromeda/obs-plan-detail/obs-scope-summary.md:345) paths
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:13) pii
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:14) pii
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:17) pii
  (.andromeda/obs-plan-detail/telemetry-strategy.md:20) strategy
- S7 · no telemetry to stdout on any role, and nothing on the hook's stderr · §11 Obs Anti-Patterns › Logs
  (.andromeda/obs-plan-detail/obs-scope-summary.md:78) scope-entities
  (.andromeda/obs-plan-detail/obs-scope-summary.md:172) scope-surfaces
  (.andromeda/obs-plan-detail/obs-scope-summary.md:211) scope-surfaces
  (.andromeda/obs-plan-detail/obs-scope-summary.md:243) logs
- S8 · typed errors carry fixed-message `Display`; no serde-sourced chain reaches stderr, `--json`, MCP `isError` or channel `error.data` · .andromeda/obs-plan-detail/error-capture-reporting.md
  (.andromeda/obs-plan-detail/obs-scope-summary.md:28) scope-entities
  (.andromeda/obs-plan-detail/obs-scope-summary.md:384) triggers
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:15) pii
  (.andromeda/obs-plan-detail/telemetry-strategy.md:32) triggers
- S9 · every new integration-test home lives under `target/e2e-home/` so G2, G4 and the secret scan cover it · .andromeda/obs-plan-detail/ci-integration.md
  (.andromeda/obs-plan-detail/ci-integration.md:24) ci
  (.andromeda/obs-plan-detail/ci-integration.md:59) ci
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:8) pii
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:28) pii
