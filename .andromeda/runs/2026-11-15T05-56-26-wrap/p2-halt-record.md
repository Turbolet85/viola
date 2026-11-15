# P2 halt record — 2026-11-14-status-line-interception-removed (for the resume and the closing block, `inputs#I10` item 5)

Stopped at P2 step 3 (Escalate + HALT), the operator away: ONE question, the answer card (`answer-card.md`,
`founder-forks.md`). No masters TX landed; no `rows-land` fired (an escalation is open). Resume at P2 step 3 on his
answer, then step 4 (`rows-land` per file: `ops-{doc}.md`, then `ops-main-{doc}.md`, then `ops-main-test-plan.2.md`).

## Citation sweep
- `cites.py apply` (dry run, then live): `rows: 0 owed a read`, `landed 0 fact(s) · 0 edge(s) · tx 1283`,
  `re-pointed 0 citation(s) in 0 file(s)`; `unresolved 1` (`map`'s, another repository's file). No `cites` batch.
- `cites.py dispositions --write`: `dispositions: 0 row(s)`, `hand re-points: 0 written in 0 file(s)`.

## Batches, each `fanout.py check --manifest …` as called (with `--run-dir`, `--marker`, `--before`, `--after`)
- `task.manifest.json` → `rowtask: 7 agents · 7 pass` · `VERDICT: 7 of 7 pass · failed: none`.
- `rec.py rows-packs` → `rows-packs: 7 master(s) · 4735 row(s) · 52 pack(s) · 2 draw(s) a pack · pin 1283`.
- `pick.manifest.json` → `pick: 104 agents · 104 pass · yes 315 of 4735 (draw 1 273 · draw 2 287 · one draw only 70)`;
  `tests-pick-p12 · [RETYPED] line 96: contract/coverage-failure/llvm-cov-exit-N is no row of the pack — read as
  contract/coverage-failure/llvm-cov-exit, the pack's row 96`; no `[MODE_NAMED]`, no dead draw.
- `rec.py rows-union` → `rows-union: 7 master(s) · picked 315 of 4735 · rule any` (architecture 56 · security-plan 61 ·
  design-system 15 · layout-templates 5 · test-plan 89 · obs-plan 81 · a11y-plan 8; standing 0 each).
- `propose.manifest.json` → `propose: 7 agents · 6 pass · failed SET_IN_READ 1` — `ops-architecture · [SET_IN_READ]
  line 145: retire name/other/hook-statusline — no whole line of arch-rows.md`; re-spawned once (`propose-r1`):
  `propose: 1 agents · 1 pass`. The re-spawn dropped that retire; main raised it (`ops-main-architecture.md`).
- `fit.manifest.json` (wave 1, 9 agents) → `fit: 9 agents · 9 pass`; wave 2 (`--fresh`, 2 agents, after main's own
  files) → `fit: 2 agents · 2 pass`. Every `[PATH_CHECK]` PASS, index and head unchanged.

## Operations offered and how each stands (landing pending the card)
- Offered by the proposers: architecture add 1 · set 12 · retire 27; security-plan add 3 · set 20 · retire 15;
  design-system add 2; layout-templates add 1; test-plan add 7 · set 13 · retire 19; obs-plan add 6 · set 19 ·
  retire 29; a11y-plan add 3 · set 2 · retire 2. Main's own: architecture retire 1; security-plan retire 3; obs-plan
  set 2; test-plan set 2 + set 2 (`.2`); a11y-plan retire 1.
- Rejected at Validate, check 2 (one fact, one home — architecture): 22 — the absence row in six masters, the
  no-shell-out ban (security, test, obs), the unknown-event word (design, test, obs, a11y), the snapshot / stamps
  compat (security, test, obs ×2), a11y's no-settings-layer, and four retires whose heir was so rejected (re-raised
  heir-less by main).
- Check 7: `not_master` 1 (obs `decision/log-schema-additive-only`, type 8; escalated Q2, not asked); `stands` 3
  (obs `contract/replayed-snapshot/endpoint-pid-started-at` as `…/live-fields`; test-plan
  `decision/plugin-dir-rewritten-on-start` as `decision/plugin-rewritten-on-start`;
  `decision/recorder-scrubs-paths-and-usernames` as `decision/recorder-scrub`, also
  `decision/hook-payloads-recorded-by-verify`) — each rejected with its retire, main setting the standing row instead.
  Every other add and text set: `fits`.
- Escalated: 1 (Q1, the card). `rows-check` refused no cell bound; main read `master-rows.md` §The sheets before its own
  operations. `ops-main-test-plan.2.md`'s `rows-check` reads `stale` (seen r2 / r3) until `ops-main-test-plan.md`
  lands — the letter's `.2` form.

## Status-line rows that stay, and why
- architecture `decision/global-statusline-untouched` — true and stronger (viola adds no settings layer at all).
- obs-plan `decision/child-settings-never-logged` — verify's Runs C and D still pass `--settings` to their children.
- obs-plan `contract/replayed-snapshot/live-fields` — its `item` cell, a key cell never set, still lists
  `statusline_command`; the promise is set (fit read the retire-and-add heir as standing under it). A letter
  conflict: `stands` → set the row, while a key cell is changed only by retire and add.
- test-plan `decision/ci-stamped-home-kill-line` — the kill line stands; its binary list is the code's.

## Refusals and non-zero exits this window
- `fanout.py check` without `--run-dir` → `--run-dir: no path given`, exit 2 (re-issued with it).
- The Bash guard refused a `cd` into the run dir (×4, one message; the files read with Read) and a `rec.py get` piped
  into a filter (re-issued bare).
- `rec.py query` with a select alias named in the projection → `Binder Error`, exit 2 (re-issued).
- `ops-main-test-plan.2.md` `rows-check` → exit 3, `stale` (expected, above).
