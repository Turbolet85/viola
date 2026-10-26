# security · standing conventions

- S1 · every new test runs under its own `--home`, and no local `--home` test directory is committed · Secret Management (Development) in the body; security-anti-patterns.md (Secrets)
  (.andromeda/security-plan-detail/secret-management.md:15) secrets
- S2 · every new log line keeps the NEVER-log floor: no R8-stripped `CLAUDE*` value, no user content, no absolute path outside `diagnostics/`, pids and `started_at` only in `diagnostics/` · bootstrap-phases.md (`logging-redaction-wire`)
- S3 · every new error variant has a fixed `Display` message and every external error carries codes only · error-handling.md
- S4 · a test-side removal under `target/e2e-home` takes only a dir the test side itself created · security-anti-patterns.md (Code Patterns)
  (.andromeda/security-plan-detail/security-anti-patterns.md:22) bans
- S5 · no new environment read or test seam (a forced-mismatch seam included) without its own Decisions Log entry; the two existing `fake-agent` seams stay compiled out of release · input-validation.md; security-anti-patterns.md (Universal)
  (.andromeda/security-plan-detail/input-validation.md:40) input
  (.andromeda/security-plan-detail/input-validation.md:43) input
- S6 · every CI upload a new test adds waits on the artifact canary scan · Secret Management (Secret scanning in CI) in the body; bootstrap-phases.md (`secret-scanning-ci-gate`)
  (.andromeda/security-plan-detail/bootstrap-phases.md:24) bootstrap
