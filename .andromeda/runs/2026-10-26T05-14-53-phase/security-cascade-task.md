# Task — security · 2026-10-26-tokio-rustsec-2026-0386-lock-bump

## The chunk
Moves the locked `tokio` from 1.53.1 to a version RUSTSEC-2026-0386 (unsound, the safe `AsyncFd` constructors) lists as patched (`>=1.53.3`), moving the exact workspace pin with it, so `cargo deny check`'s advisories family and CI's `supply-chain` job read green.
Companion crates `tokio-macros`, `tokio-stream` and `tokio-util` move only where the new tokio requires it; no other dependency is bumped, no ignore, `skip`, `allow` or licence exception is added, and `deny.toml`'s policy is unchanged.
No product behaviour change: a source file changes only if the bump forces one (a deprecated `AsyncFd` safe constructor in viola's own code read by clippy `-D warnings`).
Re-runs and reads back: `cargo deny check` at CI's pinned cargo-deny version, the tokio bans (`deny-sync.toml` per sync crate, `scripts/channel-tokio-free.sh`), `scripts/deny-probes.sh`, `scripts/rmcp-features.sh`, `scripts/release-check.sh`, fmt, clippy and the workspace tests; closes the CARRY red on ci#40257114709's `supply-chain` job against its log (RUSTSEC-2026-0386 alone behind it).

## Reached files
- Cargo.toml · edits the `[workspace.dependencies]` `tokio` pin
- Cargo.lock · edits the `tokio` entry, and the `tokio-macros`, `tokio-stream`, `tokio-util` entries only where the new tokio requires
- deny.toml · re-reads the advisory ignore list (RUSTSEC-2017-0008 the only ignore) under `cargo deny check`
- deny-sync.toml · re-reads the tokio ban per sync crate
- scripts/channel-tokio-free.sh · re-reads `viola-channel`'s tokio-free normal graph
- scripts/deny-probes.sh · re-reads that every ban fires
- scripts/rmcp-features.sh · re-reads the rmcp feature check
- scripts/release-check.sh · re-reads the release build check
- .github/workflows/ci.yml · re-reads the `supply-chain` job
- crates/viola-mcp/Cargo.toml · re-reads Tokio containment
- crates/viola-channel/Cargo.toml · re-reads the `tokio` feature containment
- crates/viola-e2e/Cargo.toml · re-reads the test-only Tokio use
- fuzz/Cargo.lock · re-reads that it holds no tokio (its own audit not moved)

## Needs
- The `tokio` entry of `[workspace.dependencies]`: kept in the exact `=` pin form, moved to a patched version `>=1.53.3`; the committed `Cargo.lock` resolving the same version.
- RUSTSEC-2026-0386 itself: `tokio`, `informational = "unsound"`, patched `>= 1.51.6, < 1.52.0` and `>= 1.53.3`; read by `[advisories]` `unsound = "all"`, so only a version move clears it.
- The `deny.toml` `[advisories]` ignore list stays exactly one entry, RUSTSEC-2017-0008 via portable-pty `=0.8.1`; the ban on widening an ignore or adding a `skip` / `allow` to reach green binds the chunk's fix route.
- The Critical CVE response SLA: an advisory against a crate viola enables is answered by a bump within 7 days, never by the disabled-feature ignore arm.
- `cargo deny check`'s four families at CI's pinned cargo-deny version (floor `>=0.20.2`): any crate the new tokio or its companions newly pull must pass the licence `allow` list exactly (MIT, Apache-2.0, Zlib, Unicode-3.0; the 0BSD exceptions only for `doctest-file` and `recvmsg`), crates.io-only sources, no yanked crate, and the C-building-crate ban.
- Tokio containment: built only in `viola-mcp`, `viola-channel` behind its `tokio` feature and the test-only `viola-e2e`; the `deny-sync.toml` ban run per sync crate as sole root, `scripts/channel-tokio-free.sh` and `scripts/deny-probes.sh` keep their meaning and still pass.
- The rmcp range `>=3.4.1, <3.5` and the lock's rmcp `>=3.4.1` hold through the re-resolve (rmcp rides on tokio); rmcp's release-graph features stay exactly `server` + `transport-io`.
- The lock's floor bytes `>=1.11.1` (RUSTSEC-2026-0007) holds through the re-resolve (tokio depends on bytes).
- The `AsyncFd::new` / `with_interest` / `try_new` / `try_with_interest` deprecation in the patched versions, against the clippy gate's `-D warnings`: a call in viola's own code is measured at P3; a dependency's use (interprocess's tokio backend) is outside viola's lint gate.
- CI's `supply-chain` job (ubuntu): `cargo deny check` over the root `Cargo.lock`, then the `fuzz/Cargo.lock` advisories-and-sources step, both green on the chunk's push.
- The workspace MSRV floor 1.96 checked by the CI `msrv` job: the new tokio and any companion it pulls must still build under 1.96.
- The release build still carries `viola` only, judged by `scripts/release-check.sh` from the build's own artifact records.
- The open-advisory line of Dependency Security (the `deps-supply-chain` group) naming this route entry as its owner: closed by the wrap's amendment, not by this chunk's edit.
- The architecture Stack row naming Tokio 1.53.1: a wrap amendment, not this chunk's edit.

## Standing conventions
- S1 · every third-party version of the root graph pinned exactly in `[workspace.dependencies]` (rmcp's minor range the one exception) and `Cargo.lock` committed · .andromeda/security-plan-detail/dependency-security.md
- S2 · an audit or gate tool is expressed as a minimum floor, and a gate command is measured at the exact version CI pins · .andromeda/security-plan-detail/dependency-security.md
- S3 · any test the chunk adds runs under its own `--home`, with no secret or real-session content in a fixture · .andromeda/security-plan-detail/secret-management.md

## Not needed
- IPC access control and server verification (Windows DACL and SQOS open, Unix socket directory, peer euid check, the dated Epoch 6 interim gaps): no endpoint code changes, even though the channel's Tokio client rides on tokio.
- Input validation floors (paste text, `MAX_FRAME` readers, hook stdin, MCP tool inputs, `config.json`, child output): no product behaviour changes.
- Data Protection (home modes, `replace_private`, repository fixtures, verify's captures, handle inheritance): no state, file or fixture write changes.
- Error Handling and the NEVER-log floor (fixed `Display` messages, MCP `isError` codes, `diagnostics/`): no error or log line changes.
- Secret Management's storage and the CI artifact canary scan: no secret, log line or upload changes.
- The vendored ConPTY binaries' own audit and `scripts/conpty-vendor.sh`: outside every cargo graph, unmoved by a tokio bump.
- The `fuzz/` workspace's crate pins, its nightly toolchain and the nightly fuzz job: the fuzz graph holds no tokio.
- Workflow hardening (SHA-pinned actions, `permissions`, zizmor findings, `windows-mutants.yml`, the perf job, the G2 exemption): no workflow is edited.
- The v1.x release prerequisites (dist, cargo-auditable, self_update signatures, no rust-cache): no release stage in v1.
- The Windows DLL search restriction and sideload, the statusline shell-out and the budget gate: untouched product paths.
- The threat model's boundaries other than supply chain, and the bootstrap phase list (a route derivation, not a chunk rule).
