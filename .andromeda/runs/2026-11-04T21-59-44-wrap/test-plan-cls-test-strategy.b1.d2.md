@answer n=1 entity=test-strategy#1.1 v=no
why: the unit tier and its macOS-compiled-out cases stand, since no file under src/ or crates/*/src/ moved and section 4 is untouched
@answer n=2 entity=test-strategy#1.2 v=no
why: the integration rule names crate-level tests/ suites, and the session changed only root tests/ files
@answer n=3 entity=test-strategy#1.3 v=no
why: the seven critical paths and four expansion scenarios are untouched, and the new stale-row outer-PTY witness is not one of them
@answer n=4 entity=test-strategy#1.4 v=no
why: the property suite and the fuzz corpus gate are untouched
@answer n=5 entity=test-strategy#1.5 v=no
why: the section 10 perf budgets and the hyperfine rows are untouched
@answer n=6 entity=test-strategy#1.6 v=no
why: the chaos text names the SIGSTOP freeze but no file, so it stays true under the moved Frozen guard
@answer n=7 entity=test-strategy#1.7 v=no
why: the mutation gate is untouched, and the default profile still has no kill line
@answer n=8 entity=test-strategy#1.8 v=no
why: the fake agent's product behaviour, fixtures and drift replays are untouched
@answer n=9 entity=test-strategy#2.1 v=yes mode=add-to
why: tests/support/mod.rs now declares pub mod frozen, so the support list gains frozen.rs, and the new root contract_os_gated_tests binary (F6) joins the root test list
@answer n=10 entity=test-strategy#3.1 v=no
why: the IPC trigger is untouched and the files it cites (handlers.rs, security_negatives_channel.rs, security_negatives_server_identity.rs, security_negatives_socket_dir.rs, channel_links.rs) are not reached
@answer n=11 entity=test-strategy#3.2 v=no
why: the paste injection trigger is untouched and no file it cites is reached
@answer n=12 entity=test-strategy#3.3 v=no
why: the hook stdin trigger is untouched and no file it cites is reached
@answer n=13 entity=test-strategy#3.4 v=no
why: the MCP trigger is untouched and no file it cites is reached
@answer n=14 entity=test-strategy#3.5 v=no
why: the CLI and env trigger is untouched and no file it cites is reached
@answer n=15 entity=test-strategy#3.6 v=no
why: cli_list.rs is reached, but the filesystem negative cases are untouched and no negative case changed, so list_refuses_a_home_another_user_may_read_with_and_without_an_instance still holds
@answer n=16 entity=test-strategy#3.7 v=no
why: the child spawn trigger is untouched, and this row places no child-cleanup case on Windows only
@answer n=17 entity=test-strategy#3.8 v=no
why: the supply chain trigger is untouched, and the junit reader is an operator-pass script that is not a CI or lint step
@answer n=18 entity=test-strategy#3.9 v=no
why: the property trigger, the proptest suite and the fuzz job are untouched
@answer n=19 entity=test-strategy#3.10 v=no
why: the chaos suite is untouched, and its SIGSTOP text names no file
@answer n=20 entity=test-strategy#3.11 v=no
why: the performance-budget trigger and its section 10 rows are untouched
@answer n=21 entity=test-strategy#3.12 v=no
why: the cross-surface scenarios 2 and E3 and the parity section are untouched
@answer n=22 entity=test-strategy#3.13 v=yes mode=add-to
why: the multi-os-compat trigger and the OS-branch test claims are now held by the census tests/contract_os_gated_tests.rs, which names every OS-gated test and file with its reason
@answer n=23 entity=test-strategy#3.14 v=no
why: the multi-version-compat trigger and its fixture sections are untouched
@answer n=24 entity=test-strategy#3.15 v=no
why: the contract-test-against-sandbox trigger covers fake agent against sandbox fixtures, which the census binary does not change
@answer n=25 entity=test-strategy#3.16 v=no
why: the error-sanitization witnesses named in this row (cli_error_bodies.rs, channel_error_bodies.rs, contract_error_bodies.rs, mcp_tools.rs) are not reached and their claims stand
