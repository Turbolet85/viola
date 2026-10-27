# Classified — obs · ci-integration.b1

## Answers
ci-integration#1.1 · no · the diagnostics upload's path and scan gate; the chunk adds no CI job or upload (Not needed)
ci-integration#1.2 · no · the failure-only agent-run harness artifact; the chunk adds no CI job or upload (Not needed)
ci-integration#1.3 · no · the nextest JUnit upload; the chunk adds no CI job or upload (Not needed)
ci-integration#1.4 · no · the per-hook hyperfine perf exports; the chunk re-measures no hook timing and adds no CI job
ci-integration#1.5 · no · no OTel exporter in v1, and the chunk adds none (Not needed)
ci-integration#2.1 · no · standing S1 · the raw tracing-macro ban and the G1 gate, which every new log line owes
ci-integration#2.2 · no · standing S5 · the in-memory writer's serde_json schema parse, the zero-schema-failure budget every new test meets
ci-integration#2.3 · yes · the chunk's forced vt100 panic witness runs in tests/chaos_feed_panic.rs's TestHome::outside_scan() home, the one carve-out, whose panic and parse-rejected lines the test asserts itself
ci-integration#2.4 · no · standing S5 · the E2E zero-panic and schema gates, the budgets every new test meets
ci-integration#2.5 · no · the obs-code mutation run is at the epoch boundary through the audit, not a gate of this chunk; obs code is scored like product code
ci-integration#3.1 · no · standing S1 · the bare-#[instrument] gate G1, the skip_all form every new log site uses
ci-integration#3.2 · yes · G2 counts a forced vt100 panic outside its seam-only exemption, so the witness must run in the chaos home outside target/e2e-home
ci-integration#3.3 · no · standing S2 · the G4 schema check of every new event, field, parser and detail value
ci-integration#4.1 · yes · names the one chaos home outside G2, G4 and the scan, where the forced vt100 panic witness runs; the step order is CI's own
