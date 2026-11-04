@answer n=1 entity=integration-test-strategy#1.1 v=no
why: the session moved no crate API, rstest fixture or module-to-module call that this row names

@answer n=2 entity=integration-test-strategy#1.2 v=no
why: the on-disk append tests it names live in no file the session reached

@answer n=3 entity=integration-test-strategy#1.3 v=no
why: the child-process boundary and fake-agent modes are unchanged, and the fake agent's product behaviour stays as before

@answer n=4 entity=integration-test-strategy#1.4 v=no
why: the IPC and socket-directory tests it names are in files the session did not reach

@answer n=5 entity=integration-test-strategy#1.5 v=no
why: the PTY and ConPTY tests it names are in files the session did not reach and viola-pty did not change

@answer n=6 entity=integration-test-strategy#1.6 v=no
why: the hook fail-open file it names, tests/hook_fail_open.rs, is not reached

@answer n=7 entity=integration-test-strategy#2.1 v=no
why: the filesystem placement witnesses it names sit in crates/viola-state and in no reached file

@answer n=8 entity=integration-test-strategy#2.2 v=yes mode=stale
why: cleanup_after_a_force_killed_wrapper_reads_the_endpoint_gone now runs on both Unix runners, so placing it on ubuntu-latest alone is stale

@answer n=9 entity=integration-test-strategy#3.0 v=no
why: the lifecycle, home-base and seeding text names no reached file and makes no Frozen or gate claim

@answer n=10 entity=integration-test-strategy#4.1 v=no
why: the wrapper channel tests it names are in files the session did not reach, and the census changes none of their claims

@answer n=11 entity=integration-test-strategy#4.2 v=yes mode=stale
why: tests/run_cli.rs, tests/cli_verify.rs and tests/cli_list.rs are reached whole, and the report does not show their widened-home and read-rule claims still hold on the changed files

@answer n=12 entity=integration-test-strategy#5.1 v=yes mode=stale
why: the Frozen guard bullet names tests/cli_instance_state.rs as its home, which moved to tests/support/frozen.rs
