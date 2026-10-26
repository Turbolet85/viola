proposals:
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 11
    section: "## Stack and Technologies"
    change: "In the Backend framework (concurrency model) row's Choice cell, \"Tokio 1.53.1 built only inside `mcp`\" becomes \"Tokio 1.53.3 built only inside `mcp`\"; the rest of the row unchanged."
    sidecar: "stack-and-technologies Backend framework row: Tokio 1.53.1 -> 1.53.3 (chunk 2026-10-26-tokio-rustsec-2026-0386-lock-bump, the workspace pin =1.53.3)."
    rationale: "Report Changes, Dependencies: `tokio` `=1.53.1` -> `=1.53.3` (the `[workspace.dependencies]` exact pin, `default-features = false` kept; `Cargo.toml:242`), the lock resolving 1.53.3 (`Cargo.lock:1633,1635`); Counts / qualifiers moved, pattern `tokio-ver`, names this row as a changed site. Containment unmoved (Crates / modules: tokio still built only in `viola-mcp`, `viola-channel`'s optional feature and the test-only `viola-e2e`)."
    basis: Cargo.toml:242
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture.md
    near: 442
    section: "## Inherited Defaults"
    change: "In the Framework line, \"Tokio 1.53.1 only in `mcp`\" becomes \"Tokio 1.53.3 only in `mcp`\"; the rest of the line unchanged."
    sidecar: "architecture body Inherited Defaults Framework line: Tokio 1.53.1 -> 1.53.3 (chunk 2026-10-26-tokio-rustsec-2026-0386-lock-bump)."
    rationale: "The same moved fact as the stack row: report Dependencies (`Cargo.toml:242`, `=1.53.3`) and Counts / qualifiers moved, pattern `tokio-ver`, listing `.andromeda/architecture.md:442` as a changed site; the report's Expected amendments carry it (architecture §Inherited Defaults)."
    basis: Cargo.toml:242
    dependent-of: D-arch-decisions
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 11
    section: "## Stack and Technologies"
    change: "Add after the Backend framework row a row in the table's shape: \"| Tokio advisory floor | The workspace pin `tokio = { version = \"=1.53.3\", default-features = false }` sits at RUSTSEC-2026-0386's patched floor (patched `>= 1.51.6, < 1.52.0` and `>= 1.53.3`) | A pin below 1.53.3 on the 1.53 line re-opens the advisory, and `cargo deny check` (no ignore for it) fails supply-chain |\""
    sidecar: "stack-and-technologies: new row, the tokio pin =1.53.3 held at RUSTSEC-2026-0386's patched floor (chunk 2026-10-26-tokio-rustsec-2026-0386-lock-bump)."
    rationale: "Report Changes, Retired: RUSTSEC-2026-0386 closed, `cargo deny check` exits 0 lacking it with no ignore widened; Cross-cutting claims `inputs#I2`: the advisory patched `>= 1.51.6, < 1.52.0` and `>= 1.53.3`; Dependencies: the pin `=1.53.3` (`Cargo.toml:242`)."
    basis: Cargo.toml:242
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 11
    section: "## Stack and Technologies"
    change: "Add after the Backend framework row a row in the table's shape: \"| Tokio `AsyncFd` constructors | Tokio 1.53.3 deprecates `AsyncFd::new`, `AsyncFd::with_interest`, `AsyncFd::try_new` and `AsyncFd::try_with_interest`; viola calls none | A future `AsyncFd` use in `viola-mcp` or `viola-channel`'s Tokio client through a deprecated constructor trips clippy `-D warnings` |\""
    sidecar: "stack-and-technologies: new row, tokio 1.53.3's deprecated AsyncFd constructors, none called by viola (chunk 2026-10-26-tokio-rustsec-2026-0386-lock-bump)."
    rationale: "Report Changes, Symbols / APIs: the four constructors tokio 1.53.3 deprecates (`inputs#I2`, `inputs#I4`) reach no viola call site (`grep -rn AsyncFd` over `crates src tests` 0 hits), and clippy `-D warnings` on the host and `x86_64-pc-windows-msvc` reads green with no `deprecated` line."
    fact: F1
note: F2's advisory floor also binds security-plan's Dependency Security group (the retired open-advisory line at `.andromeda/security-plan-detail/dependency-security.md:58`), which is that master's to place, not architecture's; cross-cutting-patterns#1.1 ruled not touched (no env var moved with the dependency bump).
