# tests · standing conventions

- S1 · a new root integration file takes a `<surface>_<topic>.rs` prefix (`cli_`, `hook_`, `channel_`, `chaos_`, `security_negatives_`; `mcp_`, `cross_`, `path_` in viola-e2e) · .andromeda/test-plan-detail/test-strategy.md
  (.andromeda/test-plan-detail/test-strategy.md:39) naming
- S2 · test functions are snake_case `<subject>_<condition>_<expected>`, table cases rstest `#[case::label]` (viola-e2e a labelled case table, viola-core a literal table) · .andromeda/test-plan-detail/unit-test-strategy.md
- S3 · each test owns a fresh home at a not-yet-existing path under `target/e2e-home`, started wrappers in a `Drop` guard, nothing shared · .andromeda/test-plan-detail/integration-test-strategy.md
  (.andromeda/test-plan-detail/e2e-test-strategy.md:38) scenario-path-1-run-start-sequence
  (.andromeda/test-plan-detail/test-scope-summary.md:104) data
- S4 · no sleep for synchronisation; waits key on `events.ndjson` offsets or `viola wait --after`, read through `tests/support/ndjson.rs` · .andromeda/test-plan-detail/test-strategy.md
- S5 · never hand-write `snapshot.json`, `stamps.json` or `budget.json`; a malformed state file only by altering the writer's own bytes · .andromeda/test-plan-detail/test-data-fixtures.md
  (.andromeda/test-plan-detail/test-data-fixtures.md:11) data
  (.andromeda/test-plan-detail/test-data-fixtures.md:12) data
- S6 · env per child via `Command::env`, `.env_clear()` re-adds `LLVM_PROFILE_FILE`, a test-started `viola` drops `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN`, path oracles on `resolved(home)` · .andromeda/test-plan-detail/integration-test-strategy.md
  (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) gate-coverage
- S7 · literal oracles for exit codes and refusal codes, compared by integer and kebab value, never imported from the product or by `Debug` · .andromeda/test-plan-detail/unit-test-strategy.md
- S8 · zero flakes: nextest `retries = 0`, no `#[ignore]` parking, a flake keeps the chunk red · .andromeda/test-plan-detail/quality-gates-coverage-targets.md
  (.andromeda/test-plan-detail/test-strategy.md:75) strategy
- S9 · per-OS coverage gates (lines 85, functions 95, regions 80) with the fixed ignore regex · .andromeda/test-plan-detail/quality-gates-coverage-targets.md
- S10 · after every E2E test the error-sanitization and secret scan and the diag schema conformance check run · .andromeda/test-plan-detail/e2e-test-strategy.md
