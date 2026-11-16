@leaf_group group=test-plan
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/testing.md lines=118
last: - 2026-11-16: A shared case table whose assertions pin one r
@leaf_read leaf=.claude/rules/verification-harness.md lines=73
last: - 2026-11-14: A test-only recorder that replaces every off-l
@leaf_edit n=1 leaf=.claude/rules/testing.md line=24 class=stale
leaf-text: StopFailure, provisionally, has no recorded payload, ledger row or verify probe
source: dec-gf63skju «StopFailure is registered with no recorded payload, no ledger row and no verify probe; its tests use a synthetic StopFailure payload.»
replace-with: StopFailure is registered with no recorded payload, ledger row or verify probe
