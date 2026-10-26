# Classified — obs · ci-integration.b1

## Answers
ci-integration#1.1 · no · E2E diagnostics upload in the test jobs, outside the supply-chain job the chunk re-reads; an upload is no check it re-runs
ci-integration#1.2 · no · harness artifact upload on test failure, a test-job step outside the re-read supply-chain job
ci-integration#1.3 · no · JUnit XML upload from the test jobs, a CI record the chunk's workspace-test re-run does not read back
ci-integration#1.4 · no · perf-job hyperfine export; the perf budgets are Not needed and the upload sits outside the supply-chain job
ci-integration#1.5 · yes · no OTel exporter in v1: the lock bump must add no exporter crate, which the banned-obs-crates cargo-deny ban in the Needs witnesses
ci-integration#2.1 · yes · the clippy -D warnings bans, the sync-crate no-Tokio cargo check and cargo deny (veil toggle, banned obs crates) are the Needs' build failure conditions the bump must keep green
ci-integration#2.2 · yes · the workspace-test re-run asserts in-memory emitted lines against schemas/diag-line.v1.json, the witness for the no-wire-or-log-line-change Need
ci-integration#2.3 · no · integration-test homes under target/e2e-home are a test-layout convention every test change owes; the chunk adds no integration test and the chaos carve-out's panic assertion is Not needed
ci-integration#2.4 · yes · the G4 schema gate over E2E output witnesses the no-wire-or-log-line-change Need; the G2 zero-panic gate and the agent-run artifacts are Not needed or outside the chunk
ci-integration#2.5 · no · mutation runs are epoch-boundary or on-demand, never a push job; the chunk runs no mutant and names no mutant gate
ci-integration#3.1 · no · standing S1 · the #[instrument(skip_all, ..)] span form its bare-instrument gate G1 checks, the discipline a forced source edit keeps
ci-integration#3.2 · no · G2 is the zero-unlogged-panics gate the scope lists as Not needed; no hook, run or heartbeat code changes
ci-integration#3.3 · yes · the G4 schema check over real process output witnesses the no-wire-or-log-line-change Need; its G1, G2 and G3 commands are S1, Not needed and a Cargo.toml profile re-check the tokio row leaves alone
ci-integration#4.1 · no · test-job step order, home retention and the perf job are CI conventions the chunk does not change; its G2 and G4 steps are classed in 3.2 and 3.3
