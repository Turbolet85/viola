# Classified — obs · ci-integration.b1

## Answers
ci-integration#1.1 · yes · the new integration tests' viola homes must sit under target/e2e-home/ so this diag-<os> upload path covers them (its gating and retention are Not needed)
ci-integration#1.2 · no · the agent-run logs and status artifact on test failure; the chunk runs no agent-run path and adds nothing to it
ci-integration#1.3 · no · the nextest JUnit XML upload; the chunk's tests appear in it unchanged and its retries rule sits in the nextest profile, not this row
ci-integration#1.4 · no · the hyperfine hook timings from run --perf; the chunk changes no hook role or latency budget
ci-integration#1.5 · no · N/A row with no OTel exporter in v1; the chunk adds none and its own assertions do not read this row
ci-integration#2.1 · yes · the new code and tests must avoid raw tracing macros, event!, print and dbg macros and bare #[instrument], and viola-channel's sync check without Tokio must stay green
ci-integration#2.2 · no · the in-memory writer schema tests check the writer's own lines, a standing guard this change does not touch; its real emitted lines are witnessed on real output by G4 (3.3)
ci-integration#2.3 · yes · every new integration test's viola home must sit under target/e2e-home/ (the reached tests/*.rs files included) so G2, G4 and the secret scan read it; the chaos carve-out is not reached
ci-integration#2.4 · no · the agent-run E2E suite and its logs, which the chunk does not add to; its G2 and G4 reach the new tests through 2.3, 3.2 and 3.3
ci-integration#2.5 · no · mutation runs only at the epoch boundary, through audit or on demand, with no push or PR job; the chunk states no mutation measure
ci-integration#3.1 · yes · every new #[instrument] span (channel.<op>) must open with skip_all as its first argument, as gate G1 checks
ci-integration#3.2 · yes · gate G2 runs on the Windows leg, so a panic in the new listener tests or in run's start must leave its one event:"panic" line in a home-level role file
ci-integration#3.3 · yes · every line the chunk's code or tests emit into a home must conform to schemas/diag-line.v1.json with no field outside the catalog
ci-integration#4.1 · yes · the new tests' homes must stay under target/e2e-home/ and be kept until G2 and G4 read them (AGENT_RUN_KEEP_HOMES=1), and a failed test step still runs G2; the step order itself is Not needed
