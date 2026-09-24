# tests extract

## Relevance
Relevant. This chunk changes the harness (the §3 contract surface), the fake agent (§7), and test modules and `tests/`. It adds one guard test and falls under the §10 mutation gate for a Rust delta.

## Constraints
- The split of `run.rs` and the `main` decompositions must leave the harness contract byte-identical. Per test-plan §3 (preamble, Exit codes), every harness command prints exactly one JSON document starting `{"v":1,"cmd":…,"ok":…}`, with key order held by serde_json `preserve_order`. It exits 0/1/2, and a usage error carries `reason:"usage"` plus a fixed `detail`. Per test-plan §3 Closed enums, no new `reason`/`detail`/`suite`/`verdict` value may appear without a §12 Decisions Log entry. Per test-plan §3 (Harness implementation), the shims forward stdout and exit code unchanged, so all logic stays once in Rust.
- Per test-plan §3 `run` step 2, `viola_e2e::harness::{boot, cleanup}` is the library API that `harness_session` and `--perf` call, with a binary-dir parameter. The split must keep that public path and signature. Whether other consumers reach deeper module paths is research's question.
- Mutant ownership. Per test-plan §3 `run` step 4 (Test scope), `--test-workspace` is not set, so each mutant is tested only by the tests of the package that owns the mutated file:
  - The `MAX_FRAME` mutants (`crates/viola-core/src/lib.rs:9`) must be killed by `viola-core`'s own tests. A boundary witness at the root consumer `src/obs.rs:195` cannot count toward those mutants.
  - Mutants from the `harness/` split and in `viola-harness` must be killed by `viola-e2e`'s tests.
  - Mutants in `src/bin/viola-fake-agent.rs` must be killed by root-package tests.
- Mutation gate. Per test-plan §10 Mutation gate and §3 `run` step 4 (Classification, Verdict), a diff naming any `.rs` path, test files included, is `counted`. It requires `missed == 0 && timeout == 0` from a fresh `mutants.out/outcomes.json`. In CI the verdict is the union of the `ubuntu-latest` and `windows-2025` legs, and a `#[cfg(windows)]` body is killed only on the leg that compiles it. Whether cargo-mutants generates mutants inside `#[cfg(test)]` modules is research's question.
- Timeout budget for new mutant-killing tests. Per test-plan §3 Bootstrap phases (test-runner-install, `[profile.mutants]`):
  - the root slow-timeout is 5 s × 2, and the `package(viola-e2e)` override is 15 s × 2;
  - cargo-mutants' auto-timeout is about `max(20 s, 5 × baseline)`;
  - any in-test wait a mutant can reach must stay below the floor, or a hang grades Timeout (red) instead of caught.
- Unit test placement. Per test-plan §4 Conventions, unit tests are inline `#[cfg(test)] mod tests`, named `<subject>_<condition>_<expected>`, and rstest `#[case::<label>]` tables use readable labels.
- Rust gates and coverage. Per test-plan §9 Lint row and §9 Build failure conditions, the gates are `cargo fmt --check`, `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings`, and `scripts/orphans-check.sh` ending `N/N targets clean` for every lib/bin target. Every new `harness/` submodule must therefore be wired into the module tree. Per test-plan §10 Coverage thresholds, lines 85 / functions 95 / regions 80 per OS still apply to non-ignored code (`viola-core`, root `src/`).

## Patterns to follow
- Per test-plan §2 (Test directory conventions), shared root E2E helpers go in `tests/support/{home.rs,…}`. This is the planned home for the `tests/cli_fake_agent.rs` clone extraction. Root `tests/support/` cannot be imported by `viola-e2e` (per test-plan §3 `run` step 2).
- Planted-red proof. Per test-plan §7 Fixture hygiene, planted inputs prove each checker class red. This matches the operator's remove-the-guard mutation run for the `MAX_FRAME` pin: mutate or remove the guard, show the test fails, restore it.
- The diag-detail schema-validator clone (`main.rs:293`↔`obs.rs:662`) loads the same schema that the harness `schema-check` validates against: `schemas/diag-detail.v1.json` via jsonschema 0.57.0 (per test-plan §3 Internal harness subcommands, `schema-check`). A shared loader must keep that single schema source.
- The panic-line assert clone (`main.rs:218`↔`obs.rs:521`) asserts the one-line `level:"ERROR", event:"panic"` contract (per test-plan §3 Log format, Agent parsing). A shared helper keeps asserting that exact contract.
- Per test-plan §3 `run` Output format, the `run` document lists `suites[]` with `passed/failed/skipped/survived/artifact`, plus the top-level `mutants` object and the `--leg` verdict file. This is the behaviour a `run.rs` split must reproduce exactly, so tests that pin these shapes are the natural way to kill the newly in-scope `viola-e2e` mutants.

## Anti-patterns to avoid
- Per test-plan §11 Unit, never import the product value as the test oracle. The `MAX_FRAME` pin compares against a literal (`16 << 20` / 16 MiB), not an expression derived from the constant. Also never test private implementation details: kill the extracted-helper mutants through public or observable harness behaviour wherever that is feasible.
- Per test-plan §11 Quality:
  - never trust the cargo-mutants exit code alone (exit 3 masks exit 2); count `missed`/`timeout`;
  - never skip a gate "just this once". This covers closing the deferred Rust gates cold.
- Per test-plan §11 CI, never widen the coverage `--ignore-filename-regex` (`viola-fake-agent|crates[/\\]viola-e2e|tests[/\\]support|fuzz[/\\]`) to pass a gate. Per test-plan §11 E2E / §10 Zero-flakiness budget, no `sleep` for synchronisation and no retries in the new tests.

## Contract bindings
- The 5-command documents bind to obs. Per test-plan §3 Status endpoint shape and §3 Log format (with §3 Bootstrap phases, log-format-bind-with-obs), the `status` shape and the `logs` wrapper shape (`{"src":"events"|"diag",…}`) are read by obs-plan §3/§9. A refactor that changes them is a harness break and an obs break at once.
- Per test-plan §3 Log format (process logs), the panic-line and diag-detail test helpers bind to obs-plan D-08 (the detail-file split) and the `schemas/diag-detail.v1.json` / `schemas/diag-line.v1.json` schemas.
- `MAX_FRAME` (16 MiB, per test-plan §1 Coverage scope, `viola-channel` entity) binds to the security-plan `Read::take(MAX_FRAME)` invariant that the scope cites.
- Mutation cost binds to CI. Per test-plan §9 Pipeline structure (Mutation row), the two `mutants (<os>)` legs plus the `mutants-verdict` union job carry the newly in-scope `viola-e2e` mutants.

## Acceptance criteria contributions
- A `viola-core` inline unit test asserts `MAX_FRAME` equals a literal 16 MiB. Its remove-the-guard mutation run shows it red, then green. The `lib.rs:9` mutants grade `caught` by `viola-core`'s own tests (per test-plan §3 `run` step 4 Test scope; §11 Unit).
- `bash scripts/agent-run.sh run --mutants` reports `"verdict":"counted"` with `missed == 0` and `timeout == 0`. CI `gate --require mutants --mutants-legs ubuntu-latest,windows-2025` exits 0 (per test-plan §10 Mutation gate; §9 Mutation row).
- `cargo fmt --check`, clippy `-D warnings`, `scripts/orphans-check.sh` (`N/N targets clean`) and `bash scripts/agent-run.sh run --unit` / `--integration` all pass with no deferral. Per-OS coverage holds lines ≥ 85 / functions ≥ 95 / regions ≥ 80 (per test-plan §9 Build failure conditions; §10 Coverage thresholds).
- The harness commands `boot`/`run`/`status`/`cleanup`/`logs` and the internal subcommands keep the following unchanged, with no new closed-enum value:
  - one JSON document per invocation, key order `v,cmd,ok`;
  - exit codes 0/1/2;
  - the `reason`/`detail` codes.

  (Per test-plan §3 preamble Exit codes; §3 Closed enums.)

## Relevant amendment history
- **2026-09-24-three-os-ci-headless-harness-skeleton, harness builds in its own target dir:** `CARGO_TARGET_DIR=target/harness`, `--features viola/fake-agent`, and the fake agent copied from `<bin dir>`. The reason was Windows `os error 5` relinking a running `viola-harness.exe`. The `run.rs` split must keep this target-dir discipline.
- **Same chunk, interim supervisor/readiness/status/cleanup:** key order held by `preserve_order`; usage errors as `reason:"usage"` + `detail`; interim `null` fields; "no field false" cleanup. These are the exact shapes the refactor must hold byte-identical.
- **Same chunk, mutation gate prebuilds the root bins:** `run --mutants` prebuilds `viola` + `viola-fake-agent` and passes `--copy-target=true`. This was added because a diff touching only `crates/viola-e2e` failed its baseline (exit 4). That is exactly this chunk's diff shape. The cost is one 2.9 GB `target/` copy per run.
- **2026-09-24-fake-agent-and-test-data-fixtures:**
  - mutation verdict `counted` | `no-rust-delta`, with stale `outcomes.json` deleted first;
  - the fake-agent contract as built: hooks from `<plugin-dir>/hooks/hooks.json`, receipt kinds, script schema, modes;
  - the `fake-agent` `main` decomposition must preserve these script and receipt semantics.
- **2026-09-24-observability-gates:** `[profile.mutants]` is 5 s × 2 with a `viola-e2e` 15 s × 2 override, and root `booted_wrapper` readiness is 10 s and exit-aware. The reason was a mutants red from waits spinning to cargo-mutants' 20 s floor. New tests that kill `run.rs` mutants must respect these bounds.
- **2026-09-24-quality-gates:**
  - `run --leg`, `detail` codes, llvm-cov failure codes, `--fuzz-replay`, `gate --mutants-legs` union, separator-agnostic `COVERAGE_IGNORE`;
  - this is more `run`-command behaviour the split must reproduce;
  - it also records the cargo-mutants `#[cfg]` limitation behind the two-leg union.
