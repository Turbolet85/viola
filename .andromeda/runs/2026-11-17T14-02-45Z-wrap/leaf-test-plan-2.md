@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/verification-harness.md lines=74
last: - 2026-11-17: A change to what the fake agent draws under `-
@leaf_edit n=1 leaf=.claude/rules/verification-harness.md line=30 class=stale
leaf-text: tying it and its stamping `viola verify` to the test thread by a parent-death kill
source: dec-fcf6eyb7 «while an in-test harness session ties it to the test thread by parent-death SIGKILL and its stamping viola verify by SIGTERM.»
replace-with: tying it to the test thread by a parent-death SIGKILL and its stamping `viola verify` by a parent-death SIGTERM
@leaf_edit n=2 leaf=.claude/rules/verification-harness.md line=30 class=omission
leaf-text: or the rstest chain `home → fake_agent_path → stamped_home → booted_wrapper`.
source: dec-w5ivfz5r «No viola-e2e test boots a harness session untied: a census over viola-e2e's test sources finds every in-test boot tied, and is read red on a planted untied boot first.»
replace-with: or the rstest chain `home → fake_agent_path → stamped_home → booted_wrapper`. No viola-e2e test boots a session untied: `viola_e2e_census_finds_no_untied_boot` finds every in-test boot in viola-e2e's test sources tied, read red on a planted untied boot first.
