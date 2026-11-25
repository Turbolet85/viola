@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/testing.md lines=133
last: - 2026-11-24: `tests/contract_error_bodies.rs` holds two rows
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=21 class=omission
leaf-text: and SessionStart fires only after Enter on the trust row, which draws the ready screen)
source: ctr-7t7jbcb4 «contract/fake-agent-cli/resume-absent · viola-fake-agent command line · --resume-absent — The argv-only --resume-absent, given with --resume, makes the fake agent fire SessionEnd with no SessionStart, close»
replace-with: and SessionStart fires only after Enter on the trust row, which draws the ready screen), the argv-only `--resume-absent`, given with `--resume`, makes it fire SessionEnd with no SessionStart, close its hooks, ignore SIGHUP and exit 1, at both launch sites (a trusted start and the trust dialog's accept)
