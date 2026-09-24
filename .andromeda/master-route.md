# Master Route — viola

<!--
Cross-version immutable index. APPEND-ONLY via promotion in /andromeda-phase — route never adds records.
One record per promoted chunk, grouped under its version:
  {marker} · {status: pending|complete} · {super-laconic description} · → {link to chunk folder}
marker = {date}-{slug} (e.g. 2026-06-04-otlp-http-ingest), minted at promotion.
-->

## viola-0.1.0
2026-09-24-three-os-ci-headless-harness-skeleton · complete · 3-OS CI + five-command headless harness skeleton, minimal fake agent, mutation gate · → viola-0.1.0/chunks/2026-09-24-three-os-ci-headless-harness-skeleton/
2026-09-24-fake-agent-and-test-data-fixtures · complete · consumer-first fake agent (script/control/receipt, plugin-dir hooks, 5 modes), root temp-home fixture chain, fixture hygiene walk, ViolaName proptests, mutation no-rust-delta verdict · → viola-0.1.0/chunks/2026-09-24-fake-agent-and-test-data-fixtures/
2026-09-24-supply-chain-and-workflow-gates · complete · deny.toml (4 families, C/telemetry/tokio/feature bans), deny + zizmor CI gates, weekly advisory run, tokio-free sync check, least-privilege workflows · → viola-0.1.0/chunks/2026-09-24-supply-chain-and-workflow-gates/
2026-09-24-diagnostics-plane · complete · closed ObsEvent vocabulary + obs_event!, per-role JSON sinks with service identity, diagnostics_level, owner-only detail files, diag-line schema, torn-line-aware logs merge · → viola-0.1.0/chunks/2026-09-24-diagnostics-plane/
