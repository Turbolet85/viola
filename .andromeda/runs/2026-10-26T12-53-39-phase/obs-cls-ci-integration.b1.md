# Classified — obs · ci-integration.b1

## Answers
ci-integration#1.1 · no · standing S4 · every new integration test's home lives under target/e2e-home so the diag-<os> upload covers it
ci-integration#1.2 · no · the agent-run harness's failure capture, which the chunk neither changes nor reads
ci-integration#1.3 · no · the nextest JUnit upload, the test job's own product, which the chunk does not change
ci-integration#1.4 · no · the hyperfine hook rows measure hook commands the chunk does not change (the task's Not needed)
ci-integration#1.5 · no · there is no OTel exporter in v1, and the chunk adds no reporter, exporter or listener
ci-integration#2.1 · no · standing S1 · the lint bans every change keeps: raw tracing (S1), G1 (S2), print/dbg (S5)
ci-integration#2.2 · no · standing S3 · in-memory writer tests check emitted lines against diag-line.v1.json, the oracle every new test keeps
ci-integration#2.3 · no · standing S4 · integration-test homes under target/e2e-home, so G2, G4, the scan and the diag-<os> upload cover them
ci-integration#2.4 · no · standing S7 · zero unlogged panics in any new test, the G2 gate (its G4 half is S3)
ci-integration#2.5 · no · the mutation gate runs only at the epoch boundary and on demand, so this chunk triggers no run
ci-integration#3.1 · no · standing S2 · the G1 bare-#[instrument] check: skip_all first and a static name on every new span
ci-integration#3.2 · no · standing S7 · the G2 zero-panic gate over home-level role files, whose seam exemption the chunk does not touch
ci-integration#3.3 · yes · the chunk's detail-cli.ndjson chain line must pass G4 against schemas/diag-detail.v1.json, a half no standing line names (diag-line is S3)
ci-integration#4.1 · no · standing S4 · test homes under target/e2e-home kept until G2, G4 and the secret scan have read them
