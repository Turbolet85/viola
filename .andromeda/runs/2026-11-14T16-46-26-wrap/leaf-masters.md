@leaf_group group=masters
@leaf_read leaf=CLAUDE.md lines=133
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=12 class=retired
leaf-text: the `run` pump, wheel and budget governor (`src/run/`)
source: dec-utofgtfh «The root bin holds the subcommand dispatch, the run pump and the wheel, all of which consume only normalised events.»
replace-with: the `run` pump and wheel (`src/run/`)
@leaf_edit n=2 leaf=CLAUDE.md line=24 class=retired
leaf-text: writes the stamps, budget and settings files
source: dec-uum4psu7 «viola-agent-claude takes no viola-state dependency and writes no file; stamps and settings files are written by viola-state and the root bin.»
replace-with: writes the stamps and settings files
@leaf_edit n=3 leaf=CLAUDE.md line=28 class=retired
leaf-text: the `run` pump, the wheel and the budget governor, all consuming only normalised events
source: dec-utofgtfh «The root bin holds the subcommand dispatch, the run pump and the wheel, all of which consume only normalised events.»
replace-with: the `run` pump and the wheel, all consuming only normalised events
