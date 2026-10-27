# obs · standing conventions

- S1 · every new log line goes through `obs_event!` under `#[instrument(skip_all, fields(..))]`, with no raw tracing level macro or `event!` outside `viola_core::obs` and no `err` / `ret` option · §11 Obs Anti-Patterns (Logs, Spans / Traces); .andromeda/obs-plan-detail/obs-anti-patterns.md
  (.andromeda/obs-plan-detail/ci-integration.md:20) ci
  (.andromeda/obs-plan-detail/ci-integration.md:32) ci
  (.andromeda/obs-plan-detail/obs-scope-summary.md:157) scope-surfaces
  (.andromeda/obs-plan-detail/telemetry-strategy.md:8) strategy
- S2 · a new `event` value, field, `parser` value or `detail` code joins the §6 catalog (closed, kebab-case, never a path or pid) and `schemas/diag-line.v1.json`'s default-deny, checked by G4 · .andromeda/obs-plan-detail/log-coverage.md
  (.andromeda/obs-plan-detail/ci-integration.md:40) ci
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:19) pii
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:22) pii
  (.andromeda/obs-plan-detail/telemetry-strategy.md:42) triggers
- S3 · no content (paste text, payload or over-size bytes, row text, command strings) in any home-level line; content goes only to the instance's `detail-<process>.ndjson` inside a chain or drift report · .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
  (.andromeda/obs-plan-detail/obs-scope-summary.md:346) paths
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:29) pii
  (.andromeda/obs-plan-detail/telemetry-strategy.md:14) strategy
- S4 · every new integration-test viola home lives under `target/e2e-home/`, so G2, G4, the secret scan and the `diag-<os>` upload cover it · .andromeda/obs-plan-detail/ci-integration.md
  (.andromeda/obs-plan-detail/ci-integration.md:8) ci
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:8) pii
- S5 · a new test meets the zero-unlogged-panics, zero-secret-hit and zero-schema-failure budgets, with no retry-once policy · .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md
  (.andromeda/obs-plan-detail/ci-integration.md:23) ci
  (.andromeda/obs-plan-detail/ci-integration.md:25) ci
- S6 · no `print!` / `eprintln!` / `dbg!` in product code paths; stdout stays reserved on every role · §11 Obs Anti-Patterns (Logs)
  (.andromeda/obs-plan-detail/obs-anti-patterns.md:8) logs
  (.andromeda/obs-plan-detail/obs-scope-summary.md:78) scope-entities
