# tests · CI Integration — category read

## Carried
- (.andromeda/test-plan.md:767) The chunk's assertions (remote-client rejection, first-pipe-instance, DACL effect) must run and pass on the `windows-2025` matrix leg, the runner this plan names for the named pipe and DACL read-back.
  > - OS: `windows-2025` (ConPTY, named pipe, DACL / SDDL read-back, `.cmd` refusal)
- (.andromeda/test-plan.md:873) A Linux dev-host run cannot close any of the chunk's Windows clauses: each is closed only by a green Windows CI leg.
  > - NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
- (.andromeda/test-plan.md:784) The Windows leg's verdict must be read from the job conclusions and artifacts through `gh`, not assumed from a local run.
  > The agent reads job conclusions with `gh run view --json jobs` and fetches artifacts with `gh run download`.
- (.andromeda/test-plan.md:46) The chunk's item (c) (an open by a principal outside the two ACEs denied) and item (a) must stay within what a hosted runner can drive; the cross-user and remote-client parts are asserted at the boundary, not by a second user, unless the chunk measures otherwise.
  > the cross-user parts (a peer of another euid, remote pipe clients, the Windows pipe DACL) cannot be driven on hosted runners and are asserted at the peer-credential check boundary.
- (.andromeda/test-plan-detail/ci-integration.md:9) The chunk's new `cfg(windows)` code and tests in `viola-channel` must pass CI's lint stage on every OS, clippy with `-D warnings` included.
  > `cargo fmt --check`; `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings`;
- (.andromeda/test-plan-detail/ci-integration.md:10) The chunk edits `cfg(windows)` code the Linux host does not build, so its plan may list the check-only Windows-target clippy as a chunk gate entry, which is neither a CI step nor a pre-push stage.
  > Beside CI, a chunk plan may list a check-only Windows-target clippy as a chunk gate entry on the Linux dev host, which does not build the `cfg(windows)` code CI lints (first listed by chunk 2026-10-13-epoch-4-cleanup; `CARGO_TARGET_DIR=target/wincheck cargo clippy --workspace --all-targets --features fake-agent --target x86_64-pc-windows-msvc -- -D warnings`, nothing linked, the target std installed as host preparation): neither a CI step nor a `pre-push` stage.
- (.andromeda/test-plan-detail/ci-integration.md:30) The chunk's new Windows tests run inside the per-OS test job's one instrumented nextest run, and the job must end green on its gate.
  > Its one instrumented nextest run replaces the Unit, Integration and `run --e2e` rows, which are the local `agent-run` equivalents, and the doctest run follows it. Each job's last step is `viola-harness gate --require <suites>`: the per-OS test job uses `coverage,doctest`
- (.andromeda/test-plan.md:813) The chunk's `cfg(windows)` listener code is covered, and its coverage gated, only by the `windows-2025` job.
  > Per-OS gating is deliberate: `#[cfg(windows)]` / `#[cfg(unix)]` code compiles only on its own runner.
- (.andromeda/test-plan.md:787) Any failing test on any of the three OSes fails the build, the chunk's Windows-only tests and the carried budget case included.
  > - Any unit / integration / E2E / contract / property test fails (on any of the 3 OSes)
- (.andromeda/test-plan-detail/ci-integration.md:34) If the chunk adds a Windows-gated file, the dispatch-only Windows mutation leg runs per package over its Windows-gated files, so the new file must be within that leg's scope for the boundary audit.
  > The audit's Windows leg is `windows-mutants.yml`: `workflow_dispatch` only, no inputs, ten `mutants (<label>)` jobs on `windows-2025` (`fail-fast: false`, 120 min), one per `matrix.include` item (the count of its `- package:` items; the root package `viola` is five of them, split by file, the fifth `viola-cmd-verify-stop` over `src/cmd/verify/stop.rs` since chunk 2026-10-13-interrupted-verify-cleanup), each running `scripts/agent-run.ps1 run --mutants --package <member> --file …` over that item's Windows-gated files, report-only, no cache or upload
- (.andromeda/test-plan.md:851) The carried budget case (`send_budget_paused_is_refused_with_the_window_word_and_no_cursor` case_4, red once under plain `cargo test`) is a flake under this budget: its root cause must be fixed within this chunk, never retried or parked.
  > **Zero-flakiness budget:** flaky tests are NOT tolerated. The nextest `retries = 0`. If a test flakes once, it gets quarantined immediately and fixed at the root cause, never with a retry-once budget. For viola, quarantine means the chunk stays red until the root cause is fixed within that chunk. `#[ignore]` is not allowed as a parking place.
- (.andromeda/test-plan.md:919) The fix of the budget case's log capture under libtest threads must not serialise tests.
  > - NEVER serialise tests (`max-threads = 1`): isolation comes from the per-test home.
- (.andromeda/test-plan.md:922) The budget case must not be ignored to reach green.
  > - NEVER ignore flaky tests; quarantine and fix immediately
- (.andromeda/test-plan.md:925) No retry may be added to pass the budget case or any new Windows test.
  > - NEVER set nextest `retries` above 0.
- (.andromeda/test-plan.md:791) The chunk's own CI runs will read `supply-chain` red on the RUSTSEC-2026-0386 advisory, a standing build-failure condition owned by the new route entry, not by this chunk; the red must be read as that cause, not as the chunk's.
  > - Supply-chain errors: a cargo deny ban/advisory/licence/source finding

## Bindings
- **Windows matrix leg ↔ body §11 Test Strategy**: the chunk's Windows clauses close only on `windows-2025` (.andromeda/test-plan.md:767), and a Linux run is no proof for them (.andromeda/test-plan.md:873); the Windows-target clippy is a dev-host chunk gate only (.andromeda/test-plan-detail/ci-integration.md:10).
- **Hosted-runner limit ↔ scope item (c) / test-scope-summary category**: the body says the remote-client and DACL cross-user parts cannot be driven on hosted runners (.andromeda/test-plan.md:46); scope (c) goes to a Windows-only live measurement where a runner cannot measure it.
- **Per-OS coverage ↔ §10 Quality Gates**: the `windows-2025` test job's `gate --require coverage,doctest` (.andromeda/test-plan-detail/ci-integration.md:30) gates the chunk's `cfg(windows)` code, per-OS gating deliberate (.andromeda/test-plan.md:813).
- **Budget-case CARRY ↔ §10 Zero-flakiness / §11 CI**: fixed at root cause within the chunk (.andromeda/test-plan.md:851), with no `max-threads = 1` (.andromeda/test-plan.md:919), no ignore (.andromeda/test-plan.md:922) and no retries (.andromeda/test-plan.md:925).
- **Supply-chain red ↔ security-plan §Dependencies and CI**: the advisory finding is a build failure (.andromeda/test-plan.md:791) whose disposition is the new route entry "tokio RUSTSEC-2026-0386 lock bump"; security-plan forbids widening an ignore to reach green.
- **Windows mutation leg ↔ the boundary audit**: a new Windows-gated file falls in `windows-mutants.yml`'s per-package scope (.andromeda/test-plan-detail/ci-integration.md:34), whose tool pins `tests/contract_windows_mutation_scope.rs` asserts.
