# tests · standing conventions

- S1 · Test functions named snake_case `<subject>_<condition>_<expected>`, critical-path scenarios `path<N>_<slug>` · .andromeda/test-plan-detail/test-strategy.md
- S2 · Integration binaries named with the `<surface>_<topic>.rs` layer prefix (`cli_`, `chaos_`, … in root; `path_`, `cross_` in `viola-e2e`) so nextest filtersets select them · .andromeda/test-plan-detail/test-strategy.md
  (.andromeda/test-plan-detail/ci-integration.md:22) ci
- S3 · Every test gets a fresh home of its own at a not-yet-existing path under `target/e2e-home`, made through `create_private_dir` when the test makes it; started wrappers sit in a `Drop` guard that kills and waits; nothing is shared · .andromeda/test-plan-detail/integration-test-strategy.md
  (.andromeda/test-plan-detail/e2e-test-strategy.md:39) scenario-path-1-run-start-sequence
  (.andromeda/test-plan-detail/e2e-test-strategy.md:152) scenario-path-5-human-takes-the-wheel-refusal-release
  (.andromeda/test-plan-detail/e2e-test-strategy.md:185) scenario-path-6-budget-governor
  (.andromeda/test-plan-detail/e2e-test-strategy.md:203) scenario-path-7-unverified-cli-version-gate
  (.andromeda/test-plan-detail/e2e-test-strategy.md:219) scenario-e2-r8-claude-env-strip
  (.andromeda/test-plan-detail/test-data-fixtures.md:30) data
- S4 · No sleep for synchronisation, waits keyed on `events.ndjson` byte offsets or logged events, nextest `retries = 0`, no `#[ignore]` parking · §2 Test Strategy and §10 Quality Gates of the body
  (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:15) gate-mutation
- S5 · Table-driven cases use labelled rstest `#[case]`s; `viola-e2e` uses a labelled case table, `viola-core` literal tables · .andromeda/test-plan-detail/unit-test-strategy.md
  (.andromeda/test-plan-detail/test-strategy.md:8) strategy
- S6 · Env set per child with `Command::env`, never `set_var`; an `.env_clear()` re-adds `LLVM_PROFILE_FILE`; a test-started `viola` drops `VIOLA_NAME` / `VIOLA_DIR` / `VIOLA_BIN` · §11 Test Anti-Patterns and .andromeda/test-plan-detail/integration-test-strategy.md
  (.andromeda/test-plan-detail/e2e-test-strategy.md:11) e2e
  (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:8) gate-coverage
- S7 · Per-OS coverage thresholds hold, with the ignore regex unwidened · .andromeda/test-plan-detail/quality-gates-coverage-targets.md
  (.andromeda/test-plan-detail/ci-integration.md:54) ci
- S8 · OS-branch code is verified only on its own CI runner, and a Linux-only local run is no proof for Windows or macOS · .andromeda/test-plan-detail/ci-integration.md
- S9 · No hand-written `stamps.json`, `snapshot.json` or `budget.json`: stamps come from `viola verify` against the fake agent · .andromeda/test-plan-detail/test-data-fixtures.md
  (.andromeda/test-plan-detail/test-data-fixtures.md:13) data
  (.andromeda/test-plan-detail/test-scope-summary.md:108) data
- S10 · Live-appended files are read through the complete-lines reader `tests/support/ndjson.rs`, and helpers are shared in `tests/support/`, never copied · .andromeda/test-plan-detail/test-strategy.md
  (.andromeda/test-plan-detail/test-strategy.md:39) naming
