# security · standing conventions

- S1 · each new test or harness case uses its own `--home`, never a committed one, and a test-side removal takes only a dir the test itself created · Secret Management (Development) / .andromeda/security-plan-detail/security-anti-patterns.md
  (.andromeda/security-plan-detail/secret-management.md:15) secrets
  (.andromeda/security-plan-detail/security-anti-patterns.md:23) bans
- S2 · each new refusal or error a verb, the hook or MCP emits carries codes and fixed messages only: no absolute path, pid, start time, endpoint or anyhow chain with a serde source · Error Handling / .andromeda/security-plan-detail/error-handling.md
- S3 · each new log or diagnostics line keeps the NEVER-log floor: paths and pids only in `instances/<name>/diagnostics/` (0600), never an R8-stripped `CLAUDE*` value · Bootstrap phases (`logging-redaction-wire`) / .andromeda/security-plan-detail/bootstrap-phases.md
  (.andromeda/security-plan-detail/threat-model-summary.md:27) threat-data
- S4 · each new file or directory the product creates under the home is 0600 / 0700 set explicitly, replaced only through `replace_private` · Data Protection / .andromeda/security-plan-detail/data-protection.md
- S5 · each new or changed CI step stays SHA-pinned, `permissions: {}` with job `contents: read`, event values only through `env:`, uploads behind the artifact canary scan · Dependency Security (CI integration) / .andromeda/security-plan-detail/dependency-security.md
  (.andromeda/security-plan-detail/bootstrap-phases.md:24) bootstrap
  (.andromeda/security-plan-detail/dependency-security.md:32) deps-supply-chain
  (.andromeda/security-plan-detail/dependency-security.md:48) deps-supply-chain
- S6 · each new dependency or feature passes `cargo deny check`, the sync-crate tokio ban and the C-build ban, with no new ignore, skip or licence exception · Dependency Security / .andromeda/security-plan-detail/dependency-security.md
  (.andromeda/security-plan-detail/dependency-security.md:14) deps
  (.andromeda/security-plan-detail/threat-model-summary.md:146) boundary-supply-chain
- S7 · each boundary widening, new environment read, new G2 exemption or new dated exception needs its own Decisions Log entry and the founder's own word · Security Anti-Patterns (Universal) / .andromeda/security-plan-detail/security-anti-patterns.md
  (.andromeda/security-plan-detail/dependency-security.md:42) deps-supply-chain
