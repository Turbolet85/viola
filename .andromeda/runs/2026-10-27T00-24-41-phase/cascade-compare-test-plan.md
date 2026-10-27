# Cascade compare — test-plan · phase · 2026-10-27-pinned-copy-race-and-reproducible-builds
E items 215 · held 26 · lost at classify 4 · lost at strong 1 · beside a placed fact - · cascade only 10 · listed 7 · body 146 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### ci-integration#1.9 · lost at classify · .andromeda/test-plan-detail/ci-integration.md:43
- (.andromeda/test-plan-detail/ci-integration.md:43) The fix must check and pass the unit suite on the 1.96 floor too. It uses no std API newer than MSRV.
  > ubuntu `msrv`: `rustup toolchain install 1.96 --profile minimal`, then `RUSTUP_TOOLCHAIN=1.96` steps for `rustc --version`, `cargo check --workspace` and `agent-run run --unit`, then `gate --require nextest-unit`

### quality-gates-coverage-targets#3.1 · lost at classify · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:25
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:25) The perf session boots through `run`'s start, the `pin_bytes` absent-copy arm included: a boot the changed pin write fails turns suite `perf` red, never a skipped row.
  > A failed build (`build`), boot (`boot: <reason>`), row (the hook's name), panic check or cleanup turns suite `perf` red with the failure named, never a missing or skipped row.

### quality-gates-coverage-targets#3.1 · lost at classify · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:26
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:26) The timed hooks spawn from the pinned path: the newest reading is the one the chunk's `run --perf` must stay under `SPINE_DEADLINE` against.
  > with both checks in the timed path the five hook rows read `max` 2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms on the Linux dev host, under `SPINE_DEADLINE`, as measured at `viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md` (Outcome, `run --perf`).

### test-strategy#1.2 · lost at strong · .andromeda/test-plan-detail/test-strategy.md:14
- (.andromeda/test-plan-detail/test-strategy.md:14) Any integration witness of two concurrent first starts must use a real tempfile home and assert_cmd against the built binary, never a mocked disk.
  > Every arch Standard Contract (Wrapper channel, On-disk, Hooks, CLI, MCP) has a crate-level `tests/` suite, using a real `interprocess` endpoint, a real tempfile home, and assert_cmd against `CARGO_BIN_EXE_viola`.

### test-strategy#3.8 · lost at classify · .andromeda/test-plan-detail/test-strategy.md:65
- (.andromeda/test-plan-detail/test-strategy.md:65) The reproducible-build measurement reads the build of the release stage, so `scripts/release-check.sh` stays the supply-chain trigger's gate.
  > Release build stage (`scripts/release-check.sh`)

### unit-test-strategy#2.1 · lost at classify · .andromeda/test-plan-detail/unit-test-strategy.md:68
- (.andromeda/test-plan-detail/unit-test-strategy.md:68) Any new root-bin unit case that captures a log line, for example a pin or race outcome logged by `run`'s start, must use the shared scoped-capture helper and never a bare `with_default`.
  > Scoped log captures: every scoped capture in the root bin's unit modules goes through `crate::test_support::with_scoped_default` (`src/main.rs`), which registers once per process a pinned, always-interested `tracing::Dispatch` over a bare `tracing_subscriber::registry()` (no layer, no filter) in a `static OnceLock`, never the global default, then runs `tracing::subscriber::with_default`;

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### ci-integration#1.12 · listed (lost at classify) · .andromeda/test-plan-detail/ci-integration.md:59
- (.andromeda/test-plan-detail/ci-integration.md:59) Any workflow step this chunk adds, such as a CI-side measurement, keeps the workflow permissions and SHA pins. `release-check.sh` keeps the runner-provided `jq` and its `tool-missing: jq` refusal.
  > Every workflow (`ci.yml`, `nightly.yml`, `windows-mutants.yml`) has `permissions: {}` at the top and `contents: read` per job. Every `uses:` is SHA-pinned; zizmor `unpinned-uses` is the assertion.

### ci-integration#1.12 · listed (lost at classify) · .andromeda/test-plan-detail/ci-integration.md:59
- (.andromeda/test-plan-detail/ci-integration.md:59) Any tool the by-section comparison needs has exactly one version source and is never installed through a second action.
  > G2 (`scripts/g2-zero-panics.sh`, in the `test` and `perf` jobs), `scripts/orphans-check.sh` (`lint`) and `scripts/release-check.sh` (`release`) use the runner-provided `jq`, never installed: the `test` and `perf` jobs presence-check it (`jq --version`) and all three scripts refuse with `tool-missing: jq`.

### ci-integration#1.12 · listed (lost at classify) · .andromeda/test-plan-detail/ci-integration.md:60
- (.andromeda/test-plan-detail/ci-integration.md:60) A dev-host measurement or gate added by this chunk installs nothing in the pre-push gate.
  > The local pre-push gate runs natively on the Linux dev host and installs nothing:

### ci-integration#1.12 · listed (lost at classify) · .andromeda/test-plan-detail/ci-integration.md:61
- (.andromeda/test-plan-detail/ci-integration.md:61) A CI job added for the measurement gets no `concurrency:` block.
  > No workflow (`ci.yml`, `nightly.yml`, `windows-mutants.yml`) has a `concurrency:` block: a concurrency group cancels pending runs even with `cancel-in-progress: false`, which would drop a push's `always()` gate and upload chain.

### quality-gates-coverage-targets#1.1 · listed (lost at classify) · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:8
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) A concurrent-first-start witness that spawns `viola` children must keep them in coverage: any test that clears the child env must re-add `LLVM_PROFILE_FILE`.
  > Child `viola` processes spawned by E2E tests are included, because `LLVM_PROFILE_FILE` propagates. Tests that call `env_clear()` must re-add it.

### quality-gates-coverage-targets#1.1 · listed (lost at classify) · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:8
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) A race control that kills an instrumented child (a racing start, an agent whose hook spawn failed) must not land the kill inside that child's exit-time profile write, or the coverage merge goes red on a short profile.
  > One mechanism for such a profile is measured (as measured at chunk 2026-10-07-a-send-ending-in-a-newline-is-confirmed, `evidence/profraw-red-green.md`): an instrumented child that exits by itself and is killed during its exit-time profile write leaves a short profile, which `llvm-profdata` refuses.

### quality-gates-coverage-targets#1.1 · listed (lost at classify) · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:9
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:9) A two-instance concurrent boot test that kills its wrappers is kill-shaped: its hooks must not be cut off in their profile write by the PTY hangup.
  > A second mechanism is measured: a test that kills the wrapper closes its PTY, and the hangup reaches the hooks the fake agent ran in that terminal's foreground process group, so a `viola hook` cut off in its exit-time profile write leaves a page-cut profile that `llvm-profdata` refuses, or none;

### quality-gates-coverage-targets#1.1 · listed (lost at classify) · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:9
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:9) A new repeated-run race case can be read for refused profiles through the census's `--case` form, a dev-host witness only, never a CI, pre-push or harness gate.
  > The witness is `scripts/profraw-census.sh <runs> <workers> [<name>]`, with a second form `--case <binary stem> <test name> <runs> <workers> [<name>]` over one integration case that offers each profile alone to `llvm-profdata merge` and counts the refused ones, a dev-host script (architecture §Occupied Resources → Repository): no CI job, `pre-push` stage or harness command runs it, and it has no unit test, only its recorded one-shot controls (must-fail on the untouched test, must-pass, and two planted readings).

### quality-gates-coverage-targets#2.0 · listed (lost at classify) · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:15
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:15) This chunk carries no mutation gate of its own; a mutation run on the `pin.rs` / `fs.rs` change is on demand only, never a chunk acceptance gate.
  > **Mutation gate (Founder Direction 1):** judged at the epoch boundary by `/andromeda-code-audit` and on demand by `run --mutants`; no chunk, pre-push or CI mutation gate exists since 2026-09-28 (the founder's ruling, 17:59).

### quality-gates-coverage-targets#2.0 · listed (lost at classify) · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:16
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:16) Any Windows-only arm the fix adds to the exe's absent-copy write (a `cfg(windows)` sharing-violation wait) is a mutant the Linux host cannot reach: recorded owed by coordinate, never equivalent or caught.
  > A mutant this host cannot compile or reach — inside a `cfg(windows)` item or block, or behind a const such as `HOST_SCRATCH = cfg!(windows)` — is recorded "not measured here; owed to {the route entry that measures it}" by coordinate, never "equivalent", and never counted as caught;

### quality-gates-coverage-targets#2.0 · listed (lost at classify) · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:17
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:17) `viola_state::fs`'s retry decision stays passed in by the caller: a change to `replace_private` / `replace_private_shared` must keep that restatement, which made the site reachable on Linux and holds its Epoch 4 kills.
  > 7 of them after a behaviour-preserving restatement that made the site reachable on Linux (`viola_state::fs`'s retry decision passed in by the caller, `viola send`'s issue-line decision a named function), each shown red under its hand-applied mutant and green restored with no mutation run made;

### quality-gates-coverage-targets#2.0 · listed (lost at classify) · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:19
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:19) A race reproduction case runs under nextest's CI profile unchanged: it must finish inside the 120 s kill with no retry.
  > Both run on ubuntu and windows; `[profile.ci]`'s 120 s kill and `retries = 0` are unchanged.

### test-data-fixtures#2.2 · listed (lost at strong) · .andromeda/test-plan-detail/test-data-fixtures.md:30
- (.andromeda/test-plan-detail/test-data-fixtures.md:30) A test that makes the fresh home itself before the two concurrent starts makes it through `viola_state::fs::create_private_dir`, never `create_dir_all`.
  > A home a test makes before viola, where the case needs it to pass the strict-modes read rule, is made through `viola_state::fs::create_private_dir`, never `create_dir_all` (a `create_dir_all` home is refused, On-disk product state above)

### test-scope-summary#7.1 · listed (lost at strong) · .andromeda/test-plan-detail/test-scope-summary.md:115
- (.andromeda/test-plan-detail/test-scope-summary.md:115) The standing test-side workaround depends on stamps, and so on `viola verify`'s `pin_exe`, which is the second caller of the absent-copy arm. Stamps must come only from `viola verify` run against the fake agent.
  > Stamps must therefore be produced by running `viola verify` against the fake agent, or the conflict must be resolved upstream. Resolved in §12 (initial entry, ratified by User review 1): the first option is used.

### test-scope-summary#7.1 · listed (lost at strong) · .andromeda/test-plan-detail/test-scope-summary.md:116
- (.andromeda/test-plan-detail/test-scope-summary.md:116) A race witness must start from no pre-seeded `bin/<key>/`, because the race exists only on a fresh home.
  > No developer-seeded state.

### test-scope-summary#7.1 · listed (lost at strong) · .andromeda/test-plan-detail/test-scope-summary.md:117
- (.andromeda/test-plan-detail/test-scope-summary.md:117) Any concurrent-pin control that makes its home itself must make it private, or the home-tree check refuses the home before the pin is reached. That would make the control vacuous.
  > A test that makes a home before viola runs makes it through `viola_state::fs::create_private_dir`, never `create_dir_all`: the instance and home-tree checks refuse a `create_dir_all` home (Unix: its 0755 mode fails the read rule; Windows: its inherited Users write ACE).

### test-strategy#1.1 · listed (lost at strong) · .andromeda/test-plan-detail/test-strategy.md:10
- (.andromeda/test-plan-detail/test-strategy.md:10) The `pin_bytes` / `pin_exe` concurrency fix in `crates/viola-state/src/pin.rs` needs inline unit cases in its own crate, and they must run on all three OSes.
  > Inline `#[cfg(test)] mod tests` in all 6 crates plus the root bin. rstest 0.27.0 `#[case]` tables cover the refusal-order matrices, the exit-code table (0/1/2/10–14/20/21) and the per-`<cli-version>` fixture replay. Runs on all 3 OSes,

### test-strategy#2.1 · listed (lost at strong) · .andromeda/test-plan-detail/test-strategy.md:42
- (.andromeda/test-plan-detail/test-strategy.md:42) Concurrent `viola run` children in a root test must be started through the one guard and runner in `tests/support/cli.rs`.
  > `cli.rs` holds the one started-child guard and runner (`Running`, `Ran`, `spawn`, `viola`), with stdout and stderr as two captures beside the exit code,

### test-strategy#2.1 · listed (lost at strong) · .andromeda/test-plan-detail/test-strategy.md:42
- (.andromeda/test-plan-detail/test-strategy.md:42) A new race witness must not define its own copy of the reader, the wait, the guard or the result struct.
  > No root test file outside `tests/support/` defines its own copy of the reader, the wait, the guard or the result struct.

### test-strategy#2.1 · listed (lost at strong) · .andromeda/test-plan-detail/test-strategy.md:43
- (.andromeda/test-plan-detail/test-strategy.md:43) A witness written as a Path binary must follow the `path<N>_<topic>.rs` naming so it reports in the E2E layer.
  > a numbered Path binary `path<N>_<topic>.rs`, e.g. `path6_budget_harness.rs`). The root package carries numbered Path binaries too (`tests/path1_run_start.rs`, sync, `[[test]]` `required-features = ["fake-agent"]`); both homes report in the E2E layer, whose regex is `^(path[0-9]*|tui|mcp|cross|chaos|contract)_` (§3 5-command `run` step 2).

## Held

- ci-integration#1.1
- ci-integration#1.11
- ci-integration#1.6
- e2e-test-strategy#14.1
- e2e-test-strategy#15.1
- e2e-test-strategy#19.1
- e2e-test-strategy#2.1
- e2e-test-strategy#2.2
- e2e-test-strategy#2.3
- e2e-test-strategy#8.1
- integration-test-strategy#1.2
- integration-test-strategy#1.5
- integration-test-strategy#2.0
- integration-test-strategy#3.2
- mocking-stubbing-discipline#2.3
- mocking-stubbing-discipline#2.5
- test-data-fixtures#1.1
- test-data-fixtures#1.3
- test-data-fixtures#3.1
- test-data-fixtures#6.1
- test-scope-summary#6.1
- test-scope-summary#8.1
- test-strategy#1.3
- test-strategy#1.6
- test-strategy#1.7
- unit-test-strategy#1.2

## Cascade only

- e2e-test-strategy#1.3
- e2e-test-strategy#8.2
- test-data-fixtures#1.2
- test-data-fixtures#1.6
- test-data-fixtures#4.1
- test-data-fixtures#5.1
- test-strategy#3.10
- test-strategy#3.13
- test-strategy#3.18
- test-strategy#3.6
