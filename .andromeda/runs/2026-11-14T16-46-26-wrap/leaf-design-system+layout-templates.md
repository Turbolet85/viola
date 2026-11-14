@leaf_group group=design-system+layout-templates
@leaf_read leaf=.claude/docs/design-summary.md lines=73
last: **Full plans:** `.andromeda/design-system.md` + `.andromeda/la
@leaf_edit n=1 leaf=.claude/docs/design-summary.md line=7 class=stale
leaf-text: (`open`, `RB`, `unable`, `human-typing`, `budget-paused`, `not-delivered`)
source: dec-nok77hz3 «Fixed phraseology vocabulary — Short fixed words (`open`, `RB`, `unable`, `human-typing`, `not-delivered`) are the vocabulary of every label, CLI column and refusal, with no filler and no guessing.»
replace-with: (`open`, `RB`, `unable`, `human-typing`, `not-delivered`)
@leaf_edit n=2 leaf=.claude/docs/design-summary.md line=7 class=retired
leaf-text: Missing readings print `unknown`, and old information carries its age (`read 4m ago`, `expired`).
source: dec-npjmkubc «Readings carry their age — Every budget reading carries its age (`read 4m ago`), and old information is shown as old (`expired`), never as current.»
replace-with: Missing readings print `unknown`.
@leaf_edit n=3 leaf=.claude/docs/design-summary.md line=58 class=stale
leaf-text: atop `viola list`: `BAY`, `5H`, `7D`, `read … ago`, gate, `skipped` (no TAPE cell)
source: ctr-pvokfxot «viola list table · BAY line — The BAY line carries the skip counts only (`BAY  skipped a - b - c`), with no TAPE cell.»
replace-with: atop `viola list`: `BAY`, then `skipped a - b - c` (the skip counts only, no TAPE cell)
@leaf_edit n=4 leaf=.claude/docs/design-summary.md line=65 class=retired
leaf-text: NEVER use status dots, toasts, spinners, skeleton shimmer or budget rings.
source: dec-n2x3flqz «Budget as figures in ruled fields — Budget prints as figures in ruled fields with the ATIS reading age, never as rings or coloured bars; `budget-paused` is a word, not a colour.»
replace-with: NEVER use status dots, toasts, spinners or skeleton shimmer.
