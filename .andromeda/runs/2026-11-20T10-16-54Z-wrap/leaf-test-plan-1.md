@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, so
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=50 class=stale
leaf-text: reads its child's screen only into closed words, by compiled literals and the box row's cell attributes, never text
source: dec-j2mfetpl «The test-only live rig reads its child's screen only as closed words, by compiled literals, their compiled shapes and box-row cell attributes, never text, which its rehearsal asserts on.»
replace-with: reads its child's screen only as closed words, by compiled literals, their compiled shapes and box-row cell attributes, never text
