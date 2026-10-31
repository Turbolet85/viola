# Classified — test-plan · ci-integration.b1

## Answers
ci-integration#1.1 · no · the lint crate list, fmt, clippy and orphan steps are unmoved, Cargo.lock is not reached and the Cargo.toml change is two [[test]] tables, so the rmcp feature assertion holds
ci-integration#1.2 · no · no dependency or lock file was reached, so the deny config, the tokio ban and the 14 deny probes are unmoved
ci-integration#1.3 · no · the unit command and nextest process-per-test parallelism hold on the changed nextest.toml, whose only change is two kill-line binary terms in the ci profile, which the stage does not state
ci-integration#1.4 · no · the integration command and parallelism hold, and the two new root [[test]] binaries carry no stated count
ci-integration#1.5 · no · the e2e command and the three-OS matrix are unmoved, and the three new mcp_tools cases add tests without a stated count
ci-integration#1.6 · no · the coverage command, the conpty_sideload binary and the gate suites hold, and the new root binaries fall under "the rest" with no count stated
ci-integration#1.7 · no · no workflow or matrix file was reached and no package or file was added to the windows-mutants matrix, so its ten jobs and five root items hold
ci-integration#1.8 · no · the perf steps (schema-check, G2, secret-scan, AGENT_RUN_KEEP_HOMES) are read and not moved, and CI read green with the narrowed schema in place
ci-integration#1.9 · no · the 1.96 floor, the rust-version and the toolchain steps are unmoved, and the Cargo.toml change is two [[test]] tables
ci-integration#1.10 · no · the fuzz-replay and nightly fuzz steps are unmoved and no fuzz directory or target was reached
ci-integration#1.11 · no · the release-check probes and the viola-only release build are unmoved, and the new [[test]] tables are not the viola binary
ci-integration#1.12 · no · the gate's inputs, breach rule and status shape are unmoved, no threshold moved, and the entity states no count
