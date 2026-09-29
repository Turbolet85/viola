# H2 reproduction — the runner reading (plan steps 10, 12, 13)

The count, fixed before any run (plan, operator + overseer at P4, 2026-09-29): **3 localised losses (class R or K)
within at most 3 measurement pushes of 200 iterations**, stopping at the first push that reaches it; a budget that ends
short is *not reproduced* with its rate bound, never re-pushed. Classes per `localisation-rule.md` (written before any
loop). The loop step is `h2-loop-step.md`'s, verbatim.

## Push 1 — reproduction
- **Commit** `d8b5051` (`chore(2026-09-29-h2-conpty-resize-probe): measurement only — H2 loop, 200 iterations on
  windows-2025`), on top of the pre-CI commit `3d04d1c` (ci#36527341834 green 15/15).
- **Run** ci#36527891850 · `ci.py conclusion --sha HEAD --wait 2400` (entry 16, recorded): `d8b5051a2d39 verdict:
  green · checks 15/15 · wall 537 s · runs ci#36527891850 completed/success`.
- **Job** `test (windows-2025)` id 109274838484 · step `H2 loop (measurement only)` success, 05:51:21Z → 05:55:46Z
  (4 m 25 s for 200 iterations). Image `windows-2025-vs2026` 20260922.246.2, provisioner 20260828.587, Microsoft
  Windows Server 2025 — the same image as the ci#36436266196 sighting. The job's own `run --coverage` step passed the
  red test once, in the full suite, before the loop.
- **Tally line (entry 17, `gh run view 36527891850 --log`):** `h2-loop: iterations 200 · losses 13`.
- **Losses:** iterations 12, 14, 41, 59, 85, 120, 129, 132, 146, 158, 163, 180, 185. Every one left both reports and
  every one reads the same shape:

  ```
  --- tests.spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code.report
  start pid={pid} raw=true size=100x30
  byte 78
  size 120x40
  --- tests.spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code.test.report
  resize-returned
  key-written
  key-flushed
  dsr-cpr 0
  ```
  (pids 2136, 8400, 8264, 1924, … — codes only; the thirteen blocks differ in the pid alone.)

| iteration | class | dsr-cpr |
|---|---|---|
| 12 · 14 · 41 · 59 · 85 · 120 · 129 · 132 · 146 · 158 · 163 · 180 · 185 | **K** (×13) | 0 (×13) |

- **Classes:** R 0 · **K 13** · E 0 · UNCLASSIFIED 0. Localised losses (R + K) **13 ≥ 3 → the count is reached at
  push 1**; no further reproduction push.

**Verdict: reproduced 3 (classes K) — 13 localised losses in 200 iterations at push 1, all class K, every one with
`dsr-cpr 0`.** Rate on the runner, isolated loop under llvm-cov: 13 / 200 = 6.5 % of iterations.

## What the localisation says
- The resize reached the child every time: the watcher read `size 120x40` with no key. The overseer's reading ("the
  RESIZE itself never reached the child") is **falsified** on 13 of 13 losses — class R never occurred.
- The key was written and flushed into ConPTY's input (`key-written`, `key-flushed`) and the child, blocked in its
  read, never received it within the 7 s `CHILD_WITHIN`.
- `dsr-cpr 0` on every loss: ConPTY wrote no `ESC [ 6 n` the rig left unanswered (the same reading as this host's
  conhost, `host-localisation.md`). Hypothesis H2-CPR (microsoft/terminal PR #19535) is **not supported** on this
  runner build.

## Branch (plan step 12, rule stated before the reading)
- **Fix branch — not selected:** its one named viola-controlled cause, the rig's unanswered CPR, needs `dsr-cpr ≥ 1`
  on the lost iteration; every loss read 0. No other cause viola controls is shown: the key left viola's writer
  whole and flushed, after `resize` returned, and was lost inside ConPTY / conhost / the child's console read.
- **Document branch — selected:** the loss sits below viola (the rstudio/rstudio#18884 class), and closing it in the
  product would need viola to hold a key behind a resize, which is banned (a11y-plan §1; plan §Constraints).
