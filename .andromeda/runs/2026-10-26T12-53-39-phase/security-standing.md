# security · standing conventions

- S1 · Every new log line keeps the NEVER-log floor: no R8-stripped `CLAUDE*` value, user content only in `diagnostics/`, no absolute path or internal type name in external error output · Bootstrap phases, `logging-redaction-wire`
  (.andromeda/security-plan-detail/secret-management.md:8) secrets
  (.andromeda/security-plan-detail/threat-model-summary.md:27) threat-data
- S2 · Every new error type's `Display` uses a fixed message, no path or payload interpolated, and anyhow chains stop at the human-mode stderr without a serde source · Error Handling
- S3 · Every new test runs under its own `--home`, and no local `--home` test directory is committed · Secret Management (Development)
  (.andromeda/security-plan-detail/secret-management.md:15) secrets
- S4 · A test-side removal under `target/e2e-home` takes only a dir the test side itself created · Security Anti-Patterns › Code Patterns
  (.andromeda/security-plan-detail/security-anti-patterns.md:22) bans
- S5 · Every file a test run leaves under `target/agent-run/` passes CI's artifact canary scan before any upload · Secret Management (Secret scanning in CI)
  (.andromeda/security-plan-detail/bootstrap-phases.md:24) bootstrap
- S6 · No new environment read or test seam outside `VIOLA_*` plumbing without its own Decisions Log entry; no flag, env var or `config.json` key switches a control off · Security Anti-Patterns › Universal
  (.andromeda/security-plan-detail/security-anti-patterns.md:37) bans
- S7 · Every new file under the home is created 0600 (dirs 0700) through `OpenOptionsExt::mode` / `replace_private`, never the umask · Security Anti-Patterns › Data Protection
  (.andromeda/security-plan-detail/data-protection.md:8) data-rest
  (.andromeda/security-plan-detail/data-protection.md:33) data-lifecycle
