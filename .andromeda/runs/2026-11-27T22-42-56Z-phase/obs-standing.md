# obs · standing conventions

- S1 · Each test file that a moved test module lands in joins the OS-gated test census. · ci-integration
- S2 · Each integration test that creates a test home keeps that home under the one scanned E2E root, where it stays until the gates run. · 9-ci-integration · ci-integration
  decision/homes-kept-until-gates-run · Test homes kept until the gates run — Integration-test homes are kept until the gate steps have run.
  decision/test-homes-kept-until-gates · Test homes kept until gate steps run — CI sets AGENT_RUN_KEEP_HOMES=1 so no home is deleted before G2, G4, the secret scan and the uploads have read it. · why: otherwise a green run leaves target/e2e-home/ empty, G2's non-empty check fails, and nothing is scanned
  decision/test-homes-under-e2e-home · Integration-test homes under one root — Every integration-test viola home lives under target/e2e-home, so the panic gate, schema gate, secret scan and diagnostics upload all cover it. · why: a hook panic exits 0, so a home outside that root would hide it · except: the one chaos_feed_panic test home
  decision/test-homes-under-scan-root · Test homes live under the scanned root — Every viola home of integration and E2E tests lives under target/e2e-home/, so G2, G4, the secret scan and the diagnostics upload cover it. · why: a hook panic exits 0, so a home outside that root would hide it · except: tests/chaos_feed_panic.rs keeps its one home outside the scan root, and asserts its deliberate panic and parse-rejected lines present itself
- S3 · No test waits on a log line by sleeping. · 10-slo-invariants-telemetry-budgets
  decision/no-sleep-based-log-waits · No sleep-based log waits — No log-based wait relies on sleeps.
- S4 · Each new product source file uses the obs_event macro, no raw tracing or print macros, and only instrument attributes that skip all arguments. · ci-integration · 11-obs-anti-patterns-never-do-these
