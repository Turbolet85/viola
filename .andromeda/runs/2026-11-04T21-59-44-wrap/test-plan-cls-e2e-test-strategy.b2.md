@answer n=1 entity=e2e-test-strategy#6.4 v=no
why: its only cleanup line names the outer PTY, child wait and TempDir drop, none of which the chunk moved
@answer n=2 entity=e2e-test-strategy#7.1 v=yes mode=stale
why: the chunk reaches tests/cli_instance_state.rs, where the cited path6 pass-through case lives, and nothing shows that citation still holds after the Frozen guard left the file
@answer n=3 entity=e2e-test-strategy#7.2 v=no
why: its verification signals name no file, OS gate or binary count that the chunk reached or changed
@answer n=4 entity=e2e-test-strategy#7.3 v=no
why: it cites tests/cli_budget.rs, tests/hook_statusline.rs and tests/support/statusline.rs, none of which the chunk reached
@answer n=5 entity=e2e-test-strategy#7.4 v=no
why: its only step is the harness cleanup, which the chunk did not touch
@answer n=6 entity=e2e-test-strategy#8.1 v=no
why: the Path 7 steps run agent-run and verify against no file the chunk reached
@answer n=7 entity=e2e-test-strategy#8.2 v=no
why: its verification signals name no file, gate or count that the chunk changed
@answer n=8 entity=e2e-test-strategy#8.3 v=no
why: it cites crates/viola-e2e/tests/mcp_tools.rs, which the chunk did not reach
@answer n=9 entity=e2e-test-strategy#8.4 v=no
why: its cleanup is the harness cleanup and a TempDir drop, both untouched by the chunk
@answer n=10 entity=e2e-test-strategy#9.1 v=no
why: the R8 env-strip steps name no file the chunk reached
@answer n=11 entity=e2e-test-strategy#9.2 v=no
why: it cites tests/tui_env_strip.rs and tests/tui_channel_fds.rs, neither of which the chunk reached
@answer n=12 entity=e2e-test-strategy#9.3 v=no
why: its only cleanup is a TempDir drop
@answer n=13 entity=e2e-test-strategy#10.1 v=no
why: it cites tests/cli_links.rs, which the chunk did not reach
@answer n=14 entity=e2e-test-strategy#11.1 v=yes mode=stale
why: the chunk reaches tests/cli_list.rs, where the cited e5_list_reads_a_live_wrapper_through_the_replay case lives, and nothing shows its two-case citation still holds
@answer n=15 entity=e2e-test-strategy#12.1 v=no
why: the security sweep cites only files the chunk did not reach, and its OS-gate claims stand under the census
@answer n=16 entity=e2e-test-strategy#13.1 v=no
why: the kill line gained only binary(run_cli) and this text states no count, so its claim that the two error-body binaries sit on that line still holds
@answer n=17 entity=e2e-test-strategy#14.1 v=yes mode=stale
why: the chunk moved the top comment of tests/cross_exit_causes.rs at line 12, which this exit-cause matrix points at, so its reading of that comment is stale
@answer n=18 entity=e2e-test-strategy#15.1 v=no
why: its security negatives cite tests and census-held OS gates the chunk did not reach or change, and its kill-line mention states no count
@answer n=19 entity=e2e-test-strategy#16.1 v=yes mode=add-to
why: tests/cross_list_rows.rs now holds the outer-PTY stale-row dim witness (F1), which this suite's escaping and column-alignment list does not yet carry
@answer n=20 entity=e2e-test-strategy#17.1 v=yes mode=add-to
why: the chaos list gains the wrapper-SIGKILL child-cleanup witness (F2) beside its after-SIGKILL bullet, and the Linux-only SIGHUP witness read through ps (F3) beside its no-EOF SIGHUP bullet
@answer n=21 entity=e2e-test-strategy#18.1 v=no
why: the property suite names no file the chunk reached, and its proptest and fuzz files are untouched
@answer n=22 entity=e2e-test-strategy#19.1 v=yes mode=add-to
why: the contract suite list gains the root binary contract_os_gated_tests (F4) and its census count of 52 functions, 7 gated files and 59 GATES rows (F5)
