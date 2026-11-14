@row_add n=1 row=absence/usage-budget-gate area=surface-cli fact=F1 detector=none
capability: A usage-budget gate: no rate-limits reading, budget file or thresholds, no usage pause of sends, no `release --budget`
basis: fct-vwne6ov4
because: The budget gate was removed whole, so the absence of the capability is what future work must respect.
@row_add n=2 row=contract/channel-release/budget-key area=surface fact=F2 detector=none
boundary: channel release
item: budget key
promise: A channel `release` carrying any `budget` key (`true`, `false`, a string, `null`) is refused `-32602` `invalid-params` with `data` null, no wheel moved and no record; the `release-from-driver` refusal is checked first.
basis: fct-xxks7iqj
because: The release params lost `budget`, and a carried key is refused rather than skipped so a human-held wheel is never handed to a driver.
@row_set n=1 row=decision/fixed-phraseology-vocabulary cell=rule seen=1 detector=D-design-cli-wording
value: Short fixed words (`open`, `RB`, `unable`, `human-typing`, `not-delivered`) are the vocabulary of every label, CLI column and refusal, with no filler and no guessing.
basis: fct-vwne6ov4
because: The word `budget-paused` was retired with the budget gate, so the vocabulary no longer names it.
@row_set n=2 row=contract/cli-separators/space-hyphen-space cell=promise seen=1 detector=D-design-cli-wording
value: Space, hyphen, space replaces a middle dot inside one field or label (`skipped 2 - 0 - 1`).
basis: fct-xxks7iqj
because: The WHEEL cell has no ` - budget-paused` form any more, so the example must be one that still prints.
@row_set n=3 row=contract/cli-exit-codes/10-14 cell=promise seen=1 detector=D-design-cli-wording
value: An `unable` refusal with its reason; a refusal object under `--json`; 11 is retired (was `budget-paused`) and never reused.
basis: fct-xxks7iqj
because: Nothing maps to exit 11 any more, so the range must say 11 is retired.
@row_set n=4 row=contract/viola-list-table/bay-line cell=promise seen=1 detector=D-design-cli-wording
value: The BAY line carries the skip counts only (`BAY  skipped a - b - c`), with no TAPE cell.
basis: fct-xxks7iqj
because: The BAY line's budget cells are gone and its line is now the skip counts alone.
@row_set n=5 row=decision/no-hint-names-viola-release cell=why seen=1 detector=none
value: The human-typing refusal reaches drivers, and a driver's `release` is refused (`-32602`, `release-from-driver`, exit 20).
basis: fct-vwne6ov4
because: The budget-paused refusal was retired, so the reason names only the refusal that still reaches drivers.
@row_set n=6 row=decision/wheel-column-width-23 cell=rule seen=1 detector=D-design-cli-wording
value: The board's WHEEL column is padded to 23 characters.
basis: fct-tu73g4df
because: The width is kept while its widest word `unknown - budget-paused` is gone, so the rule keeps the width alone.
@row_set n=7 row=contract/cli-exit-codes/11 cell=promise seen=1 fact=F3 detector=D-design-cli-wording
value: Retired (was `budget-paused`) and never reused; a `budget-paused` refusal from an older wrapper reads `unknown`, exit 14.
basis: fct-velgct6n
because: The budget-paused refusal and its exit code were retired, and the masters must state exit 11 as retired and never reused.
@row_retire n=1 row=decision/budget-paused-stops-new-sends seen=1 detector=none
why: viola has no budget governor and no budget-paused state any more.
basis: fct-vwne6ov4
because: The budget gate was removed whole, so a row about its paused state stands true of nothing.
@row_retire n=2 row=decision/readings-carry-their-age seen=1 detector=none
why: The budget reading this rule governed no longer exists.
basis: fct-vwne6ov4
because: The rate-limits reading and the board's budget cells were removed, so the rule has no reading left to govern.
@row_retire n=3 row=decision/expired-window-shown-expired seen=1 detector=D-design-cli-wording
why: No budget window is read or printed any more.
basis: fct-vwne6ov4
because: The budget windows and their board cells were removed with the gate.
@row_retire n=4 row=contract/cli-verb-output/release-budget seen=1 detector=D-design-cli-wording
why: `viola release` takes no `--budget` and prints neither the lifted nor the open line.
basis: fct-vwne6ov4
because: The `--budget` flag and its two output lines were retired.
@row_retire n=5 row=contract/refusal-hints/budget-paused seen=1 detector=D-design-cli-wording
why: The `budget-paused` refusal and its hint were removed.
basis: fct-xxks7iqj
because: No refusal prints the budget-paused hint any more.
@row_retire n=6 row=contract/viola-list-board/human-holder-budget-paused seen=1 detector=D-design-cli-wording
why: The WHEEL cell has no ` - budget-paused` form.
basis: fct-xxks7iqj
because: The board no longer prints a budget-paused holder form.
@row_retire n=7 row=contract/viola-list-board/reading-age seen=1 detector=D-design-cli-wording
why: The board prints no budget reading and so no reading age.
basis: fct-velgct6n
because: The BAY line's `read` cell is gone with the budget cells.
@row_retire n=8 row=contract/viola-list-board/window-figure seen=1 detector=D-design-cli-wording
why: The board prints no budget window figure.
basis: fct-velgct6n
because: The BAY line's `5H` and `7D` cells are gone.
@row_retire n=9 row=contract/viola-release-budget/lifted-line seen=1 detector=D-design-cli-wording
why: `viola release --budget` no longer exists, so the lifted line never prints.
basis: fct-vwne6ov4
because: The `--budget` flag and its lifted line were retired.
@row_retire n=10 row=contract/viola-release-budget/open-line seen=1 detector=D-design-cli-wording
why: `viola release --budget` no longer exists, so the open line never prints.
basis: fct-vwne6ov4
because: The `--budget` flag and its open line were retired.
@row_retire n=11 row=contract/viola-release-budget/override-time seen=1 detector=D-design-cli-wording
why: No release reply carries `override_until` any more.
basis: fct-vwne6ov4
because: The override time was retired with `release --budget`.
@row_retire n=12 row=decision/budget-as-ruled-figures seen=1 detector=none
why: viola prints no budget at all.
basis: fct-vwne6ov4
because: The budget figures and the `budget-paused` word were removed, so the rule governs nothing.
