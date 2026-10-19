- line: .andromeda/security-plan.md:205
old: «no value of either key switches the gate off. `port` goes into `u16`.»
new: «no value of either key switches the gate off. The dialog hold is the top-level key `dialog_hold_s`, read once at the wrapper's start through `run::dialog::read_hold` right after the thresholds: a JSON whole number of seconds that fits `DialogHold` (60–3600, `DialogHold::try_new`; default 60, `DIALOG_HOLD_DEFAULT`), else the hold takes 60 and is reported as one `parse-rejected{parser:"config-json", detail:"malformed"}` line; a document that is no object, or whose `v` is not 1, gives 60 and no rejection from this read; no value makes the hold zero or unbounded, and no env var or flag sets it (a boundary widening the founder answered himself, chunk 2026-10-19-dialog-hold-for-the-driver, `inputs#I2` Q1). `port` goes into `u16`.»
- line: .andromeda/security-plan.md:205
old: «the threshold read in the root bin `src/run/budget.rs`, each its own `take(MAX_FRAME)` read; `Percent` is `viola-core`'s)»
new: «the threshold read in the root bin `src/run/budget.rs`, the hold read in the root bin `src/run/dialog.rs`, each its own `take(MAX_FRAME)` read; `Percent` and `DialogHold` are `viola-core`'s)»
- line: .andromeda/security-plan.md:202
old: «the reply read is bounded at `DIALOG_DEADLINE + 5 s`»
new: «the reply read is bounded at `DIALOG_HOLD_MAX + 5 s` (3605 s: the ceiling, the wrapper replying `null` at its own configured hold's end)»
