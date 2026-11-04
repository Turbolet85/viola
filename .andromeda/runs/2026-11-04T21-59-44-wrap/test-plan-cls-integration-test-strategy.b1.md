@answer n=1 entity=integration-test-strategy#1.1 v=no
why: the module-to-module crate-API boundary names no file the session reached and no changed fact
@answer n=2 entity=integration-test-strategy#1.2 v=no
why: the module-to-disk boundary names no reached file and none of the session's changed facts
@answer n=3 entity=integration-test-strategy#1.3 v=no
why: the external child-process boundary is untouched, and the fake agent's product behaviour did not change
@answer n=4 entity=integration-test-strategy#1.4 v=no
why: the IPC text names no reached file and its socket and DACL claims are unaffected by the session
@answer n=5 entity=integration-test-strategy#1.5 v=no
why: the PTY text names tests/support/outer_pty.rs and tests/support/piped.rs, neither reached, and its spawn and resize claims stand
@answer n=6 entity=integration-test-strategy#1.6 v=no
why: the hook boundary names tests/hook_fail_open.rs, which the session did not reach, and its fail-open claims stand
@answer n=7 entity=integration-test-strategy#2.1 v=no
why: the placement witnesses live in crates/viola-state/src/fs.rs, a src file the session did not move
@answer n=8 entity=integration-test-strategy#2.2 v=yes mode=stale
why: it places cleanup_after_a_force_killed_wrapper_reads_the_endpoint_gone on ubuntu-latest alone, but the test is cfg(unix) and now runs on both Unix runners
@answer n=9 entity=integration-test-strategy#3.0 v=no
why: the lifecycle text names tests/support/home.rs and no reached file, and the Frozen guard moving into support is not shared test state, so "nothing is shared between tests" still holds
@answer n=10 entity=integration-test-strategy#4.1 v=yes mode=stale
why: it names tests/cli_instance_state.rs as the home of Frozen, which moved to tests/support/frozen.rs
@answer n=11 entity=integration-test-strategy#4.2 v=no
why: it names run_cli.rs, cli_list.rs and cli_verify.rs claims that still hold, since the session's run_cli edit only ungated a different test and the report shows no change to the cited refusals
@answer n=12 entity=integration-test-strategy#5.1 v=no
why: the cited cli_verify, cli_list, cli_plugin_install and cross_exit_causes cases keep their claims, the cross_exit_causes case stays in its table, and the reached edits there are reason lines and the census
