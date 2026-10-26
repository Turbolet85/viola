proposals:
  - detector: D-arch-decisions
    severity: warning
    section: Inherited Defaults
    change: "the Framework row reads \"Tokio 1.53.3 only in `mcp`\" in place of \"Tokio 1.53.1 only in `mcp`\"; the rest of the row (Rust 1.98.1, MSRV 1.96, edition 2024, rmcp 3.4.1, clap 4.6.7) unchanged"
    sidecar: "§Inherited Defaults Framework row: Tokio 1.53.1 → 1.53.3 (the `[workspace.dependencies]` exact pin bumped to `=1.53.3`, closing RUSTSEC-2026-0386), chunk 2026-10-26-tokio-rustsec-2026-0386-lock-bump"
    rationale: "report Changes, Dependencies: bumped `tokio` `=1.53.1` → `=1.53.3` (`Cargo.toml:242`, `Cargo.lock:1633,1635`); Counts / qualifiers moved names pattern `tokio-ver` site `.andromeda/architecture.md:442` as stating the pinned version as current truth; Expected amendments: architecture §Inherited Defaults \"Tokio 1.53.1 only in `mcp`\" → 1.53.3. The containment it states (Tokio only in `mcp`) is unchanged — Crates / modules: none, the four sole-root bans exit 0."
    basis: ".andromeda/architecture.md:442"
note: "The other architecture `tokio-ver` site, `.andromeda/architecture-detail/stack-and-technologies.md:11` (group `stack`, the Backend framework row's \"Tokio 1.53.1 built only inside `mcp`\" → 1.53.3), is a standing detail group and so the cascade's, not proposed here. D-arch-resources: no drift — the report's Symbols / APIs and Crates / modules bullets are none. D-retired: no architecture body, key-file or detail passage names RUSTSEC-2026-0386 (grep over the body, the five key files and architecture-detail/: 0 hits); its one standing site is security-plan's."
