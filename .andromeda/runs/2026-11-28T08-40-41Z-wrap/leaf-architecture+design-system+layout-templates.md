@leaf_group group=architecture+design-system+layout-templates
@leaf_read leaf=.claude/rules/frontend.md lines=29
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/rules/frontend.md line=8 class=stale
leaf-text: Path-scoped rules for viola's one built design surface, human CLI output (cli);
source: dec-vvpcu2np «decision/console-own-surface · Terminal fleet console as its own surface — The terminal fleet console, `viola console`, is its own surface in the human's terminal»
replace-with: Path-scoped rules for viola's built design surfaces, human CLI output (cli) and the terminal fleet console (console);
@leaf_edit n=2 leaf=.claude/rules/frontend.md line=8 class=stale
leaf-text: the keyboard-driven terminal fleet console tiled on Hyprland (matrix#v2-14) takes its own surface section at the route entry «Console scaffold».
source: dec-vvpcu2np «decision/console-own-surface · Terminal fleet console as its own surface — The terminal fleet console, `viola console`, is its own surface in the human's terminal, entered on the alternate screen with the cursor hidden»
replace-with: the keyboard-driven terminal fleet console tiled on Hyprland (matrix#v2-14), `viola console`, is its own surface in the human's terminal, entered on the alternate screen with the cursor hidden and restored on every end, in plain ASCII with no colour; its frame floor is 80x24 (at or above it the whole frame from a cleared top-left, `viola console` on row 1 and `q quit` on the last row; below it one line `viola console needs 80x24; this terminal is <cols>x<rows>`, an unread size counting as 0x0), and `q`, Ctrl-C or the end of its input end it with exit 0, every other key ignored.
@leaf_edit n=3 leaf=.claude/rules/frontend.md line=25 class=stale
leaf-text: (`board:` holding `list, focus`, `handoff:` holding
source: ctr-pbkoayxj «contract/help-output/verb-groups · viola --help output · verb groups — `viola --help` lists the served verbs as `board: list, focus, console` / `traffic: send, wait, last, answer`»
replace-with: (`board:` holding `list, focus, console`, `handoff:` holding
@leaf_edit n=4 leaf=.claude/rules/frontend.md line=25 class=omission
leaf-text: (the exit-1 refusals of `run` / `revive` / `adopt` / `open` / `focus` / `up` / `verify`,
source: ctr-pwvbecet «contract/cli-refusal-line/fixed-unable-form · CLI refusal line · fixed unable form — The exit-1 refusals of `run`, `revive`, `adopt`, `open`, `focus`, `up`, `verify` and `console`»
replace-with: (the exit-1 refusals of `run` / `revive` / `adopt` / `open` / `focus` / `up` / `verify` / `console`,
