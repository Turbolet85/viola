# Classified — test-plan · ci-integration.b1

## Answers
ci-integration#1.1 · no · lint commands, the sync-crate list and every lib and bin target are unchanged, since no crate or manifest is among the reached files
ci-integration#1.2 · no · no manifest, lockfile or crate is among the reached files, so the cargo-deny, tokio-ban and deny-probe gates read the same graph
ci-integration#1.3 · no · the unit command and nextest process-per-test still hold on the changed `.config/nextest.toml`, whose only change adds one integration binary to the ci kill-line filter
ci-integration#1.4 · no · the integration command and its parallel claim still hold on the changed `.config/nextest.toml`, whose only change is one kill-line binary, and the entity names no binary
ci-integration#1.5 · no · the E2E command and its per-OS matrix are unchanged and no workflow or script moved, and the claim names no nextest setting the kill-line change alters
ci-integration#1.6 · no · the coverage run still covers the `conpty_sideload` binary with the rest on the changed `.config/nextest.toml`, whose only change adds a kill-line timeout and no coverage exclusion
ci-integration#1.7 · no · the mutants matrix and its ten items sit in `windows-mutants.yml`, which no session change reached and which is untouched, so the item count and file split hold
ci-integration#1.8 · no · the perf command, G2, schema-check and secret-scan steps and the perf gate are unchanged, and `run --perf` still times the same five hooks
ci-integration#1.9 · no · the 1.96 toolchain pin, the MSRV commands and the workspace `rust-version` are unchanged and no dependency moved, and the claim still holds on the changed `.config/nextest.toml`
ci-integration#1.10 · no · the fuzz replay and nightly fuzz commands and the `fuzz/` tree are unchanged, and no fuzz target is among the reached files
ci-integration#1.11 · no · the release-check probes and the viola-only build are unchanged, and no manifest or bin target is among the reached files
ci-integration#1.12 · no · the gate's reads of `junit-*.xml`, `run-summary.json`, the llvm-cov and hyperfine summaries and its §10 breach rule still hold on the changed `.config/nextest.toml`, whose only change is one kill-line filter binary
