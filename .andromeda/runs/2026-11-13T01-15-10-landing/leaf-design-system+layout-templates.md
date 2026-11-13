@leaf_group group=design-system+layout-templates
@leaf_read leaf=.claude/docs/design-summary.md lines=73
last: **Full plans:** `.andromeda/design-system.md` + `.andromeda/la
@leaf_edit n=1 leaf=.claude/docs/design-summary.md line=7 class=omission
leaf-text: (`open`, `RB`, `unable`, `human-typing`, `budget-paused`)
source: dec-nok77hz3 «Short fixed words (`open`, `RB`, `unable`, `human-typing`, `budget-paused`, `not-delivered`) are the vocabulary of every label, CLI column and refusal, with no filler and no guessing.»
replace-with: (`open`, `RB`, `unable`, `human-typing`, `budget-paused`, `not-delivered`)
@leaf_edit n=2 leaf=.claude/docs/design-summary.md line=7 class=stale
leaf-text: for `/clear`, until its new-session post-condition does; for `/compact` on a verified CLI, until its same-session compact post-condition does, and on an unverified CLI `/compact` stays `unconfirmable`
source: ctr-myohn6fm «`[  ] unconfirmable …` prints on stdout with exit 0 for a local command with no measured post-condition: /remote-control, and /compact or /clear on an unverified CLI version.»
replace-with: on a verified CLI, for `/clear` until its new-session post-condition does and for `/compact` until its same-session compact post-condition does; on an unverified CLI both `/clear` and `/compact` stay `unconfirmable`
@leaf_edit n=3 leaf=.claude/docs/design-summary.md line=59 class=omission
leaf-text: - Refusal lines — `unable  <name>  <reason>  <detail>` on stderr, then the cause's `hint:` line last
source: ctr-p2ulheic «The exit-1 refusals of `viola run`, `revive` and `verify`, `viola list`'s exit-21 strict-modes refusal, and the exit-21 strict-modes refusals of `link`, `unlink`, `send`, `wait`, `last`, `answer`»
replace-with: - Refusal lines — `unable  <name>  <reason>  <detail>` on stderr, then the cause's `hint:` line last; the exit-1 refusals of `run`, `revive` and `verify` and every exit-21 strict-modes refusal use the fixed `unable: <text>` / `hint:` pair instead
