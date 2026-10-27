# tests · standing conventions

- S1 · Test functions named `<subject>_<condition>_<expected>`; integration binaries by the `<surface>_<topic>.rs` prefix (`cli_`, `hook_`, `channel_`, `chaos_`, … in root; `path_`, `mcp_`, `cross_` in `viola-e2e`) · .andromeda/test-plan-detail/test-strategy.md
  (.andromeda/test-plan-detail/test-strategy.md:39) naming
- S2 · Table-driven cases as rstest `#[case::readable_label]`, except `viola-core` (literal tables in one `#[test]`) and `viola-e2e` (a labelled case table, no rstest edge) · .andromeda/test-plan-detail/unit-test-strategy.md
- S3 · Every proptest block at `cases: 512`, its seeds kept in committed `proptest-regressions/` · .andromeda/test-plan-detail/test-strategy.md
- S4 · Each test makes its own fresh home through `viola_state::fs::create_private_dir` at a not-yet-existing path under `target/e2e-home`, with started wrappers held by a `Drop` guard and nothing shared between tests · .andromeda/test-plan-detail/integration-test-strategy.md
  (.andromeda/test-plan-detail/e2e-test-strategy.md:39) scenario-path-1-run-start-sequence
  (.andromeda/test-plan-detail/test-data-fixtures.md:30) data
- S5 · Child env is set per child with `Command::env`, never `std::env::set_var`; `.env_clear()` re-adds `LLVM_PROFILE_FILE`; a test-started `viola` drops `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN` · .andromeda/test-plan-detail/integration-test-strategy.md
  (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) gate-coverage
- S6 · No sleep for synchronisation: waits key on `events.ndjson` byte offsets or `viola wait --after`, and live-appended files are read only through `tests/support/ndjson.rs` · .andromeda/test-plan-detail/test-strategy.md
- S7 · Oracles are literals in the test (refusal codes, exit codes, the `MAX_FRAME` boundary), never imported from the product's own lists; refusals are compared by serde kebab value and integer exit, never `Debug` · §11 Test Anti-Patterns of the body
  (.andromeda/test-plan-detail/test-data-fixtures.md:19) data
- S8 · Each new fixture or fake script holds synthetic text only and passes the fixture-hygiene walk (path, username, email, schema) · .andromeda/test-plan-detail/test-data-fixtures.md
  (.andromeda/test-plan-detail/test-data-fixtures.md:78) data-fixture-hygiene
- S9 · Every E2E test is followed by the error-sanitisation and secret scan and the diagnostics schema conformance check · .andromeda/test-plan-detail/e2e-test-strategy.md
  (.andromeda/test-plan-detail/e2e-test-strategy.md:215) scenario-e2-r8-claude-env-strip
  (.andromeda/test-plan-detail/e2e-test-strategy.md:258) suite-error-sanitization
- S10 · Coverage gates hold per OS (lines 85, functions 95, regions 80), and the coverage ignore regex is never widened · .andromeda/test-plan-detail/quality-gates-coverage-targets.md
- S11 · Zero flakiness: nextest `retries = 0`, no `#[ignore]` parking, and a flake keeps the chunk red until its root cause is fixed · .andromeda/test-plan-detail/quality-gates-coverage-targets.md
