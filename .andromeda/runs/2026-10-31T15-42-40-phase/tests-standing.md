# tests · standing conventions

- S1 · Each new test takes its own fresh tempfile home at a not-yet-existing path under `target/e2e-home`, with started wrappers in a `Drop` guard and the Windows x64 companion seeding as written · §5 Setup / teardown lifecycle, .andromeda/test-plan-detail/integration-test-strategy.md
  (.andromeda/test-plan-detail/e2e-test-strategy.md:39) scenario-path-1-run-start-sequence
  (.andromeda/test-plan-detail/e2e-test-strategy.md:92) scenario-path-3-wait-last-event-driven-readback-6-live-flow
  (.andromeda/test-plan-detail/e2e-test-strategy.md:152) scenario-path-5-human-takes-the-wheel-refusal-release
  (.andromeda/test-plan-detail/e2e-test-strategy.md:204) scenario-path-7-unverified-cli-version-gate
  (.andromeda/test-plan-detail/e2e-test-strategy.md:220) scenario-e2-r8-claude-env-strip
  (.andromeda/test-plan-detail/integration-test-strategy.md:39) integration
- S2 · Test functions and files follow `<subject>_<condition>_<expected>` and `<surface>_<topic>.rs`, and each rstest case name is a readable label · §2 naming conventions and §4 Conventions
- S3 · Root test files share `tests/support/` and copy none of it, and a reader of a live-appended file goes through `tests/support/ndjson.rs` · §2 Directory pattern
- S4 · A test-started `viola` drops `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN`, env is set per child with `Command::env`, `.env_clear()` re-adds `LLVM_PROFILE_FILE`, and a path oracle compares `resolved(home)` · §5 CLI pattern and §11 Integration
  (.andromeda/test-plan-detail/e2e-test-strategy.md:11) e2e
  (.andromeda/test-plan-detail/e2e-test-strategy.md:212) scenario-e2-r8-claude-env-strip
- S5 · Expected exit codes and refusal values are literals in the test, never imported from the product, asserted as the integer and the serde kebab-case value, never by `Debug` string · §11 Unit
- S6 · No sleep for synchronisation, waits key on logged events or a status field, and nothing parses the rendered child screen · §11 E2E and §2 Agent-runnable invariants
- S7 · On-disk product state comes only from real `viola` runs, a test-made home goes through `viola_state::fs::create_private_dir`, and `stamps.json`, `snapshot.json` and `budget.json` are never hand-written · §7 Seed strategies and §11 Test Data
  (.andromeda/test-plan-detail/test-data-fixtures.md:12) data
  (.andromeda/test-plan-detail/test-data-fixtures.md:13) data
  (.andromeda/test-plan-detail/test-scope-summary.md:113) data
- S8 · Each case sits at the lowest layer that can hold it, and §6 asserts only cross-process behaviour · §11 Test Strategy and §2 Test pyramid
- S9 · Every OS-specific branch compiles and is tested on its own runner, and a Linux run proves nothing for Windows or macOS · §11 Test Strategy and §9 Matrix builds
- S10 · No retries, no `#[ignore]` as a parking place, and a flake keeps the chunk red until its root cause is fixed · §10 Zero-flakiness budget
- S11 · New code stays inside the per-OS coverage gates of 85 lines, 95 functions and 80 regions, with no threshold lowered and no ignore regex widened · §10 Coverage thresholds and §11 CI
- S12 · Each new or changed process-log line validates against its schema and holds no `Cookie` or stripped-variable value, and a failure reports by place and class, never the bytes · §6 Schema conformance and Error sanitization
  (.andromeda/test-plan-detail/e2e-test-strategy.md:215) scenario-e2-r8-claude-env-strip
