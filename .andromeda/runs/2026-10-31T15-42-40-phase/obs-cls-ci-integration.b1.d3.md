# Classified — obs · ci-integration.b1

## Answers
ci-integration#1.1 · no · standing S1 · every integration-test viola home lives under target/e2e-home/, so G2, G4, the secret scan and the diag upload cover it
ci-integration#1.2 · no · the agent-run logs and status failure capture is a CI workflow artifact the chunk edits no workflow for; the WATCH tally reads the JUnit record (#1.3)
ci-integration#1.3 · yes · the chunk tallies the verify_stopped WATCH on its CI reads, and this JUnit record with the failing test ID joined to diag lines by time and instance is that read
ci-integration#1.4 · no · hyperfine perf exports and hook latency gates; the chunk leaves the perf budgets and hook paths as they are
ci-integration#1.5 · no · no OTel exporter in v1; the chunk adds no exporter or egress
ci-integration#2.1 · no · standing S2 · no raw tracing macro outside viola_core::obs, and no print or dbg on hook, run or mcp paths, the clippy gate every change owes
ci-integration#2.2 · yes · the chunk adds process-exit detail values to schemas/diag-line.v1.json, and this in-memory serde_json parse of emitted lines against that schema must accept them
ci-integration#2.3 · no · standing S1 · integration-test viola homes live under target/e2e-home/; the chaos carve-out is not touched by the chunk
ci-integration#2.4 · no · standing S6 · zero event:"panic" lines, secret-scan hits and schema-conformance failures across test homes, in the E2E gates
ci-integration#2.5 · no · the epoch-boundary cargo-mutants run on obs code owes equally to every obs change; neither the scope nor a list the chunk extends names it
ci-integration#3.1 · no · standing S9 · the G1 gate, a span written as #[instrument(skip_all, ...)] with an allow-list; the chunk adds no span the scope names
ci-integration#3.2 · no · standing S6 · the G2 zero-panic gate across test homes, its seam exemption and probe
ci-integration#3.3 · no · standing S6 · the G4 schema-conformance gate across test homes, a failing line being a build failure
ci-integration#4.1 · no · standing S1 · test homes kept until G2, G4 and the secret scan have read them, and the gate steps run if: always()
ci-integration#5.1 · no · standing S1 · the secret scan's coverage of test homes after each job, the open v1-47 reading the chunk does not decide
