proposals:
  - detector: D-tests-count-literals
    severity: warning
    section: §3 → Bootstrap phases (derive for route / setup-project)
    change: the test-runner-install Dev-deps bullet "in `viola-e2e`" should read tokio `=1.53.3` (was `=1.53.1`), with default features off and `rt` + `process` (no `test-util`) kept, every other pin on the line unchanged
    sidecar: test-plan §3 Bootstrap phases key file — viola-e2e's tokio pin `=1.53.1` → `=1.53.3` (chunk 2026-10-26-tokio-rustsec-2026-0386-lock-bump, the RUSTSEC-2026-0386 lock bump)
    rationale: the report's Dependencies bullet bumps the `[workspace.dependencies]` exact pin `tokio` `=1.53.1` → `=1.53.3` (Cargo.toml:242); Counts / qualifiers moved, pattern `tokio-ver`, lists this key file's line 24 as a standing master site stating the pinned tokio version as current truth, and Expected amendments carries "test-plan §3 Test Harness Contract — the Bootstrap phases key file's tokio version"
    basis: .andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:24
note: the three other test-plan `tokio-ver` sites the report lists (`.andromeda/test-plan-detail/mocking-stubbing-discipline.md:8`, `:17` — "tokio 1.53.1 `test-util`", the "in no manifest yet" qualifier kept — and `.andromeda/test-plan-detail/test-data-fixtures.md:21` — "tokio `=1.53.1`") sit in standing groups of extracted category files, so their edits are the cascade's, not proposed here; the test-plan body holds no `1.53.1` or RUSTSEC-2026-0386 literal; D-tests-coverage, D-tests-framework, D-tests-obs-harness find no drift (no symbol, runner or harness change per the report's Changes); the Retired advisory stands in no test-plan passage.
