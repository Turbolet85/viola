# obs · standing conventions

- S1 · every new log line goes through `obs_event!` under `#[instrument(skip_all, fields(..))]`, no raw tracing macros · ## 11. Obs Anti-Patterns › ### Logs
  (.andromeda/obs-plan-detail/ci-integration.md:20) ci
  (.andromeda/obs-plan-detail/ci-integration.md:32) ci
- S2 · every new field or `detail` code is in the §6 catalog and validates against `schemas/diag-line.v1.json` (G4, default-deny) · ## 6. Log Coverage / log-coverage.md
  (.andromeda/obs-plan-detail/ci-integration.md:23) ci
  (.andromeda/obs-plan-detail/ci-integration.md:40) ci
  (.andromeda/obs-plan-detail/telemetry-strategy.md:42) triggers
  (.andromeda/obs-plan-detail/telemetry-strategy.md:43) triggers
- S3 · a `detail` code is closed kebab-case and never holds a path or pid · ## 6. Log Coverage / ## 11. Obs Anti-Patterns › ### PII Scrubbing
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:19) pii
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:22) pii
- S4 · every new integration-test home lives under `target/e2e-home/` so G2, G4, the secret scan and the `diag-<os>` upload cover it · ## 9. CI Integration / ci-integration.md
  (.andromeda/obs-plan-detail/ci-integration.md:8) ci
- S5 · zero unlogged panics over home-level role files (G2), no retry-once policy · ## 10. SLO Invariants & Telemetry Budgets / slo-invariants-telemetry-budgets.md
  (.andromeda/obs-plan-detail/ci-integration.md:25) ci
  (.andromeda/obs-plan-detail/ci-integration.md:36) ci
  (.andromeda/obs-plan-detail/metric-coverage.md:30) metrics
  (.andromeda/obs-plan-detail/obs-scope-summary.md:464) triggers
- S6 · no telemetry on stdout on any role, nothing on `hook`'s stderr, `with_writer` explicit · ## 3. Observability Harness Contract / ## 11. Obs Anti-Patterns › ### Logs
  (.andromeda/obs-plan-detail/obs-scope-summary.md:78) scope-entities
- S7 · the secret-scan floor over every `diagnostics/` file (no `CLAUDE*` value, no content canary in a home-level line, 0600) · ## 8. PII Scrubbing & Compliance / pii-scrubbing-compliance.md
  (.andromeda/obs-plan-detail/obs-scope-summary.md:68) scope-entities
  (.andromeda/obs-plan-detail/obs-scope-summary.md:309) paths
  (.andromeda/obs-plan-detail/obs-scope-summary.md:385) triggers
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:8) pii
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:11) pii
- S8 · spans named `<area>.<operation>`, static, no instance name or path · ## 4. Span / Trace Coverage / span-trace-coverage.md
  (.andromeda/obs-plan-detail/telemetry-strategy.md:8) strategy
