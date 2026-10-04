# Red B — the mutants harness build timeout (ci#37196414168, `msrv`)

## The CI reading
- Job 111419122898, step "Unit on 1.96": `run_mutants_passes_when_the_change_is_tested` FAIL at `mutants.rs:425:9`
  (`survived` 1, not 0). cargo-mutants printed `Unmutated baseline in 0s build + 0s test`,
  `Auto-set build timeout to 1s`, `caught … replace three -> u32 with 0 in 0s build + 0s test` and
  `TIMEOUT … replace three -> u32 with 1 in 1s build`: the second mutant timed out in its BUILD.

## The mechanism (cargo-mutants 27.1.0 source, local registry copy)
- `timeouts.rs:80-95` `build_timeout`: `baseline × build_timeout_multiplier`, no lower bound (the test timeout alone
  takes `max(minimum_test_timeout = 20 s, …)`). `Auto-set build timeout to {}s` prints `as_secs()`, truncated.
- `main.rs:233-238`: `--build-timeout-multiplier` `conflicts_with = "build_timeout"` — the two cannot ride together.

## The reproduction (this host, Linux, 2026-10-04)
- Command: `RUSTUP_TOOLCHAIN=1.96 TMPDIR=<repo parent>/viola-mutants-scratch/rb CARGO_TARGET_DIR=target/msrv-rb cargo
  nextest run -p viola-e2e --lib -E 'test(/run_mutants_passes_when_the_change_is_tested|run_mutants_reports_survivors_of_an_untested_change/)' --no-capture`
  (fresh short `TMPDIR` under the NOCOW scratch), at `c540254` plus this chunk's red A edit only.
- `run_mutants_passes_when_the_change_is_tested`: `ok Unmutated baseline in 0s build + 0s test` ·
  `INFO Auto-set build timeout to 0s` · `INFO Auto-set test timeout to 20s` · `caught … with 0 in 0s build + 0s test`
  · `caught … with 1 in 0s build + 0s test` → PASS.
- `run_mutants_reports_survivors_of_an_untested_change`: the same baseline and `Auto-set build timeout to 0s`, both
  mutants `MISSED` → PASS.
- **Verdict: the red is not reproduced here** — this host's mutant rebuilds finished inside the sub-second bound. The
  mechanism IS reproduced: the build timeout was auto-set to under one second (printed `0s`) from a sub-second
  baseline, with no floor. The CI run's own log stays the witness of the failing timing.

## The fix and its basis
- `MUTANTS_PROGRESS` (`crates/viola-e2e/src/harness/run/mutants.rs`) carries `--build-timeout=400` in place of
  `--build-timeout-multiplier=5`. The counting rule (`survived = missed + timeout`) and its unit pin
  `mutants_suite_counts_missed_and_timeout_as_survivors` are untouched.
- N = 400 s ≥ 5 × the largest `Unmutated baseline in <s> build` measured:
  - `target/run-archive/*` (196–205): holds no `outcomes.json` and no cargo-mutants log — 0 baselines;
  - this chunk's reproduction: 0 s;
  - the committed evidence of earlier mutation runs on the real workspace (read because the archive holds none —
    a lower basis would time out the audit's own mutant builds): largest `ok Unmutated baseline in 78s build + 38s
    test` (chunk 2026-09-24-epoch-1-cleanup, `evidence/operator-pass-3.md:27`, which cargo-mutants auto-set to 390 s);
    others read 8–71 s.
  - 5 × 78 = 390 → 400.
