# tests · Quality Gates & Coverage Targets — category read

## Carried
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) Every witness and owed case that starts a `viola` child with a cleared or hand-built environment (the planted stripped `CLAUDE*` value among them) must re-add the profile variable, so the error paths it drives count toward the coverage gate.
  > Child `viola` processes spawned by E2E tests are included, because `LLVM_PROFILE_FILE` propagates. Tests that call `env_clear()` must re-add it.
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) A witness that kills a child to drive a failure must not kill an instrumented child that exits by itself during its exit-time profile write, because the short profile reds the coverage merge.
  > an instrumented child that exits by itself and is killed during its exit-time profile write leaves a short profile, which `llvm-profdata` refuses.
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:9) A kill-shaped witness (a killed wrapper driving MCP `instance-unreachable` or a channel failure) must run its hooks through the fake agent's own-process-group spawn, so the hangup cuts no `viola hook` profile.
  > a test that kills the wrapper closes its PTY, and the hangup reaches the hooks the fake agent ran in that terminal's foreground process group, so a `viola hook` cut off in its exit-time profile write leaves a page-cut profile that `llvm-profdata` refuses, or none; with every command the fake agent spawns in a process group of its own (the fake agent's exit bullet, §7) the census over the kill-shaped `chaos_wait_parked_wrapper_killed_exits_21` reads 0 refused profiles of 800 in 200 runs (as measured at `viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup/evidence/profraw-cause.md`).
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:15) The chunk must not add or claim a chunk-level mutation gate, and its acceptance must not require a mutation run; its product changes are scored at the epoch-boundary audit.
  > **Mutation gate (Founder Direction 1):** judged at the epoch boundary by `/andromeda-code-audit` and on demand by `run --mutants`; no chunk, pre-push or CI mutation gate exists since 2026-09-28 (the founder's ruling, 17:59).
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:15) Each new product error mapping or fixed message must be held by a nextest case, never by a doctest alone, or the later boundary score reads its mutant missed.
  > A mutant caught only by a doctest counts as missed under nextest, so the missing unit test must be added.
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:25) Any error-surface or `channel-response` change that reaches a hook's logging or channel path falls inside the timed `--perf` rows and must be judged by them.
  > so every timed hook takes the real logging and channel path, never the no-instance no-op (obs-plan §10 obs-overhead row)
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:26) This is the perf gate's newest recorded Linux reading, the comparison line for a change in the timed path; the gate itself stays `SPINE_DEADLINE`.
  > with both checks in the timed path the five hook rows read `max` 2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms on the Linux dev host, under `SPINE_DEADLINE`, as measured at `viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md` (Outcome, `run --perf`).
- (.andromeda/test-plan.md:816) The chunk's tier thresholds must hold after its change, the new error paths included.
  > | **Comprehensive (viola)** | **≥ 85%** | **≥ 80%** (enforced as region coverage `--fail-under-regions 80`) | **≥ 95%** |
- (.andromeda/test-plan.md:819) The thresholds must hold on each OS job, so OS-branch error paths the chunk touches need a case on their own runner.
  > Gates are enforced per OS job with `cargo llvm-cov nextest … --fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80`. Per-OS gating is deliberate: `#[cfg(windows)]` / `#[cfg(unix)]` code compiles only on its own runner.
- (.andromeda/test-plan.md:821) Only these four trees are excluded: product code on the three surfaces counts in full, and a helper in `tests/support` or `crates/viola-e2e` adds no coverage credit.
  > Excluded via the separator-agnostic `--ignore-filename-regex` `(viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\])`, the harness const `COVERAGE_IGNORE`.
- (.andromeda/test-plan.md:827) The mutation posture stands as the epoch boundary's, including the rule that an unreachable mutant is recorded as owed, for any on-demand run the chunk makes.
  > **Mutation gate:** judged at the epoch boundary by `/andromeda-code-audit` and on demand by `run --mutants`, with no chunk, pre-push or CI mutation gate; the run's `outcomes.json` must show `missed == 0` (after the host-excluded records), `timeout == 0` and `unviable <= caught`, because a run whose mutant builds mostly failed is no evidence of test strength; a mutant this host cannot compile or reach is recorded as owed, never equivalent or caught.
- (.andromeda/test-plan.md:839) A perf breach caused by the chunk must be fixed in the product or escalated, with no rerun of the verdict.
  > There is no rerun, no outlier trimming and no second attempt. A breach is fixed in the product or escalated through a Decisions Log entry, never retried away.
- (.andromeda/test-plan.md:841) The chunk's CI read must include the per-OS `perf` job, which the `test` job and the pre-push do not stand in for.
  > CI runs it in its own per-OS `perf` job (`test` and the pre-push carry no perf step), ending in `gate --require perf`.
- (.andromeda/test-plan.md:845) These hook rows and their `max` gates must stay met after any change in the hook's channel or logging path.
  > | `viola hook session-end` (synthetic payload via `--input`, live wrapper), per OS | mean, median, stddev | `max < 1.0 s` (`jq -e '.results[0].max < 1.0'`) |
  > | Spine hooks `session-start`, `user-prompt-submit`, `stop` (same method, `VIOLA_NAME`/`VIOLA_DIR` exported to hyperfine) | mean, median, stddev | `max <` `viola_core::SPINE_DEADLINE` (1.0 s) |
  > | `viola hook pre-tool-use` on an unverified CLI (`hook.dialog` → `null` at once), timed by `run --perf` and required by `perf::ROWS` | mean, median, stddev | `max <` `viola_core::SPINE_DEADLINE` (1.0 s) |
- (.andromeda/test-plan.md:857) Every new witness and owed case must be deterministic at `retries = 0`; a flake keeps this chunk red until its root cause is fixed here, and `#[ignore]` must not park one.
  > **Zero-flakiness budget:** flaky tests are NOT tolerated. The nextest `retries = 0`. If a test flakes once, it gets quarantined immediately and fixed at the root cause, never with a retry-once budget. For viola, quarantine means the chunk stays red until the root cause is fixed within that chunk. `#[ignore]` is not allowed as a parking place. Agent-driven dev cannot distinguish a flake from a real bug; retry policies mask actual failures.
- (.andromeda/test-plan.md:859) The chunk's verdict must read clean against each condition, on every OS.
  > **Build failure conditions:**
  > - Test failure (any level, any OS)
  > - Coverage below threshold for tier (per OS)
  > - Performance budget regression (hyperfine `max` gate)
  > - At a `run --mutants` (the epoch-boundary audit, or on demand): any missed mutant the harness does not leave out as host-excluded (§3 `run` step 4, Verdict), any timed-out mutant, or unviable mutants outnumbering caught ones (`unviable-exceeds-caught`)
  > - Lint / clippy / cargo-deny / zizmor / cargo-modules orphans failure, or a release build carrying a test-only binary
- (.andromeda/test-plan.md:897) This is the guard on the witnesses' environment setup, the ban form of the `env_clear()` rule above.
  > NEVER call `.env_clear()` without re-adding `LLVM_PROFILE_FILE`, because that run silently drops out of coverage.
- (.andromeda/test-plan.md:927) A coverage shortfall on the error paths must be closed by tests, never by widening the exclusion.
  > NEVER widen the coverage `--ignore-filename-regex` (`viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\]`) to pass a gate.
- (.andromeda/test-plan.md:936) These quality bans (§10's own guards, through its § Quality pointer) bind the chunk's gate block, its cold rebuild included.
  > - NEVER lower coverage threshold to pass build (raise quality, not bar)
  > - NEVER skip quality gates "just this once"

## Bindings
- **Coverage thresholds ↔ §9 CI Integration**: the per-OS thresholds are enforced by CI's instrumented run and `viola-harness gate`, and a breach on any OS fails the build, (.andromeda/test-plan.md:819), (.andromeda/test-plan.md:753), (.andromeda/test-plan.md:765), (.andromeda/test-plan.md:794).
- **`env_clear()` rule ↔ §6 drivers and the E2 and sanitisation witnesses**: a witness planting a stripped `CLAUDE*` value under the §6 error-sanitization suite must keep its profile variable, as the hook driver already must, (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8), (.andromeda/test-plan.md:471), (.andromeda/test-plan.md:605).
- **Kill-shaped witnesses ↔ §7 fake agent exit**: the profile rule holds only through the fake agent's process-group and exit behaviour, which a witness must use and not bypass, (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:9), (.andromeda/test-plan.md:676).
- **Perf rows ↔ obs-plan §10 obs-overhead row**: the timed path must stay the real logging and channel path that row names, so a `channel-response` line change is read against both, (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:25), (.andromeda/test-plan.md:846).
- **Mutation posture ↔ §1 trigger and §9 Mutation row**: all three place mutation at the boundary or on demand and outside a chunk's default run, the pre-push and CI, (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:15), (.andromeda/test-plan.md:272), (.andromeda/test-plan.md:755).
- **Zero-flakiness ↔ the chunk's PREREQ cases in §4 and §6**: the owed `ServeError::Initialize` case and the owed cap-driving cases must meet the same `retries = 0` rule and count in the same coverage gate, (.andromeda/test-plan.md:857), (.andromeda/test-plan.md:390), (.andromeda/test-plan.md:623).
- **Zero-flakiness ↔ the scope's Path 1 CI-timeout watch**: no retry may answer a recurrence of that kill on this chunk's CI reads; the rule here and the scope's tally stand side by side, (.andromeda/test-plan.md:857).
