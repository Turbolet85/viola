# obs · standing conventions

- S1 · each new log line goes through `obs_event!` under `#[instrument(skip_all, fields(..))]`, carrying only fields named in the Section 6 catalog and passing `schemas/diag-line.v1.json` (default-deny, G4) · §6 Log Coverage / .andromeda/obs-plan-detail/log-coverage.md
  (.andromeda/obs-plan-detail/ci-integration.md:23) ci
  (.andromeda/obs-plan-detail/obs-scope-summary.md:157) scope-surfaces
  (.andromeda/obs-plan-detail/obs-scope-summary.md:259) log-fields
  (.andromeda/obs-plan-detail/telemetry-strategy.md:42) triggers
- S2 · a new `event` value or `detail` code needs an Obs Decisions Log entry, and a `detail` code holds no path or pid · §11 Logs, §12 / .andromeda/obs-plan-detail/log-coverage.md
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:19) pii
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:22) pii
- S3 · a new span is named `<area>.<operation>` in snake_case with a static name and an explicit field allow-list, never bare `#[instrument]`, `err` or `ret` (G1) · §4 / .andromeda/obs-plan-detail/span-trace-coverage.md
  (.andromeda/obs-plan-detail/ci-integration.md:32) ci
  (.andromeda/obs-plan-detail/span-trace-coverage.md:195) paths
- S4 · every new integration or E2E test home lives under `target/e2e-home/` so G2, G4, the secret scan and the `diag-<os>` upload cover it · §9 / .andromeda/obs-plan-detail/ci-integration.md
  (.andromeda/obs-plan-detail/ci-integration.md:8) ci
  (.andromeda/obs-plan-detail/ci-integration.md:24) ci
  (.andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:31) slo-budgets
- S5 · new test output counts toward the zero budgets: 0 unlogged panics (G2), 0 schema-conformance failures (G4), 0 secret-scan hits · §10 / .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md
  (.andromeda/obs-plan-detail/ci-integration.md:25) ci
  (.andromeda/obs-plan-detail/metric-coverage.md:30) metrics
  (.andromeda/obs-plan-detail/obs-scope-summary.md:465) triggers
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:8) pii
  (.andromeda/obs-plan-detail/telemetry-strategy.md:20) strategy
  (.andromeda/obs-plan-detail/telemetry-strategy.md:40) triggers
- S6 · a new CI step (for example a clean-build comparison) is its own `run:` step with `shell: bash` on all three OSes, uses Actions pinned by SHA, and keeps the G2 / G4 / secret-scan `if: always()` ordering with uploads gated on the scan · §9 / .andromeda/obs-plan-detail/ci-integration.md
  (.andromeda/obs-plan-detail/ci-integration.md:12) ci
  (.andromeda/obs-plan-detail/ci-integration.md:13) ci
  (.andromeda/obs-plan-detail/ci-integration.md:59) ci
- S7 · no `print!` / `eprintln!` / `dbg!` in product code paths, and no raw tracing level macro or `event!` outside `viola_core::obs` · §11 Logs / .andromeda/obs-plan-detail/obs-anti-patterns.md
  (.andromeda/obs-plan-detail/obs-scope-summary.md:78) scope-entities
