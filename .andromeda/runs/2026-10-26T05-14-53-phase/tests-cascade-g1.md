# tests · cascade read · group 1 of 3

## Carried
- (.andromeda/test-plan-detail/test-strategy.md:65) The supply-chain vector's CI gates must read green against the bumped lock: `cargo deny` over the root lockfile, the sole-root tokio ban, the ban probes, `scripts/release-check.sh` and the weekly root-lockfile advisories are the verdicts this chunk re-reads, and RUSTSEC-2017-0008 stays the only ignore.
  > | Supply chain (V9) | CI gates | §9 Lint stage (sync-crate `cargo check`, `scripts/orphans-check.sh`) + Supply-chain stage (cargo deny incl. the `fuzz/Cargo.lock` audit, sole-root tokio ban, ban probes, zizmor) + Release build stage (`scripts/release-check.sh`) + `nightly.yml` weekly advisories (root and fuzz lockfiles) |
  (.andromeda/test-plan.md:224)
  > - **Trigger type:** security-vector-coverage (supply chain)
  >   - **Source:** security excerpt, Vector 9; arch excerpt, Stack (tokio ban)
  >   - **Required test type:** CI gates on exit code: `cargo deny check` (only ignore RUSTSEC-2017-0008), `cargo check` of sync crates without tokio, zizmor on workflows, SHA-pinned Actions assertion.
  (.andromeda/test-plan.md:83)
  > - **Entity:** dependency and boundary policy (the `cargo deny` tokio and C-crate bans, the `cargo check` of sync crates without tokio, clippy `-D warnings`, the cargo-modules orphans gate per lib/bin target plus the on-demand `--acyclic` boundary review, the release build carrying `viola` only, SHA-pinned Actions / zizmor)
  >   - **Source:** arch excerpt, Stack (std threads + Tokio; rustfmt / clippy / cargo-deny / cargo-modules) and CI/CD; security excerpt, Vector 9
  >   - **Testability:** testable (exit-code gates in CI)
- (.andromeda/test-plan-detail/test-strategy.md:44) Tokio's test-side containment must hold after the bump: tokio stays in `viola-e2e` alone among the test homes, the root package and the sync crates carry no tokio dev-dependency, and the ban runs `deny-sync.toml` once per crate of `scripts/sync-crates.txt` as sole root with no allowlist or `--exclude` added to reach green.
  > - **Why a test-only crate:** `crates/viola-e2e` hosts `viola-harness` and the tokio-based test clients (rmcp client), so the root package and the sync crates carry no tokio dev-dependency. It is never a root of the tokio ban: the ban runs `deny-sync.toml` once per crate in `scripts/sync-crates.txt`, each as the sole root, so `viola-e2e`'s tokio graph lies outside it with no `wrappers` allowlist and no `--exclude` (an `--exclude` false-fails on a feature-unified optional tokio).
  (.andromeda/test-plan-detail/test-strategy.md:43)
  > - **Tokio-client E2E suites and the harness binary `viola-harness`:** `crates/viola-e2e/` (publish = false).
  (.andromeda/test-plan.md:315)
  > - **Directory pattern:** flat `crates/viola-<area>` plus the root bin and one test-only crate, `crates/viola-e2e` (`publish = false`), which hosts `viola-harness` and the tokio-based test clients so that the root package and the sync crates carry no tokio dev-dependency; unit tests are inline, crate integration tests sit in each crate's `tests/`, and the sync E2E suites in root `tests/` share one set of helpers in `tests/support/`, which no root test file copies.
- (.andromeda/test-plan-detail/test-scope-summary.md:8) The `viola-channel` Tokio client feature, whose tokio the chunk moves, must keep passing its coverage on the new version; its re-run is part of the chunk's witness, not a re-check.
  > - **Entity:** `viola-channel`: JSON-RPC 2.0 over ndjson on `interprocess` local sockets. Covers the sync client/server, the Tokio client feature, the FNV-1a endpoint hash, `MAX_FRAME` (16 MiB) and the result/refusal/error shapes
- (.andromeda/test-plan-detail/test-strategy.md:14) The crate-level MCP integration suite, whose handlers run over the Tokio client, must pass on the bumped tokio as the integration witness that the bump changes no MCP contract shape.
  > - **Tier-specific depth (Comprehensive, viola):** Required. Every arch Standard Contract (Wrapper channel, On-disk, Hooks, CLI, MCP) has a crate-level `tests/` suite, using a real `interprocess` endpoint, a real tempfile home, and assert_cmd against `CARGO_BIN_EXE_viola`.
  (.andromeda/test-plan.md:449)
  > - **MCP:** adapter-level handler tests in `crates/viola-mcp/tests/handlers.rs` drive the transport-free `viola_mcp::Tools` handle (the same `Server::call` `serve` answers with) against a `viola_channel::Server` test wrapper (`TestWrapper`, its snapshot written through `viola_state::snapshot::write_snapshot`); the handlers under test use the Tokio client (`viola_channel::tokio_client::TokioClient`, `viola-channel`'s `tokio` feature), never the sync client (chunk 2026-10-17-mcp-server-for-drivers). The full stdio path is E2E (§6).
- (.andromeda/test-plan-detail/test-scope-summary.md:62) The MCP stdio surface is the Tokio-bearing E2E path the bump reaches: its rmcp-client run over a `viola mcp` child must keep every signal it asserts on the new tokio.
  > - **Surface:** ipc-internal (MCP stdio server `viola mcp`, tools `send · wait · last · answer · list`)
  >   - **Driver:** language-native MCP stdio test client. It spawns `viola mcp --home <tmp>` and speaks JSON-RPC over stdin/stdout, with JSON schema assertion.
  (.andromeda/test-plan-detail/test-strategy.md:61)
  > | MCP (V5) | E2E (rmcp client) | §6 Scenarios 2, 5, 7 + Security sweep |
- (.andromeda/test-plan-detail/test-strategy.md:17) The E2E level runs through the rmcp client, a tokio-based client of `viola-e2e`, so the E2E suite must pass on the bumped tokio before the chunk claims no behaviour change.
  > - **Tier-specific depth (Comprehensive, viola):** Required for all 7 critical paths plus 4 expansion scenarios (Section 6). They run against harness-booted `viola run … -- viola-fake-agent`, using the outer PTY (portable-pty) and the rmcp client.
- (.andromeda/test-plan-detail/test-strategy.md:70) The workspace's Tokio paths must be re-run on the full CI matrix, and the chunk's acceptance reads all three runners, never a Linux-only local run, as the proof for Windows and macOS.
  > | multi-os-compat | CI matrix | §9 Matrix builds |
  (.andromeda/test-plan.md:248)
  > - **Trigger type:** multi-os-compat
  >   - **Source:** arch excerpt, CI/CD (matrix `[windows-2025, macos-latest, ubuntu-latest]`) and Test-Relevant Conventions ("Every OS-specific branch compiles and is tested on its CI runner"); creator excerpt, Founder Direction 4 and D3
  >   - **Required test type:** full workspace and fake-agent E2E on all three runners. OS-branch tests cover named pipe vs Unix socket, ConPTY vs openpty, Windows DACL / strict-modes vs Unix modes, and macOS euid + dir-only server verification.
- (.andromeda/test-plan.md:873) A green Linux-dev-host run of the bumped lock is no verdict for the Windows or macOS legs of the Tokio paths; those are read only from their own CI runners.
  > - NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
- (.andromeda/test-plan-detail/test-strategy.md:10) The unit suite the chunk re-runs (and the MSRV job re-runs on Rust 1.96 against the bumped lock) runs on all three OSes under nextest, never a plain `cargo test`, whose known-cause reds are no verdict on the bump.
  > Runs on all 3 OSes, except the two real-cargo-mutants harness tests, compiled out on macOS (§10 Mutation gate).
  (.andromeda/test-plan-detail/test-strategy.md:11)
  > - **Runner:** nextest is the runner; a plain `cargo test` run (`cargo test --bin viola`) is not supported, and under it the root-bin cases `run::gate::tests::feed_panic_writes_one_parse_rejected_line_until_the_size_changes`, `obs::tests::log_config_rejection_writes_one_warn_line` and `case_2_sigterm` are known-cause reds, recorded with no owner.
  (.andromeda/test-plan.md:349)
  > **Framework:** cargo test (libtest, current Rust stable pinned in `rust-toolchain.toml`, ≥ 1.96; MSRV 1.96, edition 2024), executed through cargo-nextest 0.9.146 (process-per-test, JUnit). Doctests run through `cargo test --workspace --doc`, because nextest cannot run them.
- (.andromeda/test-plan-detail/test-scope-summary.md:78) The workspace-test re-run goes through the harness `run`, whose exit 0 requires every suite to pass; the mutation step stays outside the chunk's default run.
  > - `run`:
  >   - Invokes, in order:
  >     1. the workspace unit and integration tests (`cargo test`)
  >     2. fake-agent E2E against `fixtures/claude/*`
  >     3. none in 0.1.0: the browser suite left with the browser leg, and the mutation step keeps its number 4
  >     4. mutation testing scoped to the chunk diff (Founder Direction 1: "surviving mutants are red") — only under `run --mutants`, never in the default or `--all`; its cadence is the epoch-boundary code audit
  >   - Exit 0 only if every suite passes and no mutant survives. Non-zero otherwise, with a JSON summary on stdout (suite, passed, failed, survived).
- (.andromeda/test-plan.md:338) The harness binary is built from `viola-e2e`, whose tokio graph the bump moves, so the harness must still build and report each command's verdict as one JSON document and exit code on the new lock.
  > **Harness implementation:** `scripts/agent-run.sh` (POSIX) and `scripts/agent-run.ps1` (PowerShell) are identical thin shims. They run `cargo run -q -p viola-e2e --bin viola-harness -- <command> [flags]` and forward its exit code and stdout unchanged.
  (.andromeda/test-plan.md:341)
  > **Exit codes:** every harness command exits 0 on success, 1 on failure and 2 on usage error. Each prints exactly one JSON document on stdout, starting `{"v":1,"cmd":"<command>","ok":<bool>,…}` (key order held by serde_json `preserve_order`).

## Bindings
- **Supply-chain CI gates ↔ dependency and boundary policy entity ↔ the CI plan**: the V9 row's gates (.andromeda/test-plan-detail/test-strategy.md:65) are the exit-code gates of the policy entity (.andromeda/test-plan.md:83) and the supply-chain trigger (.andromeda/test-plan.md:224), and their verdicts are the §9 supply-chain, lint and release-build stages another group carries; the chunk's acceptance reads them at CI's pinned versions.
- **Test-only crate split ↔ sole-root tokio ban**: tokio living only in `viola-e2e` among test homes (.andromeda/test-plan-detail/test-strategy.md:44) is what lets the per-sync-crate `deny-sync.toml` ban and the channel tree form re-read green with no allowlist; a bump that pulled tokio into a sync crate's graph would red both.
- **Tokio client feature ↔ MCP integration ↔ MCP stdio E2E**: the channel's Tokio client (.andromeda/test-plan-detail/test-scope-summary.md:8) is what the `handlers.rs` suite drives (.andromeda/test-plan.md:449), and the full stdio path above it is the rmcp-client E2E surface (.andromeda/test-plan-detail/test-scope-summary.md:62); together they are the Tokio-bearing witnesses of the bump.
- **Unit runner ↔ multi-os matrix ↔ the Linux-only ban**: the unit and E2E re-runs (.andromeda/test-plan-detail/test-strategy.md:10) count only on all three CI runners (.andromeda/test-plan-detail/test-strategy.md:70), never on a Linux-only local run (.andromeda/test-plan.md:873).
- **Harness `run` ↔ harness binary in `viola-e2e`**: the workspace-test verdict (.andromeda/test-plan-detail/test-scope-summary.md:78) is printed by a binary built from the bumped tokio graph (.andromeda/test-plan.md:338).
