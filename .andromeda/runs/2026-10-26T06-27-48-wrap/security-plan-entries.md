## 2026-10-26-tokio-rustsec-2026-0386-lock-bump — viola-channel's Tokio client pin at tokio 1.53.3
**Section:** Dependency Security (dependency-security.md: the `values · deps` group's arch-bans item)
**Change:**
- `viola-channel`'s Tokio client, behind its `tokio` feature, reads tokio `=1.53.3`, `io-util` (was `=1.53.1`, per 2026-10-17-mcp-server-for-drivers — the tokio ban's tree form and the rmcp release-graph gate; now `=1.53.3`). The rest of the item stands as written: the C-building ban in `deny.toml`, the `deny-sync.toml` tokio ban for three target triples run per sync crate as the sole root over the four crates of `scripts/sync-crates.txt`, the `cargo tree` form `scripts/channel-tokio-free.sh`, and `scripts/deny-probes.sh`'s `channel-tokio-edge` probe and control at `13/13 banned, control clean`.
**Why:** the chunk bumped the `[workspace.dependencies]` exact pin of `tokio` from `=1.53.1` to `=1.53.3` (`default-features = false` kept) and the lock's tokio entry with it, every other crate unmoved; tokio's containment did not move: the four sole-root bans pass, `channel-tokio-free: ok`, `deny-probes: 13/13 banned, control clean`.
**Ref:** .andromeda/runs/2026-10-26T06-27-48-wrap/

## 2026-10-26-tokio-rustsec-2026-0386-lock-bump — the RUSTSEC-2026-0386 supply-chain red closed
**Section:** Dependency Security (dependency-security.md: the file's last group, a `values · deps-supply-chain` group with one item, removed whole; the file now ends with the `values · deps-supply-chain` group of pinned actions)
**Change:**
- Retired: the item stating that the open advisory RUSTSEC-2026-0386 (`unsound`) against the locked tokio `=1.53.1` fails CI's `supply-chain` job, owned by the working-route entry "tokio RUSTSEC-2026-0386 lock bump" (a lock bump of `tokio` to `>=1.53.3`, no ignore-list widening). Was an open advisory and its owner; now no open-advisory item stands, and the group that held it is gone.
**Why:** the chunk named in that owner line closed the advisory. With tokio locked at `=1.53.3`, a version the advisory lists as patched, `cargo deny check` (all four families, cargo-deny 0.20.2, the CI pin) exits 0 and its output does not contain RUSTSEC-2026-0386. `deny.toml` is untouched, so no ignore was widened, and CI's `supply-chain` job reads green on the pushed HEAD.
**Kept:** RUSTSEC-2017-0008 stays the only advisory ignore, stated by the body's `[advisories]` line. The `deps-supply-chain` subject stays, because three other groups carry it.
**Supersedes:** 2026-10-25-windows-endpoint-admission — the supply-chain red on RUSTSEC-2026-0386 and its owner
**Ref:** .andromeda/runs/2026-10-26T06-27-48-wrap/
