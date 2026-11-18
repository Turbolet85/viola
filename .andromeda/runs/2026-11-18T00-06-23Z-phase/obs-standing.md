# obs · standing conventions

- S1 · every new or changed integration test that drives a process checks the diagnostics lines it emits against the line schema · ci-integration
  contract/diag-line-schema/os · diag-line v1 schema · os — os is closed to linux.
  contract/diag-line-schema/pty-backend · diag-line v1 schema · pty_backend — pty_backend is any string.
  decision/diag-lines-schema-checked-in-unit-tests · Emitted lines schema-checked in integration and E2E tests — Emitted role lines are checked against schemas/diag-line.v1.json by integration tests and the E2E schema_check over real process output; unit tests check only detail lines, against schemas/diag-detail.v1.json.
- S2 · every new test home lives under the one integration-test root the secret scan covers and is kept until the gates run · 9-ci-integration
  decision/homes-kept-until-gates-run · Test homes kept until the gates run — Integration-test homes are kept until the gate steps have run.
  decision/secret-scan-scope · Secret scan scope — The secret scan covers every test-home diagnostics file, the harness capture and the nextest JUnit file. · why: nextest stores failing tests' captured output in the JUnit file · except: the mutants run's chunk diff, skipped by exact path and excluded from the harness upload
  decision/test-homes-kept-until-gates · Test homes kept until gate steps run — CI sets AGENT_RUN_KEEP_HOMES=1 so no home is deleted before G2, G4, the secret scan and the uploads have read it. · why: otherwise a green run leaves target/e2e-home/ empty, G2's non-empty check fails, and nothing is scanned
  decision/test-homes-under-e2e-home · Integration-test homes under one root — Every integration-test viola home lives under target/e2e-home, so the panic gate, schema gate, secret scan and diagnostics upload all cover it. · why: a hook panic exits 0, so a home outside that root would hide it · except: the one chaos_feed_panic test home
  decision/test-homes-under-scan-root · Test homes live under the scanned root — Every viola home of integration and E2E tests lives under target/e2e-home/, so G2, G4, the secret scan and the diagnostics upload cover it. · why: a hook panic exits 0, so a home outside that root would hide it · except: tests/chaos_feed_panic.rs keeps its one home outside the scan root, and asserts its deliberate panic and parse-rejected lines present itself
- S3 · no test waits for a log line by sleeping; every wait has an explicit bound · 10-slo-invariants-telemetry-budgets
