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

## The document branch, as done
- **Measured limit recorded** in `.claude/docs/gotchas.md` ("A key written right after a ConPTY resize can be lost
  (H2)") and `.claude/docs/services/viola-pty.md` (Crate-specific gotchas): count, class, `dsr-cpr` readings, builds,
  run ids. The gotchas entry states the **product window remains** — in `viola run` a human key typed right after a
  resize can still be lost — names the measured rate (13 / 200, 6.5 %, from push 1 above), and says the reshaped
  test does not cover that window (operator ruling, P5 review 2026-09-29).
- **Red test reshaped** (`crates/viola-pty/src/lib.rs`): `resize` then `wait_line("size 120x40")` then the key — the
  observed signal the localisation shows (class K: the resize landed, the key racing it was lost), never a timer. No
  retry, `#[ignore]`, skip or compile-out. The `hold` sibling still writes its key right after the resize (the H1
  witness, unchanged).

## Push 2 — verification of the reshaped test (not a reproduction push)
- **Commit** `dce98ad` (`fix(…): the H2 red test sends its key once the child sees the new size, and the loop measures
  it (measurement only)`), the loop step still in ci.yml. Measurement pushes in all: **2 of the 3** allowed.
- **Run** ci#36529038462 · `ci.py conclusion --sha HEAD --wait 2400` (entry 16, recorded): `dce98ad16123 verdict: red ·
  checks 15/15 · first-fail +131 s test (ubuntu-latest)` — the ubuntu red is folded below; it is not an H2 reading.
- **Job** `test (windows-2025)` id 109278323590 · success · step `H2 loop (measurement only)` 06:05:22Z → 06:07:55Z.
  Same image, `windows-2025-vs2026` 20260922.246.2, provisioner 20260828.587.
- **Tally (entry 17, the job log):** `h2-loop: iterations 200 · losses 0`. R 0 · K 0 · E 0 · UNCLASSIFIED 0.
- **Witness:** the forced reading red (push 1: 13 K in 200, the key written right after `resize` returned), then green
  200/200 (push 2: the key written after the child's `size 120x40`), same runner image, same loop, same instrumentation.
  Control: push 1 is the unchanged test under the identical loop.

## A red push 2 raised: `test (ubuntu-latest)`, corrupt coverage profile (folded)
- ci#36529038462 job 109278323561: all 919 tests PASS (`Summary … 919 tests run: 919 passed`), then `llvm-profdata merge`
  failed on `target/llvm-cov-target/viola-4974-15303557482808059277_2.profraw` ("invalid instrumentation profile data
  (file header is corrupt)") → `run --coverage` `llvm-cov-exit-1` · `llvm-cov-summary-missing`. The same shape as
  ci#36481260151 (chunk `2026-09-28-mutation-testing-to-the-epoch-boundary`, `evidence/macos-mutants-phases.md`).
- **The channel:** the harness self-tests' nested cargo over the throwaway `viola` crate inherits cargo-llvm-cov's
  `CARGO_LLVM_COV`, `LLVM_PROFILE_FILE`, `RUSTC_WRAPPER`, `__CARGO_LLVM_COV_RUSTC_WRAPPER_RUSTFLAGS`, so the throwaway's
  binaries are built instrumented and write into the outer run's profile set. Those nested runs are the suite's by-design
  mid-flight terminations (cargo-mutants, a fail-fast nextest profile).
- **Measured on this host (known positive):** `cargo llvm-cov nextest --no-report --profile ci -p viola-e2e -E
  'test(=harness::run::tests::run_reports_unit_integration_and_doctest_suites)'`, counting the new `.profraw` files per
  run: with the channel open, **17** (two runs), including 2 signatures no other run shows (`2995…` ×1, `3916…` ×3 —
  the throwaway crate's binaries); with the channel closed, **13** (three runs), those two signatures absent. The
  corrupt file's own signature is not attributable from the job log (every instrumented process writes `viola-%p-%m`),
  so that THIS file came through the channel is an INFERENCE, carried as such: the channel is the one measured path
  that puts binaries outside the suite into the set, and it is now closed. A recurrence on a later HEAD would place
  the cause elsewhere.
- **Folded** (operator's word at the implement invocation, "Fold every red into this chunk.", scope-record.md):
  `crates/viola-e2e/src/harness/run.rs` `test_support::{uninstrumented, run}` remove the four names from every nested
  tool command a self-test runs for real, and `run/mutants.rs`'s `run_private` and its `run` go through it. Test code
  only; the harness's own `run --coverage` is unchanged. The final HEAD run is the check, never the close.

## Founder hand-off (plan step 13)
- **Count:** reproduced — 13 localised losses in 200 iterations at the first measurement push (the fixed count was 3
  within at most 3 pushes of 200). Rate 6.5 % of isolated loop iterations on the windows-2025 runner under llvm-cov.
- **Classes:** K 13 · R 0 · E 0. The resize always reached the child; the key, written and flushed right after the
  resize returned, was lost below viola. `dsr-cpr 0` on every loss (runner) and on the host: no unanswered
  cursor-position request.
- **Branch taken:** document. The limit is recorded in gotchas.md and services/viola-pty.md; the red test now sends
  its key after the child sees the new size (200/200 on the runner). The product window — a human key typed right after
  a resize in `viola run` — remains, and no test covers it.
- **The founder's product question, in the founder's words, unanswered:** "the founder's product question on H2 stays
  open beside this probe".
