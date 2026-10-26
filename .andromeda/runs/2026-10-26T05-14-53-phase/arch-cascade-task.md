# Task — arch · 2026-10-26-tokio-rustsec-2026-0386-lock-bump

## The chunk
Moves the workspace's exact `tokio` pin and the locked `tokio` from 1.53.1 to a version RUSTSEC-2026-0386 lists as patched (`>=1.53.3`, the `=` pin form kept; the exact version is P4's lean), so `cargo deny check` stops failing on that advisory and CI's `supply-chain` job reads green, RUSTSEC-2017-0008 (via portable-pty `=0.8.1`) still the only advisory ignore.
Companion crates `tokio-macros`, `tokio-stream` and `tokio-util` move only where the new tokio requires it; no other dependency is bumped, and no source file changes unless the bump forces one (the advisory's deprecation of the safe `AsyncFd` constructors read by clippy's `-D warnings`, measured at P3).
Re-runs and reads back, never re-builds: `cargo deny check` at CI's pinned cargo-deny version with the ignore list unchanged, the per-sync-crate `deny-sync.toml` tokio ban, `scripts/channel-tokio-free.sh`, `scripts/deny-probes.sh`, `scripts/rmcp-features.sh`, `scripts/release-check.sh`, fmt, clippy and the workspace tests.
Closes the CARRY that names this entry as the owner of the `supply-chain` red on every CI run since the 2026-10-20 wrap (ci#40257114709 at `b1249fb1` the newest), confirming from that run's log that RUSTSEC-2026-0386 is its only finding.

## Reached files
- Cargo.toml · edits `[workspace.dependencies]` tokio pin
- Cargo.lock · edits the `tokio` entry and, only where the new tokio requires it, `tokio-macros`, `tokio-stream`, `tokio-util`
- deny.toml · re-reads `cargo deny check` (advisories, bans, licences, sources; the ignore list unchanged)
- deny-sync.toml · re-reads the sole-root tokio ban per sync crate
- scripts/sync-crates.txt · re-reads the four sync crates the tokio ban runs over
- scripts/channel-tokio-free.sh · re-reads `viola-channel`'s tokio-free normal graph without its `tokio` feature
- scripts/deny-probes.sh · re-reads every ban firing
- scripts/rmcp-features.sh · re-reads rmcp's exact `server` + `transport-io` features
- scripts/release-check.sh · re-reads the release build carrying `viola` only
- crates/viola-mcp/Cargo.toml · re-reads tokio containment (the one product crate enabling tokio)
- crates/viola-channel/Cargo.toml · re-reads tokio containment (its `tokio` feature, `interprocess/tokio`)
- crates/viola-e2e/Cargo.toml · re-reads tokio containment (test-only tokio)
- fuzz/Cargo.lock · re-reads no tokio entry (its own advisories and sources audit not moved)
- .github/workflows/ci.yml · re-reads the `supply-chain` job

## Needs
- The Stack value naming Tokio 1.53.1 built only inside `mcp`, and the Inherited Defaults line naming the same version: the chunk changes the version they carry (their amendment is the wrap's, not the chunk's edit).
- The Concurrency / Backend Framework keystone (Tokio only in `mcp`, enforced by the sole-root `cargo deny` tokio ban and a `cargo check` of the sync crates on three OSes): the bump must leave that containment and its CI enforcement unchanged.
- Module Boundaries: among the product crates only `viola-mcp` enables `tokio`; `fuzz/` is its own workspace — the bump adds no tokio edge to any other crate.
- The Tokio containment cross-cutting pattern: no new runtime or async code outside `viola-mcp`, so a forced source change (if any) stays on the `mcp` side.
- The workspace crates registry: `viola-channel`'s `tokio = ["dep:tokio", "interprocess/tokio"]` feature enabled by `viola-mcp`, the sync crates listed in `scripts/sync-crates.txt`, the test-only `viola-e2e` — the graph the tokio bans read.
- The `fuzz/` workspace with its own committed `Cargo.lock`, excluded from the root graph: the bump does not reach it and its separate audit stays as it is.
- The Message Broker / IPC decision (interprocess 2.4.4 local sockets, its Tokio backend behind `viola-channel`'s feature): interprocess's own use of the deprecated `AsyncFd` constructors is not linted by viola's gate, and interprocess's version does not move.
- The MCP decision (rmcp 3.4.1 stdio server, which requires Tokio; rmcp features exactly `server` + `transport-io`): the new tokio must resolve under rmcp's pin with rmcp unchanged.
- The advisory's deprecation of `AsyncFd::new`, `with_interest`, `try_new`, `try_with_interest` in the patched versions: viola's own code must not call them, since clippy runs with `-D warnings`; a call found is the only reason a source file may change.
- The workspace pin form: every version in `[workspace.dependencies]` is pinned, tokio exactly with `=` and `default-features = false`; the new pin keeps that form.
- The other versions the plan names (portable-pty `=0.8.1`, interprocess 2.4.4, rmcp 3.4.1, clap 4.6.7, tempfile 3.27.0, serde_json 1.0.151, chrono 0.4.45, thiserror 2.0.20, anyhow 1.0.104, vt100 0.16.2, nutype 0.8.0) stay where they are: no other dependency is bumped.
- The PTY decision's portable-pty `=0.8.1` pin: the source of RUSTSEC-2017-0008, the one advisory ignore, which stays the only one.
- The bumped crates must stay inside the root graph's licence allow list and crates.io-only sources, with no new licence exception, skip or allow.
- `target/supply-chain/` (`deny.json`, `deny-fuzz.json`, `zizmor.json`, the CI `supply-chain` artifact): the evidence the chunk's green and the closed red are read from.
- `target/deny-probes/run-<utc>-<pid>/`: the throwaway projects `scripts/deny-probes.sh` writes when it is re-run.
- The release build carrying `viola` only (no test-only binary or test-only feature): re-read after the lock moves.
- Config management's rule that environment variables are not a configuration channel, a dependency reading one breaking it like a direct read: the new tokio under the enabled features must add no environment read.
- Deployment by `cargo install --path .` against the committed `Cargo.lock`: the lock is the shipped dependency set the bump changes.

## Standing conventions
- (none)

## Not needed
- Conventions (RefusalReason words, exit codes, MCP result shapes): the chunk changes no refusal, exit code or MCP result.
- Standard Contracts (the `list` payload, snapshot envelope, ledger stamps, instance snapshot, channel methods, event data, hook statusline, session liveness): no wire contract changes.
- The capability ledger and the CLI Version Compatibility decision: no `claude` behaviour, ledger row or `viola verify` probe is touched.
- The Screen Model, Delivery Confirmation, Hook Transport and Hook Contract decisions: no readiness, send-confirmation or hook behaviour changes.
- The Human Takeover Wheel, Links and Budget Governor decisions: no wheel, link or budget logic is touched.
- The Session Liveness, Plugin Scope and CLI Conventions decisions: no liveness, plugin file or CLI flag changes.
- The PTY decision's ConPTY sideload, DLL-search and DA1 parts: only its portable-pty pin bears on the ignore list.
- The Database / State Store and Snapshot Writer decisions' disk formats and lock files: no state file or write path changes.
- IPC endpoint names, Claude Code integration names and the `VIOLA_*` environment variables: no endpoint, plugin, MCP tool name or env var changes.
- The filesystem registry's home layout and test homes (`instances/`, `budget.json`, `ledger/`, `target/e2e-home/`, `target/conpty-seed/`, `target/mutants/`): none is created or changed.
- The schemas under `schemas/` and the recorded fixtures under `fixtures/claude/`: no schema or fixture changes.
- The vendored ConPTY binaries and their audit (`vendor/conpty/`, `scripts/conpty-vendor.sh`): outside every lockfile, untouched.
- The fuzz workspace's targets, corpus and nightly toolchain: the fuzz lockfile holds no tokio.
- Project Intent's scale path and v1.x distribution plans: no release or distribution work.
