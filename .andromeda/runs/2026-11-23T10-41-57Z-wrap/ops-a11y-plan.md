@row_set n=1 row=contract/cli-usage-error/exit-2 cell=promise seen=1 fact=F1 detector=none
value: A clap usage error exits 2; `viola plugin` alone is such an error, and so is a launch pair written among the caller's words after `--` in `viola run` or `viola revive`, whose one fixed stderr line names `--model <MODEL>` and `--effort <EFFORT>` and echoes none of the caller's words.
basis: fct-ydz7awsz, fct-dgfm4vwr
because: The chunk adds a usage refusal of a pair after the dashes in run and revive, exit 2 with one fixed line echoing none of the caller's words, so the exit-2 contract must name it.
@row_set n=2 row=contract/viola-list-board/order cell=promise seen=2 fact=F3 detector=none
value: The board is the BAY context line, the uppercase caption row and the wrapped rows drawn as a tree, then the unwrapped read-only rows behind their separator, then the links behind their `-- LINKS --` separator; the board shows no launch pair, which only `list --json` and the MCP `list` item carry.
basis: fct-xb6c4igy, fct-aahvlfhn, fct-objkygma
because: The launch pair now crosses into the list --json and MCP list item as optional model and effort while the six-column human board stays unchanged, so the board's structure row must say the pair is not on it.
@row_add n=1 row=decision/default-pair-never-refuses-start area=cognitive-accessibility fact=F2 detector=none
name: Default launch pair never refuses a start
rule: The config.json default_model and default_effort keys never refuse a start: a missing or malformed value takes that half of the compiled default pair, and no value of either key disables a control.
guard: setting_the_default_pair_does_not_disable_a_control
basis: fct-xb6c4igy, fct-e3akdikc
because: The chunk adds two config.json keys whose bad values fall back to the compiled default rather than refusing, a rule future settings must keep so no setting stops a start or disables a control.
