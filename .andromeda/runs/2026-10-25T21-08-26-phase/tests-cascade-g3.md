# tests · cascade read · group 3 of 3

## Carried
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:19) The listener's first-instance property (a second listener on a held name fails) and the re-verified squatted-name start refusal must bind the held name through the interprocess server API before `run`, with no other fake server and never through `try_overwrite`.
  > | Fake servers | interprocess 2.4.4 server API (squatter listener); `viola-fake-agent` as the fake CLI | The squatter binds the computed endpoint before `run`; no HTTP fake servers |
  (.andromeda/test-plan.md:724)
  > **Fake servers:** the interprocess server API binds the squatter before `run`, and `viola-fake-agent` is the fake CLI; no HTTP fake servers.
  (.andromeda/test-plan.md:889)
  > NEVER use interprocess `try_overwrite`, or reuse an endpoint name across tests. Take the endpoint only from `snapshot.json`.
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:18) The remote-client, first-instance and DACL-effect assertions must run against real pipes and real Win32 read-backs, with doubles only on traits viola already defines at its seams, and the re-verified root tests (`tests/path1_run_start.rs`, `tests/cli_instance_state.rs`) must resolve the fake agent through `CARGO_BIN_EXE_viola-fake-agent`.
  > `#[cfg_attr(test, mockall::automock)]` on seam traits only; external processes are real stub executables, resolved through `CARGO_BIN_EXE_viola-fake-agent` in root `tests/` only.
  (.andromeda/test-plan.md:726)
  > **Anti-monkey-patching:** prefer dependency injection over runtime patching of production code. Doubles exist only for traits viola already defines at its seams (research, mockall key detail).
  (.andromeda/test-plan.md:913)
  > NEVER monkey-patch production code at runtime (use dependency injection)
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:8) The case_4 fix in `src/run/send/budget.rs` must keep the budget gate's instant handed to its pure decision as an argument, add no new `chrono::Utc::now()` read outside the clock seam, and make its log capture wait on a signal, never on a test-owned timer or sleep.
  > `chrono::Utc::now()` is read only through the clock seam, with one exception as landed: the budget gate's three evaluation points read it directly and hand the instant to the pure decision
  (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:17)
  > Inject the `Clock` dependency through constructors and call `MockClock::advance(Duration::from_secs(6))`; or hand a pure decision its instant as an argument, as the budget gate's `decide` takes `chrono::DateTime<Utc>`
  (.andromeda/test-plan.md:705)
  > `chrono::Utc::now()` is read only through the clock seam, except at the budget gate's three evaluation points, and a paused clock in a test never reaches a spawned wrapper.
  (.andromeda/test-plan.md:943)
  > NEVER use real time without injection (use deterministic clocks) in test-side code: no test-owned timer, sleep or elapsed-time verdict.
- (.andromeda/test-plan-detail/ci-integration.md:18) case_4 has run green only under the Unit stage's nextest process-per-test, so the chunk must make it hold under plain `cargo test` (libtest threads) three times or fix its capture at the root cause, and keep it green under nextest, never reaching green through a retry, `#[ignore]` or serialised threads.
  > - **Runs:** `scripts/agent-run.sh run --unit` (pwsh `scripts/agent-run.ps1 run --unit` on windows)
  >   - **Parallel:** yes (nextest process-per-test)
  (.andromeda/test-plan.md:851)
  > **Zero-flakiness budget:** flaky tests are NOT tolerated. The nextest `retries = 0`. If a test flakes once, it gets quarantined immediately and fixed at the root cause, never with a retry-once budget. For viola, quarantine means the chunk stays red until the root cause is fixed within that chunk. `#[ignore]` is not allowed as a parking place.
  (.andromeda/test-plan.md:919)
  > NEVER serialise tests (`max-threads = 1`): isolation comes from the per-test home.
  (.andromeda/test-plan.md:925)
  > NEVER set nextest `retries` above 0.
- (.andromeda/test-plan-detail/ci-integration.md:22) The new listener negatives in `tests/security_negatives_channel.rs` and the re-verified `tests/cli_instance_state.rs` must pass under the local `run --integration`, in parallel, as the equivalent of CI's instrumented run.
  > - **Runs:** `agent-run run --integration`
  (.andromeda/test-plan.md:745)
  > - **Unit / Integration / E2E:** `agent-run run --unit`, `--integration` and `--e2e`, the local equivalents of CI's one instrumented coverage run.
- (.andromeda/test-plan-detail/ci-integration.md:26) Path 1's squatted-endpoint leg (exit 1) in `tests/path1_run_start.rs` must be re-verified through `run --e2e` on all three OSes, its Windows reading taken on the windows-2025 leg.
  > - **Runs:** `agent-run run --e2e` on all 3 OSes
- (.andromeda/test-plan-detail/ci-integration.md:30) The admission assertions count as fully asserted only when they run in the windows-2025 per-OS test job's one instrumented nextest run, which ends in `gate --require coverage,doctest`; a Linux dev-host run is no proof of the Windows branch.
  > Its one instrumented nextest run replaces the Unit, Integration and `run --e2e` rows, which are the local `agent-run` equivalents, and the doctest run follows it. Each job's last step is `viola-harness gate --require <suites>`: the per-OS test job uses `coverage,doctest`
  (.andromeda/test-plan.md:767)
  > - OS: `windows-2025` (ConPTY, named pipe, DACL / SDDL read-back, `.cmd` refusal)
  (.andromeda/test-plan.md:873)
  > NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
- (.andromeda/test-plan-detail/ci-integration.md:54) The new `#[cfg(windows)]` listener code and its tests must hold the windows-2025 job's coverage gate (lines 85, functions 95, regions 80) as `viola-harness gate` judges it, with no threshold lowered to pass.
  > - **Runs:** `viola-harness gate`, which reads `junit-*.xml`, `run-summary.json`, the llvm-cov JSON summary and the hyperfine JSON, and exits non-zero on any §10 breach
  (.andromeda/test-plan.md:813)
  > Gates are enforced per OS job with `cargo llvm-cov nextest … --fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80`. Per-OS gating is deliberate: `#[cfg(windows)]` / `#[cfg(unix)]` code compiles only on its own runner.
  (.andromeda/test-plan.md:930)
  > NEVER lower coverage threshold to pass build (raise quality, not bar)
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) Any new or re-verified test that spawns `viola run` or a listener child (the squatter, a second listener, a DACL-effect probe) and clears its env must re-add `LLVM_PROFILE_FILE`, so the run stays in the per-OS coverage number.
  > Child `viola` processes spawned by E2E tests are included, because `LLVM_PROFILE_FILE` propagates. Tests that call `env_clear()` must re-add it.
  (.andromeda/test-plan.md:817)
  > - Child `viola` processes spawned by E2E tests are included, because `LLVM_PROFILE_FILE` propagates; tests that call `env_clear()` must re-add it
  (.andromeda/test-plan.md:891)
  > NEVER call `.env_clear()` without re-adding `LLVM_PROFILE_FILE`, because that run silently drops out of coverage.
- (.andromeda/test-plan-detail/ci-integration.md:14) The chunk's own CI runs will read `supply-chain` red on the tokio advisory, owned by a later route entry: no ignore, skip or allow is added to reach green, and every other job is read by its own conclusion.
  > `cargo deny --format json check` (`deny.toml`: advisories, licences, sources, bans)
  (.andromeda/test-plan.md:791)
  > - Supply-chain errors: a cargo deny ban/advisory/licence/source finding
  (.andromeda/test-plan.md:858)
  > - Lint / clippy / cargo-deny / zizmor / cargo-modules orphans failure, or a release build carrying a test-only binary
- (.andromeda/test-plan-detail/ci-integration.md:10) The chunk edits `#[cfg(windows)]` channel code, so the plan may list the check-only Windows-target clippy as a dev-host chunk gate entry, while the windows-2025 `lint` job's clippy `-D warnings` stays the gate.
  > Beside CI, a chunk plan may list a check-only Windows-target clippy as a chunk gate entry on the Linux dev host, which does not build the `cfg(windows)` code CI lints (first listed by chunk 2026-10-13-epoch-4-cleanup; `CARGO_TARGET_DIR=target/wincheck cargo clippy --workspace --all-targets --features fake-agent --target x86_64-pc-windows-msvc -- -D warnings`, nothing linked, the target std installed as host preparation): neither a CI step nor a `pre-push` stage.
  (.andromeda/test-plan-detail/ci-integration.md:9)
  > `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings`
  (.andromeda/test-plan.md:741)
  > fmt, clippy `-D warnings`, the tokio-free `cargo check` of each listed sync crate, the cargo-modules orphans gate with its probe, and the rmcp release-graph assertion, per OS; the `--acyclic` review and a dev-host Windows-target clippy are not gates.

## Bindings
- **mocking (fake servers) ↔ §11 Integration**: the squatter that proves the first-instance property and the squatted-name refusal is bound through the interprocess server API, never `try_overwrite`, and each test takes a fresh endpoint name (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:19) (.andromeda/test-plan.md:889)
- **ci (coverage report) ↔ quality-gates (per-OS coverage)**: the windows-2025 test job is the only place the new `#[cfg(windows)]` admission code is compiled, measured and gated, so the Windows assertions land there or stay owed (.andromeda/test-plan-detail/ci-integration.md:30) (.andromeda/test-plan.md:813)
- **quality-gates (`LLVM_PROFILE_FILE`) ↔ ci (coverage report)**: a spawned child that clears its env without re-adding the profile variable drops out of the windows-2025 coverage number the gate reads (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) (.andromeda/test-plan-detail/ci-integration.md:30)
- **ci (unit stage) ↔ quality-gates (zero-flakiness) ↔ mocking (time)**: case_4 is fixed at its root cause within the chunk, without retries, `#[ignore]` or serialisation, and its capture waits on a signal with the budget instant passed as an argument (.andromeda/test-plan-detail/ci-integration.md:18) (.andromeda/test-plan.md:851) (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:8)
- **ci (supply-chain) ↔ quality-gates (build failure conditions)**: the advisory red is a cargo-deny failure that stays visible on the chunk's runs; nothing is widened to clear it (.andromeda/test-plan-detail/ci-integration.md:14) (.andromeda/test-plan.md:858)
