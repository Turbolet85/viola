@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/verification-harness.md lines=72
last: - 2026-10-20: `target/debug/viola` is a hard link cargo re-p
@leaf_edit n=1 leaf=.claude/rules/verification-harness.md line=41 class=omission
leaf-text: and one that lands before the exit begins still ends it (measured).
source: fct-nwgyqyjs «so the fake agent's SIGHUP-ignoring exit, shared Unix code, is witnessed on Linux only (§7 Fake agent).»
replace-with: and one that lands before the exit begins still ends it (measured). That exit, shared Unix code, is witnessed on Linux only: `tests/cli_fake_agent.rs` `fake_agent_exit_outlives_a_hangup_that_lands_while_it_exits` (`cfg(target_os = "linux")`) reads the SIGHUP disposition through `ps -o sigignore= -p <pid>`, which `macos-latest`'s `ps` cannot print.
