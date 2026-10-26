# tests · standing conventions

- S1 · test functions named `<subject>_<condition>_<expected>` in snake_case, rstest `#[case::label]` tables (literal tables in `viola-core`, labelled case tables in `viola-e2e`) · §2 Test Strategy naming / §4 Unit conventions (test-strategy.md, unit-test-strategy.md)
- S2 · integration binaries named `<surface>_<topic>.rs` (`cli_`, `hook_`, `security_negatives_*`, `chaos_`, `contract_`), helpers shared from `tests/support/` and never copied, live-appended files read through `tests/support/ndjson.rs` · §2 Test directory + naming conventions
  (.andromeda/test-plan-detail/test-strategy.md:39) naming
- S3 · a fresh tempfile home per test at a not-yet-existing path under `target/e2e-home` that viola creates, wrappers in a `Drop` guard, nothing shared, env per child via `Command::env`, `.env_clear()` re-adding `LLVM_PROFILE_FILE` · §5 Setup / teardown lifecycle / §7 Test data lifecycle (integration-test-strategy.md, test-data-fixtures.md)
  (.andromeda/test-plan-detail/e2e-test-strategy.md:38) scenario-path-1-run-start-sequence
  (.andromeda/test-plan-detail/e2e-test-strategy.md:151) scenario-path-5-human-takes-the-wheel-refusal-release
  (.andromeda/test-plan-detail/e2e-test-strategy.md:184) scenario-path-6-budget-governor
  (.andromeda/test-plan-detail/e2e-test-strategy.md:202) scenario-path-7-unverified-cli-version-gate
  (.andromeda/test-plan-detail/e2e-test-strategy.md:218) scenario-e2-r8-claude-env-strip
  (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) gate-coverage
- S4 · no sleep for synchronisation; waits key on `events.ndjson` offsets, `viola wait --after` or a status field · §2 Agent-runnable invariants / §11 E2E
- S5 · state files (`snapshot.json`, `ledger/stamps.json`, `budget.json`) never hand-written; a reader's own unit case plants a malformed one only by altering the writer's bytes · §7 Seed strategies / §11 Test Data
  (.andromeda/test-plan-detail/test-data-fixtures.md:12) data
- S6 · filesystem never mocked (a real tempfile home); mockall doubles only on existing seams; Windows foreign-SID / non-persistent-ACL descriptors only as synthetic SDDL stubs · §8 Mocking & Stubbing Discipline (mocking-stubbing-discipline.md)
  (.andromeda/test-plan-detail/test-strategy.md:12) strategy
- S7 · after every E2E test the error sanitization and secret scan and the diagnostics schema conformance run: code-only error payloads, no absolute path, a hit reported by place and class · §6 Non-path suites (e2e-test-strategy.md)
- S8 · coverage gated per OS (lines 85, functions 95, regions 80) and zero flakiness (`retries = 0`, no `#[ignore]` parking) · §10 Quality Gates & Coverage Targets (quality-gates-coverage-targets.md)
  (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:15) gate-mutation
- S9 · OS-branch code compiled and tested on its own CI runner, a Linux-only local run never proof for Windows or macOS · §1 multi-os-compat trigger / §9 Matrix builds (ci-integration.md)
