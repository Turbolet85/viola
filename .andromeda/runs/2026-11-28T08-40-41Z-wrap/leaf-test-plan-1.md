@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/testing.md lines=134
last: - 2026-11-25: A case whose start must end before its session
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=50 class=omission
leaf-text: verdicts come from hook events, `events.ndjson`, channel payloads and the fake-agent receipt
source: dec-i7epgvzm «TUI verdicts come from events.ndjson, the fake-agent receipt and the exit status; the fleet console's verdicts come from its console.ndjson lines and its terminal's line mode, never from the screen.»
replace-with: verdicts come from hook events, `events.ndjson`, channel payloads and the fake-agent receipt (the fleet console's from its `console.ndjson` lines and its terminal's line mode, never from the screen)
@leaf_edit n=2 leaf=.claude/rules/testing.md line=40 class=omission
leaf-text: channel payloads, the fake-agent receipt and exit codes
source: dec-i7epgvzm «TUI verdicts come from events.ndjson, the fake-agent receipt and the exit status; the fleet console's verdicts come from its console.ndjson lines and its terminal's line mode, never from the screen.»
replace-with: channel payloads, the fake-agent receipt and exit codes (the fleet console's from its `console.ndjson` lines and its terminal's line mode, never from the screen)
@leaf_edit n=3 leaf=.claude/rules/testing.md line=41 class=omission
leaf-text: never import the product's own `IDENTITY_FLOOR`;
source: dec-j64chwgj «The relied-row count is read from the ledger's own count constant at every test site, held by one literal pin beside the relied list.»
replace-with: never import the product's own `IDENTITY_FLOOR` (the one exception: the relied-row count is read from the ledger's own count constant at every test site, held by one literal pin beside the relied list);
