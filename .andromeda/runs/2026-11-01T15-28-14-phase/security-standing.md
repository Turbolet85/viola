# security · standing conventions

- S1 · Each new test uses its own `--home`, and the pre-push gate's children carry only `HOME` and `PATH`, so no host environment value crosses into one · Secret Management, Development; `.andromeda/security-plan-detail/secret-management.md`
- S2 · Each new log line stays under the NEVER-log floor · Bootstrap phases, `logging-redaction-wire`; `.andromeda/security-plan-detail/bootstrap-phases.md`
- S3 · Each new test-side removal under `target/e2e-home` takes only a directory the test side itself created · Security Anti-Patterns, Code Patterns; `.andromeda/security-plan-detail/security-anti-patterns.md`
  (.andromeda/security-plan-detail/security-anti-patterns.md:23) bans
- S4 · Each new test's role files pass the G2 zero-panics gate, which takes no further exemption without its own Decisions Log entry · Dependency Security; `.andromeda/security-plan-detail/dependency-security.md`
  (.andromeda/security-plan-detail/dependency-security.md:48) deps-supply-chain
- S5 · What a new test leaves under `target/agent-run/` passes the artifact canary scan, a non-0600 diagnostics file included · Secret Management; `.andromeda/security-plan-detail/secret-management.md`
  (.andromeda/security-plan-detail/bootstrap-phases.md:24) bootstrap
- S6 · Test-only helpers stay behind the test-only features and out of the release graph `scripts/release-check.sh` judges · Dependency Security; `.andromeda/security-plan-detail/dependency-security.md`
  (.andromeda/security-plan-detail/input-validation.md:50) input
- S7 · A dependency a change adds keeps the workspace pin, the licence list and the bans, `tokio` in the sync crates included · Dependency Security; `.andromeda/security-plan-detail/dependency-security.md`
  (.andromeda/security-plan-detail/dependency-security.md:20) deps
  (.andromeda/security-plan-detail/threat-model-summary.md:166) boundary-supply-chain
- S8 · A CI step a change adds keeps SHA-pinned actions, `permissions: {}` with job-level `contents: read`, and no event value in `run:` · Dependency Security; `.andromeda/security-plan-detail/dependency-security.md`
  (.andromeda/security-plan-detail/dependency-security.md:38) deps-supply-chain
  (.andromeda/security-plan-detail/dependency-security.md:54) deps-supply-chain
- S9 · A file or directory a change creates under the home gets explicit 0600 or 0700, never the umask · Security Anti-Patterns, Data Protection; `.andromeda/security-plan.md`
  (.andromeda/security-plan-detail/data-protection.md:28) data-rest
