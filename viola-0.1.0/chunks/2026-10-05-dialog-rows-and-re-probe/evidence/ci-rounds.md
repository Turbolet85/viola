# CI rounds after the operator pre-CI commit — 2026-10-05-dialog-rows-and-re-probe

**Authority:** the overseer's decision, founder-delegated (2026-10-05, relayed by the operator): "option 1 now (settle
the screen before the Run C/D kill, as Run B already does), then option 2: measure the hook-count contention on the
runner first (a timing-only push recording per-test durations and the hook-process count, on the coverage leg), and
only then remove the contention … NOT option 3: the bound stays. Show red-before-green where you can, fold every CI
round into this chunk, and read each round with ci.py."

| round | commit | what | `ci.py conclusion` |
|---|---|---|---|
| 0 | `43e6245` | the operator pre-CI commit | **red** · 15/15 · `test (ubuntu-latest)` failed (`operator-pass.md`) |
| 1 | `4179973` | option 1: Run C / Run D settle after their last Stop before the kill (`run-kill-settle.md`, red before green) | green · 15/15 · wall 374 s · ci#37318179233 |
| 2 | `9629757` | option 2, timing-only: one `verify-timing` line per verify (wall ms, hook-process count, `viola` processes alive at its start), shown by a temporary `ci`-profile success-output override | green · 15/15 · wall 369 s · ci#37319370056 |
| 3 | `74e719b` | the measurement reverted (code identical to round 1) | **red** · 15/15 · `test (ubuntu-latest)`: `cli_verify verify_a_complete_set_prints_fourteen_steps_and_stamps_every_row` FAIL at 7.005 s (`viola never exited`, the 7 s bound); no corrupt profile; Windows and macOS green · ci#37320693487 |
| 4 | `c914216` | the verify-driven tests join the `verify_window_` class: no test-side bound on the verify call, a `ci` per-test kill of 20 s for the twelve verify-driven binaries, 45 s for `verify_window_` tests (`verify-window-class.md`; the overseer's reversal of "not option 3", recorded there) | green · 15/15 · wall 363 s · ci#37322552375 attempt 1 |
| 4b | `c914216` | the same commit re-run (`gh run rerun`), the second read the decision asked for | green · 15/15 · wall 486 s · ci#37322552375 attempt 2; ubuntu's slowest verify-driven test 6.74 s (two verifies), under the 20 s kill |

## Round 2's measurement (read from each `test` job's log; 54 / 58 / 54 verify calls)
| leg | verify calls | wall ms median | p90 | max | 29 hooks (dialog replay) median | 17 hooks (no replay) median |
|---|---|---|---|---|---|---|
| ubuntu (coverage) | 54 | 2 582 | 2 968 | 4 534 | 2 592 (n 46) | 3 298 (n 2) |
| windows-2025 | 58 | 3 106 | 3 205 | 3 771 | 3 118 (n 50) | 2 851 (n 2) |
| macos-latest | 54 | 3 384 | 3 772 | 5 205 | 3 440 (n 46) | 3 148 (n 2) |

- **The hook-count contention hypothesis is not supported.** A verify that spawns 17 hook processes is no faster than
  one that spawns 29 (ubuntu 3.3 s vs 2.6 s); the `viola` processes alive at a verify's start do not move its wall
  (0-2 alive: median 2 626 ms, n 7; 3-6 alive: median 2 579 ms, n 47); and the coverage leg is the fastest of the three,
  not the slowest.
- **What a verify costs is fixed work.** Four interactive runs settle seven times on the compiled 300 ms quiet period
  (Run A's start; Run B's start and turn; Run C's start and last Stop; Run D's start and Stop), about 2.1 s before any
  process start-up, by design.
- **The round-0 red, read two-sided across the four ubuntu runs' JUnit** (the 83 tests that drive no verify and took over
  0.2 s before the chunk): their median ratio to the pre-chunk run is 1.00 in every run, but their summed time was
  117.6 s in the red run against 101.8 s in both green rounds (91.0 s pre-chunk): the red run carried a heavier tail. In it
  the four side-by-side `verify_record_refuses_a_dirty_kept_row` cases read 4.2-7.2 s; in round 1 the same cases read
  3.0-3.3 s. Cause, as measured: the verify floor this chunk raised by design (about 1.1 s → 2.6 s on ubuntu) met a slow
  tail in one run, and one case crossed the unchanged 7 s test bound. The worst verify since is 5.2 s (macOS, round 2).
- **Not done, per the decision's order:** no contention was measured, so no nextest group was added (removing a cause
  the measurement does not show would be a change without a basis). The bound stays.

## Round 3, and the cause across all five ubuntu runs (each run's JUnit, the coverage leg)
| run | untouched tests' summed time | their median ratio to pre-chunk | their p90 ratio | `cli_verify` median (common tests) | verdict |
|---|---|---|---|---|---|
| pre-chunk `75198e5` | 91.0 s | 1.00 | 1.00 | 0.99 s | green |
| round 0 `43e6245` | 117.6 s | 1.00 | **2.46** | 2.26 s | red (7.16 s) |
| round 1 `4179973` | 101.8 s | 1.00 | 1.08 | 2.14 s | green |
| round 2 `9629757` | 101.8 s | 1.00 | 1.31 | 2.08 s | green |
| round 3 `74e719b` | 128.3 s | 1.00 | **3.09** | 3.02 s | red (7.005 s) |

("Untouched" = the 83 tests that drive no verify and took over 0.2 s before the chunk.)

- **Cause, as measured:** both reds fall in runs with a runner-wide slow tail (the untouched tests' p90 at 2.5-3x their
  pre-chunk time, the median unmoved), and only there does a verify test cross the 7 s bound. This chunk doubled
  verify's fixed floor by design (seven 300 ms settles over four interactive runs: about 1.0 s → 2.1 s median on ubuntu),
  so a 2.5-3x tail now reaches the unchanged bound, where the pre-chunk floor (about 3 s under the same tail) never did.
  Hook count and concurrency inside the tests do not drive it (round 2).
- **Not done:** no nextest group (no contention measured); the bound stays (the decision). Removing slow work now means
  changing verify's own design (for example running Run C and Run D side by side, or fewer settles), which is beyond
  the decision's words: it goes back to the overseer.
