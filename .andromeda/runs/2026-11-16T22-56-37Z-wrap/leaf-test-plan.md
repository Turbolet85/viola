@leaf_group group=test-plan
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/testing.md lines=119
last: - 2026-11-16: A CI red read at the operator pass that is not
@leaf_read leaf=.claude/rules/verification-harness.md lines=73
last: - 2026-11-14: A test-only recorder that replaces every off-l
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=27 class=omission
leaf-text: the Stop array's length or `null`, and an API-error turn end's carries `cause` too
source: ctr-5jhc5ouo «The Stop payload's session_crons array length, or null when the key is absent, null or not an array and on an API-error end; it reaches events.ndjson, CLI wait --json and MCP wait only»
replace-with: the Stop array's length or `null`, and `scheduled_wakes`, the Stop payload's `session_crons` array length or `null` (the key absent, null or not an array, and on an API-error end; it reaches `events.ndjson`, CLI `wait --json` and MCP `wait` only, never `last`, `list`, the snapshot or the replay), and an API-error turn end's carries `cause` too
