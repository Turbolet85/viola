## 2026-10-26-tokio-rustsec-2026-0386-lock-bump — the pinned Tokio version moved to 1.53.3
**Section:** §Stack and Technologies (stack-and-technologies.md: the Backend framework (concurrency model) row) · §Inherited Defaults (architecture.md: the Framework line)
**Change:**
- The Backend framework row's Choice cell reads "Tokio 1.53.3 built only inside `mcp`" (was 1.53.1); the rest of the row unchanged.
- The Inherited Defaults Framework line reads "Tokio 1.53.3 only in `mcp`" (was 1.53.1); Rust 1.98.1, MSRV 1.96, edition 2024, rmcp 3.4.1 and clap 4.6.7 unchanged.
**Why:** the `[workspace.dependencies]` exact pin moved `tokio` `=1.53.1` → `=1.53.3` (`default-features = false` kept) and the lock resolves 1.53.3, a version RUSTSEC-2026-0386 lists as patched, so `cargo deny check` exits 0 without it and with no ignore widened. The containment both sites state is unmoved: Tokio stays built only in `viola-mcp`, `viola-channel`'s optional `tokio` feature and the test-only `viola-e2e`, with the four sole-root `deny-sync.toml` bans exiting 0 and `channel-tokio-free: ok`.
**Kept:** the stack table carries the version only, at category grain. It gains no Tokio advisory-floor row, because that advisory's record belongs to security-plan's Dependency Security group. It gains no row for the four `AsyncFd` constructors Tokio 1.53.3 deprecates either: viola calls none of them, and clippy `-D warnings` is the gate that holds that fact.
**Ref:** .andromeda/runs/2026-10-26T06-27-48-wrap/
