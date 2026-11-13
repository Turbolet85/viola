@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, s
@leaf_read leaf=.claude/rules/testing.md lines=109
last: - 2026-11-06: A sweep for a `cfg` predicate reads the predic
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=21 class=omission
leaf-text: (email and row-seam usernames refused too)
source: dec-dm5fgnwe «A recorded screen fixture keeps a box rule row as its rule chars alone and the box's prompt row under that rule as the prompt char alone, and the fixture hygiene walk refuses a prompt row with text after»
replace-with: (email and row-seam usernames refused too, and a box prompt row with text after the prompt char or a box rule row with any other text: a box row is recorded as its literal alone, `planted_box_row_beyond_its_literal_is_rejected`)
@leaf_edit n=2 leaf=.claude/rules/testing.md line=35 class=omission
leaf-text: every input embeds the tests-owned canary string.
source: dec-dm5fgnwe «A recorded screen fixture keeps a box rule row as its rule chars alone and the box's prompt row under that rule as the prompt char alone, and the fixture hygiene walk refuses a prompt row with text after»
replace-with: every input embeds the tests-owned canary string; a recorded screen fixture keeps a box rule row as its rule chars alone and the box's prompt row under it as the prompt char alone, and the hygiene walk refuses a prompt row with text after the prompt char or a rule row with any other text (`planted_box_row_beyond_its_literal_is_rejected`).
