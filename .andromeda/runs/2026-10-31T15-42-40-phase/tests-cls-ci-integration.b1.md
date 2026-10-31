# Classified — tests · ci-integration.b1

## Answers
ci-integration#1.1 · yes · cargo fmt and clippy -D warnings over the new tests/cross_exit_causes.rs and the edited crates, and cargo check and orphans-check over the edited viola-core lib.rs
ci-integration#1.2 · yes · the new rstest dev-dependency must pass cargo deny, and the viola-channel client.rs edit must keep its normal graph tokio-free
ci-integration#1.3 · yes · the scope names the root bin's exit-code mapping unit cases and the clock-driven stale-heartbeat unit case, which run in this nextest unit stage
ci-integration#1.4 · yes · the new root-tests file tests/cross_exit_causes.rs and the matrix rows in tests/run_cli.rs, tests/cli_verify.rs and tests/cli_revive.rs run in this integration stage
ci-integration#1.5 · yes · the reached crates/viola-e2e/tests/mcp_tools.rs and the Linux and Windows OS-specific cases run on this stage's three-OS matrix
ci-integration#1.6 · yes · the per-OS instrumented run carries the new tests, the coverage they add and the Windows script-child case on the Windows runner
ci-integration#1.7 · no · mutation runs only at the epoch boundary and windows-mutants.yml is no chunk gate; the chunk adds no CI job and edits no workflow
ci-integration#1.8 · no · the perf gate times hook calls and moves no hook or perf bound; the hook contract is held by the fail-open matrix
ci-integration#1.9 · yes · new code and the rstest dev-dependency must compile and pass the unit run on the 1.96 MSRV floor
ci-integration#1.10 · no · the fuzz replay covers parsers and the chunk changes no parser; the scope excludes the cargo-fuzz targets
ci-integration#1.11 · no · the release build compiles the product bin without test-only features; rstest is dev-only and the chunk adds no feature to the release artifact
ci-integration#1.12 · no · standing S11 · the per-OS coverage thresholds of §10 that this gate enforces
