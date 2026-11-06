@answer n=1 entity=integration-test-strategy#1.1 v=no
why: the rstest 0.27.0 pin and the module examples are outside the dependency move (portable-pty, nix, serial family, pin-utils, memoffset, termios) and cite no reached file
@answer n=2 entity=integration-test-strategy#1.2 v=no
why: the concurrent-append claims name ubuntu-latest and tempfile 3.27.0, neither touched by the dependency move, and cite no reached file
@answer n=3 entity=integration-test-strategy#1.3 v=no
why: the fake agent is still copied to the test PATH as claude, task item 5 keeping the plain claude name and the registered bin/claude path
@answer n=4 entity=integration-test-strategy#1.4 v=yes mode=stale
why: its rule lives in tests/security_negatives_socket_dir.rs, reached by the Windows-remnant retirement, and the task does not show the Unix socket-dir refusal claims still hold
@answer n=5 entity=integration-test-strategy#1.5 v=yes mode=stale
why: the tools line pins portable-pty =0.8.1, now =0.9.0 under the dependency move, and crates/viola-pty/src/pump.rs is reached
@answer n=6 entity=integration-test-strategy#1.6 v=yes mode=stale
why: the tools line names assert_cmd 2.2.2 in section 5, which task item 10 re-words to the literal pins, and its rule lives in reached tests/hook_fail_open.rs
@answer n=7 entity=integration-test-strategy#2.1 v=no
why: the forced-window witness lives in crates/viola-state/src/fs.rs, which the change does not reach, and its Linux and ubuntu-latest readings are unchanged
@answer n=8 entity=integration-test-strategy#2.2 v=yes mode=stale
why: its rule lives in crates/viola-e2e/tests/harness_lifecycle.rs, reached by the Windows-remnant retirement, and the task does not show its cfg(unix) ubuntu-latest claim still holds
@answer n=9 entity=integration-test-strategy#3.0 v=yes mode=stale
why: its rule lives in tests/support/home.rs, reached by the resolved verbatim-strip removal, and the task does not show the prepare_home_base statement still holds
@answer n=10 entity=integration-test-strategy#4.1 v=yes mode=stale
why: its rules live in reached tests/channel_endpoint.rs, tests/hook_fail_open.rs and tests/cli_send.rs, and the task does not show their claims still hold
@answer n=11 entity=integration-test-strategy#4.2 v=yes mode=stale
why: its rules live in reached tests/run_cli.rs, tests/cli_verify.rs and tests/cli_list.rs, and the task does not show their refusal claims still hold
@answer n=12 entity=integration-test-strategy#5.1 v=yes mode=contradict
why: it backs the home resolution units with plain_drive_* in src/cmd/mod.rs, reads the help program as viola.exe, cites the retired section 6 rewritten-path warning, and treats statusline-rate-limits as relied on every build
