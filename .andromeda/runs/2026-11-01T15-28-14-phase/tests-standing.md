# tests · standing conventions

- S1 · Integration binaries named by the `<surface>_<topic>.rs` prefix and test functions in `<subject>_<condition>_<expected>` snake_case · §2 Test directory + naming conventions
  (.andromeda/test-plan-detail/test-strategy.md:39) naming
- S2 · Each test seeds its own fresh tempfile home under `target/e2e-home`, made through `viola_state::fs::create_private_dir` where the test makes it, with no state shared between tests · §5 Setup / teardown lifecycle and §7 Seed strategies
  (.andromeda/test-plan-detail/integration-test-strategy.md:39) integration
  (.andromeda/test-plan-detail/test-data-fixtures.md:32) data
  (.andromeda/test-plan-detail/test-scope-summary.md:113) data
  (.andromeda/test-plan-detail/test-strategy.md:12) strategy
- S3 · Env set per child with `Command::env`, never `std::env::set_var`, and `.env_clear()` re-adding `LLVM_PROFILE_FILE` · §11 Integration
  (.andromeda/test-plan-detail/e2e-test-strategy.md:213) scenario-e2-r8-claude-env-strip
  (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) gate-coverage
- S4 · No sleep for synchronisation, waits keyed on an explicit signal, nextest `retries = 0` and no `#[ignore]` parking · §11 E2E and §10 Zero-flakiness budget
  (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:15) gate-mutation
  (.andromeda/test-plan-detail/test-strategy.md:76) strategy
- S5 · The filesystem is real and never mocked, and mockall doubles exist only for traits viola already defines at its seams · §8 Mocking & Stubbing Discipline
  (.andromeda/test-plan-detail/integration-test-strategy.md:11) integration
  (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:8) mocking
  (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:14) mocking
  (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:22) mocking
- S6 · Table cases are named rstest `#[case]` rows, or labelled case tables in `viola-e2e` and literal tables in `viola-core` · §4 Conventions
  (.andromeda/test-plan-detail/integration-test-strategy.md:8) integration
- S7 · Expected exit codes and refusal words are test literals, asserted as the integer and the serde kebab-case value, never read from the product or by `Debug` string · §11 Unit
- S8 · Each OS job holds lines 85, functions 95 and regions 80, and the coverage ignore regex is never widened to pass · §10 Coverage thresholds and §11 CI
- S9 · OS-branch code is proven only on its own CI runner, a Linux-only local run proving nothing for macOS, which is CI-only · §11 Test Strategy and §9 Matrix builds
- S10 · A case sits at the lowest layer that can hold it, §6 asserting only cross-process behaviour · §11 Test Strategy
- S11 · A new diagnostics line validates against obs-plan's `diag-line` or `diag-detail` schema · §6 Schema conformance
- S12 · A started wrapper sits in a `Drop` guard and is never run through `assert_cmd` `.assert()`, a test-started `viola` drops `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN`, and root suites share `tests/support/` helpers without copying them · §11 E2E and §5 Cross-module patterns
  (.andromeda/test-plan-detail/e2e-test-strategy.md:39) scenario-path-1-run-start-sequence
