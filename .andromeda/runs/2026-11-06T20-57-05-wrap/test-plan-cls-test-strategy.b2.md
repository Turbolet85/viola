@answer n=1 entity=test-strategy#3.16 v=yes mode=stale
why: the session reaches crates/viola-e2e/tests/mcp_tools.rs, the witness of mcp_tools_error_results_hold_no_path_or_upstream_text, and no report shows that claim still holds on the changed file
@answer n=2 entity=test-strategy#3.17 v=yes mode=stale
why: the session reaches crates/viola-e2e/src/harness/run/mutants.rs, where the mutation rule lives, and no report shows that rule unchanged on the changed file
@answer n=3 entity=test-strategy#3.18 v=no
why: .config/nextest.toml changed only in its timing comments and the session adds no screen parsing, blanket approval or retry policy, so the no-flake discipline still holds
