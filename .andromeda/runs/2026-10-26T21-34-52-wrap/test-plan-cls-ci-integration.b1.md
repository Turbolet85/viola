# Classified — test-plan · ci-integration.b1

## Answers
ci-integration#1.1 · no · the Lint commands and sync-crate list are unchanged and no CI script or Cargo file is reached
ci-integration#1.2 · no · deny.toml, deny-sync.toml and both lockfiles are not reached, so the supply-chain steps still hold
ci-integration#1.3 · no · the Unit stage command and its nextest parallelism are unchanged and the harness is not reached
ci-integration#1.4 · no · the Integration stage command, parallelism and cache are unchanged and the harness is not reached
ci-integration#1.5 · no · the E2E stage command and its per-OS matrix are unchanged and the harness is not reached
ci-integration#1.6 · no · tests/conpty_sideload.rs is reached, but its [[test]] entry (required-features fake-agent) lives in Cargo.toml, not reached, and ci#40403389629 is green 15/15 with the changed binary
ci-integration#1.7 · no · the workflow is not reached, the mutation matrix items and their file split are unchanged, and src/cmd/verify/stop.rs is not reached
ci-integration#1.8 · no · the perf steps and perf-<hook>.json names are unchanged, and schema-check still passes on the one-word schema change in ci#40403389629
ci-integration#1.9 · no · the MSRV pin and its commands are unchanged, rust-toolchain.toml is not reached, and ci#40403389629 is green 15/15
ci-integration#1.10 · no · fuzz/ is not reached, the replay command is unchanged, and the report says no parser changed
ci-integration#1.11 · no · the release build is --bin viola and Cargo.toml is not reached, so no binary or feature changed
ci-integration#1.12 · no · the gate reads the same inputs and the §10 thresholds are unchanged, only a perf reading moved
