@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/testing.md lines=132
last: - 2026-11-22: A rename of a product constant or a move of a ne
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=26 class=omission
leaf-text: or any send during a running turn (after a `prompt-submitted` of any origin, before `turn-ended`) → exit 13 `turn-running`
source: ctr-iirgwvem «contract/viola-send/turn-running · viola send · turn-running refusal — While a turn runs, or from a switch's request to the new child's session-start, send exits 13 not-delivered with detail turn-running»
replace-with: or any send during a running turn (after a `prompt-submitted` of any origin, before `turn-ended`) or from a switch's request to the new child's `session-start` → exit 13 `turn-running`
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=31 class=omission
leaf-text: transport works, dialog answers withheld (exit 12)
source: dec-kvxmspwy «On an unverified CLI version send still confirms and the question event is logged once with an empty hook stdout, while answer and switch are refused unverified-cli.»
replace-with: transport works, dialog answers withheld (exit 12) and `switch` refused `unverified-cli`
