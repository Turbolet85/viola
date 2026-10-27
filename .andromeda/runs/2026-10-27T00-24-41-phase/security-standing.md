# security · standing conventions

- S1 · each new test or witness uses its own owner-only `--home` and removes only what it created · Secret Management (Development) / security-anti-patterns.md
  (.andromeda/security-plan-detail/secret-management.md:15) secrets
- S2 · each new log line (a pin wait or race outcome) goes through `obs_event!` and carries no path, pid, hash path or upstream text · Bootstrap phases (`logging-redaction-wire`) / bootstrap-phases.md
  (.andromeda/security-plan-detail/secret-management.md:8) secrets
- S3 · each new error variant has a fixed `Display`, with path-bearing fields excluded · Error Handling / error-handling.md
- S4 · each new file or directory created under the home (a lock sibling, a temp file) has an explicit 0700 / 0600 mode, never the umask · Data Protection / data-protection.md
- S5 · each new CI artifact or upload runs only after the artifact canary `secret-scan` passes · Secret Management / secret-management.md
  (.andromeda/security-plan-detail/bootstrap-phases.md:24) bootstrap
- S6 · each new dependency (a file-lock or comparison crate) is workspace-pinned, passes `cargo deny check` and builds no C, and none enters a sync crate's graph as tokio · Dependency Security / dependency-security.md
  (.andromeda/security-plan-detail/dependency-security.md:14) deps
- S7 · each new or changed CI job or step pins actions by SHA, keeps `permissions: {}` / `contents: read` and passes no event value through `${{ }}` in `run:` · Dependency Security (CI integration) / dependency-security.md
  (.andromeda/security-plan-detail/dependency-security.md:32) deps-supply-chain
  (.andromeda/security-plan-detail/dependency-security.md:48) deps-supply-chain
