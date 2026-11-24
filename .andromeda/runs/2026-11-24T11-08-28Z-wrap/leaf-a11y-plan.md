@leaf_group group=a11y-plan
@leaf_read leaf=.claude/docs/a11y-summary.md lines=58
last: **Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/a11y.md lines=30
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/docs/a11y-summary.md line=44 class=omission
leaf-text: whether `viola run` or a passed `viola revive` started it.
source: ctr-3nzitcr6 «The passthrough is entered by viola run, by a viola revive whose preflight passed and by a viola adopt whose preflight passed.»
replace-with: whether `viola run`, a passed `viola revive` or a passed `viola adopt` started it.
@leaf_edit n=2 leaf=.claude/docs/a11y-summary.md line=47 class=omission
leaf-text: or in a start a passed `viola revive` made, past
source: ctr-3nzitcr6 «The passthrough is entered by viola run, by a viola revive whose preflight passed and by a viola adopt whose preflight passed.»
replace-with: or in a start a passed `viola revive` or a passed `viola adopt` made, past
@leaf_edit n=3 leaf=.claude/docs/a11y-summary.md line=53 class=omission
leaf-text: The one `claude agents --json` read `viola list` and `viola revive` each make
source: dec-2rugwdjd «A failed, oversize, malformed or timed-out claude agents --json read leaves the list board as the board read gave it and passes revive's and adopt's preflight.»
replace-with: The one `claude agents --json` read `viola list`, `viola revive` and `viola adopt` each make
@leaf_edit n=4 leaf=.claude/rules/a11y.md line=19 class=omission
leaf-text: (under `viola revive --json` a `pump_child` error prints no document:
source: ctr-25d7tamc «Where a verb prints error: internal error, --json prints the internal-error document with null detail on stdout instead, stderr empty; run, revive's and adopt's start, mcp and hook stay silent there.»
replace-with: (under `viola revive --json` or `viola adopt --json` a `pump_child` error prints no document:
@leaf_edit n=5 leaf=.claude/rules/a11y.md line=22 class=omission
leaf-text: / `viola run` / a passed `viola revive`, and
source: ctr-3nzitcr6 «The passthrough is entered by viola run, by a viola revive whose preflight passed and by a viola adopt whose preflight passed.»
replace-with: / `viola run` / a passed `viola revive` / a passed `viola adopt`, and
