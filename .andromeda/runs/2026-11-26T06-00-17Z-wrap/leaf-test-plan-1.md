@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/testing.md lines=134
last: - 2026-11-25: A case whose start must end before its session
@leaf_edit n=1 leaf=.claude/rules/testing.md line=36 class=omission
leaf-text: (write through the writer, then pad, replace or re-version).
source: dec-shiemenq «A verify test that needs a failing relied row takes it from a spine fixture set with no SessionEnd payload in the trusted folder, never by moving the fake agent's trusted root off verify's folder.»
replace-with: (write through the writer, then pad, replace or re-version). A verify test that needs a failing relied row takes it from a spine fixture set with no SessionEnd payload in the trusted folder, never by moving the fake agent's trusted root off verify's folder: a trusted root moved off verify's folder reads as the untrusted-folder refusal, which stamps nothing.
