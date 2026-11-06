@account n=1 entity=integration-test-strategy#1.4 v=not_touched
why: tests/security_negatives_socket_dir.rs gained only header lines 1-2 and no socket-directory test or check is removed, so the 0755 / symlink refusals and the client's missing directory check still hold
@account n=2 entity=integration-test-strategy#1.5 v=proposed
why: its Tools line names portable-pty `=0.8.1`, which the dependency move retired for `=0.9.0`
@account n=3 entity=integration-test-strategy#1.6 v=not_touched
why: tests/hook_fail_open.rs gained one comment line (7) and the report removes none of its cases, so Env::Made, no_instance_dir and the widened-instance rstest function still hold
@account n=4 entity=integration-test-strategy#2.2 v=not_touched
why: harness_lifecycle.rs gained one line (65) and the census reads GATES 41 / 39 / 2 unchanged, so the cfg(unix) cleanup witness on ubuntu-latest still holds
@account n=5 entity=integration-test-strategy#3.0 v=not_touched
why: the report removes only resolved's verbatim strip from tests/support/home.rs, which this entity never names; prepare_home_base, wait_ready and the chaos carve-out still hold
@account n=6 entity=integration-test-strategy#4.1 v=not_touched
why: channel_endpoint.rs and cli_send.rs lose only the rewritten-path witness, and the cases this entity names (the conn-less method line, send_empty_text_is_refused_at_once) still stand
@account n=7 entity=integration-test-strategy#4.2 v=not_touched
why: run_cli.rs, cli_verify.rs and cli_list.rs gain comment lines only and the report removes none of the strict-modes or home-tree cases this entity names
@account n=8 entity=integration-test-strategy#5.1 v=proposed
why: it names the retired `plain_drive_*` units, the `.exe` usage-program form, the retired §6 rewritten-path warning, and a live `0 fail` that the measured statusline-rate-limits red disproves
