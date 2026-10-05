# Operator pass — 2026-10-05-dialog-rows-and-re-probe

Driven by the implementer on the operator's word at the `/andromeda-implement` invocation ("Run the operator pass with
the ci.py conclusion read (leg=operator)"), after the block read 17 green, 0 red on the final tree (`pre-push`, entry 20,
included) and the live round read COMPLETE (`round-131207Z.txt`).

## Entry 21 — hygiene (by hand), 2026-10-05
- First read: exit 0, `hygiene: clean — read 50 (runs 41 · evidence 4 · inputs 5)`. Atoms `exit 0` ✓ · `contains hygiene:
  clean` ✓. Re-read after this file was written, before the commit (below).

## The pre-CI commit and entry 22 — the push
- `43e6245` `chore(2026-10-05-dialog-rows-and-re-probe): operator pre-CI commit, for the run this chunk's verdict reads`
  (the whole tree, after hygiene read clean again: `read 51 (runs 41 · evidence 5 · inputs 5)`).
- Entry 22: `git diff --quiet && git diff --cached --quiet && git push origin HEAD` → exit 0,
  `75198e5..43e6245  HEAD -> build/viola-0.1.0`.

## Entry 23 — the CI conclusion: RED
- `ci.py conclusion --sha HEAD --wait 1800` → exit 0: `43e6245ffbce verdict: red · checks 15/15 · first-fail +223 s test
  (ubuntu-latest) · runs ci#37316283001`. Atom `contains verdict: green` ✗. The run then completed: 14 of 15 jobs
  green (Windows and macOS `test` included); `test (ubuntu-latest)` failed.
- **The red, two halves, both in the coverage step (`agent-run.sh run --coverage`):**
  - nextest: `cli_verify verify_record_refuses_a_dirty_kept_row::case_1_home_path` FAIL at 7.161 s, `viola never
    exited` from `tests/support/watch.rs:54` (the test-side bound `WITHIN` = 7 s on one `viola verify`); 1 607 of 1 608
    passed.
  - `llvm-profdata merge`: three `.profraw` files with a corrupt header, then `no profile can be merged`.
- **Measured on the failing runner (the run's JUnit, two-sided against the pre-chunk run ci#37301006304, same leg):**
  verify-driven `cli_verify` tests went 1.11 s → 2.45 s (median); the rstest cases scheduled side by side went 1.11 s →
  4.23-7.16 s (`dirty_kept_row` cases 1-4, `leaves_neither_run_dir_behind` both cases); the ubuntu suite's summed test time
  went 222 s → 358 s. Windows and macOS (no coverage) keep every verify test under 5.3 s.
- **Local, measured (harness build, no coverage):** one verify against the fake agent over the recorded 2.1.287 set takes
  2.11 s: Run A 360 ms, Run B 636 ms, Run C 479 ms, Run D 405 ms; before this chunk only Run A and Run B ran.
- **HYPOTHESIS (not measured):** the coverage slowdown is the hook-process count. One verify now spawns 29 instrumented
  `viola hook` processes, where it spawned 7 before, and each writes and merges a profile under contention. The corrupt
  profiles come from the processes hung up when the test's bound killed `viola verify`, or from Run C / Run D killing the
  fake the instant the last Stop capture appears, while that Stop hook may still be exiting. That second is the hazard
  the fake agent's own docs record.
- **Open:** the red stays open. The fix is not chosen here: raising the test bound is against the testing rule for a
  timing red; the options go to the operator.
