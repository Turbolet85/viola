# tests · Quality Gates & Coverage Targets — category read

## Carried
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) Every `viola` child the new exit-cause table and the moved test files spawn must stay in the coverage run, so a case that clears the environment re-adds the profile variable.
  > Child `viola` processes spawned by E2E tests are included, because `LLVM_PROFILE_FILE` propagates. Tests that call `env_clear()` must re-add it.
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) A table case that kills an instrumented child straight after its start (a refused or mid-call cause) must not leave a short profile that fails the coverage merge.
  > an instrumented child that exits by itself and is killed during its exit-time profile write leaves a short profile, which `llvm-profdata` refuses.
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:9) The endpoint-lost-mid-call cause is kill-shaped like the named chaos case, so its case must keep to the fake agent's process-group discipline and not bring the page-cut profile back.
  > A second mechanism is measured: a test that kills the wrapper closes its PTY, and the hangup reaches the hooks the fake agent ran in that terminal's foreground process group, so a `viola hook` cut off in its exit-time profile write leaves a page-cut profile that `llvm-profdata` refuses, or none; with every command the fake agent spawns in a process group of its own (the fake agent's exit bullet, §7) the census over the kill-shaped `chaos_wait_parked_wrapper_killed_exits_21` reads 0 refused profiles of 800 in 200 runs (as measured at `viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup/evidence/profraw-cause.md`).
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:9) A profile census over a new kill-shaped case is a dev-host reading through this script's `--case` form, and the chunk must not present it as a CI or harness gate.
  > The witness is `scripts/profraw-census.sh <runs> <workers> [<name>]`, with a second form `--case <binary stem> <test name> <runs> <workers> [<name>]` over one integration case that offers each profile alone to `llvm-profdata merge` and counts the refused ones, a dev-host script (architecture §Occupied Resources → Repository): no CI job, `pre-push` stage or harness command runs it, and it has no unit test, only its recorded one-shot controls (must-fail on the untouched test, must-pass, and two planted readings).
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:15) The chunk's acceptance must name no mutation gate of its own: its new code is scored by the epoch-boundary audit or an on-demand run.
  > **Mutation gate (Founder Direction 1):** judged at the epoch boundary by `/andromeda-code-audit` and on demand by `run --mutants`; no chunk, pre-push or CI mutation gate exists since 2026-09-28 (the founder's ruling, 17:59).
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:15) New code-word and cause-mapping logic must be held by a nextest case, because a doctest alone leaves its mutant missed at the audit that scores this code.
  > A mutant caught only by a doctest counts as missed under nextest, so the missing unit test must be added.
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:26) A code or log form the chunk adds to the shared server check or strict-modes path must leave the hook's timed path under the spine deadline, with the checks' order kept.
  > Every timed hook also runs its instance strict-modes check (`check_instance`) before any `config.json` read or obs init, and makes its server check (the peer pid and its OS start time against the one snapshot read) before its first frame; with both checks in the timed path the five hook rows read `max` 2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms on the Linux dev host, under `SPINE_DEADLINE`, as measured at `viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md` (Outcome, `run --perf`).
- (.andromeda/test-plan.md:817) The chunk's new and moved code across the root bin and the crates must leave these thresholds met on every OS job.
  > | **Comprehensive (viola)** | **≥ 85%** | **≥ 80%** (enforced as region coverage `--fail-under-regions 80`) | **≥ 95%** |
- (.andromeda/test-plan.md:820) Each OS job must clear the gate alone, so a cause reached only on one OS, such as a `cfg(windows)` or `cfg(unix)` arm of a code or hint, needs a case on that OS's runner.
  > - Gates are enforced per OS job with `cargo llvm-cov nextest … --fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80`. Per-OS gating is deliberate: `#[cfg(windows)]` / `#[cfg(unix)]` code compiles only on its own runner.
  > - Coverage rolls up across all workspace crates.
- (.andromeda/test-plan.md:826) A doctest on a new code or hint function must not be counted toward the coverage reading.
  > - Doctests are run but are not part of the coverage number.
- (.andromeda/test-plan.md:847) The spine hook rows must stay under this gate after the chunk's changes to the client check and the codes.
  > | Spine hooks `session-start`, `user-prompt-submit`, `stop` (same method, `VIOLA_NAME`/`VIOLA_DIR` exported to hyperfine) | mean, median, stddev | `max <` `viola_core::SPINE_DEADLINE` (1.0 s) |
- (.andromeda/test-plan.md:858) The scope's WATCH on `verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1` and any new table case are held to this budget: a flake seen in this chunk keeps the chunk red until its root cause is fixed in it, with no retry and no `#[ignore]`.
  > **Zero-flakiness budget:** flaky tests are NOT tolerated. The nextest `retries = 0`. If a test flakes once, it gets quarantined immediately and fixed at the root cause, never with a retry-once budget. For viola, quarantine means the chunk stays red until the root cause is fixed within that chunk. `#[ignore]` is not allowed as a parking place.
- (.andromeda/test-plan.md:861) The chunk's verdict must read these conditions on all three OSes, the moved documents' tests included.
  > - Test failure (any level, any OS)
  > - Coverage below threshold for tier (per OS)
  > - Performance budget regression (hyperfine `max` gate)
- (.andromeda/test-plan.md:928) The new table and its helpers must reach the gate without any widening of the coverage exclusion.
  > - NEVER widen the coverage `--ignore-filename-regex` (`viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\]`) to pass a gate.
- (.andromeda/test-plan.md:932) The WATCH's margin must not be answered by a retry setting.
  > - NEVER set nextest `retries` above 0.
- (.andromeda/test-plan.md:937) A cause arm that is hard to reach must be tested or named by the scope's rule, never covered by a lowered bar or a skipped gate.
  > - NEVER lower coverage threshold to pass build (raise quality, not bar)
  > - NEVER skip quality gates "just this once"

## Bindings
- **Per-OS coverage gate ↔ §6 Exit-cause matrix**: the table's cases, including those placed in `tests/security_negatives_server_identity.rs`, `tests/security_negatives_strict_modes.rs`, `tests/run_cli.rs`, `tests/cli_verify.rs` and `tests/cli_revive.rs`, are what must carry each cause's new arms past the per-OS thresholds, (.andromeda/test-plan.md:820) with (.andromeda/test-plan.md:610).
- **Coverage exclusion ↔ §2 file naming and §4 table form**: the `cross_` prefix is named under `viola-e2e`, whose source sits outside the coverage number and which holds labelled case tables rather than rstest, while the scope names root `tests/cross_exit_causes.rs` with an rstest `#[case]` table; the chunk must settle the table's home, and its `viola` children count toward coverage in either home, (.andromeda/test-plan.md:324) with (.andromeda/test-plan.md:361) and (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8).
- **Coverage thresholds ↔ §9 CI**: the per-OS `run --coverage` job ending in `viola-harness gate --require <suites>` is the reader of the §10 numbers, and a reading below them on any OS fails the build, (.andromeda/test-plan.md:754) with (.andromeda/test-plan.md:795).
- **Profile survival ↔ §7 fake agent**: the kill-shaped case relies on the fake agent's process groups and its exit's SIGHUP handling, (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:9) with (.andromeda/test-plan.md:677).
- **Mutation gate ↔ §9 Mutation and the code audit**: no chunk or CI job runs mutation, and the audit's `missed == 0` reading is where the chunk's new code answers, (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:15) with (.andromeda/test-plan.md:756).
