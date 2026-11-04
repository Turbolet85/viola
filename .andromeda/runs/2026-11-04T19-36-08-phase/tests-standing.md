# tests · standing conventions

- S1 · a new integration test binary takes the surface_topic prefix (cli_, hook_, tui_, channel_, chaos_, contract_ in root; path_, mcp_, cross_ in viola-e2e) so the layer filtersets select it · test-strategy
- S2 · a new test function is named subject_condition_expected in snake_case, a critical-path scenario pathN_slug · ## 2. Test Strategy
- S3 · each test makes its own home at a not-yet-existing path under target/e2e-home (through create_private_dir, never create_dir_all), started wrappers sit in a Drop guard that kills and waits, and nothing is shared between tests · integration-test-strategy
  (test-plan/e2e-test-strategy fct-4n3xwm4s) scenario-path-1-run-start-sequence
  (test-plan/e2e-test-strategy fct-2rmlc6ik) scenario-path-7-unverified-cli-version-gate
  (test-plan/e2e-test-strategy fct-tmgp6fr5) scenario-e2-r8-claude-env-strip
  (test-plan/test-scope-summary fct-6upjfbpd) data
- S4 · waits key on events.ndjson byte offsets, viola wait --after or a status field, never a sleep; a live-appended file is read only through the complete-lines reader tests/support/ndjson.rs · ## 2. Test Strategy
- S5 · env is set per child with Command::env, never std::env::set_var, and any env_clear re-adds LLVM_PROFILE_FILE; a test-started viola drops VIOLA_NAME, VIOLA_DIR and VIOLA_BIN · ## 11. Test Anti-Patterns
  (test-plan/e2e-test-strategy fct-ennvbmje) e2e
- S6 · zero flakiness: nextest retries stay 0, a flaking test keeps the chunk red until its root cause is fixed, and #[ignore] is no parking place · quality-gates-coverage-targets
- S7 · verdicts come from hook events, events.ndjson, channel payloads and the fake-agent receipt, never a parse of the rendered child screen; child exit is read only via child.wait(); assert_cmd .assert() is never used on a long-lived viola run wrapper · ## 11. Test Anti-Patterns
  (test-plan/e2e-test-strategy fct-pyb4ohg2) scenario-path-5-human-takes-the-wheel-refusal-release
- S8 · stamps.json, snapshot.json and budget.json are never hand-written: stamps come from viola verify against the fake agent, snapshots from the wrapper, budget from viola hook statusline · test-data-fixtures
  (test-plan/test-data-fixtures fct-jg3hj3ma) data
  (test-plan/test-data-fixtures fct-4ejdt5qs) data
- S9 · shared root-test helpers live once in tests/support and no root test file copies them; viola-e2e tables are labelled case tables, not rstest · ## 2. Test Strategy
  (test-plan/test-strategy fct-hckuzazj) naming
- S10 · a case is asserted at the lowest layer that can hold it, and each surface a scenario lists gets its own driver step and signal · ## 11. Test Anti-Patterns
- S11 · a CI job or step keeps permissions {} at the top and contents read per job, SHA-pinned uses, no concurrency block, rust-cache as the only cache, and its uploads behind the secret scan with retention-days 7 · ci-integration
