# Classified — test-plan · ci-integration.b1

## Answers
ci-integration#1.1 · no · the lint commands and per-OS matrix are unchanged, `--all-targets` already covers the new cfg(windows) cases, the src/main.rs helper is test-only so the orphans reading holds, and the Cargo.toml change is only a dev-dependency table that leaves the fake-agent feature and the rmcp release-graph check as they were
ci-integration#1.2 · no · `cargo deny check bans licenses sources` and the channel tokio-free check are green on the changed root Cargo.toml with Cargo.lock unchanged, and the advisories check reads that same unchanged lock
ci-integration#1.3 · no · the unit stage's `run --unit` command and its nextest process-per-test runner are unchanged, and its text names no test set that the witness test or the moved captures alter
ci-integration#1.4 · no · the integration stage's command, parallelism and cache are unchanged, and its text names no test set that the five new Windows-only cases alter
ci-integration#1.5 · no · the e2e run is not reached, since the session changed only root-package unit and integration tests and no e2e test or product behaviour
ci-integration#1.6 · no · the per-OS test job's steps are unchanged, the coverage run's "with the rest" already takes in the new cases, and the `[[test]]` conpty_sideload entry holds on a Cargo.toml that gained only a dev-dependency table
ci-integration#1.7 · no · the mutants matrix (ten jobs, five root items, workflow_dispatch only) and the mutation gate are unchanged, and no mutation run or matrix item was added
ci-integration#1.8 · no · the perf runs and their gate are unchanged, and the reached hook and obs files changed only their test modules, so the measured product code is the same
ci-integration#1.9 · no · the MSRV toolchain pin, steps and workspace rust-version still hold, since Cargo.lock is unchanged and the Cargo.toml change is only a cfg(windows) dev-dependency table
ci-integration#1.10 · no · the fuzz replay and nightly fuzz run in the separate `fuzz` working directory, which the session did not reach
ci-integration#1.11 · no · the release build (`--locked --bin viola`) and its probes are unchanged, the release graph is unchanged, and a dev-dependency feature reaches no release build under resolver 3
ci-integration#1.12 · no · the gate's inputs (junit-*.xml, the llvm-cov summary, the hyperfine JSON) and its §10 breach rule are unchanged, and the new cases land in junit-windows-2025, which the gate already reads
