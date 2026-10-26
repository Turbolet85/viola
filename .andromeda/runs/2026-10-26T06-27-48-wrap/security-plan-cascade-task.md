# Task — security-plan · 2026-10-26-tokio-rustsec-2026-0386-lock-bump

## The session
The session bumped the workspace's exact tokio pin from `=1.53.1` to `=1.53.3` (`default-features = false` kept) and re-locked it with `cargo update -p tokio --precise 1.53.3`. The lock delta is 4 lines, the tokio entry alone: its version and its checksum.
It retired the open advisory RUSTSEC-2026-0386 (unsound, tokio; patched `>= 1.51.6, < 1.52.0` and `>= 1.53.3`) against the locked tokio. `cargo deny check` over all four families (cargo-deny 0.20.2, the CI pin) exits 0, and the advisory is gone from its output. No ignore was added or widened, and `deny.toml` is untouched.
CI job `supply-chain` was red on every run since the 2026-10-20 wrap. It reads green on ci#40278510123 (sha `948f7f01`, 15/15 checks), and its steps after the deny step (fuzz lockfile audit, tokio ban per sync crate, ban probes, zizmor) ran green on CI for the first time since that red.
It re-measured tokio containment unmoved: the four sole-root `deny-sync.toml` bans exit 0, `channel-tokio-free: ok`, and `deny-probes: 13/13 banned, control clean`. No source, crate manifest, deny file, workflow or toolchain changed. No symbol, IPC method, env var or crate was added.

## Reached files
- Cargo.toml
- Cargo.lock

## Touches
- dependency-security `deps` group (line 14): the pinned tokio version moves from `=1.53.1` to `=1.53.3`. The exact `[workspace.dependencies]` pin form and `default-features = false` stay as they were.
- dependency-security `deps-supply-chain` group (line 58): the open-advisory line naming RUSTSEC-2026-0386 against the locked tokio is retired, because this chunk closed it. The same line also states tokio `1.53.1` and names its owner route entry «tokio RUSTSEC-2026-0386 lock bump», which is this chunk. All three facts on it go together.
- The `supply-chain` CI job's standing red since the 2026-10-20 wrap: it is now green on ci#40278510123. The report places this fact only on the line-58 open-advisory line, and no other master site carries it.
- The `[advisories]` ignore list was re-read and is unmoved: RUSTSEC-2017-0008 (portable-pty `=0.8.1`) stays the only ignore, and no ignore was widened to reach green.
- The audit tool floors were re-read and are unmoved: cargo-deny `>=0.20.2` was run at the 0.20.2 CI pin, and zizmor `>=1.30.1` at 1.30.1. Job 4's `cargo deny check` over all four families and the separate fuzz lockfile audit (advisories, sources) both read green.
- The tokio containment bans were re-read and are unmoved: `deny-sync.toml` per sync crate as sole root over `scripts/sync-crates.txt`, `scripts/channel-tokio-free.sh` and `scripts/deny-probes.sh` (13/13 banned, control clean). Tokio is still built only in `viola-mcp`, in `viola-channel`'s optional `tokio` feature and in the test-only `viola-e2e`.
- The Pinning rules: the committed `Cargo.lock` and the `[workspace.dependencies]` exact pins. One exact pin moved (tokio). `tokio-macros` 2.7.2, `tokio-stream` 0.1.19, `tokio-util` 0.7.20 and every other locked crate stay where they were. Any stated lock delta or crate version other than tokio's is unmoved.
- The lock-resolve floors in Pinning (bytes `>=1.11.1` for RUSTSEC-2026-0007): tokio now resolves inside RUSTSEC-2026-0386's patched range through its exact pin. The report adds no floor line of that shape.
- Critical CVE response SLA: an advisory against a crate viola enables is answered by a bump, not an ignore. This chunk is that path exercised for tokio, and no `ignore` entry with a disabled-feature reason was added.
- The weekly `nightly.yml` advisory run (`cargo deny check advisories` over the root and fuzz lockfiles) is unchanged. It now reads the root lockfile with tokio 1.53.3.
- The rmcp feature gate (`scripts/rmcp-features.sh`, `server transport-io`) was re-read and is unmoved. So is the `[[bans.features]]` rmcp ban.
- Threat-model `boundary-supply-chain` (lines 144–150): its trust boundary (`cargo deny check`, the sync-crate tokio bans, `Cargo.lock`, the fuzz lockfile audit) held. Any open-advisory state or tokio version stated there moves the same way as line 58.
- Toolchain and MSRV: tokio 1.53.3 declares rust-version 1.71 and the same dependency set as 1.53.1. `RUSTUP_TOOLCHAIN=1.96 cargo check --workspace` is green, so the 1.98.1 pin and the 1.96 floor are unmoved.
- tokio 1.53.3 deprecates `AsyncFd::{new, with_interest, try_new, try_with_interest}`. None of them reaches a viola call site, and clippy `-D warnings` reads green with no `deprecated` line on the host and on `x86_64-pc-windows-msvc`. No security-relevant tokio API use changed.
- The CI integration controls were re-read and hold, with no workflow changed: SHA-pinned actions, `permissions: {}` and `contents: read`, and the zizmor step.

## New facts
- (none)

## Untouched
- authentication-authorization: the Windows SDDL listener, the Unix socket directory and peer euid checks, client server verification and its dated Epoch 6 exceptions. No source or IPC method changed.
- input-validation: paste text, channel frames, hook stdin, MCP tool inputs, `config.json`, statusline source and shell-out, child output and the `MAX_FRAME` constant. No parser or reader changed.
- data-protection: home 0700/0600 modes, `replace_private`, repository fixtures, verify's accepted risks and residuals. No state or fixture surface was reached.
- secret-management: the R8 strip, the artifact canary scan classes and the pre-push `env -i` launch. No secret, seam or env var was added.
- error-handling: fixed `Display` messages, channel `-32603`, MCP `isError` codes and hook fail-open. No error path changed.
- dependency-security `dep-conpty` groups (lines 8 and 26): the vendored ConPTY binaries, their four pins in `src/conpty.rs` and `scripts/conpty-vendor.sh`. They sit outside every lockfile and did not move.
- dependency-security `exception` groups (lines 20 and 42): the `0BSD` per-crate licence exceptions and the `fuzz/` test-only exemption. No licence or exemption changed.
- security-anti-patterns: every ban (listeners, socket placement, SQOS, `try_overwrite`, mutable action refs, rust-cache, self_update signatures, the G2 exemption). None was added, loosened or exercised differently.
- bootstrap-phases: the phase list and its `dep-audit-tooling-install` and `dep-security-ci-gate` contents. No tool, gate or phase was added or reordered.
- threat-model-summary boundaries other than supply chain (IPC, paste, terminal, hook stdin, MCP, CLI, filesystem, child spawn), and the data and infrastructure groups. No surface changed.
- Security Decisions Log: there is no new ratification, exception or licence entry. The bump needed no founder widening.
