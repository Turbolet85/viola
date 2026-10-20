# Light gate — 2026-10-19-newer-claude-cli-rows (P7.1)

The call: `gate.py run --plan …/plan.md --run-dir {run_dir} --marker … --void 8:…,20:…`, backgrounded. Its trail is
`gate-2026-10-19-newer-claude-cli-rows.json`. The summary, verbatim:
`entries 35 · green 19 · red 2 (24,32) · recorded 0 · timeout 0 · not-run 14 · voided 2 (8,20)`.

The ASSERT, entry by entry:
- **Green by `expect`:** entries 1–7, 21–23, 25–31; entries 8 and 20 green under `--void` (the block cleared, the
  founder's own answers, `inputs#I6`).
- **Entry 24, the preservation guard: red, this chunk's own,** on the one recorded companion
  `tests/contract_fixture_hygiene.rs`. The commit and flip proceed on the overseer's technical direction (`inputs#I8`
  item 2), the guard's text unedited.
- **Entry 32, `pre-push`: `red — not this chunk's`.**
  - Basis: the same coverage-merge red, every test passing, recorded on the tree before this chunk's edits, at the
    switch chunk's implement run and its wrap's light gate (`19.2.log` of `.andromeda/runs/2026-10-19T19-16-36-wrap`).
    Here: `viola-3309919-…_31.profraw: invalid instrumentation profile data (file header is corrupt)`, coverage suite
    2974 passed, failures `llvm-cov-exit-1` and `llvm-cov-summary-missing`.
  - Owner: «Epoch 5 cleanup» CARRY item (8).
  - Authority: `inputs#I10` (the overseer's technical answer inside the founder's ruling «Искать причину»).
  - It recurs the folded WATCH, re-watched on «Epoch 5 cleanup» at origin this chunk, tally 0. The P5 retirement at 3
    green runs is superseded by this recurrence.
- **`leg = 'round'` / `'live'` entries 9–17, not re-run:** re-verified by the round listings in `evidence/`:
  - `round-050522Z.txt`: 9 green, 10 recorded, 11 green;
  - `round-051156Z.txt`: 12 green, 13 green;
  - `round-051400Z.txt`: 14 green, 15 recorded, 16 green.
  - Entry 17 has no tool line (unreachable without leg 18); its loop by hand is recorded empty in
    `evidence/live-round.md`.
- **Entries 18 and 19, the option-2 verify and its census:** not run. Fork 1 is option 1 (`inputs#I6`), and the plan
  names them fired only under option 2.
- **`leg = 'operator'` entries 33–35:** re-verified by `evidence/operator-pass.md` at the final HEAD `4eaca1d3`:
  - hygiene clean;
  - push `3bdcd4a7→4eaca1d3`;
  - `verdict: green · checks 15/15`, ci#39374691224, attempt 1.
