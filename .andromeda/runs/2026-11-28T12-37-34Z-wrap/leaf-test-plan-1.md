@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/testing.md lines=134
last: - 2026-11-25: A case whose start must end before its session
@leaf_edit n=1 leaf=.claude/rules/testing.md line=20 class=omission
leaf-text: a plain `spawn` of `list` runs with `PATH` set to an empty bin dir, so no test reads the host's `claude` sessions
source: dec-d4avllgy «A test's own list child, and every start of a verb that reads the claude agents list in the background, runs with PATH set to one bin dir alone, empty or holding the planted fake claude»
replace-with: a plain `spawn` of `list` runs with `PATH` set to an empty bin dir, and every start of a verb that reads the `claude` agents list in the background runs with `PATH` set to one bin dir alone, empty or holding the planted fake `claude`, so no test reads the host's `claude` sessions
