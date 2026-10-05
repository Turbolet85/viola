# Operator pass — 2026-10-05-permission-end-to-end

The implementer drove this pass on the operator's word at the `/andromeda-implement` invocation ("Run the operator pass
with the ci.py conclusion read (leg=operator) as usual"). The block had read 16 green, 0 red on the final tree, with
`pre-push` (entry 16) re-run green on that tree just before this pass (39.67 s; coverage 1612/1612, playwright 1/1,
`gate` no breaches). The block has no live round (zero `claude` sessions).

## Entry 17 — hygiene (by hand), 2026-10-05
- First read: exit 0, `hygiene: clean — read 42 (runs 38 · evidence 1 · inputs 3)`. Atoms: `exit 0` ✓,
  `contains hygiene: clean` ✓. Re-read after this file was written, before the commit (below).

## The pre-CI commit and entry 18 — the push
- `c06caf3` `chore(2026-10-05-permission-end-to-end): operator pre-CI commit, for the run this chunk's verdict reads`
  (the whole tree, after hygiene read clean again: `read 43 (runs 38 · evidence 2 · inputs 3)`).
- Entry 18: `git diff --quiet && git diff --cached --quiet && git push origin HEAD` → exit 0,
  `2bd08e9..c06caf3  HEAD -> build/viola-0.1.0`.

## Entry 19 — the CI conclusion: GREEN
- `ci.py conclusion --sha HEAD --wait 1800` → exit 0: `c06caf3e5c6b verdict: green · checks 15/15 · wall 1424 s · runs
  ci#37333536319 completed/success` (polled 47× over 1427 s). Atoms: `exit 0` ✓, `contains verdict: green` ✓.
- The permission cases ran on all three `test` legs: windows-2025 (371 s), macos-latest (325 s), ubuntu-latest
  (1422 s).
- **The wall, read from the run's step times:** the previous run, ci#37328148791 at `2bd08e9`, took 415 s. This run's
  extra ~1000 s is all in ubuntu `test`'s `Chromium (with system deps)` step (1177 s), the runner's Playwright
  system-package install. The suite's own steps kept their size: `Coverage and doctest (sh shim)` 116 s, `Harness
  lifecycle` 8 s, `Browser suite` 3 s, `Secret scan` 62 s. No job queued more than 14 s. Nothing in this chunk touches
  that step (no workflow, Node or Playwright change). It is recorded here for the wrap's CI read, not as a red.
