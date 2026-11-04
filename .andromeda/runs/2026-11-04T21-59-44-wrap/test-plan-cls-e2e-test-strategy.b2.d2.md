@answer n=1 entity=e2e-test-strategy#6.4 v=no
why: the Path 5 cleanup names no file, count or guard that the session moved
@answer n=2 entity=e2e-test-strategy#7.1 v=no
why: the statusline case stays in tests/cli_instance_state.rs, the change moving only the Frozen guard out of that file
@answer n=3 entity=e2e-test-strategy#7.2 v=no
why: the Path 6 verification signals name no file, count or guard that the session moved
@answer n=4 entity=e2e-test-strategy#7.3 v=no
why: the as-landed cases sit in cli_budget, statusline support and mcp_tools, none of them a file the session reached
@answer n=5 entity=e2e-test-strategy#7.4 v=no
why: the cleanup line is harness cleanup alone
@answer n=6 entity=e2e-test-strategy#8.1 v=no
why: the Path 7 steps name no file, count or gate that the session changed
@answer n=7 entity=e2e-test-strategy#8.2 v=no
why: the Path 7 verification signals name no file, count or gate that the session moved
@answer n=8 entity=e2e-test-strategy#8.3 v=no
why: the MCP leg of Path 7 sits in mcp_tools.rs, a file the session did not reach
@answer n=9 entity=e2e-test-strategy#8.4 v=no
why: the cleanup line is harness cleanup and TempDir drop alone
@answer n=10 entity=e2e-test-strategy#9.1 v=no
why: the E2 steps name no file, count or OS gate that the session moved
@answer n=11 entity=e2e-test-strategy#9.2 v=no
why: the E2 signals name tui_env_strip and tui_channel_fds, neither a file the session reached
@answer n=12 entity=e2e-test-strategy#9.3 v=no
why: the cleanup line is TempDir drop alone
@answer n=13 entity=e2e-test-strategy#10.1 v=no
why: the E3 link and unlink text sits in tests/cli_links.rs, a file the session did not reach
@answer n=14 entity=e2e-test-strategy#11.1 v=no
why: tests/cli_list.rs gained only an OS-gate reason line, so the e5 list cases and their live reading still stand
@answer n=15 entity=e2e-test-strategy#12.1 v=no
why: the security sweep names hook_fail_open.rs and the viola-mcp tests, none reached, and the census leaves its OS-split claims as they are
@answer n=16 entity=e2e-test-strategy#13.1 v=no
why: tests/cross_exit_causes.rs gained reason lines and a top-comment line, while the case this sweep names keeps its file and this text does not point at the top comment
@answer n=17 entity=e2e-test-strategy#14.1 v=yes mode=stale
why: the top comment at line 12 of tests/cross_exit_causes.rs now names the guard's new home, so the matrix text that points at it needs a re-read
@answer n=18 entity=e2e-test-strategy#15.1 v=no
why: the security negatives named sit in files the session did not reach, and their OS gates stay as written under the census
@answer n=19 entity=e2e-test-strategy#16.1 v=yes mode=add-to
why: the stale-row dim witness of F1 joins the list row integrity suite in tests/cross_list_rows.rs beside its escaping and alignment cases
@answer n=20 entity=e2e-test-strategy#17.1 v=no
why: the chaos suite names no file the session reached, and its SIGSTOP and Frozen text names the guard without a file, so it stays true
@answer n=21 entity=e2e-test-strategy#18.1 v=no
why: the property suite names no file or OS gate that the session reached
@answer n=22 entity=e2e-test-strategy#19.1 v=yes mode=add-to
why: the new contract binary tests/contract_os_gated_tests.rs (F4) joins the contract binaries, its census reading 52 OS-gated functions and 7 gated files, 59 GATES rows (F5)
