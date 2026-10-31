# Classified — tests · ci-integration.b1

## Answers
ci-integration#1.1 · yes · clippy --all-targets -D warnings and orphans-check bind the chunk's new cross_exit_causes.rs and its edited src and crate code
ci-integration#1.2 · yes · viola-channel client.rs is edited, so its normal graph must stay tokio-free (channel-tokio-free and the sole-root tokio ban); a new rstest or other dependency must pass cargo deny
ci-integration#1.3 · yes · the root bin exit-code mapping unit cases and the clock-driven stale-heartbeat unit case run in this layer
ci-integration#1.4 · yes · cross_exit_causes.rs and the cause rows of the other root binaries (cli_revive, cli_verify, run_cli, security_negatives_*) run in the integration layer this stage runs
ci-integration#1.5 · yes · the mcp_tools.rs e2e file and the Linux and Windows server-check and Windows script-child cases run in this per-OS matrix, with the macOS gap recorded
ci-integration#1.6 · yes · the one instrumented run carries every new case and the watch; its per-OS thresholds stay standing S11
ci-integration#1.7 · no · mutation runs at the epoch boundary and windows-mutants.yml is out of the chunk (Not needed: no chunk gate for mutation)
ci-integration#1.8 · yes · the schema-check and secret-scan steps rule over the process-log lines and diag-line.v1.json detail values the chunk changes; the hyperfine measurement is not moved
ci-integration#1.9 · yes · the chunk's new code and the rstest dev-test must compile and pass unit on the Rust 1.96 floor
ci-integration#1.10 · no · cargo-fuzz replay and nightly fuzz; the chunk changes no parser and no fuzz target (Not needed)
ci-integration#1.11 · no · release-check judges the release build's artifacts; the chunk adds no test-only feature and its rstest table is dev-only
ci-integration#1.12 · yes · the gate reads the junit records that witness the new cases and the watch's green tally; its §10 breach verdicts stay standing S10 and S11
