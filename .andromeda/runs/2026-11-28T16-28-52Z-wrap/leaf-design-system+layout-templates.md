@leaf_group group=design-system+layout-templates
@leaf_read leaf=.claude/docs/design-summary.md lines=74
last: **Full plans:** `.andromeda/design-system.md` + `.andromeda/la
@leaf_edit n=1 leaf=.claude/docs/design-summary.md line=49 class=stale
leaf-text: `q quit` on the last row
source: dec-2yqgdjfn «decision/console-frame-floor · Console frame floor 80x24 — The console's frame floor is 80x24: at or above it the whole frame is drawn from a cleared top-left, `viola console` on row 1 and the key line on the last row»
replace-with: the key line on the last row
@leaf_edit n=2 leaf=.claude/docs/design-summary.md line=49 class=omission
leaf-text: plain ASCII, no colour;
source: dec-vvpcu2np «except: The editing line of send, switch and open echoes its text as typed through row escaping, each scalar 1, 2 or 0 columns wide.»
replace-with: plain ASCII, no colour, bar the editing line of send, switch and open, which echoes its text as typed (UTF-8 included, controls escaped), each scalar 1, 2 or 0 columns wide;
@leaf_edit n=3 leaf=.claude/docs/design-summary.md line=49 class=stale
leaf-text: `q`, Ctrl-C or end of input end it with exit 0, every other key ignored
source: dec-qe2dfahr «decision/console-actions-run-cli-verbs · Fleet console actions run the CLI verbs — The fleet console's j, k and the arrows move the selection, and l last, p pause, r release, f focus, v revive (`viola up`), s send»
replace-with: `q`, Ctrl-C (from every mode) or end of input end it with exit 0; j, k and the arrows move the selection (marked by its gutter, over wrapped rows only), and l last, p pause, r release, f focus, v revive (`viola up`), s send, m switch, a answer and o open each run on the selected row as its CLI verb in a child, one at a time (release and revive after a `y` confirm; s, m and o through one editing line; a static running line while a child runs, other action keys reading busy; the child's escaped output lines the result rows); every key bound to nothing in its mode ignored
@leaf_edit n=4 leaf=.claude/docs/design-summary.md line=49 class=stale
leaf-text: a fleet taller than the body rows ending in `... <n> more`
source: ctr-b3tds34a «contract/fleet-console-view/more-line · Fleet console view · more line — A fleet taller than the console's body rows, its scroll window not at the fleet's end, ends in a line of the form `... <n> more`.»
replace-with: a fleet taller than the body rows ending in `... <n> more` while its scroll window is not at the fleet's end, and beginning with `... <n> above` once the window is past its top
@leaf_edit n=5 leaf=.claude/docs/design-summary.md line=50 class=stale
leaf-text: acting on no strip
source: dec-molivk3g «decision/list-is-the-board · viola list is the board — `viola list` is the board, `verify` and `plugin install` act on no strip, `console` acts on its selected strip only through the CLI verbs it runs as children»
replace-with: acting on its selected strip only through the CLI verbs it runs as children
@leaf_edit n=6 leaf=.claude/docs/design-summary.md line=34 class=omission
leaf-text: ASCII only, high density.
source: dec-mp7cc53i «decision/cli-ascii-only · ASCII-only CLI output — CLI output is ASCII only, separators included, at high density. · except: the fleet console's editing line echoes the typed text as typed, UTF-8 included, controls escaped»
replace-with: ASCII only, high density, bar the fleet console's editing line, which echoes the typed text as typed, UTF-8 included, controls escaped.
