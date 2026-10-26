# security extract

## Relevance
relevant — the chunk is a supply-chain remediation squarely inside security-plan §Dependency Security (the open-advisory line names this route entry as its owner); no runtime trust boundary, input, data or error surface is touched.

## Constraints
- The fix is a version bump, never an ignore: RUSTSEC-2017-0008 (via portable-pty `=0.8.1`) stays the only `[advisories]` ignore, and `unsound = "all"` / `yanked = "deny"` stay as set (per security-plan §Dependency Security, `deny.toml` additions; the open-advisory line in `.andromeda/security-plan-detail/dependency-security.md`, group `deps-supply-chain`, names the target `>=1.53.3` and "no ignore-list widening").
- The disabled-feature `ignore` path of the CVE SLA does not apply: tokio's `AsyncFd` sits in features viola builds, so the response the SLA names for an enabled crate is a bump (per security-plan §Dependency Security, Critical CVE response SLA). Whether any viola graph actually enables tokio's `net` / `AsyncFd` surface (directly or via interprocess's `tokio` backend) is research's question; the floor holds either way.
- Exact pinning holds through the move: `[workspace.dependencies]` pins every third-party version of the root graph exactly (rmcp's minor range the only exception), and `Cargo.lock` stays committed and must resolve the pinned version (per security-plan §Dependency Security, Pinning). The new tokio pin keeps the `=` form; companion crates (`tokio-macros`, `tokio-stream`, `tokio-util`) move only as the new tokio requires.
- Tokio containment is unchanged: the `deny-sync.toml` tokio ban per sync crate as sole root over `scripts/sync-crates.txt`, and `viola-channel`'s `cargo tree` form `scripts/channel-tokio-free.sh` (no `tokio v` line without the `tokio` feature, one with it), each still proven by `scripts/deny-probes.sh` (per security-plan §Dependency Security, the arch bans line; detail group `deps`).
- `[sources]` stays crates.io only and the licence `allow` list stays exactly MIT / Apache-2.0 / Zlib / Unicode-3.0 with `0BSD` admitted only by the two existing per-crate exceptions; a new licence a bumped crate might pull needs its own Decisions Log entry, never an `allow` widening (per security-plan §Dependency Security, `deny.toml` additions).
- The fuzz lockfile's own audit (`cargo deny --manifest-path fuzz/Cargo.toml check advisories sources`) and its test-only exemption keep their meaning; whether `fuzz/Cargo.lock` holds any tokio is research's question (per security-plan §Dependency Security, the `fuzz/` exemption; detail group `deps` exception).
- rmcp's release-graph features stay exactly `server` + `transport-io` and the `[[bans.features]]` rmcp bans stay in force if the bump re-resolves rmcp's tokio edge (per security-plan §API Security, MCP transport).

## Patterns to follow
- Audit with `cargo deny check` at CI's pinned cargo-deny version (floor `>=0.20.2`), all four families over the root `Cargo.lock`, the CI `supply-chain` job the authoritative reading (per security-plan §Dependency Security, Audit tool / CI integration).
- Prove the bans fire after the lock move: `bash scripts/deny-probes.sh` (`13/13 banned, control clean`) and `bash scripts/channel-tokio-free.sh` (per security-plan §Dependency Security, detail group `deps`).
- Release graph check: `scripts/release-check.sh` (`--probe` first, 5/5 refused, clean control) still refuses any test-only feature in the artifacts after the re-resolve (per security-plan §Dependency Security, CI integration).
- Treat the advisory DB as moving without code: the weekly `nightly.yml` `cargo deny check advisories` is the standing signal that surfaced this class (per security-plan §Dependency Security, Update policy / CI integration).

## Anti-patterns to avoid
- Never widen an ignore, add a `skip` / `allow`, add a licence exception, or loosen `deny.toml` / `deny-sync.toml` policy to reach green (per security-plan §Dependency Security, `deny.toml` additions; `.claude/rules/security.md` Dependencies and CI).
- Never silence a deprecation the patched tokio introduces (`AsyncFd::new` / `with_interest` / `try_new` / `try_with_interest`) with an `#[allow(deprecated)]` or a lint-gate change to pass `-D warnings`; and never adopt the `unsafe` constructors without a justified safety comment — whether viola's own code calls them at all is research's question (per security-plan §Security Anti-Patterns, Code Patterns, the never-reach-green-by-silencing rule as applied to the lint gate; §Dependency Security).
- Never add a Tokio edge into a sync crate's normal graph while moving the pin (per security-plan §Dependency Security, the arch bans line).

## Contract bindings
- security §Dependency Security ↔ tests §CI Integration: CI's `supply-chain` job (`cargo deny check`, the per-sync-crate tokio ban step, the fuzz lockfile audit step) is the gate whose green closes the CARRY; the chunk's verdict reads that job on its push.
- security §Dependency Security ↔ architecture §Build system / Stack row: the tokio version named in the Stack row and in the `viola-channel` Tokio client note (detail group `deps`, tokio `=1.53.1`) and the open-advisory line (detail group `deps-supply-chain`) are wrap amendments once the pin moves, not this chunk's edits.

## Acceptance criteria contributions
- `cargo deny check` (CI's pinned cargo-deny) reads no RUSTSEC-2026-0386 finding and no other advisory, ban, licence or source failure, with `deny.toml`'s ignore list byte-identical to before (only RUSTSEC-2017-0008) (per security-plan §Dependency Security, `deny.toml` additions).
- `Cargo.lock` resolves tokio to a patched version (`>=1.53.3`, or the `>=1.51.6, <1.52.0` band) matching an exact `=` pin in `[workspace.dependencies]`, and no third-party crate outside the tokio family changes version (per security-plan §Dependency Security, Pinning).
- `bash scripts/deny-probes.sh` and `bash scripts/channel-tokio-free.sh` pass, and `deny-sync.toml` per sync crate reads no tokio (per security-plan §Dependency Security, the arch bans line).
- `git diff` on `deny.toml`, `deny-sync.toml` and the lint configuration is empty, and no `#[allow(deprecated)]` is added (per security-plan §Dependency Security, `deny.toml` additions).
