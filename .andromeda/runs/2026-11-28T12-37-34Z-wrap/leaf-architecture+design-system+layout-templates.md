@leaf_group group=architecture+design-system+layout-templates
@leaf_read leaf=.claude/rules/frontend.md lines=29
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/rules/frontend.md line=20 class=omission
leaf-text: and the terminal fleet console (matrix#v2-14) reuses them;
source: dec-lz3h724z «decision/shared-captions-fixed · Shared captions fixed — The six captions, words and slot order are fixed, and the terminal fleet console reuses them and adds `LAST` after them.»
replace-with: and the terminal fleet console (matrix#v2-14) reuses them and adds a seventh, `LAST`, after them (the age of the instance's newest log line in one unit, `12s` · `4m` · `2h` · `3d`; `unknown` on a wrapped row with no log line, `n/a` on an unwrapped row);
@leaf_edit n=2 leaf=.claude/rules/frontend.md line=8 class=omission
leaf-text: every other key ignored.
source: dec-oy45f7bh «decision/console-fleet-body-lines · Console fleet drawn as body lines — The terminal fleet console draws the fleet as body lines between its title line and its key line, the board's rows in the board's forest order and indent.»
replace-with: every other key ignored; it draws the fleet as body lines between its title line and its key line, the board's rows in the board's forest order and indent.
@leaf_edit n=3 leaf=.claude/rules/frontend.md line=8 class=omission
leaf-text: in plain ASCII with no colour;
source: dec-lda2y5rr «decision/console-redraw-on-change · Console re-read and redraw on a change — The terminal fleet console re-reads the board once a second and redraws only when its view changes, with no redraw while the terminal size and»
replace-with: in plain ASCII with no colour, re-reading the board once a second and redrawing only when its view changes;
@leaf_edit n=4 leaf=.claude/rules/frontend.md line=8 class=omission
leaf-text: an unread size counting as 0x0)
source: ctr-kjcaytca «contract/fleet-console-view/line-cut · Fleet console view · line cut — A console line wider than the terminal is cut whole at its columns ending in `...`, never inside an escape; an unwrapped NAME is not cut separately»
replace-with: an unread size counting as 0x0; a line wider than the terminal is cut whole at its columns ending in `...`, never inside an escape, an unwrapped NAME not cut separately, and a fleet taller than the body rows ends in `... <n> more`)
@leaf_edit n=5 leaf=.claude/rules/frontend.md line=26 class=omission
leaf-text: nothing is stripped.
source: dec-mxgvkhis «decision/escape-controls-in-human-output · Control characters escaped in human output — Human output of `last`, `list` and the fleet console escapes C0/C1 control characters.»
replace-with: the fleet console's human output escapes C0/C1 control characters too; nothing is stripped.
