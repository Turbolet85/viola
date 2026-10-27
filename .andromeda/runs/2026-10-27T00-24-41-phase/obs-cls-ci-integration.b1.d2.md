# Classified — obs · ci-integration.b1

## Answers
ci-integration#1.1 · no · standing S4 · each new integration or E2E test home lives under target/e2e-home/, which the diag-<os> upload covers
ci-integration#1.2 · yes · the witness reads its failed boot and missing session-start in agent-run status and the merged logs (Needs names both), which this row captures on test failure before the scan
ci-integration#1.3 · no · the nextest JUnit record is a convention each new test owes equally, and no chunk rule names JUnit
ci-integration#1.4 · no · the hyperfine export times the hook spawns, which the pin-write fix does not reach, and no perf budget is in scope
ci-integration#1.5 · no · N/A, no OTel exporter in v1, and the Not-needed list drops an OTel SDK or OTLP exporter
ci-integration#2.1 · yes · the cargo check of sync crates without Tokio binds (Needs: viola-state's new instrumentation stays sync and Tokio-free); its clippy bans and G1 are standing S7 and S3
ci-integration#2.2 · no · standing S1 · each new log line passes schemas/diag-line.v1.json, checked by the in-memory unit-test oracle
ci-integration#2.3 · no · standing S4 · each new integration or E2E test home lives under target/e2e-home/, which G2, G4, the secret scan and the diag upload read
ci-integration#2.4 · yes · the witness reads its failed boot and missing session-start through the agent-run status and logs verbs this row names; its G2 and G4 gates are standing S5
ci-integration#2.5 · no · the mutation run is an epoch-boundary audit with no push or PR job, and the oracles each new obs site owes are a convention every obs change owes equally
ci-integration#3.1 · no · standing S3 · G1: no bare #[instrument], so an instrumented function opens with skip_all and a static <area>.<operation> name
ci-integration#3.2 · yes · the Needs' exemption binds: a path remap, linker flag or profile key the measurement motivates must keep the seam panic's panic_location exactly src/cmd/hook/seam.rs:<digits>, which G2 compares as a whole string
ci-integration#3.3 · yes · the G3 profile check binds (every Cargo profile keeps panic = "unwind" under any key the measurement changes), and G4's detail scope binds the io chain the Needs send to detail-<process>.ndjson, which must pass diag-detail.v1.json
ci-integration#4.1 · no · standing S6 · a new CI step keeps the G2 / G4 / scan if: always() ordering, and the witness's homes already survive under ci.yml's AGENT_RUN_KEEP_HOMES=1
