@leaf_group group=design-system+layout-templates
@leaf_read leaf=.claude/docs/design-summary.md lines=73
last: **Full plans:** `.andromeda/design-system.md` + `.andromeda/
@leaf_edit n=1 leaf=.claude/docs/design-summary.md line=49 class=stale
leaf-text: takes its own surface section at the route entry «Console scaffold»
source: dec-vvpcu2np «The terminal fleet console, `viola console`, is its own surface in the human's terminal, entered on the alternate screen with the cursor hidden and restored on every end»
replace-with: is its own surface, `viola console`, in the human's terminal: entered on the alternate screen with the cursor hidden and restored on every end, plain ASCII, no colour; frame floor 80x24 (`viola console` on row 1, `q quit` on the last row; below it the one line `viola console needs 80x24; this terminal is <cols>x<rows>`); `q`, Ctrl-C or end of input end it with exit 0, every other key ignored
@leaf_edit n=2 leaf=.claude/docs/design-summary.md line=50 class=omission
leaf-text: `viola up [<NAME>...] [--dry-run] [--json]` (re-opens
source: ctr-pmmvureb «Shows the terminal fleet console in the human's terminal; it takes no argument, the global `--home` only and no `--json`, and exits 0 on quit, interrupt or end of input, 1 on a refusal»
replace-with: `viola console` (shows the terminal fleet console, acting on no strip; no argument, the global `--home` only, no `--json`; exit 0 on quit, interrupt or end of input, 1 on a refusal or an internal error; with stdin or stdout not a terminal, before any home read, `unable: viola console needs a terminal` / `hint: run viola console in a terminal window`; a home failing its read rule `unable: viola console could not use its home` / `hint: make the viola home readable by you alone, then run viola console again`, no line written), `viola up [<NAME>...] [--dry-run] [--json]` (re-opens
@leaf_edit n=3 leaf=.claude/docs/design-summary.md line=59 class=stale
leaf-text: `up` and `verify` and every exit-21 strict-modes refusal
source: ctr-pwvbecet «The exit-1 refusals of `run`, `revive`, `adopt`, `open`, `focus`, `up`, `verify` and `console` and the exit-21 strict-modes refusals of `list`, `link`»
replace-with: `up`, `verify` and `console` and every exit-21 strict-modes refusal
