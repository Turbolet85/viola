# tests · standing conventions

- S1 · each new or renamed test is named in snake_case as subject, condition, expected, and a critical-path scenario as path<N>_<slug> · section 2, Test Strategy (Test function naming)
- S2 · table-driven cases in viola-e2e are a labelled case table in one #[test] with each assertion naming its label, adding no rstest manifest edge · section 4, Unit Test Strategy (Test grouping)
- S3 · a test decides on values it injects, never on the host's clock, disk or state: no sleep for synchronisation, no test-owned timer, waits keyed on logged events or offsets · section 11, Test Anti-Patterns (E2E and Universal)
  (test-plan/test-data-fixtures fct-ovluqlfu) data
- S4 · each test makes its own --home through tempfile, sets env per child with Command::env and never std::env::set_var, and re-adds LLVM_PROFILE_FILE after env_clear · section 11, Test Anti-Patterns (Integration)
  (test-plan/e2e-test-strategy fct-ennvbmje) e2e
  (test-plan/e2e-test-strategy fct-4n3xwm4s) scenario-path-1-run-start-sequence
  (test-plan/test-strategy fct-hlf5klew) strategy
- S5 · every workflow has permissions {} at the top and contents read per job, every uses SHA-pinned, no concurrency block, and rust-cache as the only cache, in ci.yml only · section 9, CI Integration
  (test-plan/ci-integration fct-a43jfmhh) ci
- S6 · each tool has one version source, ci.yml's pins, and the pre-push gate checks the host's tools against them and installs nothing · section 9, CI Integration
- S7 · each CI test job ends in viola-harness gate --require its suites, and every artifact leaves CI only behind the secret scan, with retention-days 7 · file ci-integration
- S8 · no flaky test is tolerated: nextest retries 0, no retry budget, and no #[ignore] as a parking place · section 10, Quality Gates & Coverage Targets (Zero-flakiness budget)
- S9 · a harness command prints exactly one JSON document on stdout with exit 0, 1 or 2, never a human-only message · section 3, Test Harness Contract
