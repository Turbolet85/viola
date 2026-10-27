# Classified — obs · ci-integration.b1

## Answers
ci-integration#1.1 · no · standing S4 · every new integration or E2E test home lives under target/e2e-home/ so G2, G4, the secret scan and the diag-<os> upload cover it, a convention the task lists
ci-integration#1.2 · yes · the Needs name status's state and last_error and the merged logs as where a failed boot is read, and this step captures both on test failure before the scan
ci-integration#1.3 · no · standing S6 · a new CI step keeps the G2 / G4 / secret-scan if: always() ordering with uploads gated on the scan, a convention the task lists
ci-integration#1.4 · no · standing S6 · a new CI step keeps the G2 / G4 / secret-scan if: always() ordering with uploads gated on the scan, a convention the task lists
ci-integration#1.5 · no · no OTel exporter in v1, and the task's Not needed list names an OTel SDK or OTLP exporter as out of scope
ci-integration#2.1 · yes · the Needs name viola-state instrumentation staying sync and Tokio-free, so the sync-crate ban over its graph and cargo check of sync crates bind
ci-integration#2.2 · yes · the scope's new WARN/ERROR pin line, logged once with a closed code, is what an in-memory writer test would parse against diag-line.v1.json
ci-integration#2.3 · yes · a hook that fails exits 0, so the witness's two-instance homes must sit under target/e2e-home/ for G2 and G4 to see a hook panic
ci-integration#2.4 · yes · the witness is an E2E run whose failed boot is read through agent-run status and logs, and G2 and G4 gate its output
ci-integration#2.5 · yes · the new WARN/ERROR obs_event! call in pin_bytes is obs code the epoch-boundary cargo-mutants run scores, so its logged-once test must catch a removal of that call
ci-integration#3.1 · yes · G1 checks the instrument attribute the new viola-state log line sits under, which the Needs name as instrumentation added there, skip_all first
ci-integration#3.2 · yes · G2 reads the witness's homes for an unlogged panic, and the Needs name the seam exemption any build change must keep
ci-integration#3.3 · yes · G4 checks the new WARN/ERROR line and its closed process-exit detail code against the diag schemas
ci-integration#4.1 · yes · the witness is red before the fix and an E2E test, so its homes must survive until G2 and G4 have read them and a failed test step must not skip G2
