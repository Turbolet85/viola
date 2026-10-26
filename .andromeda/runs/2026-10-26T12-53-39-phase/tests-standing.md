# tests · standing conventions

- S1 · Integration binaries named `<surface>_<topic>.rs` (`cli_`, `channel_`, `security_negatives_` in root), so the layer filtersets select them · §2 Test Strategy / .andromeda/test-plan-detail/test-strategy.md
  (.andromeda/test-plan-detail/ci-integration.md:22) ci
  (.andromeda/test-plan-detail/test-strategy.md:39) naming
- S2 · Test functions named snake_case `<subject>_<condition>_<expected>`; rstest `#[case::label]` tables carry readable labels · §2 Test Strategy, §4 Unit Test Strategy / .andromeda/test-plan-detail/unit-test-strategy.md
- S3 · Each test gets a fresh tempfile home at a not-yet-existing path under `target/e2e-home`, and started wrappers sit in a `Drop` guard; nothing is shared between tests · §5 Integration Test Strategy / .andromeda/test-plan-detail/integration-test-strategy.md
  (.andromeda/test-plan-detail/e2e-test-strategy.md:38) scenario-path-1-run-start-sequence
  (.andromeda/test-plan-detail/e2e-test-strategy.md:151) scenario-path-5-human-takes-the-wheel-refusal-release
  (.andromeda/test-plan-detail/e2e-test-strategy.md:202) scenario-path-7-unverified-cli-version-gate
  (.andromeda/test-plan-detail/e2e-test-strategy.md:218) scenario-e2-r8-claude-env-strip
  (.andromeda/test-plan-detail/integration-test-strategy.md:32) integration
  (.andromeda/test-plan-detail/test-scope-summary.md:105) data
- S4 · The endpoint is taken only from `snapshot.json`; never interprocess `try_overwrite` or a reused endpoint name · §5, §11 Integration
  (.andromeda/test-plan-detail/e2e-test-strategy.md:13) e2e
  (.andromeda/test-plan-detail/integration-test-strategy.md:17) integration
- S5 · Env goes per child through `Command::env`, never `std::env::set_var`; `.env_clear()` re-adds `LLVM_PROFILE_FILE`; a test-started `viola` drops `VIOLA_NAME` / `VIOLA_DIR` / `VIOLA_BIN`, with a path oracle on `resolved(home)` · §5, §11 Integration
  (.andromeda/test-plan-detail/e2e-test-strategy.md:11) e2e
  (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) gate-coverage
- S6 · Waits key on `events.ndjson` byte offsets through the complete-lines reader `tests/support/ndjson.rs`, never a sleep; root suites share `tests/support/` helpers and copy none · §2 Test Strategy, §11 E2E
- S7 · Channel frames are validated by jsonschema validators built from the product's schemars schemas · §5 Integration Test Strategy / .andromeda/test-plan-detail/integration-test-strategy.md
- S8 · On-disk product state comes only from real `viola` runs, so `snapshot.json` and `stamps.json` are never hand-written; a home viola created starts from `stamped_home` · §7 Test Data & Fixtures, §11 Test Data
  (.andromeda/test-plan-detail/test-data-fixtures.md:11) data
- S9 · After every E2E test, the error-sanitization and secret scan and the diagnostics schema conformance run · §6 / .andromeda/test-plan-detail/e2e-test-strategy.md
  (.andromeda/test-plan-detail/e2e-test-strategy.md:254) suite-error-sanitization
- S10 · Zero flakiness: nextest `retries = 0`, no `#[ignore]` parking, and coverage gates held per OS · §10 Quality Gates & Coverage Targets / .andromeda/test-plan-detail/quality-gates-coverage-targets.md
  (.andromeda/test-plan-detail/ci-integration.md:30) ci
  (.andromeda/test-plan-detail/ci-integration.md:54) ci
  (.andromeda/test-plan-detail/test-strategy.md:75) strategy
- S11 · OS-branch behaviour is verified on its own CI runner, and a Linux-only local run proves nothing for Windows or macOS · §1 multi-os-compat trigger, §11 Test Strategy
