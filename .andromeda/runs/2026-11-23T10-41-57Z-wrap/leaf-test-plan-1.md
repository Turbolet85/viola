@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/testing.md lines=132
last: - 2026-11-22: A rename of a product constant or a move of a
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=25 class=stale
leaf-text: `lineage{source:wrapper}` → `session-start`
source: ctr-fkljkm43 «The events log reads wheel with cause start, then lineage with source wrapper, then launch with source wrapper, then the agent's session-start with source hook.»
replace-with: `lineage{source:wrapper}` → `launch{source:wrapper}` → `session-start`
