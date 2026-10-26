# Classified — test-plan · ci-integration.b1

## Answers
ci-integration#1.1 · no · the lint commands and the sync-crates list are unchanged, and clippy -D warnings stays green on tokio 1.53.3 with no deprecated AsyncFd line
ci-integration#1.2 · no · the supply-chain steps and their two endings are re-read unchanged, and cargo deny exits 0 on the bumped lock with no ignore added
ci-integration#1.3 · no · the unit-stage command and its nextest process-per-test parallelism are unchanged, and no test surface moved
ci-integration#1.4 · no · the integration-stage command and its parallelism are unchanged, and no test surface moved
ci-integration#1.5 · no · the E2E command and its per-OS matrix are unchanged, and no test or product surface moved
ci-integration#1.6 · no · the coverage run, its conpty_sideload binding and the ConPTY vendor verification are not touched by a lock-only bump, the root Cargo.toml's only change being the tokio pin
ci-integration#1.7 · no · the mutation stage keeps no push or pull-request job, and the windows-mutants tool pins and package items are unchanged
ci-integration#1.8 · no · the perf command and its hyperfine 1.20.0 pin are unchanged, and no hook or perf surface changed
ci-integration#1.9 · no · the 1.96 MSRV floor is unchanged, and tokio 1.53.3 declares rust-version 1.71 below that floor
ci-integration#1.10 · no · the fuzz-replay command and its toolchain steps are unchanged, and no fuzz manifest or lockfile is among the reached files
ci-integration#1.11 · no · the release-check and its --locked build are not touched by a lock-only bump, and the committed lock matches the moved pin
ci-integration#1.12 · no · the gate command and the §10 thresholds it reads are unchanged, and no coverage, perf or mutation threshold moved
