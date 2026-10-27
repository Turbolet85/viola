# security · standing conventions

- S1 · Each new test runs under its own `--home` (never the user's `~/.viola` or `~/.claude`), with a fake agent first on `PATH` where it reaches `claude agents --json` · .andromeda/security-plan-detail/secret-management.md
  (.andromeda/security-plan-detail/secret-management.md:15) secrets
- S2 · Each new test input and fixture is synthetic: no R8-stripped `CLAUDE*` value, real prompt, home path or username; the CI artifact canary scan reads it before any upload · .andromeda/security-plan-detail/bootstrap-phases.md
  (.andromeda/security-plan-detail/bootstrap-phases.md:24) bootstrap
  (.andromeda/security-plan-detail/data-protection.md:70) data-rest
- S3 · A new log line goes only through `obs_event!`, and user content, tool `input` and drift-report values reach only `instances/<name>/diagnostics/` (0600), never stderr, `--json` errors, MCP `isError` or channel `error.data` · .andromeda/security-plan-detail/bootstrap-phases.md
- S4 · A new fuzz or property target stays in the separate `fuzz/` workspace, never linked into `viola` and never in the root `[workspace]`, its lockfile audited on its own · .andromeda/security-plan-detail/dependency-security.md
  (.andromeda/security-plan-detail/dependency-security.md:20) deps
  (.andromeda/security-plan-detail/threat-model-summary.md:156) boundary-supply-chain
- S5 · A test-only helper, seam or feature stays out of the release build (`scripts/release-check.sh` refuses `test-support` / `fake-agent` artifacts); a new environment read or seam needs its own Decisions Log entry · .andromeda/security-plan-detail/dependency-security.md
  (.andromeda/security-plan-detail/input-validation.md:42) input
  (.andromeda/security-plan-detail/input-validation.md:45) input
  (.andromeda/security-plan-detail/threat-model-summary.md:168) threat-infra
- S6 · A new CI step pins actions by full SHA, keeps `permissions: {}` / `contents: read`, and takes no `github.event` value through `${{ }}` in `run:` · .andromeda/security-plan-detail/dependency-security.md
  (.andromeda/security-plan-detail/dependency-security.md:32) deps-supply-chain
  (.andromeda/security-plan-detail/dependency-security.md:48) deps-supply-chain
- S7 · A test-side removal under `target/e2e-home` removes only a dir the test itself created · .andromeda/security-plan-detail/security-anti-patterns.md
  (.andromeda/security-plan-detail/security-anti-patterns.md:23) bans
