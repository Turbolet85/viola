# obs · standing conventions

- S1 · Every integration-test viola home lives under `target/e2e-home/`, so G2, G4, the secret scan and the upload cover it · §9 CI Integration
  (.andromeda/obs-plan-detail/ci-integration.md:8) ci
  (.andromeda/obs-plan-detail/ci-integration.md:24) ci
  (.andromeda/obs-plan-detail/ci-integration.md:59) ci
  (.andromeda/obs-plan-detail/ci-integration.md:65) ci
- S2 · A new log line goes through `obs_event!` with explicit typed fields, never a raw tracing macro, `corr` recorded unstringified · §11 Logs
  (.andromeda/obs-plan-detail/ci-integration.md:20) ci
- S3 · A field is logged only if named in the Section 6 catalogue, default-deny against `schemas/diag-line.v1.json`, gated by G4 · §8 PII Scrubbing, log-coverage.md
  (.andromeda/obs-plan-detail/obs-scope-summary.md:479) triggers
- S4 · A `detail` value is a closed kebab-case code, never a path or pid, low-class metadata going as typed fields · §6 Log Coverage, §11 PII Scrubbing
- S5 · No test retries or retry-once policy, and no log-based wait relying on sleeps · §10 SLO Invariants, §11 SLO
  (.andromeda/obs-plan-detail/obs-scope-summary.md:507) triggers
- S6 · Zero tolerance on `event:"panic"` lines, secret-scan hits and schema-conformance failures across test homes · §10 Error budget
  (.andromeda/obs-plan-detail/ci-integration.md:25) ci
  (.andromeda/obs-plan-detail/ci-integration.md:36) ci
  (.andromeda/obs-plan-detail/metric-coverage.md:30) metrics
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:8) pii
  (.andromeda/obs-plan-detail/telemetry-strategy.md:40) triggers
- S7 · Every input a test feeds the product is synthetic, and the content canary never appears in a home-level file · §8 PII Scrubbing, pii-scrubbing-compliance.md
  (.andromeda/obs-plan-detail/obs-scope-summary.md:151) scope-entities
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:23) pii
- S8 · No `print!`, `eprintln!` or `dbg!` on `hook`, `run` or `mcp` paths, output owned by the `--json`, human stderr and decision-body modules · §11 Logs
- S9 · A span added on a new path is `#[instrument(skip_all, ...)]` with an allow-list, a static name, no `err` or `ret` · §4 Span / Trace Coverage, §11 Spans / Traces
  (.andromeda/obs-plan-detail/ci-integration.md:32) ci
  (.andromeda/obs-plan-detail/telemetry-strategy.md:8) strategy
- S10 · Every verdict over telemetry is an assertion that sets an exit code, never human review · §2 Telemetry Strategy, §11 CI
