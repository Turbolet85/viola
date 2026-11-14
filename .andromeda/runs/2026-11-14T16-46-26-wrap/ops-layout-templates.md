@row_set n=1 row=decision/board-planning-forms-wording cell=rule seen=1 detector=D-design-cli-wording
value: The board's planning forms (bare skip counts, the empty board's three lines) are its wording.
basis: fct-vwne6ov4
because: The board's BAY budget cells and the `budget-paused` WHEEL form are retired, so the rounded figure, the read age and `human - budget-paused` are no longer board wording.
@row_set n=2 row=contract/bay-line/cell-order cell=promise seen=1 detector=D-design-cli-wording
value: `BAY`, then `skipped a - b - c`, separated by the two-space gutter.
basis: fct-xxks7iqj
because: The BAY line is now `BAY  skipped a - b - c`; the `5H`, `7D`, `read` and gate cells are gone.
@row_set n=3 row=contract/cli-exit-codes/11 cell=promise seen=1 detector=none
value: Retired (was `budget-paused`), never reused; no refusal maps to 11.
basis: fct-velgct6n
because: Exit 11 is retired with the budget gate and the masters must state it as retired and never reused.
@row_set n=4 row=contract/cli-exit-codes/14 cell=promise seen=1 fact=F3 detector=none
value: `unknown`; an older wrapper's `budget-paused` reply also reads as `unknown`.
basis: fct-xxks7iqj
because: The CLI exit map reads an older wrapper's `budget-paused` reply as `unknown` (14), which the exit 14 row must carry.
@row_set n=5 row=contract/viola-list-json/document cell=promise seen=1 detector=D-design-cli-wording
value: One JSON document on stdout with no header line, no colour and no glyph; its item and skipped shapes are the list payload contract.
basis: fct-xxks7iqj
because: The `list` payload is now `{items, skipped}`; its `budget` member is gone.
@row_set n=6 row=contract/viola-send-output/unable cell=promise seen=1 detector=D-design-cli-wording
value: `[/ ] unable …` prints on stderr with exit 13 (or 10/12/14) when the wrapper refuses, or exit 13 when the client refuses the text before any frame (not-delivered with control-character or empty-text).
basis: fct-xxks7iqj
because: Nothing maps to exit 11 any more, so a send refusal never exits 11.
@row_retire n=1 row=contract/release-budget-output/gate-lifted seen=1 detector=D-design-cli-wording
why: `viola release --budget` and its budget gate lifted line are retired.
basis: fct-vwne6ov4
because: The release `--budget` flag and its lifted / open lines are removed with the budget gate.
@row_retire n=2 row=contract/release-budget-output/gate-open seen=1 detector=D-design-cli-wording
why: `viola release --budget` and its budget gate open line are retired.
basis: fct-vwne6ov4
because: The release `--budget` flag and its lifted / open lines are removed with the budget gate.
@row_retire n=3 row=contract/viola-list-board/gate-closed seen=1 detector=D-design-cli-wording
why: The board has no budget gate cell, so `budget-paused` never prints on the BAY line.
basis: fct-vwne6ov4
because: The board's BAY budget cells and the `budget-paused` word are retired.
@row_retire n=4 row=contract/viola-list-board/window-expired seen=1 detector=D-design-cli-wording
why: The board shows no budget window figure, so no `expired` form prints.
basis: fct-vwne6ov4
because: The BAY line's `5H` and `7D` cells are retired with the budget gate.
@row_add n=1 row=absence/usage-budget-gate area=surface-cli fact=F1 detector=none
capability: A usage-budget gate: no rate-limits reading, budget file, budget-paused state or refusal, or board budget display.
basis: fct-vwne6ov4
because: The budget gate is removed whole, a whole capability's absence the CLI surface must state.
@row_add n=2 row=contract/viola-release-args/budget area=surface-cli fact=F2 detector=none
boundary: viola release arguments
item: --budget
promise: `viola release --budget` is refused by clap as an unknown argument, exit 2.
basis: fct-tjydxlay
because: The `--budget` flag is removed and now refused, a CLI contract replacing the retired release budget output.
