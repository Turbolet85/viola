# Light gate — 2026-11-15-mutation-and-coverage-baselines-green (wrap 2026-11-15T20-06-27-wrap)

`gate.py run --chunk 2026-11-15-mutation-and-coverage-baselines-green --run-dir .andromeda/runs/2026-11-15T20-06-27-wrap`
(no `--skip`, no `--void`), its summary line as printed:
`entries 20 · green 14 · red 1 (15) · recorded 0 · timeout 0 · not-run 5`

- Entries 1–12, 16, 17 — green by their `expect` (entry 12: the workspace under `--profile mutants`, 70.06 s; entry 17
  `pre-push`, 100.0 s).
- Entry 15 — `red · last line '0' ✗ (last: '6')`. The reading the light gate took, verbatim per the wrap directive
  (inputs#I6 item 1): red — the plan's wording: entry 15 («no product source changes», the file-level count of
  `src` and `crates/*/src/*` reading `last line 0`) against the scope record's 8 widening lines — the six product
  files the fold edited inside `#[cfg(test)]` modules only (`evidence/fold-test-modules.md`: `outside 0` in place,
  `outside 1` with a planted non-test line) · word: "Fold it in now (widening)" — the operator, 2026-11-15, relaying
  the overseer's answer, whose bound 2 names the entry's meaning «no NON-TEST product line changed» (inputs#I4). The
  phase revision door is closed past the pre-CI commit, so the entry stands un-reworded; on its meaning it holds. No
  gate or letter halted on it.
- Entry 13 (leg operator) — re-verified from `evidence/operator-entries.txt`: `recorded` ×2 — 13.log `3285 passed …
  1 failed` (the fold's red), 13.2.log `3286 tests run: 3286 passed (161 slow)` after the fold.
- Entry 14 (leg operator) — `recorded`: case_1_sigint 5.569 s · case_2_sigterm 5.592 s, below the 12 s bound.
- Entry 18 (leg operator, fired as written) — `hygiene: refused 1 files — P1 1` on
  `.andromeda/runs/2026-11-15T18-24-10-phase/p5-card-1.md:14 ×1 · tmp`, the row held on the delegate's word
  (inputs#I5), the card byte for byte (`evidence/operator-pass.md` entry 1).
- Entry 19 (leg operator) — green, `history moved: refs/remotes/origin/build/viola-0.2.0 73b06303→af36e2b4`.
- Entry 20 (leg operator, fired as written) — `af36e2b4c573 verdict: green · checks 7/7 · wall 629 s · runs
  ci#43754700112 completed/success`, the pass's final HEAD (no commit after the pre-CI one).
