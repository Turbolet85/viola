# P5 controls — 2026-10-13-epoch-4-survivors

Each control fired as the entry through `gate.py run --plan … --entry {n} --project-root {clone}` (no `--run-dir`),
over a scratch clone (`git clone --shared` of the project at HEAD `c6e130181e519777791140332726e1d998de7bd4`, in the
session scratchpad, the plan copied in). Each entry first fired unmodified in the clone and read what it read on the
real tree. Inputs live in the clone only.

| entry | clone input | reading |
|---|---|---|
| 7 (ledger) | unmodified (no `evidence/survivors.ndjson`) | red · exit 1 (FileNotFoundError) — as on the tree |
| 7 | a 47-row ledger keyed on the audit record (16 `owed`, 2 `not-measured-here`, 29 `test`, every `by` set) | green · last line `1` |
| 7 | the same ledger with its last row dropped (46 rows) | red · last line `0` |
| 8 (typed.rs hunks) | unmodified | green · last line `0` — as on the tree |
| 8 | a line inserted after `src/cmd/verify/typed.rs:300` (outside the test module) | red · last line `1` |
| 8 | a line inserted after `src/cmd/verify/typed.rs:700` (inside the test module) | green · last line `0` |
| 9 (preservation) | unmodified | green · exit 0 — as on the tree |
| 9 | a line inserted in `src/main.rs` (not listed) | red · exit 1 |
| 9 | a line inserted in `src/run/wheel.rs` and in `crates/viola-state/src/fs.rs` (both listed) | green · exit 0 |
