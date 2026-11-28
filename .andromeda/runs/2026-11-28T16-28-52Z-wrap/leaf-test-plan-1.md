@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/testing.md lines=134
last: - 2026-11-25: A case whose start must end before its session
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=50 class=stale
leaf-text: (the fleet console's from its `console.ndjson` lines and its terminal's line mode, never from the screen)
source: dec-i7epgvzm «TUI verdicts come from events.ndjson, the fake-agent receipt and the exit status; the fleet console's verdicts come from its console.ndjson lines, its terminal's line mode and the records of the verbs it runs, never from the screen.»
replace-with: (the fleet console's from its `console.ndjson` lines, its terminal's line mode and the records of the verbs it runs, never from the screen)
@leaf_edit n=2 leaf=.claude/rules/testing.md line=40 class=stale
leaf-text: (the fleet console's from its `console.ndjson` lines and its terminal's line mode, never from the screen)
source: dec-i7epgvzm «TUI verdicts come from events.ndjson, the fake-agent receipt and the exit status; the fleet console's verdicts come from its console.ndjson lines, its terminal's line mode and the records of the verbs it runs, never from the screen.»
replace-with: (the fleet console's from its `console.ndjson` lines, its terminal's line mode and the records of the verbs it runs, never from the screen)
@leaf_edit n=3 leaf=.claude/docs/tests-summary.md line=51 class=omission
leaf-text: which only the gated live reading R5 proves (the fake-agent rehearsal of its script is never a live reading).
source: dec-7rxiy6xv «The fleet console's actions are proven only over the fakes; real focus and real terminals, a real claude's replies and dialogs, the editing line under a real input method and the key line at the operator's»
replace-with: which only the gated live reading R5 proves (the fake-agent rehearsal of its script is never a live reading), nor of the fleet console's real actions (real focus and real terminals, a real claude's replies and dialogs, the editing line under a real input method and the key line at the operator's tile width), which only the gated live reading of v2-15 proves.
