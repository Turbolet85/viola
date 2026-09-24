# tests extract

## Relevance
Relevant. This chunk turns test-plan §10 and the §9 CI rows into gates, and it builds the §3 `gate`, `--coverage` and `--fuzz-replay` harness surfaces. Most of it sits inside the tests domain.

## Constraints
- **Coverage floors.** test-plan §10 Coverage thresholds (Comprehensive row) and its Stack adjustments require floors of lines 85, functions 95 and regions 80, enforced separately in each OS job.
  - Coverage rolls up across all workspace crates.
  - The ignore regex is exactly `viola-fake-agent|crates/viola-e2e|tests/support|fuzz/`.
  - Doctests run but do not count toward the number.
  - Child `viola` processes are included through the propagated `LLVM_PROFILE_FILE`.
  - The `--coverage` command form is per test-plan §3 `run` (`--coverage` bullet): one instrumented `cargo llvm-cov nextest … --profile ci` run replaces steps 1+2 and reports as the single suite `coverage`, then `doctest` follows. Its JUnit source is `target/llvm-cov-target/nextest/ci/junit.xml`, copied to `junit-coverage.xml` (per §3 `gate` Inputs).
  - Whether HEAD meets 85/95/80 on each OS is research's question.
- **`gate` contract.** test-plan §3 Internal harness subcommands (`gate`) fixes these:
  - the inputs: `junit-<suite>.xml`, `run-summary.json` merged by suite, `mutants.out/outcomes.json`, `llvm-cov-summary.json` and `perf-<hook>.json`;
  - the breach set: `suite-missing`, `suite-failed`, `suite-skipped`, `coverage`, `mutants`, `perf` and `artifact-missing`. A missing artifact is a breach, never a pass;
  - the rule that `run` deletes each JUnit source before every invocation and never copies a stale file;
  - the output shape `{"v":1,"cmd":"gate","ok",breaches[…]}` and exit codes 0, 1 and 2.
  - The `suite` enum is closed at the nine `run` values (§3 Closed enums). Any new enum value needs a §12 Decisions Log entry.
- **Per-job `--require` lists.** test-plan §9 Coverage report row and §3 Bootstrap `quality-gate-config-emit` require `gate --require` as the last step of every job:
  - per-OS test: `coverage,doctest`
  - Mutation: `mutants`
  - MSRV: `nextest-unit`
  - Fuzz replay: `fuzz-replay`
  - `playwright` and `perf` are listed there as well, but they have no producing job yet.
- **MSRV job.** test-plan §9 MSRV row and Matrix builds (Language version) require:
  - ubuntu, dtolnay/rust-toolchain `1.96` (a separately named toolchain, not the `rust-toolchain.toml` pin);
  - `cargo check --workspace` followed by `agent-run run --unit`;
  - a separate rust-cache key.
  - Whether every locked dependency builds on 1.96 is research's question.
- **Fuzz replay.** test-plan §3 `run` (`--fuzz-replay` bullet), §3 Bootstrap `ci-tool-install`, §2 Property-based row and §9 Fuzz replay row require:
  - `cargo +nightly fuzz run <target> -- -runs=0` over the committed `fuzz/corpus/<target>/`, ubuntu only;
  - a missing or empty corpus fails as `reason:"corpus-empty"`, and a missing cargo-fuzz exits 1 with `reason:"tool-missing"`;
  - suite `fuzz-replay` counts per target;
  - reproducers are committed with their fix, and nothing is cached between runs;
  - `nightly.yml` runs `-max_total_time=120` per target, uploads `fuzz/artifacts/` and fails on any crash;
  - the fuzz job has no cache and gets cargo-fuzz 0.13.2 plus nightly only in that job.
- **Property suite.** test-plan §6 Property suite and §4 Fixture pattern require proptest 1.11.0 `proptest!` blocks at `cases: 512` over the seven named parsers. The fuzz targets `fuzz/fuzz_targets/<parser>.rs` (§2 directory conventions) mirror them through shared `arbitrary` 1.4.2 inputs. §7 Seed strategies (Randomized data) requires committed `proptest-regressions/` seeds. Which of the seven parsers exist at HEAD is research's question.
- **Zero flakiness.** test-plan §10 Zero-flakiness budget and §3 Bootstrap `test-runner-install` require:
  - `[profile.ci] retries = 0`;
  - no `#[ignore]` or `test.skip` parking;
  - quarantine means the chunk stays red until the root cause is fixed.
  - The `gate` `suite-skipped` breach fires only on a real JUnit `<skipped/>` (§3 `run` Output format, `skipped` bullet).
  - The plan itself does not mandate a retry-config probe. Whether the chunk's "checked property" goes into `gate` or into a separate probe is an implementation choice within §11 CI.

## Patterns to follow
- **Internal subcommand pattern.** Model `gate` on `schema-check` and `secret-scan` (test-plan §3 Internal harness subcommands):
  - implemented once in `viola-harness` and forwarded unchanged by both shims;
  - one JSON document on stdout starting `{"v":1,"cmd":…,"ok":…}`, with key order held by `preserve_order`;
  - exit codes 0, 1 and 2;
  - usage errors carry `reason:"usage"` plus a `detail` (§3 preamble, Exit codes).
- **Mutants verdict reuse.** `gate`'s `mutants` breach should consume the existing `counted` / `no-rust-delta` verdict (test-plan §3 `run` step 4 Classification and Verdict; §10 Mutation gate). A `no-rust-delta` run is an explicit pass. A counted run needs `missed == 0 && timeout == 0`, never the exit code alone.
- **Grammar grows per chunk.** A selector whose producing surface is not built yet is a usage error or an explicit `tool-missing` / `corpus-empty` failure, never a vacuous pass (test-plan §3 preamble). This applies to `--fuzz-replay` targets whose parser does not exist yet.
- **`env_clear()` rule.** Any test that calls `env_clear()` re-adds `LLVM_PROFILE_FILE` (test-plan §3 Bootstrap `coverage-tooling-install`; §11 Integration).
- **Test for the `file_mode` fix.** Use a `#[cfg(unix)]` test named `<subject>_<condition>_<expected>` (test-plan §4 Conventions) whose asserted outcome depends on the mode value. The mutant must be killed by `viola-e2e`'s own tests, because cargo-mutants' default package-scoped test scope applies (test-plan §3 `run` step 4, Test scope).

## Anti-patterns to avoid
- **Weakening a gate.** Never widen `--ignore-filename-regex`, lower a coverage floor, set nextest `retries` above 0, add a retry-once policy, or skip a gate "just this once". Close a shortfall with tests (test-plan §11 CI; §11 Quality).
- **Unseeded randomness.** Never use non-deterministic generators without seed control. Never run property tests without committed `proptest-regressions/` replay (test-plan §11 Test Data; §11 Universal, last bullet).
- **Trusting cargo-mutants' exit code.** Never trust it alone, because exit 3 masks exit 2. Never use mutable-tag Actions or a cache beyond Swatinem/rust-cache in `ci.yml` (test-plan §11 Quality; §11 CI).

## Contract bindings
- **tests ↔ obs (scan-gated uploads).** `target/agent-run/` (including `artifacts/`) leaves CI only in obs-plan §9's scan-gated `harness-${{ matrix.os }}` upload (test-plan §3 `gate`, last bullet; §9 Test report format).
  - test-plan §9 Test report format still lists `mutants.out/` as an uploaded artifact. It sits outside the secret-scan scope in §3 `secret-scan` (Scope bullet).
  - Scope item 8, whether to scan `mutants.out/` or stop uploading it, therefore needs a test-plan §9 amendment either way. If it is scanned, the obs-plan §9 scan roots or the §3 `secret-scan` scope need one too.
- **tests ↔ security (workflow discipline).** Every workflow needs `permissions: {}`, per-job `contents: read`, SHA-pinned `uses:`, and zizmor as the pin assertion (test-plan §9 paragraph after the table). The zizmor `concurrency-limits` decision (scope item 7) interacts with the Mutation row's `AGENT_RUN_CHUNK_BASE = github.event.before` (test-plan §9 Mutation row; §3 `run` step 4, Base): cancelling a run could leave the base unreachable (`base-missing`).
- **tests ↔ arch.**
  - Where `fuzz/` sits in the workspace, and keeping it out of the release build, the cargo-deny graph and stable `cargo check --workspace`, are arch-tree decisions. Tests only requires `fuzz/` to be in the coverage ignore regex (test-plan §10 Stack adjustments).
  - The MSRV floor 1.96 is the workspace `rust-version` requested from arch (test-plan §9 MSRV row; §12, obs-plan D-22).
- **tests ↔ a11y and perf (deferred).** `gate` must parse `perf-<hook>.json` and `junit-playwright.xml` per §10 and §3 `gate`, but no job requires them until their chunks land. Playwright `retries: 0` / `forbidOnly` (§3 Bootstrap `test-runner-install`, Node side) has no subject yet.

## Acceptance criteria contributions
- (tests) In each of the three OS `test` jobs, `agent-run run --coverage` passes with `--fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80` under the unwidened ignore regex, followed by `doctest`. Then `viola-harness gate --require coverage,doctest` exits 0 (per test-plan §10 Coverage thresholds and §3 `gate`).
- (tests) `viola-harness gate` exits 1 with the matching breach in each of these cases (per test-plan §3 `gate`):
  - a required suite is absent (`suite-missing`);
  - an artifact is missing (`artifact-missing`);
  - a suite has `failed > 0` or `skipped > 0`;
  - coverage is below the floor;
  - `missed + timeout > 0`.

  Each case is proven by planted-artifact tests.
- (tests) The ubuntu fuzz job passes `agent-run run --fuzz-replay` and then `gate --require fuzz-replay`. An empty corpus dir fails with `corpus-empty`, and a missing cargo-fuzz fails with `tool-missing` (per test-plan §3 `run` `--fuzz-replay` bullet and §3 Bootstrap `ci-tool-install`).
- (tests) The ubuntu `mutants` job for this chunk's push reads `missed == 0 && timeout == 0`, including both `secret_scan.rs:230:5` `file_mode` mutants. The MSRV job passes `cargo check --workspace` and `run --unit` on 1.96, then `gate --require nextest-unit` (per test-plan §10 Mutation gate and §9 MSRV row).

## Relevant amendment history
- **2026-09-24-three-os-ci-headless-harness-skeleton: harness builds in its own target dir.** The harness's cargo work moved to `target/harness`. The sweep explicitly left the llvm-cov line (:558) unchanged, because "llvm-cov builds in its own target dir". The `--coverage` path must therefore keep reading JUnit from `target/llvm-cov-target/…`, not from `target/harness`.
- **Same chunk: integration filterset, mutation diff, base and trigger.**
  - The `mutants` job runs on push and pull_request, with base = PR base sha or else `github.event.before` through `env:`.
  - Its cause: a committed-only diff was empty, and the only remote branch is `build/viola-0.1.0`.
  - Relevant to the `concurrency:` decision, which changes run cancellation.
- **Same chunk: nextest mutants profile and toolchain source.**
  - rustfmt and clippy come from `rust-toolchain.toml` components, pinned at 1.98.1.
  - The `dtolnay` sweep left the nightly fuzz toolchain and the MSRV 1.96 job (:756, :1393) unchanged, because they are separate toolchains. dtolnay remains correct for this chunk's MSRV and fuzz jobs.
- **2026-09-24-fake-agent-and-test-data-fixtures: mutation verdict for Rust-free diffs.**
  - Added the `counted` / `no-rust-delta` verdict: stale `outcomes.json` is deleted first, and a counted run with no fresh `outcomes.json` is red.
  - Cause: cargo-mutants 27.1.0 exits 0 on a Rust-free diff and leaves `mutants.out/` untouched.
  - `gate`'s `mutants` breach must honour this verdict rather than re-derive it.
- **2026-09-24-supply-chain-and-workflow-gates: Lint row and new Supply-chain row.**
  - The Lint row's `cargo check` reads `scripts/sync-crates.txt`.
  - The new Supply-chain row covers cargo deny, the sole-root tokio ban, `deny-probes.sh` and zizmor.
  - The Lint row's `cargo tree -e features` and cargo-modules items were left in place. Scope item 1 asks P3 whether they have a subject or another owner.
- **2026-09-24-observability-gates: internal gate subcommands, readiness, mutants floor, runner jq, scan-gated uploads.**
  - Declared `schema-check` and `secret-scan` as internal subcommands, the model for `gate`.
  - Added `gate`'s CI-upload sentence: `target/agent-run/` leaves CI only via the scan-gated `harness-<os>` artifact, and the unscanned `agent-run-<os>` upload was retired.
  - Set `[profile.mutants]` slow-timeout to 5 s×2, with a 15 s×2 `viola-e2e` override, so in-test waits stay below cargo-mutants' 20 s floor.
  - Cause: run `35995290314` graded hangs as Timeout.
  - Relevant to items 8 and 9: the `mutants.out/` upload was not covered by this retirement, and new `viola-e2e` gate tests must respect the 20 s floor.
