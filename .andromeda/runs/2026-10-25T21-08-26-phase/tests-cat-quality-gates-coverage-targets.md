# tests · Quality Gates & Coverage Targets — category read

## Carried
- (.andromeda/test-plan.md:813) The chunk's new `cfg(windows)` listener code and its Windows-only tests must hold the coverage gate on the `windows-2025` job itself, since per-OS gating is the rule and no other runner compiles that code.
  > Gates are enforced per OS job with `cargo llvm-cov nextest … --fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80`. Per-OS gating is deliberate: `#[cfg(windows)]` / `#[cfg(unix)]` code compiles only on its own runner.
- (.andromeda/test-plan.md:810) The thresholds the Windows leg must meet after the chunk's additions are the Comprehensive row.
  > | **Comprehensive (viola)** | **≥ 85%** | **≥ 80%** (enforced as region coverage `--fail-under-regions 80`) | **≥ 95%** |
- (.andromeda/test-plan.md:815) No new test-only Windows helper may be kept out of coverage by touching the exclusion regex; only the four named trees are excluded.
  > Excluded via the separator-agnostic `--ignore-filename-regex` `(viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\])`, the harness const `COVERAGE_IGNORE`.
- (.andromeda/test-plan.md:921) Widening that exclusion to reach a green Windows coverage reading is banned without a Decisions Log entry first.
  > NEVER widen the coverage `--ignore-filename-regex` (`viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\]`) to pass a gate.
- (.andromeda/test-plan.md:930) The coverage bar is never lowered to absorb hard-to-reach Windows pipe branches.
  > - NEVER lower coverage threshold to pass build (raise quality, not bar)
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) Any test the chunk adds that spawns a child (a second listener process, a `viola run` against a held name) with `env_clear()` must re-add `LLVM_PROFILE_FILE`, and must not kill an instrumented child mid exit-time profile write, which leaves a profile `llvm-profdata` refuses.
  > Child `viola` processes spawned by E2E tests are included, because `LLVM_PROFILE_FILE` propagates. Tests that call `env_clear()` must re-add it.
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) The measured kill-during-profile-write mechanism binds the shape of any kill-shaped Windows test the chunk writes.
  > an instrumented child that exits by itself and is killed during its exit-time profile write leaves a short profile, which `llvm-profdata` refuses.
- (.andromeda/test-plan.md:851) The CARRY on `send_budget_paused_is_refused_with_the_window_word_and_no_cursor` case_4 (red once under plain `cargo test`) falls under the zero-flakiness budget: it is fixed at its root cause inside this chunk, the chunk staying red until then, never parked with `#[ignore]`.
  > **Zero-flakiness budget:** flaky tests are NOT tolerated. The nextest `retries = 0`. If a test flakes once, it gets quarantined immediately and fixed at the root cause, never with a retry-once budget. For viola, quarantine means the chunk stays red until the root cause is fixed within that chunk. `#[ignore]` is not allowed as a parking place.
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:19) The CARRY's fix and any new Windows test take no retry: the CI profile's kill and zero retries stand.
  > `[profile.ci]`'s 120 s kill and `retries = 0` are unchanged.
- (.andromeda/test-plan.md:948) No retry budget may be introduced to make the libtest-thread red of the CARRY case pass.
  > - NEVER add retry-once policies or any retry budget for flaky tests (§10 Zero-flakiness budget).
- (.andromeda/test-plan.md:854) Every Windows assertion the chunk adds (remote-client rejection, first-instance on the listener, the DACL's effect) must pass on the Windows leg for the chunk to be green, and its coverage holds per OS.
  > - Test failure (any level, any OS)
  > - Coverage below threshold for tier (per OS)
- (.andromeda/test-plan.md:873) A Linux dev-host run is no proof of the chunk's Windows pipe behaviour; acceptance reads the `windows-2025` job.
  > - NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
- (.andromeda/test-plan.md:767) The Windows matrix leg the chunk's assertions run on is the one named for named-pipe and DACL read-back coverage.
  > - OS: `windows-2025` (ConPTY, named pipe, DACL / SDDL read-back, `.cmd` refusal),
- (.andromeda/test-plan.md:46) Scope item (c), the DACL's effect against an outside principal, and remote-client rejection are bounded by what a hosted runner can drive; what it cannot is asserted at the boundary, never claimed as gated.
  > the cross-user parts (a peer of another euid, remote pipe clients, the Windows pipe DACL) cannot be driven on hosted runners and are asserted at the peer-credential check boundary.
- (.andromeda/test-plan.md:821) The chunk carries no mutation acceptance criterion: mutation is judged only at the epoch boundary or on demand.
  > **Mutation gate:** judged at the epoch boundary by `/andromeda-code-audit` and on demand by `run --mutants`, with no chunk, pre-push or CI mutation gate;
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:16) A mutant in the chunk's `cfg(windows)` listener code is unreachable on the Linux dev host and is recorded as owed by coordinate if an on-demand run meets it, never equivalent or caught.
  > A mutant this host cannot compile or reach — inside a `cfg(windows)` item or block, or behind a const such as `HOST_SCRATCH = cfg!(windows)` — is recorded "not measured here; owed to {the route entry that measures it}" by coordinate, never "equivalent", and never counted as caught; `missed == 0` then reads over the measurable set (the overseer's ruling, 2026-10-04).
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:18) The chunk's Windows-only code is scored at the Epoch 6 boundary by the dispatch-only Windows mutation workflow, so its new tests must be able to kill mutants there.
  > The boundary audit measures the `cfg(windows)` code by dispatching `windows-mutants.yml` (founder ruling C2, 2026-10-04; §9 Mutation row): `run --mutants --package <member> --file …` per matrix item on `windows-2025` (§9 Mutation row), verdict `package`, report-only and never a gate.
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:25) The per-OS perf job times hooks through the real channel path, so any change to the Windows listener's open or admission path lands under the hook deadline gate on `windows-2025`.
  > hyperfine runs with the repo root as cwd and `VIOLA_NAME=builder` / `VIOLA_DIR=<perf home>/instances/builder` set, so every timed hook takes the real logging and channel path, never the no-instance no-op
- (.andromeda/test-plan.md:840) The hook deadline the listener path must stay under is the product constant.
  > | Spine hooks `session-start`, `user-prompt-submit`, `stop` (same method, `VIOLA_NAME`/`VIOLA_DIR` exported to hyperfine) | mean, median, stddev | `max <` `viola_core::SPINE_DEADLINE` (1.0 s) |
- (.andromeda/test-plan.md:858) A cargo-deny finding is a build failure condition, so the chunk's CI reads red on `supply-chain` (RUSTSEC-2026-0386) until its owning route entry lands; that red is named and owed, never waived.
  > - Lint / clippy / cargo-deny / zizmor / cargo-modules orphans failure, or a release build carrying a test-only binary
- (.andromeda/test-plan.md:931) The supply-chain red is not skipped as a gate for this chunk; it stays a visible red owned by the new route entry.
  > - NEVER skip quality gates "just this once"

## Bindings
- **per-OS coverage gate ↔ security-plan §Authentication & Authorization "IPC access control (Windows)"**: the clause's Windows half the chunk asserts is `cfg(windows)` code measured only by the `windows-2025` coverage job (.andromeda/test-plan.md:813), and the hosted-runner limit on remote clients and the DACL's effect (.andromeda/test-plan.md:46) bounds which of its controls can be gated there.
- **zero-flakiness budget ↔ the scope's CARRY on `send_budget_paused_is_refused_with_the_window_word_and_no_cursor` case_4**: the CARRY's acceptance (three plain `cargo test` runs, or a log-capture fix) is held to root-cause fixing inside the chunk with no retry and no `#[ignore]` (.andromeda/test-plan.md:851, .andromeda/test-plan-detail/quality-gates-coverage-targets.md:19).
- **mutation host-exclusion ↔ test-plan §9 Mutation row / §3 `run` step 4 Verdict**: Windows-only mutants are owed to the boundary's `windows-mutants.yml` dispatch (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:16, .andromeda/test-plan-detail/quality-gates-coverage-targets.md:18).
- **perf gate ↔ obs-plan §10 obs-overhead row**: the hook rows run through the real channel path, which this chunk's listener change sits on (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:25, .andromeda/test-plan.md:840).
- **build failure on cargo-deny ↔ security-plan §Dependencies and CI / the scope's new route entry "tokio RUSTSEC-2026-0386 lock bump"**: the red stays a build failure owned by that entry, with no ignore-list widening (.andromeda/test-plan.md:858, .andromeda/test-plan.md:931).
- **coverage exclusion ↔ test-plan §11 CI**: no Windows test helper moves into an excluded tree to pass the gate (.andromeda/test-plan.md:815, .andromeda/test-plan.md:921).
