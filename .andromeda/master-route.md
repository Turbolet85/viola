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
2026-09-24-log-redaction-and-never-log-floor · complete · catch-site anyhow chain to the owner-only instance detail file (cmd::Failure + obs::report_internal_error), CLAUDE* canary floor at HEAD sinks, Unix kill of the CI-missed read_diagnostics_level mutant; veil/skip-all/ChannelError/drift_report carried (no subject at HEAD) · → viola-0.1.0/chunks/2026-09-24-log-redaction-and-never-log-floor/
2026-09-24-observability-gates · complete · obs CI gates: panic-hook-first witness, print/dbg + level-macro clippy bans with a fail-closed raw event! grep (lint-probes), fmt/clippy + G1–G4 + canary secret scan before scan-gated uploads; exit-aware 10 s readiness fixing the 2834e4d mutants timeout · → viola-0.1.0/chunks/2026-09-24-observability-gates/
2026-09-24-quality-gates · complete · quality CI gates: viola-harness gate verdict per job, per-OS llvm-cov floors (85/95/80, separator-agnostic ignore regex), MSRV 1.96 job via rustup, two-leg mutation (ubuntu + windows) with a union verdict, seeded fuzz replay (fuzz/ workspace, viola_name target), zero-retry contract test; mutants.out upload removed, zizmor concurrency declined, file_mode one body · → viola-0.1.0/chunks/2026-09-24-quality-gates/
2026-09-24-workspace-tree-and-code-graph-planes · complete · workspace tree + code-graph planes: 3-OS release job via scripts/release-check.sh (viola only), orphans gate per lib/bin target via scripts/orphans-check.sh (5 targets, 3 OS), fuzz/Cargo.lock advisories+sources audit (CI + weekly), arch tree lists 45 test/obs/a11y artifacts (v1-23 verified), code-graph planes recorded (rust = workspace; ts = e2e-web only), --acyclic on demand, rmcp cargo-tree CARRY to viola-mcp, quality-gates CI witness (run 36019646063) · → viola-0.1.0/chunks/2026-09-24-workspace-tree-and-code-graph-planes/
