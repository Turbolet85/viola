@leaf_group group=architecture+design-system+layout-templates
@leaf_read leaf=.claude/rules/frontend.md lines=29
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/rules/frontend.md line=8 class=stale
leaf-text: `viola console` on row 1 and `q quit` on the last row
source: dec-2yqgdjfn «at or above it the whole frame is drawn from a cleared top-left, `viola console` on row 1 and the key line on the last row»
replace-with: `viola console` on row 1 and the key line on the last row
@leaf_edit n=2 leaf=.claude/rules/frontend.md line=8 class=omission
leaf-text: in plain ASCII with no colour, re-reading the board
source: dec-vvpcu2np «except: The editing line of send, switch and open echoes its text as typed through row escaping, each scalar 1, 2 or 0 columns wide.»
replace-with: in plain ASCII with no colour bar the editing line of send, switch and open, which echoes its text as typed (UTF-8 included) through row escaping, each scalar 1, 2 or 0 columns wide, re-reading the board
@leaf_edit n=3 leaf=.claude/rules/frontend.md line=8 class=stale
leaf-text: end it with exit 0, every other key ignored
source: dec-qe2dfahr «The fleet console's j, k and the arrows move the selection, and l last, p pause, r release, f focus, v revive (`viola up`), s send, m switch, a answer and o open each run on the selected row as its CLI»
replace-with: end it with exit 0 from every mode; j, k and the arrows move the selection over wrapped rows, and l last, p pause, r release, f focus, v revive (`viola up`), s send, m switch, a answer and o open each run on the selected row as its CLI verb in a child, one at a time (a static running line standing meanwhile, the verb's own escaped output its result rows), every key bound to nothing in its mode ignored
@leaf_edit n=4 leaf=.claude/rules/frontend.md line=8 class=stale
leaf-text: a fleet taller than the body rows ends in `... <n> more`
source: ctr-b3tds34a «A fleet taller than the console's body rows, its scroll window not at the fleet's end, ends in a line of the form `... <n> more`.»
replace-with: a fleet taller than the body rows, its scroll window not at the fleet's end, ends in `... <n> more` and one scrolled past its top begins with `... <n> above`
