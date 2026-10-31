# Cascade compare — test-plan · phase · 2026-10-31-exit-cause-code-catalogue
E items 241 · held 57 · lost at classify 3 · lost at strong 3 · beside a placed fact - · cascade only 26 · listed 3 · body 103 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### ci-integration#1.3 · lost at strong · .andromeda/test-plan-detail/ci-integration.md:19
- (.andromeda/test-plan-detail/ci-integration.md:19) The unit cases the chunk adds (code words, the reply screen, each cause's hint) must pass this local command, the pwsh shim included on Windows.
  > `scripts/agent-run.sh run --unit` (pwsh `scripts/agent-run.ps1 run --unit` on windows)

### ci-integration#1.7 · lost at classify · .andromeda/test-plan-detail/ci-integration.md:35
- (.andromeda/test-plan-detail/ci-integration.md:35) The chunk's plan must carry no mutation gate and no mutation CI job: mutation is judged at the epoch boundary and on demand only.
  > no push or pull-request job and no gate since 2026-09-28: mutation testing runs at the epoch boundary through `/andromeda-code-audit`, and on demand through `agent-run run --mutants` [`--file`] (§3 `run` step 4, §10 Mutation gate).

### ci-integration#1.8 · lost at strong · .andromeda/test-plan-detail/ci-integration.md:39
- (.andromeda/test-plan-detail/ci-integration.md:39) A `process-exit` `detail` value the chunk adds to `schemas/diag-line.v1.json` must validate in the CI schema step and leave the secret scan clean, and any hook path the chunk touches must keep the perf job's gate green.
  > `jq --version`, `run --perf`, G2 (probe then check), `schema-check`, `secret-scan`, the scan-gated uploads, then `gate --require perf`.

### ci-integration#1.9 · lost at strong · .andromeda/test-plan-detail/ci-integration.md:43
- (.andromeda/test-plan-detail/ci-integration.md:43) The chunk's code and its unit cases must compile and pass on the 1.96 floor, not only on the pinned stable.
  > ubuntu `msrv`: `rustup toolchain install 1.96 --profile minimal`, then `RUSTUP_TOOLCHAIN=1.96` steps for `rustc --version`, `cargo check --workspace` and `agent-run run --unit`, then `gate --require nextest-unit`

### ci-integration#1.11 · lost at classify · .andromeda/test-plan-detail/ci-integration.md:51
- (.andromeda/test-plan-detail/ci-integration.md:51) Any test-only seam a table case needs to drive a cause (an internal error, an endpoint lost mid-call) must stay out of the release build, which this check fails on.
  > It prints `release-check: viola only` or fails on an artifact built with a test-only feature (`test-support`, `fake-agent`), on a test-only binary or on no executable, and never lists `target/release/`.

### quality-gates-coverage-targets#2.0 · lost at classify · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:15
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:15) The chunk's acceptance must name no mutation gate of its own: its new code is scored by the epoch-boundary audit or an on-demand run.
  > **Mutation gate (Founder Direction 1):** judged at the epoch boundary by `/andromeda-code-audit` and on demand by `run --mutants`; no chunk, pre-push or CI mutation gate exists since 2026-09-28 (the founder's ruling, 17:59).

### quality-gates-coverage-targets#2.0 · lost at classify · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:15
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:15) New code-word and cause-mapping logic must be held by a nextest case, because a doctest alone leaves its mutant missed at the audit that scores this code.
  > A mutant caught only by a doctest counts as missed under nextest, so the missing unit test must be added.

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### integration-test-strategy#3.0 · listed (lost at strong) · .andromeda/test-plan-detail/integration-test-strategy.md:40
- (.andromeda/test-plan-detail/integration-test-strategy.md:40) Every table case that drives a check-guarded path on all OSes must start from this home, never one a test makes with plain `create_dir_all`.
  > A case that drives a check-guarded path on every OS starts from `stamped_home` (§7 Seed strategies).

### test-data-fixtures#1.3 · listed (lost at classify) · .andromeda/test-plan-detail/test-data-fixtures.md:12
- (.andromeda/test-plan-detail/test-data-fixtures.md:12) Any stamped state the table needs must come from `viola verify` against the fake agent, and no new test code may write the stamps file.
  > `viola verify --home <home>` run against the fake agent. No test or harness code writes the file.

### test-scope-summary#7.1 · listed (lost at classify) · .andromeda/test-plan-detail/test-scope-summary.md:122
- (.andromeda/test-plan-detail/test-scope-summary.md:122) Any table case that makes a home before viola runs must make it through the named helper, or the case reads a refusal it did not plant.
  > A test that makes a home before viola runs makes it through `viola_state::fs::create_private_dir`, never `create_dir_all`: the instance and home-tree checks refuse a `create_dir_all` home (Unix: its 0755 mode fails the read rule; Windows: an inherited write grant to a SID strict-modes does not trust, whichever SID holds it).

## Held

- ci-integration#1.1
- ci-integration#1.4
- ci-integration#1.5
- ci-integration#1.6
- e2e-test-strategy#1.1
- e2e-test-strategy#1.5
- e2e-test-strategy#11.1
- e2e-test-strategy#12.1
- e2e-test-strategy#13.1
- e2e-test-strategy#14.1
- e2e-test-strategy#15.1
- e2e-test-strategy#17.1
- e2e-test-strategy#19.1
- e2e-test-strategy#2.2
- e2e-test-strategy#4.1
- e2e-test-strategy#4.3
- integration-test-strategy#1.3
- integration-test-strategy#1.4
- integration-test-strategy#1.5
- integration-test-strategy#4.1
- integration-test-strategy#4.2
- integration-test-strategy#5.1
- mocking-stubbing-discipline#1.1
- mocking-stubbing-discipline#2.3
- mocking-stubbing-discipline#2.4
- mocking-stubbing-discipline#2.5
- quality-gates-coverage-targets#1.1
- quality-gates-coverage-targets#3.1
- test-data-fixtures#1.1
- test-data-fixtures#1.10
- test-data-fixtures#1.15
- test-data-fixtures#1.2
- test-data-fixtures#1.6
- test-data-fixtures#1.7
- test-data-fixtures#2.1
- test-data-fixtures#2.2
- test-data-fixtures#6.1
- test-scope-summary#1.1
- test-scope-summary#10.1
- test-scope-summary#3.1
- test-scope-summary#4.1
- test-scope-summary#5.3
- test-scope-summary#6.1
- test-scope-summary#8.1
- test-strategy#1.1
- test-strategy#1.2
- test-strategy#1.7
- test-strategy#2.1
- test-strategy#3.1
- test-strategy#3.16
- test-strategy#3.6
- unit-test-strategy#1.1
- unit-test-strategy#1.2
- unit-test-strategy#1.3
- unit-test-strategy#1.4
- unit-test-strategy#1.5
- unit-test-strategy#2.1

## Cascade only

- e2e-test-strategy#1.4
- e2e-test-strategy#2.1
- e2e-test-strategy#2.3
- e2e-test-strategy#3.1
- e2e-test-strategy#4.2
- e2e-test-strategy#6.1
- e2e-test-strategy#6.3
- e2e-test-strategy#7.1
- e2e-test-strategy#7.2
- e2e-test-strategy#7.3
- integration-test-strategy#1.2
- integration-test-strategy#1.6
- mocking-stubbing-discipline#2.2
- test-data-fixtures#1.14
- test-scope-summary#5.2
- test-strategy#1.3
- test-strategy#1.5
- test-strategy#1.6
- test-strategy#3.10
- test-strategy#3.12
- test-strategy#3.13
- test-strategy#3.18
- test-strategy#3.3
- test-strategy#3.4
- test-strategy#3.5
- test-strategy#3.7
