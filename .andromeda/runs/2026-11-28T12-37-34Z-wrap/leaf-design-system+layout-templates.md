@leaf_group group=design-system+layout-templates
@leaf_read leaf=.claude/docs/design-summary.md lines=73
last: **Full plans:** `.andromeda/design-system.md` + `.andromeda/
@leaf_edit n=1 leaf=.claude/docs/design-summary.md line=49 class=omission
leaf-text: plain ASCII, no colour; frame floor 80x24
source: ctr-zqzjekwi «contract/fleet-console-view/captions · Fleet console view · caption row — The console's caption row is the board's six captions, then a seventh, `LAST`.»
replace-with: plain ASCII, no colour; the fleet drawn as body lines between the title line and the key line, the board's rows in the board's forest order and indent, under the board's six captions then a seventh, `LAST` (the age of the instance's newest log line in one unit, `12s`, `4m`, `2h`, `3d`; `unknown` on a wrapped row with no log line, `n/a` on an unwrapped row); a line wider than the terminal cut whole at its columns ending in `...`, never inside an escape, an unwrapped NAME not cut separately; a fleet taller than the body rows ending in `... <n> more`; the board re-read once a second and redrawn only when the view changes, the claude agents list read on its own thread at most once every 5 s; frame floor 80x24
@leaf_edit n=2 leaf=.claude/docs/design-summary.md line=50 class=stale
leaf-text: `hint: make the viola home readable by you alone, then run viola console again`, no line written)
source: ctr-fh4crnmq «contract/strict-modes-refusal/console · strict-modes refusal · viola console — Exit 1 with the fixed pair unable: viola console could not use its home, then hint: make the viola home readable by you alone»
replace-with: `hint: make the viola home readable by you alone, then run viola console again`, at its start with no line written and the mode left as found, mid-run after the terminal is restored with one process-exit line of detail `strict-modes-failed`; a board directory that cannot be listed ends it as `error: internal error`)
@leaf_edit n=3 leaf=.claude/docs/design-summary.md line=50 class=omission
leaf-text: six fixed columns and words the terminal fleet console (matrix#v2-14) reuses;
source: dec-lz3h724z «The six captions, words and slot order are fixed, and the terminal fleet console reuses them and adds `LAST` after them.»
replace-with: six fixed columns and words the terminal fleet console (matrix#v2-14) reuses, adding `LAST` after them;
