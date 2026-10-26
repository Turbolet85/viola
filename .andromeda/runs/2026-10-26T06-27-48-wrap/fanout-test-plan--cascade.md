proposals:
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 21
    section: "## 7. Test Data & Fixtures"
    change: "In the Time-sensitive data row, the parenthesis reads `(as landed at chunk 2026-10-17-mcp-server-for-drivers, the pin moved from `=1.53.1` at chunk 2026-10-26-tokio-rustsec-2026-0386-lock-bump: tokio `=1.53.3` with default features off, `viola-mcp` `rt`, `viola-channel` `io-util` and dev `rt`, `viola-e2e` `rt` + `process`)`; the rest of the row unchanged."
    sidecar: "test-plan §7 Time-sensitive data row: tokio `=1.53.1` -> `=1.53.3` (chunk 2026-10-26-tokio-rustsec-2026-0386-lock-bump, the RUSTSEC-2026-0386 lock bump); features and the no-`test-util` fact unchanged."
    rationale: "Report Changes, Dependencies: `tokio` `=1.53.1` -> `=1.53.3` (`Cargo.toml:242`, `default-features = false` kept); Counts / qualifiers moved, pattern `tokio-ver`, lists this site as changed; Crates / modules: tokio's per-crate features unmoved (`viola-mcp` `rt`, `viola-channel` `io-util` + dev `rt`, `viola-e2e` `rt` + `process`)."
    basis: .andromeda/test-plan-detail/test-data-fixtures.md:21
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/mocking-stubbing-discipline.md
    near: 8
    section: "## 8. Mocking & Stubbing Discipline"
    change: "In the Time item, \"from tokio 1.53.1 `test-util`\" reads \"from tokio 1.53.3 `test-util`\"; the rest of the item unchanged."
    sidecar: "test-plan §8 Time item: tokio 1.53.1 -> 1.53.3 `test-util` (chunk 2026-10-26-tokio-rustsec-2026-0386-lock-bump)."
    rationale: "Report Changes, Dependencies: the workspace pin and the lock moved tokio 1.53.1 -> 1.53.3; Counts / qualifiers moved, pattern `tokio-ver`, lists `mocking-stubbing-discipline.md:8` as changed; Expected amendments carries it."
    basis: .andromeda/test-plan-detail/mocking-stubbing-discipline.md:8
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/mocking-stubbing-discipline.md
    near: 17
    section: "## 8. Mocking & Stubbing Discipline"
    change: "In the Time mocking row, \"tokio 1.53.1 `test-util` (in no manifest yet, as landed at chunk 2026-10-17-mcp-server-for-drivers)\" reads \"tokio 1.53.3 `test-util` (in no manifest yet, as landed at chunk 2026-10-17-mcp-server-for-drivers)\"; the qualifier and the rest of the row unchanged."
    sidecar: "test-plan §8 Time mocking row: tokio 1.53.1 -> 1.53.3 `test-util`, the \"in no manifest yet\" qualifier kept (chunk 2026-10-26-tokio-rustsec-2026-0386-lock-bump)."
    rationale: "Report Changes, Dependencies: tokio 1.53.1 -> 1.53.3; Counts / qualifiers moved, pattern `tokio-ver`, lists `mocking-stubbing-discipline.md:17` as changed; Crates / modules and Schema / config: none changed, so no manifest gained `test-util` and the qualifier holds."
    basis: .andromeda/test-plan-detail/mocking-stubbing-discipline.md:17
note: "The `tokio-ver` site in test-plan's key file `.andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:24` (report Counts / qualifiers moved) is no category entity of mine and was not picked — the key-file pass owns it; test-plan's body states no tokio version literal."
