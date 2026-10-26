# Task — tests · 2026-10-26-tokio-rustsec-2026-0386-lock-bump

## The chunk
Moves the locked `tokio` from 1.53.1 to a version the advisory RUSTSEC-2026-0386 lists as patched (the floor `>=1.53.3`), moving the workspace's exact `=` pin with it, so `cargo deny check`'s advisories family stops failing and CI's `supply-chain` job reads green again.
The companion crates `tokio-macros`, `tokio-stream` and `tokio-util` move only where the new tokio requires it; no other dependency is bumped, and RUSTSEC-2017-0008 (via portable-pty `=0.8.1`) stays the only advisory ignore.
No product behaviour changes: no source file changes unless the bump forces one (the safe `AsyncFd` constructors deprecated in the patched versions, read by clippy `-D warnings`), and no wire contract, exit code, refusal word or log line changes.
It re-runs the tokio containment gates (`deny-sync.toml` per sync crate, `scripts/channel-tokio-free.sh`, `scripts/deny-probes.sh`) and the other gates (fmt, clippy, the workspace tests, `scripts/rmcp-features.sh`, `scripts/release-check.sh`) against the new lock.
It closes the red `supply-chain` read of ci#40257114709 (at `b1249fb1`) against that run's log, showing RUSTSEC-2026-0386 alone stood behind it, and reads CI's `supply-chain` job green on the chunk's push (the full row read at the wrap).

## Reached files
- Cargo.toml · edits the `[workspace.dependencies]` tokio pin
- Cargo.lock · edits the tokio entry and, only where the new tokio requires it, the tokio-macros, tokio-stream and tokio-util entries
- deny.toml · re-reads `cargo deny check` (advisories, bans, licences, sources) with its ignore list unchanged
- deny-sync.toml · re-reads the sole-root tokio ban per sync crate
- scripts/sync-crates.txt · re-reads the list of sync crates the tokio ban runs over
- scripts/channel-tokio-free.sh · re-reads the `viola-channel` tree form of the tokio ban
- scripts/deny-probes.sh · re-reads the proof that every ban fires
- scripts/rmcp-features.sh · re-reads the rmcp release-graph feature assertion
- scripts/release-check.sh · re-reads the release build carrying `viola` only
- crates/viola-mcp/Cargo.toml · re-reads Tokio's containment (Tokio built in `viola-mcp`)
- crates/viola-channel/Cargo.toml · re-reads Tokio's containment (Tokio only behind the `tokio` feature)
- crates/viola-e2e/Cargo.toml · re-reads Tokio's containment (the test-only crate's Tokio clients)
- .github/workflows/ci.yml · re-reads the `supply-chain` job's verdict on the chunk's push

## Needs
- The dependency and boundary policy entity (§1): the `cargo deny` tokio ban, the tokio-free `cargo check` of sync crates, clippy `-D warnings` and the release build carrying `viola` only, all exit-code gates in CI, re-run against the bumped lock.
- The supply-chain coverage trigger (§1, security Vector 9): `cargo deny check` with RUSTSEC-2017-0008 as its only ignore; the bump clears RUSTSEC-2026-0386 without widening that list.
- The CI `supply-chain` job (§9): one fail-closed ubuntu job running cargo deny over the root lockfile, the sole-root tokio ban per sync crate with the channel's tree form, the deny probes and zizmor, uploading their JSON; its advisory finding is the red this chunk closes.
- §9 supply-chain build failure conditions as re-read verdicts: a cargo deny advisory/ban/licence/source finding fails; `scripts/channel-tokio-free.sh` must end `channel-tokio-free: ok`; `scripts/deny-probes.sh` must end `deny-probes: 13/13 banned, control clean`.
- The cargo-deny version the acceptance reads is CI's pinned one (§9: each tool has exactly one version source, ci.yml's pins).
- The weekly `nightly.yml` advisories run (§9) over the root lockfile: the bumped lock is what it re-audits.
- The CI lint job (§9): fmt, clippy `-D warnings`, the tokio-free `cargo check` of each listed sync crate and the rmcp release-graph assertion, per OS; a call in viola's own code to a deprecated safe `AsyncFd` constructor would turn it red, a dependency's own use (interprocess's tokio backend) is not linted.
- §9 lint failure conditions as re-read verdicts: a listed sync crate failing to build without tokio (or an empty list) fails; `scripts/rmcp-features.sh` must end `rmcp-features: server transport-io`.
- §9 release-build failure condition as a re-read verdict: `scripts/release-check.sh` must end `release-check: viola only`, its probe `release-check probes: 5/5 refused, control clean`.
- The workspace tests on the new tokio (§9 per-OS `test` job, `agent-run run --coverage`, the local `run --unit` / `--integration` / `--e2e`), ending in `viola-harness gate --require <suites>`.
- The Tokio-bearing test paths the bump reaches: the MCP adapter-level handler tests in `crates/viola-mcp/tests/handlers.rs` that use `viola_channel::tokio_client::TokioClient` (§5 MCP), and the E2E MCP stdio path driven by the rmcp client over a `viola mcp` child (§6 ipc-internal MCP stdio).
- Tokio's test-side containment (§2 directory convention): `crates/viola-e2e` hosts the tokio-based test clients so the root package and the sync crates carry no tokio dev-dependency; the bump keeps that split.
- `viola-mcp`'s timer under tokio's paused clock (§8 Time) stays as it is; no test re-times it.
- The MSRV job (§9): an ubuntu job checks the workspace and runs the unit suite on Rust 1.96 against the bumped lock, part of the full CI row read at the wrap.
- The multi-os-compat matrix (§9): the workspace's Tokio paths are re-run on `windows-2025`, `macos-latest` and `ubuntu-latest`; a Linux-only local run is no proof for the other two (§11 Test Strategy).
- The red read closure: ci#40257114709's `supply-chain` log is read and its failure shown to be RUSTSEC-2026-0386 alone, no other advisory, licence, ban or source finding behind it.
- Agent-read CI verdicts (§9): job conclusions through `gh run view --json jobs` and artifacts through `gh run download`, the cargo-deny JSON among them.

## Standing conventions
- S1 · A gate's verdict is its printed ending line and exit code, read from the bare command, never a pipe's last command · .andromeda/test-plan-detail/ci-integration.md
- S2 · Each tool runs at the one version ci.yml pins; the local pre-push checks the host against those pins and installs nothing · .andromeda/test-plan-detail/ci-integration.md
- S3 · Zero flakiness: nextest `retries = 0`, a red stays red until its root cause is fixed, no `#[ignore]` parking · .andromeda/test-plan-detail/quality-gates-coverage-targets.md
- S4 · No gate is widened, skipped or retried to reach green (the coverage ignore regex, a threshold, a retry budget) · body §11 Test Anti-Patterns (CI, Quality)
- S5 · A test added for a forced source change follows `<subject>_<condition>_<expected>` naming and the layer prefixes (`cli_`, `mcp_`, …) · .andromeda/test-plan-detail/test-strategy.md

## Not needed
- The critical-path scenario procedures Paths 1–7 and E1–E5: no step or signal changes; they only re-run inside the workspace suite.
- The fake agent's contract behaviour, the recorded `fixtures/claude/<cli-version>/` sets, the `Mcp.` exchanges and fixture hygiene: no fixture is recorded, added or scrubbed.
- The mutation gate and `run --mutants`: judged at the epoch boundary through `/andromeda-code-audit`, never by a chunk.
- The performance budgets and the `perf` job's hook-deadline rows: the hook path is sync and no hook code changes.
- The fuzz workspace, its own lockfile audit, the corpus replay and the nightly `cargo fuzz run`: the fuzz workspace holds no tokio, so none of them moves.
- The ConPTY vendor verification (`scripts/conpty-vendor.sh`): the vendored binaries sit outside every lockfile.
- The security control negatives, exit-cause matrix, `list` row integrity, chaos, property and contract suites: no case is added or changed.
- Windows DACL / strict-modes stubs and the untestable zones (real `claude` CLI, cross-OS-user peers, non-persistent-ACL volumes).
- The harness command grammar, session record and status endpoint shape.
- Live `viola verify`, the capability ledger rows and the stamped CLI versions: no Claude-specific shape changes.
- The coverage thresholds' values and the per-crate unit coverage lists (§4): nothing in them is amended by a dependency bump.
