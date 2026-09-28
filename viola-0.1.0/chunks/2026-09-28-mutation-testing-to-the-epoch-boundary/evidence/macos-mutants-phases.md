# macOS harness mutants phases — the ONE measurement push (plan step 8)

**Subject:** `viola-e2e` `harness::run::mutants::tests::run_mutants_passes_when_the_change_is_tested` and
`…::run_mutants_reports_survivors_of_an_untested_change` on `test (macos-latest)` (kill line 120 s, `[profile.ci]`
30 s × 4, byte-unchanged). **Bound:** at most one measurement-only push (the operator's P5 ruling, 2026-09-28).

## Runs read
| run · head | what | macOS JUnit time (passes / survivors) |
|---|---|---|
| ci#36480299135 · `d5deb01` (operator pre-CI commit) | green 15/15, no measurement | 101.554 s / 101.475 s |
| ci#36481260151 · `a7c1560` (the measurement push) | phase lines, cargo-mutants `-L debug --all-logs`, the in-test nested build | 87.539 s / 87.204 s |

Read through `ci.py conclusion` (entry 26), `gh run download … -n junit-macos-latest` (entry 27) and the job's log
(`gh api …/actions/jobs/<id>/logs`). Nothing downloaded is committed.

## What the phases show (ci#36481260151, macOS; the two tests agree to ~0.03 s)
| phase | passes | survivors |
|---|---|---|
| (b) nested cargo cold build of the throwaway crate's test targets, inherited env, original tree | 0.413 s | 0.407 s |
| (b) first execution of each fresh test binary (`--list`) | 3 ms · 3 ms | 3 ms · 3 ms |
| (b) second execution of the same binary | 2 ms · 3 ms | 3 ms · 2 ms |
| (a) prebuild (`cargo build --package viola`) | 25 ms | 29 ms |
| (a) cargo-mutants baseline **Build** (`cargo nextest run --no-run`, its copied tree) | **80.873 s** | **80.886 s** |
| (a) cargo-mutants baseline Test | 1.079 s | 1.069 s |
| (a) mutant builds, same copied tree (2 each) | 1.759 s · 1.418 s | 1.760 s · 1.350 s |
| (a) mutant tests | 0.794 s · 0.686 s | 0.695 s · 0.548 s |
| the whole cargo-mutants call | 86.916 s | 86.582 s |

- The baseline Build's own cargo output is one `Compiling viola` and `Finished \`test\` profile … in 1m 17s`: no
  `Blocking` / lock line anywhere in the job log (0 hits). cargo-mutants logs `starting jobserver n_tasks=3` before it
  and a reflinked source copy (`reflink_used=true`, 37 files) under the runner's temp dir.
- **(c) inherited names** (names only), identical on both tests: `CARGO_LLVM_COV`, `LLVM_PROFILE_FILE`, `RUSTC_WRAPPER`,
  `__CARGO_LLVM_COV_RUSTC_WRAPPER_RUSTFLAGS`, and nextest's own `NEXTEST_*`. **Absent:** `CARGO_MAKEFLAGS`, `MAKEFLAGS`,
  `MFLAGS`, `RUSTFLAGS`, `CARGO_ENCODED_RUSTFLAGS`, `RUSTC_WORKSPACE_WRAPPER`, `RUSTDOCFLAGS`, `CARGO_BUILD_JOBS`. The
  same names on ubuntu (`NEXTEST_LD_LIBRARY_PATH` in place of macOS's `NEXTEST_DYLD_FALLBACK_LIBRARY_PATH`).

## What that rules out, and what stays unnamed
- **XProtect first-launch scanning: ruled out** — a fresh binary's first launch costs 3 ms, the same as its second.
- **The inherited jobserver: ruled out** — no jobserver variable reaches the nested cargo.
- **The inherited coverage wrapper as such: not the cost** — the 0.41 s build ran through the same `RUSTC_WRAPPER`.
- **The phase that holds the time is named:** cargo-mutants' baseline build in its copied tree (~77 s of cargo time),
  while the same units rebuild in 1.3–1.8 s per mutant in that same tree and cold-build in 0.41 s outside it.
- **The cause inside that phase stays unnamed.** What differs from the fast build is only cargo-mutants' side of it —
  its own jobserver (3 tasks), the reflinked copy under the runner's temp dir, `nextest --no-run` under the `test`
  profile — and the one push reads no finer line inside the 77 s (the baseline cargo prints nothing between its
  `Compiling` and `Finished`). No in-repo change is shown to reach it.

## Arm taken (plan step 9): runner-side / unnamed
Under the operator's P5 ruling ("a cause the measurement does not name … is treated as runner-side"), no second
measurement push and no loop: the two tests carry `#[cfg(not(target_os = "macos"))]` with this file named. Basis of the
ruling: the mutants arm's only consumer is `/andromeda-code-audit` on the Windows host. Both tests keep running on
ubuntu and windows (and on this Windows host in `run --unit`); the 120 s kill and `retries = 0` are unchanged. Never
`#[ignore]` (test-plan §10).

## A red this push raised, and its cause (folded)
`test (ubuntu-latest)` went red on ci#36481260151: `cargo llvm-cov report` could not merge
`target/llvm-cov-target/viola-<pid>-<sig>_1.profraw` ("invalid instrumentation profile data (file header is corrupt)").
**Cause:** the names above show the throwaway crate's nested cargo inherits cargo-llvm-cov's `RUSTC_WRAPPER` and
`LLVM_PROFILE_FILE`, so the throwaway crate (package `viola`, like the product) is built coverage-instrumented and every
execution of its binaries writes a `viola-%p-%m` profile into the OUTER run's profile set. The measurement added direct
executions of those binaries (`--list`, twice each, in both tests at once); one left a corrupt header and the merge
failed. **Folded:** the measurement code, and with it those added executions, is removed; the final HEAD run's green
`test (ubuntu-latest)` is read below as the check, never as the close.
**Carried, not fixed here (a finding for the wrap):** the channel itself predates this chunk — every harness test that
builds and runs the throwaway crate for real (`run_private`'s cargo-mutants, `run.rs`'s `mini` nextest cases) runs it
through the same inherited wrapper, and has been green in every run before this one. Closing it for all of them
(`env_remove` of the four names in the nested cargo, never `env_clear`) touches call sites beyond this chunk's lists, so
it goes to the operator / the wrap as a route item rather than a half-fix in one helper.
