@answer n=1 entity=e2e-test-strategy#1.1 v=yes mode=stale
why: the trycmd text of §6 is re-worded to the literal pins (CARRY 1) and tests/cli_output_plain.rs, reached by the EXE_SUFFIX removal, is cited as pinning these layouts
@answer n=2 entity=e2e-test-strategy#1.2 v=yes mode=stale
why: the assert_cmd text of §6 is re-worded to the literal pins (CARRY 1)
@answer n=3 entity=e2e-test-strategy#1.3 v=yes mode=stale
why: the pin portable-pty =0.8.1 now reads =0.9.0 with the dependency move
@answer n=4 entity=e2e-test-strategy#1.4 v=no
why: it cites only the wrapper channel client, interprocess, tests/channel_error_bodies.rs and tests/tui_env_strip.rs, none reached by the session
@answer n=5 entity=e2e-test-strategy#1.5 v=yes mode=stale
why: it cites crates/viola-e2e/tests/mcp_tools.rs and mcp_recorded_exchange.rs, both reached whole by the session, as holding its MCP stdio claims
@answer n=6 entity=e2e-test-strategy#1.6 v=no
why: it names only cargo-nextest 0.9.146 and cargo test --doc, with no three-runner or per-OS timing reading and no reached file
@answer n=7 entity=e2e-test-strategy#2.1 v=no
why: the path 1 steps name no reached file, retired item or Windows reading
@answer n=8 entity=e2e-test-strategy#2.2 v=yes mode=stale
why: it cites tests/run_cli.rs and tests/security_negatives_socket_dir.rs, both reached whole, for the start refusals
@answer n=9 entity=e2e-test-strategy#2.3 v=yes mode=stale
why: it cites tests/support/start.rs, reached whole, as the home of the shared start-refusal helpers
@answer n=10 entity=e2e-test-strategy#2.4 v=no
why: the cleanup names only the Drop guard and TempDir drop, with no reached file
@answer n=11 entity=e2e-test-strategy#3.1 v=yes mode=stale
why: it cites tests/cli_send.rs and crates/viola-e2e/tests/mcp_tools.rs, both reached whole, for the confirmed-send witnesses
@answer n=12 entity=e2e-test-strategy#3.2 v=no
why: the send steps name no reached file and no retired item, and the leading-slash rule they touch stands
@answer n=13 entity=e2e-test-strategy#3.3 v=yes mode=stale
why: it cites tests/cli_send.rs, reached whole, for its confirmed-send cases and carries trycmd-redacted assert_cmd text re-worded per CARRY 1
@answer n=14 entity=e2e-test-strategy#3.4 v=no
why: the cleanup is a single agent-run command naming no reached file
@answer n=15 entity=e2e-test-strategy#4.1 v=yes mode=stale
why: it cites tests/cli_wait_last.rs and crates/viola-e2e/tests/mcp_tools.rs, both reached whole, for the wait and last witnesses
@answer n=16 entity=e2e-test-strategy#4.2 v=no
why: the wait and last steps name no reached file and no retired item
@answer n=17 entity=e2e-test-strategy#4.3 v=no
why: the verification signal names no reached file and no changed item, so its wake and exit 21 claims stand
@answer n=18 entity=e2e-test-strategy#4.4 v=no
why: the cleanup names only harness cleanup and process checks, with no reached file
@answer n=19 entity=e2e-test-strategy#4.5 v=yes mode=stale
why: it cites tests/run_cli.rs, reached whole, for run_child_ends_when_its_wrapper_is_terminated
@answer n=20 entity=e2e-test-strategy#5.1 v=yes mode=stale
why: it cites crates/viola-e2e/tests/mcp_tools.rs, reached whole, for mcp_tools_path4_path5_answer_by_id_and_a_paused_send
@answer n=21 entity=e2e-test-strategy#5.2 v=no
why: the dialog steps name no reached file and no retired item
@answer n=22 entity=e2e-test-strategy#5.3 v=yes mode=add-to
why: F1, the planted annotations-drift test dialog_drift_names_a_changed_annotation, joins the annotations snapshot claim
@answer n=23 entity=e2e-test-strategy#5.4 v=no
why: the cleanup names only harness cleanup, with no reached file
@answer n=24 entity=e2e-test-strategy#6.1 v=yes mode=stale
why: it cites crates/viola-e2e/tests/mcp_tools.rs, reached whole, for mcp_tools_path4_path5_answer_by_id_and_a_paused_send
@answer n=25 entity=e2e-test-strategy#6.2 v=no
why: the wheel steps name no reached file and no retired item
