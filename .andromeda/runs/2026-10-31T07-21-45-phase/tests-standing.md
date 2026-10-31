# tests · standing conventions

- S1 · A new root or `viola-e2e` test file takes its `<surface>_<topic>.rs` prefix, a test function `<subject>_<condition>_<expected>` in snake_case · §2 Test directory + naming conventions; test-strategy.md
- S2 · A test owns a fresh home at a not-yet-existing path under `target/e2e-home`, made by viola or `create_private_dir`, its started wrappers in a `Drop` guard, nothing shared between tests · §5 Setup / teardown lifecycle, §7 Seed strategies; integration-test-strategy.md
  (.andromeda/test-plan-detail/e2e-test-strategy.md:39) scenario-path-1-run-start-sequence
  (.andromeda/test-plan-detail/e2e-test-strategy.md:92) scenario-path-3-wait-last-event-driven-readback-6-live-flow
  (.andromeda/test-plan-detail/e2e-test-strategy.md:204) scenario-path-7-unverified-cli-version-gate
  (.andromeda/test-plan-detail/e2e-test-strategy.md:220) scenario-e2-r8-claude-env-strip
  (.andromeda/test-plan-detail/integration-test-strategy.md:39) integration
- S3 · A child's env is set by `Command::env`, never `set_var`; `.env_clear()` re-adds `LLVM_PROFILE_FILE`; a test-started `viola` drops `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN` · §5 Cross-module patterns, §11 Integration
  (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) gate-coverage
- S4 · No sleep or test-owned timer for synchronisation, and verdicts come from exit codes, `events.ndjson`, channel payloads and receipts, never the rendered child screen · §2 Agent-runnable invariants, §11 E2E and Universal
  (.andromeda/test-plan-detail/test-scope-summary.md:52) scope-surfaces
  (.andromeda/test-plan-detail/test-strategy.md:75) strategy
- S5 · Oracles are literals in the test, never the product's own tables; codes are compared by serde kebab-case value and integer exit code, never by `Debug` · §11 Unit
- S6 · A case sits at the lowest layer that can hold it, and §6 asserts only cross-process behaviour · §11 Test Strategy, §2 Test pyramid
- S7 · Table-driven cases are rstest `#[case]` with readable labels; `viola-e2e` uses a labelled case table and `viola-core` a literal table, adding no manifest edge · §4 Conventions; unit-test-strategy.md
- S8 · Root tests share `tests/support/` helpers uncopied, and a reader of a live-appended file goes through `tests/support/ndjson.rs` · §2 Directory pattern; test-strategy.md
  (.andromeda/test-plan-detail/test-strategy.md:39) naming
- S9 · `ledger/stamps.json`, `snapshot.json` and `budget.json` are never hand-written, and a reader's unit case alters only the writer's own bytes · §7 Seed strategies, §11 Test Data; test-data-fixtures.md
  (.andromeda/test-plan-detail/test-data-fixtures.md:12) data
  (.andromeda/test-plan-detail/test-data-fixtures.md:13) data
- S10 · A new or changed diagnostics line validates against obs-plan's schema, and a failing check prints place and keyword, never content · §6 Schema conformance; e2e-test-strategy.md
- S11 · A new test runs on all three CI OSes, an OS branch being verified on its own runner, never by a Linux-only local run · §1 multi-os-compat trigger, §11 Test Strategy; ci-integration.md
  (.andromeda/test-plan-detail/ci-integration.md:18) ci
  (.andromeda/test-plan-detail/ci-integration.md:22) ci
  (.andromeda/test-plan-detail/ci-integration.md:26) ci
- S12 · New code stays inside the per-OS coverage gates, `retries = 0`, and no `#[ignore]` as a parking place · §10 Coverage thresholds and Zero-flakiness budget; quality-gates-coverage-targets.md
  (.andromeda/test-plan-detail/ci-integration.md:54) ci
  (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:15) gate-mutation
