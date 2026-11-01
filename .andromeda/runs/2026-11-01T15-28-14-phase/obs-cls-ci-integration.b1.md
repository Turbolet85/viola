# Classified — obs · ci-integration.b1

## Answers
ci-integration#1.1 · no · standing S9 · a new integration test's viola home lives under target/e2e-home/, so the diag-<os> upload covers it
ci-integration#1.2 · no · the harness-<os> capture and upload is a CI artifact; the scope adds no upload and no chunk line names it
ci-integration#1.3 · no · the nextest JUnit upload is a CI artifact; the scope adds no upload and no chunk line names it
ci-integration#1.4 · no · the per-OS perf job and hyperfine samples; no perf, budget or hook-timing gate is named by the scope
ci-integration#1.5 · no · OTel exporter is N/A; the chunk adds no exporter and the file-sink pipeline stands
ci-integration#2.1 · yes · the cargo check of sync crates without Tokio binds: the Tokio client and the viola-channel instrumentation stay sync with Tokio only behind its feature (Needs), and the new client refusals sit in the sync client
ci-integration#2.2 · yes · in-memory unit lines are parsed against schemas/diag-line.v1.json, so the new detail value and fields the chunk emits must be admitted there (Needs)
ci-integration#2.3 · no · standing S9 · a new integration test's viola home lives under target/e2e-home/
ci-integration#2.4 · yes · agent-run logs and status over the end-to-end suite bind: the new refusal lines, the status last_error enum and the endpoint_gone reads are named by the Needs
ci-integration#2.5 · no · cargo-mutants runs at the epoch boundary and on demand, not on this chunk; no chunk line names mutation testing
ci-integration#3.1 · no · standing S5 · a new span has a static <area>.<operation> name, skip_all first, under gate G1
ci-integration#3.2 · no · standing S9 · G2 covers a new integration test's home under target/e2e-home/; the zero-panic gate and its seam exemption are unchanged
ci-integration#3.3 · yes · G4 checks the new home-level lines against schemas/diag-line.v1.json; a new detail value must be admitted there or the gate fails (Needs)
ci-integration#4.1 · no · standing S9 · a new test's home under target/e2e-home/ is covered by G2, G4 and the secret scan, and the chunk adds no CI step
ci-integration#5.1 · no · standing S9 · a new integration test's viola home lives under target/e2e-home/, so the secret scan covers it
