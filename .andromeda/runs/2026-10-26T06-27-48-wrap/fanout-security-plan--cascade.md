proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/dependency-security.md
    near: 14
    section: "## Dependency Security"
    change: "In the deps group's arch-bans item, viola-channel's Tokio client reads (tokio `=1.53.3`, `io-util`) in place of (tokio `=1.53.1`, `io-util`); the rest of the item (the sole-root `deny-sync.toml` bans over `scripts/sync-crates.txt`, `scripts/channel-tokio-free.sh`, the `channel-tokio-edge` probe and `13/13 banned, control clean`) stays as written."
    sidecar: "dependency-security deps: viola-channel's Tokio client pin tokio `=1.53.1` → `=1.53.3` (chunk 2026-10-26-tokio-rustsec-2026-0386-lock-bump)."
    rationale: "Report Changes › Dependencies: the `[workspace.dependencies]` exact pin `tokio` `=1.53.1` → `=1.53.3`, `default-features = false` kept (Cargo.toml:242), lock entry 1.53.3 (Cargo.lock:1633,1635); Counts / qualifiers moved, pattern `tokio-ver`, lists this site as changed. Crates / modules: tokio containment unmoved (`channel-tokio-free: ok`, `deny-probes: 13/13 banned, control clean`), so only the version moves."
    basis: Cargo.toml:242
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/dependency-security.md
    near: 58
    section: "## Dependency Security"
    change: "Remove the deps-supply-chain item stating the open advisory RUSTSEC-2026-0386 (`unsound`) against the locked tokio `=1.53.1` failing CI's `supply-chain` job and naming its owner «tokio RUSTSEC-2026-0386 lock bump»: the advisory is closed by the lock bump to tokio `=1.53.3` with no ignore widened, so the line's open fact, its tokio version and its owner retire together; RUSTSEC-2017-0008 remaining the only ignore stays stated by the body's `[advisories]` line."
    sidecar: "dependency-security deps-supply-chain: the open RUSTSEC-2026-0386 line retired — closed by chunk 2026-10-26-tokio-rustsec-2026-0386-lock-bump (tokio =1.53.3; cargo deny check exit 0; supply-chain green on ci#40278510123)."
    rationale: "Report Changes › Retired names this line (dependency-security.md:58) as an open fact this chunk closed: `cargo deny check` (all four families, cargo-deny 0.20.2) exits 0 and lacks RUSTSEC-2026-0386; Counts / qualifiers moved: `rustsec-0386` ×2 and `owner-title` 1 on this line changed, the `sc-red` fact standing only here; Outcome: `deny.toml` untouched, supply-chain green on ci#40278510123."
    basis: Cargo.lock:1633
note: the security-plan body (.andromeda/security-plan.md, Dependency Security and the Threat Model supply-chain vector at line 53) states no tokio version and no open-advisory line, so no body edit follows; the other tokio-ver sites (architecture, test-plan, the bootstrap-phases key file, the three leaf rows) are other masters' cascades.
