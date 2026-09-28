# CARRY — the macOS harness mutants tests' 73 s cold baseline compile

On the overseer's word (2026-09-28): "No more measurement: finish the chunk. The tests are green with ~38 s margin, and
the lock wait is gone. Record the remaining 73 s cold compile, with your jobserver/RUSTFLAGS hypothesis, in the evidence
as a CARRY to the head-of-queue chunk the :53 wrap will insert. The founder ruled at 17:59 that mutation testing leaves
chunks and CI for the epoch-boundary audit, so that chunk reworks this harness mutants arm anyway."

**Carried to:** the head-of-queue chunk the :53 wrap inserts (the one that reworks the harness mutants arm under the
founder's 17:59 ruling). **Subject:** `viola-e2e` `harness::run::mutants::tests::run_mutants_passes_when_the_change_is_tested`
and `…::run_mutants_reports_survivors_of_an_untested_change` on the `test (macos-latest)` job, whose kill line is 120 s
(`[profile.ci]` 30 s × 4); about 4.7 s each on the Windows host and green on the Linux pre-push leg.

## What was measured (per-phase lines on the runner; the measurement code is removed)
| run · head | test time | baseline Build (`cargo nextest run --no-run`, cargo-mutants' copied tree) | notes |
|---|---|---|---|
| ci#36408767764 · `9b4f6f4` | 101.6 / 101.9 s | — | green, 18 s under the kill |
| ci#36429783467 · `01f22aa` | > 120 s | — | both killed (TIMEOUT), the red folded here |
| ci#36431093491 · `65dd401` | 90.6 / 90.9 s | — | green |
| ci#36435153705 · `1027f87` | ~101.5 / ~102 s | 95.9 / 96.1 s | prebuild 0.2 s, copy 26 KB, mutant builds 1.3–2.1 s |
| ci#36436266196 · `8cc9f14` | 109.9 / 110.2 s | 101.1 / 101.2 s | `--all-logs`: `Blocking waiting for file lock on package cache`, then `Finished … in 1m 36s` |
| ci#36448654074 · `a5f7a67` | 81.6 / 82.0 s | 76.26 / 76.28 s | private `CARGO_HOME`: 0 `Blocking` lines; `Compiling viola … Finished … in 1m 13s` |

**Removed in this chunk:** the package-cache lock wait. The three harness tests that run real cargo give it a private
`CARGO_HOME` beside the throwaway workspace (which has no dependencies, so no registry is needed).

**Remaining:** the cold compile of the one-file throwaway crate takes ~73 s on the macOS runner, while each mutant
rebuild in the same copied tree takes 1–4 s. Both tests' baselines end within 0.02 s of each other, in every measured
run, and ~25 s after the rest of the suite (943 tests) has finished — so a shared cause, not CPU from the other tests.

## Hypothesis (NOT measured — a labelled HYPOTHESIS, not a finding)
The nested cargo inherits from the suite's `cargo llvm-cov nextest` either the outer cargo's jobserver (`CARGO_MAKEFLAGS`
/ `MAKEFLAGS` / `MFLAGS`), whose tokens its rustc then waits on, or the coverage `RUSTFLAGS` /
`CARGO_ENCODED_RUSTFLAGS` / `RUSTC_WRAPPER` / `LLVM_PROFILE_FILE`. The probe proposed and not run: print those variables'
NAMES in the nested cargo's environment and run it with `CARGO_LOG=cargo::core::compiler::job_queue=debug`, to tell a
token wait from compile time; the likely removal is a clean environment (its own jobserver) for the nested cargo in the
test runner only.
