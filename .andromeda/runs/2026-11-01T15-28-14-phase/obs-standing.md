# obs · standing conventions

- S1 · A new log line goes through `obs_event!` with explicit typed fields and a static message equal to the event name, never a raw tracing macro · §11 Logs and §6 of the body, log-coverage.md
- S2 · A logged field must be named in the Section 6 catalog, default-deny, and validated by the schemas under gate G4 · §8 Default-deny posture and §9 of the body, log-coverage.md
  (.andromeda/obs-plan-detail/ci-integration.md:23) ci
  (.andromeda/obs-plan-detail/ci-integration.md:40) ci
  (.andromeda/obs-plan-detail/telemetry-strategy.md:42) triggers
- S3 · A `detail` code is closed and kebab-case and never holds a path or pid · §6 and §11 PII Scrubbing of the body, log-coverage.md
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:19) pii
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:22) pii
- S4 · A new `event` value or a renamed field needs a Decisions Log entry · §11 Logs and §12 of the body
- S5 · A new span has a static `<area>.<operation>` name, `skip_all` first and an explicit field allow-list, with no `err` or `ret`, under gate G1 · §4, §9 and §11 Spans of the body, span-trace-coverage.md
  (.andromeda/obs-plan-detail/ci-integration.md:32) ci
  (.andromeda/obs-plan-detail/telemetry-strategy.md:8) strategy
- S6 · A new line takes its level from the levels mapping: WARN for exit 21, ERROR for exit 1 or 20, INFO for other refusals · §6 of the body
- S7 · No telemetry on stdout and no print or `dbg!` macros in `hook`, `run` or `mcp` paths; human stderr goes through the owning output modules · §3 and §11 Logs of the body
  (.andromeda/obs-plan-detail/ci-integration.md:20) ci
  (.andromeda/obs-plan-detail/obs-scope-summary.md:79) scope-entities
- S8 · A new typed error has a fixed-message `Display` holding no path or payload, and anyhow stays at the root bin's dispatch edge · §7 of the body, error-capture-reporting.md
  (.andromeda/obs-plan-detail/telemetry-strategy.md:32) triggers
- S9 · A new integration test's viola home lives under `target/e2e-home/`, so G2, G4, the secret scan and the `diag-<os>` upload cover it · §9 of the body, ci-integration.md
  (.andromeda/obs-plan-detail/ci-integration.md:8) ci
  (.andromeda/obs-plan-detail/ci-integration.md:24) ci
  (.andromeda/obs-plan-detail/ci-integration.md:25) ci
  (.andromeda/obs-plan-detail/ci-integration.md:36) ci
  (.andromeda/obs-plan-detail/ci-integration.md:65) ci
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:8) pii
- S10 · A new test takes no retry-once policy and no sleep-based log wait, and asserts by exit code or a `jq` / harness verdict · §10 and §11 SLO and CI of the body, slo-invariants-telemetry-budgets.md
  (.andromeda/obs-plan-detail/obs-scope-summary.md:509) triggers
- S11 · A new dependency stays Tokio-free in the sync crates, adds no C-building crate and passes `cargo deny` · §1 Telemetry triggers and §10 of the body, obs-scope-summary.md
  (.andromeda/obs-plan-detail/obs-scope-summary.md:417) triggers
  (.andromeda/obs-plan-detail/telemetry-strategy.md:35) triggers
- S12 · A new line uses the same file sink, fields and schema on every OS, with no OS-specific exporter · §1 Telemetry triggers of the body, obs-scope-summary.md
  (.andromeda/obs-plan-detail/obs-scope-summary.md:246) logs
