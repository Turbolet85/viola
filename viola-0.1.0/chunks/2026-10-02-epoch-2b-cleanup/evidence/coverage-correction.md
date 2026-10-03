# Coverage re-take (step 16) — correction to the Epoch 2b code audit

Corrects `.andromeda/runs/2026-10-01T09-18-50-code-audit/`, whose `coverage.line` reads "not measured" (the audit's run 2
failed 80 of 985 in-tree). Floors are test-plan §10: lines 85 · functions 95 · regions 80. The `COVERAGE_IGNORE` regex is
unchanged. Both readings use this chunk's tree. The C: copy's seven code files are byte-equal to the repository's.

| tree | command | tests | lines | functions | regions | `gate --require coverage,doctest` |
|---|---|---|---|---|---|---|
| repository (D:) — gate entries 13–14 | `bash scripts/agent-run.sh run --coverage` | 905 passed, **89 failed** | 94.41 % (2 348 / 2 487) | **94.33 %** (283 / 300) | 94.17 % (4 295 / 4 561) | `ok:false`: `suite-failed` (89), `functions 94.33 < 95` |
| C: copy, same day | the same two commands | **994 passed, 0 failed** | **97.59 %** (2 427 / 2 487) | **97.67 %** (293 / 300) | **97.50 %** (4 447 / 4 561) | **`ok:true`**, no breach |

**Reading.** The correction is the C: row: the Epoch 2b boundary's Windows coverage is **lines 97.59 · functions 97.67 ·
regions 97.50**, above every floor (the Epoch 2 reading was lines 96.77). The D: row is M2's red, not a coverage
shortfall. Its 89 failures are `watch.rs` deadlines and the spine bound (`m2-diagnosis.md`). Tests that end at a deadline
never run the code past it, and that is the 10 functions between the two rows. The plan's acceptance ("read `"ok":true` in
the repository tree") is therefore **not met in-repo**. It waits on M2 (the founder's decision), and gate entries 13 and 14
are recorded red with that basis.
