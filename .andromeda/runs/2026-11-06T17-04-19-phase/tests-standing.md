# tests · standing conventions

- S1 · each new Rust integration test binary takes its surface_topic prefix (cli_, hook_, tui_, channel_, chaos_, contract_ in root; path_, mcp_, cross_ in viola-e2e) so the layer filtersets select it · ## 2. Test Strategy
- S2 · each new test function is named subject_condition_expected in snake_case, and table cases carry readable labels · ## 4. Unit Test Strategy
- S3 · each new test makes its own fresh home under target/e2e-home through viola_state::fs::create_private_dir, shares nothing, and seeds no state by hand · ## 7. Test Data & Fixtures
  (test-plan/integration-test-strategy fct-qmrx4mno) integration
  (test-plan/mocking-stubbing-discipline fct-pwscsozw) mocking
  (test-plan/test-data-fixtures fct-bxlupfu3) data
  (test-plan/test-data-fixtures fct-jg3hj3ma) data
  (test-plan/test-data-fixtures fct-ubb4ig4c) data
  (test-plan/test-data-fixtures fct-fvd26vjh) data
  (test-plan/test-scope-summary fct-6upjfbpd) data
- S4 · a new test never sleeps for synchronisation; its waits key on events.ndjson byte offsets or wait --after, and it reads live-appended files only through tests/support/ndjson.rs · ## 11. Test Anti-Patterns
- S5 · a new test sets env per child with Command::env, never set_var, and any env_clear re-adds LLVM_PROFILE_FILE · ## 11. Test Anti-Patterns
  (test-plan/e2e-test-strategy fct-ennvbmje) e2e
- S6 · no retries, no #[ignore] parking, and zero flakiness; a flaky test keeps the chunk red until its root cause is fixed · ## 10. Quality Gates & Coverage Targets
  (test-plan/test-strategy fct-brzfiy4i) strategy
- S7 · the product is Linux-only; a new test gate naming windows or macos is a census fault · test-scope-summary
- S8 · any workflow change keeps permissions {} at the top, contents read per job, every uses SHA-pinned, rust-cache as the only cache, and no concurrency block · ## 9. CI Integration
- S9 · no test or CI step runs the real claude CLI; real-CLI readings are local live measurements only · ## 11. Test Anti-Patterns
