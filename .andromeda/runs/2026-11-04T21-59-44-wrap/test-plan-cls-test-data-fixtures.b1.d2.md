@answer n=1 entity=test-data-fixtures#1.1 v=no
why: the session moved no product state and its Windows seed_conpty carve-out is not among the gates it changed
@answer n=2 entity=test-data-fixtures#1.2 v=no
why: the run_cli.rs change only takes the Windows gate off run_child_ends_when_its_wrapper_is_terminated and this widened-home claim is a different test
@answer n=3 entity=test-data-fixtures#1.3 v=no
why: ledger/stamps.json is written only by viola verify and the session moved no ledger writer or stamped-home helper
@answer n=4 entity=test-data-fixtures#1.4 v=no
why: budget.json is written by the fake statusline path and the session moved no budget writer or budget reader
@answer n=5 entity=test-data-fixtures#1.5 v=no
why: tests/support/home.rs holding plant_statusline_source is not a reached file, the only support change being pub mod frozen in tests/support/mod.rs
@answer n=6 entity=test-data-fixtures#1.6 v=no
why: the socket-root helpers sit in tests/support/home.rs and tests/support/start.rs, neither of which the session reached
@answer n=7 entity=test-data-fixtures#1.7 v=no
why: the widen helpers sit in tests/support/home.rs, which the session did not reach
@answer n=8 entity=test-data-fixtures#1.8 v=no
why: the socket-directory Form lives in tests/security_negatives_socket_dir.rs, which the session did not reach
@answer n=9 entity=test-data-fixtures#1.9 v=no
why: claude_bin and the spawn_on path live in tests/support/fake.rs and support::cli, neither reached, and the list spawn path is unchanged
@answer n=10 entity=test-data-fixtures#1.10 v=no
why: no recorded hook payload or fixture file moved and section 7 is untouched
@answer n=11 entity=test-data-fixtures#1.11 v=no
why: no recorded MCP fixture moved and tests/mcp_recorded_exchange.rs is not a reached file
@answer n=12 entity=test-data-fixtures#1.12 v=yes mode=stale
why: cross_exit_causes.rs, reached with its top comment moved and gate reasons added, holds the CAUSES literals this line cites, and cli_list.rs, reached whole, holds the board literal it cites
@answer n=13 entity=test-data-fixtures#1.13 v=no
why: no insta snapshot was regenerated and no snapshot file was reached
@answer n=14 entity=test-data-fixtures#1.14 v=no
why: the fake-agent hook transcripts are compared to recorded payloads that did not move
@answer n=15 entity=test-data-fixtures#1.15 v=no
why: no proptest strategy or regression seed was changed
@answer n=16 entity=test-data-fixtures#1.16 v=no
why: no clock seam, test-util feature or manifest moved, the session touching no Cargo.toml or Cargo.lock
@answer n=17 entity=test-data-fixtures#1.17 v=no
why: tests/channel_error_bodies.rs and tests/support/events.rs are not reached and the PTY env_set path is unchanged
@answer n=18 entity=test-data-fixtures#2.1 v=no
why: the server-identity impostor files tests/security_negatives_server_identity.rs and tests/hook_fail_open.rs are not reached
@answer n=19 entity=test-data-fixtures#2.2 v=no
why: crates/viola-mcp/tests/handlers.rs is not a reached file and record_live is unchanged
@answer n=20 entity=test-data-fixtures#2.3 v=yes mode=stale
why: tests/cli_verify.rs, reached whole, holds the ledger catch-site case and the per-OS regular-file oracle this line cites
@answer n=21 entity=test-data-fixtures#2.4 v=no
why: the endpoint-directory helpers sit in handlers.rs, hook_fail_open.rs, security_negatives_server_identity.rs and src/cmd/client.rs, none reached
@answer n=22 entity=test-data-fixtures#3.1 v=no
why: contract_fake_agent_drift.rs is not a reached file and no fake-agent copy step changed
@answer n=23 entity=test-data-fixtures#4.1 v=no
why: src/bin/viola-fake-agent.rs did not change and the fake agent's hook-command reading is unchanged
@answer n=24 entity=test-data-fixtures#5.1 v=no
why: the fake agent's receipt kinds are unchanged and tests/support/fake.rs is not a reached file
@answer n=25 entity=test-data-fixtures#6.1 v=no
why: the fake agent's modes are unchanged and the Held guards in tests/security_negatives_server_identity.rs and tests/hook_fail_open.rs are not reached
