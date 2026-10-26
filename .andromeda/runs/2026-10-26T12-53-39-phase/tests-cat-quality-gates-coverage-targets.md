# tests · Quality Gates & Coverage Targets — category read

## Carried
- (.andromeda/test-plan.md:810) The new `link` / `unlink` channel methods, the two verbs, their result lines and the `unlink` product path through `LinkSet::remove` must keep each OS job at or above the Comprehensive thresholds; the chunk must not lower them.
  > | **Comprehensive (viola)** | **≥ 85%** | **≥ 80%** (enforced as region coverage `--fail-under-regions 80`) | **≥ 95%** |
- (.andromeda/test-plan.md:813) Coverage is gated per OS, so any OS-branched code the verbs reach (named pipe vs Unix socket client path) must be covered on its own runner.
  > Gates are enforced per OS job with `cargo llvm-cov nextest … --fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80`. Per-OS gating is deliberate: `#[cfg(windows)]` / `#[cfg(unix)]` code compiles only on its own runner.
- (.andromeda/test-plan.md:815) The chunk's new test helpers count as excluded only inside the four existing trees; new product code may not be routed out of the number.
  > Excluded via the separator-agnostic `--ignore-filename-regex` `(viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\])`, the harness const `COVERAGE_IGNORE`.
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) The `viola link` / `viola unlink` children the root cases (`tests/channel_links.rs`, Scenario E3) spawn are counted toward coverage only if their profile env survives; any case that clears the env must re-add it.
  > Child `viola` processes spawned by E2E tests are included, because `LLVM_PROFILE_FILE` propagates. Tests that call `env_clear()` must re-add it.
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) A new case that starts an instrumented child and kills it straight after must not leave a short profile that fails the coverage merge; such a case takes an uninstrumented child or lets the instrumented one finish its exit.
  > One mechanism for such a profile is measured (as measured at chunk 2026-10-07-a-send-ending-in-a-newline-is-confirmed, `evidence/profraw-red-green.md`): an instrumented child that exits by itself and is killed during its exit-time profile write leaves a short profile, which `llvm-profdata` refuses.
- (.andromeda/test-plan.md:819) The verbs' and methods' coverage must come from nextest cases, not doctests.
  > Doctests are run but are not part of the coverage number.
- (.andromeda/test-plan.md:821) The chunk carries no mutation gate of its own; its new code (the two methods, the verbs, `LinkSet::remove`'s product caller) is scored at the Epoch 6 boundary audit and must then read no missed mutant.
  > **Mutation gate:** judged at the epoch boundary by `/andromeda-code-audit` and on demand by `run --mutants`, with no chunk, pre-push or CI mutation gate; the run's `outcomes.json` must show `missed == 0` (after the host-excluded records), `timeout == 0` and `unviable <= caught`, because a run whose mutant builds mostly failed is no evidence of test strength; a mutant this host cannot compile or reach is recorded as owed, never equivalent or caught.
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:16) Any `cfg(windows)` arm the verbs add that the Linux dev host cannot reach is recorded by coordinate as owed, never claimed caught or equivalent.
  > A mutant this host cannot compile or reach — inside a `cfg(windows)` item or block, or behind a const such as `HOST_SCRATCH = cfg!(windows)` — is recorded "not measured here; owed to {the route entry that measures it}" by coordinate, never "equivalent", and never counted as caught;
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:26) Adding the two methods to the wrapper's dispatch must leave the timed hook rows (which run through that same wrapper and their server check) under the spine deadline in the per-OS `perf` job.
  > Every timed hook also makes its server check (the peer pid and its OS start time against the one snapshot read) before its first frame; with the check in the timed path the hook rows read `max` 2.2–2.4 ms over 30 runs each on the Linux dev host, as measured at `viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/report.md` (Outcome, `run --perf`).
- (.andromeda/test-plan.md:851) The new link / unlink cases must be deterministic: no retry, no `#[ignore]`, and a flake keeps the chunk red until its root cause is fixed inside it.
  > **Zero-flakiness budget:** flaky tests are NOT tolerated. The nextest `retries = 0`. If a test flakes once, it gets quarantined immediately and fixed at the root cause, never with a retry-once budget. For viola, quarantine means the chunk stays red until the root cause is fixed within that chunk. `#[ignore]` is not allowed as a parking place.
- (.andromeda/test-plan.md:853) The chunk is red on any of these, on any OS.
  > **Build failure conditions:**
  > - Test failure (any level, any OS)
  > - Coverage below threshold for tier (per OS)
  > - Performance budget regression (hyperfine `max` gate)
- (.andromeda/test-plan.md:858) The verbs' new code must pass lint, clippy, cargo-deny, zizmor and the cargo-modules orphans gate (the `LinkSet::remove` dead-code allowance ending is part of that).
  > - Lint / clippy / cargo-deny / zizmor / cargo-modules orphans failure, or a release build carrying a test-only binary
- (.andromeda/test-plan.md:921) The coverage exclusion regex must not be widened to bring the chunk to green.
  > NEVER widen the coverage `--ignore-filename-regex` (`viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\]`) to pass a gate.
- (.andromeda/test-plan.md:925) No retry budget may be added for the chunk's cases.
  > NEVER set nextest `retries` above 0.
- (.andromeda/test-plan.md:930) The §10 guards bind the chunk's gate reading.
  > - NEVER lower coverage threshold to pass build (raise quality, not bar)
  > - NEVER skip quality gates "just this once"

## Bindings
- **Per-OS coverage gate ↔ §9 CI Integration**: the thresholds (.andromeda/test-plan.md:810) are read by the per-OS `run --coverage` job ending in `viola-harness gate --require <suites>` (.andromeda/test-plan.md:747); the chunk's new cases land inside that one instrumented run.
- **Child-process coverage ↔ §6 Scenario E3 and §5 integration**: the `LLVM_PROFILE_FILE` rule (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) covers the `viola link` / `viola unlink` children E3 drives (.andromeda/test-plan.md:562) and the `env_clear` ban in §11 Integration (.andromeda/test-plan.md:891).
- **Mutation gate ↔ the Epoch 6 boundary audit**: no chunk gate (.andromeda/test-plan.md:821); the verbs' code is scored by `/andromeda-code-audit` at the boundary under `missed == 0` (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:15).
- **Perf gate ↔ obs-plan §10 obs-overhead row**: the hook rows time the real logging and channel path through the wrapper whose dispatch the chunk extends (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:25), gated on `max <` `viola_core::SPINE_DEADLINE` (.andromeda/test-plan.md:840).
- **Zero-flakiness ↔ §2 determinism**: no retries and no sleep-based synchronisation (.andromeda/test-plan.md:305) back the flake rule (.andromeda/test-plan.md:851).
