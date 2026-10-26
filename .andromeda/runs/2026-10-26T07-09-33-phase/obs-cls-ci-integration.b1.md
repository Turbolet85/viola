# Classified — obs · ci-integration.b1

## Answers
ci-integration#1.1 · no · the diag upload's path, gate and retention are unchanged and the new lines fall under the existing glob (Not needed: artifact uploads)
ci-integration#1.2 · no · the failure-only harness capture and its harness-<os> upload are unchanged (Not needed: artifact uploads)
ci-integration#1.3 · no · the junit upload takes the chunk's new tests unchanged and adds no row (Not needed: artifact uploads)
ci-integration#1.4 · yes · the five per-hook hyperfine exports are the samples gate --require perf reads for the hook perf budget that the added peer-credential and start-time read must stay inside
ci-integration#1.5 · no · no OTel exporter in v1 and the chunk touches no exporter or reporter (Not needed: OTel SDK, OTLP exporter)
ci-integration#2.1 · yes · the scope names Vector 9, so the cargo check of the sync crates without Tokio binds the sync client and the endpoint pid read; the clippy, G1 and print bans are standing S1, S2, S7 and not carried
ci-integration#2.2 · no · the in-memory schema-parse oracle is owed equally by every new log-line test (its oracles); the new codes bind through ci-integration#3.3
ci-integration#2.3 · no · standing S9 · every new integration-test home lives under target/e2e-home/ so G2, G4 and the secret scan cover it
ci-integration#2.4 · no · the E2E record and its G2 and G4 reads are owed equally by every new E2E test (its record); the new codes bind through ci-integration#3.3
ci-integration#2.5 · no · the mutation run is the epoch-boundary and on-demand audit with no push job, and the Windows mutants form is Not needed; the scope names no mutation gate
ci-integration#3.1 · no · standing S2 · every new span is #[instrument(skip_all, name = ..., fields(...))] with an explicit allow-list, never err or ret
ci-integration#3.2 · no · the zero-unlogged-panics machinery is Not needed, and zero panic lines in a record is owed equally by every new test and change (its record)
ci-integration#3.3 · yes · the new exit-21 and fail-open detail codes must be admitted by diag-line.v1.json, and the detail-<process> chain lines by diag-detail.v1.json, which G4 checks on the G2 scope and on every detail file
ci-integration#4.1 · no · standing S9 · every new test home lives under target/e2e-home/ and is kept until the gate steps have read it; the step order it also states is Not needed (no CI job added)
