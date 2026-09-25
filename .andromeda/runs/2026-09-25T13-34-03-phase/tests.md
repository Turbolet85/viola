# tests extract

## Relevance
Relevant. This chunk lands the `viola-pty` seam, the tui outer-PTY driver, the E2 scenario, the `.cmd` refusal negative, and the harness move from stdin pipes to PTYs. All of these are named test surfaces in the plan.

## Constraints
- **Tier.** The tier is Comprehensive (per test-plan §1 Test Scope Summary). New code must meet the §10 per-OS coverage floors: lines 85, functions 95, regions 80. It must also pass the mutation gate: `missed == 0`, `timeout == 0`, `unviable <= caught`. A `#[cfg(windows)]` body (for example the `TerminateProcess` fallback) is killed only on the windows-2025 leg under the two-leg union (per test-plan §10 Mutation gate; §9 Mutation row).
- **Exit on the handle, never on EOF.** `viola-pty` reports exit from `wait()` on the process handle, never from reader EOF. The unit tests use a `mockall` double of the six-method `Pty` seam: its reader never returns EOF while `wait()` returns an exit status. Doubles are only allowed through `#[cfg_attr(test, mockall::automock)]` on seam traits (per test-plan §4 viola-pty; §8 Process mocking row).
- **Integration against a real PTY.** The module ↔ PTY integration uses a real ConPTY/openpty spawn of `viola-fake-agent` through `viola-pty`. It must cover three things: the no-EOF mode detects exit on the handle, a resize is propagated, and a `.cmd` child is refused under `#[cfg(windows)]` (per test-plan §5 Boundary types, Module ↔ PTY). The Windows npm shim is tested with a fixture `.cmd` shim next to a stub `claude.exe` in a tempdir (per test-plan §5 Module ↔ external API; §4 viola-agent-claude).
- **R8 strip oracle.** The R8 strip is checked against a literal 14-name S6 list written into the test, never imported from the product constant (per test-plan §4 viola-agent-claude; §11 Unit). Env is set per child with `Command::env`, never `set_var` (per test-plan §7 Test data lifecycle; §11 Integration).
- **Harness switch to outer PTYs.** The harness switches to outer PTYs once the seam exists:
  - `supervise` owns one portable-pty outer PTY per instance and drains each master on its own thread.
  - `cleanup` step 1 writes `\x03` into each PTY instead of the stdin pipe.
  - Exit is confirmed via `child.wait()` with a 10 s deadline, then `kill()`.
  (per test-plan §3 `boot` step 5, `cleanup` steps 1–2, Internal harness subcommands `supervise`.) The interim readiness check (`process-start` lines for `self`/`claude-child`) stays in force, because the scope excludes `snapshot.json`, `heartbeat` and `events.ndjson` (per test-plan §3 Readiness signal).
- **Root `booted_wrapper` bound.** The root `booted_wrapper` stays bounded at 10 s and exit-aware (`child.try_wait()`). This keeps it under cargo-mutants' 20 s auto-timeout floor, and the `[profile.mutants]` slow-timeout is 5 s × 2 (per test-plan §3 `run` step 2; §3 Bootstrap test-runner-install).
- **CI gates this chunk touches.**
  - The orphans gate runs per lib/bin target on all three `lint` legs (per test-plan §9 Lint row).
  - `viola-pty` joins `scripts/sync-crates.txt` for the tokio-free `cargo check` and the sole-root tokio ban (per test-plan §9 Lint and Supply-chain rows).
  - `scripts/install-ripgrep.sh` keeps its checksum verification against the release's published sha256 and prints `tool-missing: <tool>` (per test-plan §3 Bootstrap ci-tool-install).

## Patterns to follow
- **Outer-PTY driver.** The tui driver is `tests/support/outer_pty.rs` over the `viola-pty` seam. Verdicts come only from the fake-agent receipt, the diagnostics lines and the exit status (per test-plan §6 Drivers per surface, tui row; §2 directory conventions).
- **Fixture chain.** Root `tests/` use the rstest chain `home → fake_agent_path → stamped_home → booted_wrapper`. The `viola_e2e::fixtures` copy lands with its first consumer among Paths 1/5/E1/E2/E5 (E2 is this chunk's candidate). Both copies must hold one contract (per test-plan §3 `run` step 2).
- **Fake-agent receipts and modes.** The fake agent's `env {names}` receipt (sorted names, never values) and `fds` receipt (Unix only) are the E2 oracle. `--exit-no-eof` is the no-EOF mode (per test-plan §7 Fake agent).
- **Long-lived wrapper handling.** A long-lived `viola run` is held with std::process plus a `Drop` guard that calls `kill()` then `wait()` (per test-plan §5 Setup/teardown lifecycle; §11 E2E).
- **Naming.** Test binaries use the `tui_` / `chaos_` prefixes so the §3 layer filtersets select them. Functions follow `<subject>_<condition>_<expected>` (per test-plan §2 File naming, Test function naming; §3 `run` step 1 integration filterset).

## Anti-patterns to avoid
- Never detect child exit by reading master EOF. Never parse the rendered child screen or PTY master output for content (per test-plan §11 E2E).
- Never use `assert_cmd` `.assert()` on the long-lived `viola run` wrapper. Never use `sleep(N)` for sync: wait on file-state or receipt offsets (per test-plan §11 E2E).
- Never treat a Linux-only local run as proof for Windows. ConPTY, `.cmd` and handle-inheritance behaviour is verified only on windows-2025 (per test-plan §11 Test Strategy).

## Contract bindings
- **tests ↔ security (secret scan).** The stripped `CLAUDE*` values must never reach a log or `diagnostics/` file. The `secret-scan` `claude-stripped` class scans for the fixed canaries planted in root tests. The `scan_patterns.rs` contract test requires every `canary-<kind>-value-<hex4>` literal to be in the scan table, so new E2 canaries must be registered there (per test-plan §3 Internal harness subcommands `secret-scan`; §6 E2 Verification signal).
- **tests ↔ obs (readiness and logs).** Interim readiness reads the obs-owned `diagnostics/run-<name>.ndjson` `process-start`/`process-exit` lines. `exit_source` vocabulary changes are grepped by the harness, and new diag lines must pass `schema-check` G4 (per test-plan §3 Readiness signal, §3 Log format, Bootstrap log-format-bind-with-obs).
- **tests ↔ arch (E2 run contract).** E2 expects `VIOLA_NAME` and `VIOLA_DIR` to be present in the child env. The scope leaves open whether this chunk sets them, so whether E2 can land in full here is for P3/P4 to settle (per test-plan §6 E2 Verification signal).
- **tests ↔ security (handle inheritance).** Windows handle non-inheritance is asserted at the `viola-channel` unit boundary, not in E2. The channel is out of scope here, so only the Unix fds 0/1/2 + PTY-slave check can bind now (per test-plan §4 viola-channel; §6 E2).

## Acceptance criteria contributions
- (tests) Four `viola-pty` tests pass under `cargo nextest` on windows-2025:
  - a unit test with a mocked non-EOF reader and a `wait()` exit
  - a real-ConPTY integration test where the `--exit-no-eof` fake child's exit is detected on the handle
  - a propagated resize
  - a `.cmd`/`.bat` child refused with `run` exit 1 (`#[cfg(windows)]`)
  (per test-plan §4 viola-pty; §5 Module ↔ PTY; §6 Exit-cause matrix and Chaos suite)
- (tests) E2 passes, if landed. `viola run` with the 14 S6 vars set to random canary values produces an `env` receipt holding none of the stripped names. `secret-scan` then reports 0 `claude-stripped` hits. The npm-shim test resolves the fixture `.cmd` shim to the stub `claude.exe` (per test-plan §6 E2; §4 viola-agent-claude).
- (tests) The mutation gate passes on the chunk diff: the union over the ubuntu and windows legs has 0 missed and 0 timeout, and no leg has `unviable > caught`. The coverage gate `--fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80` passes per OS (per test-plan §10 Mutation gate; §10 Coverage thresholds).
- (tests) Four harness and CI checks pass:
  - `agent-run boot` then `cleanup` over outer PTYs returns `ok:true`, with `processes_gone:true` and no field `false`.
  - `scripts/orphans-check.sh` ends `N/N targets clean` on all three `lint` legs, with `viola-pty` and any cfg-gated module present.
  - The ripgrep install still fails on a sha256 mismatch.
  - `deny-probes.sh` still ends `13/13 banned, control clean` with the RUSTSEC-2017-0008 ignore via portable-pty.
  (per test-plan §3 `cleanup` Verification; §9 Lint and Supply-chain rows; §3 Bootstrap ci-tool-install)

## Relevant amendment history
- **2026-09-24-three-os-ci-headless-harness-skeleton, interim supervisor.** The supervisor holds a stdin pipe per wrapper "until `viola-pty` exists". Interim readiness is the `process-start` lines, and `endpoint_gone`, `port_free` and `url_file_removed` are `null` in the interim. This chunk is the named exit from the stdin-pipe interim. Why: detach flags were removed, and the grammar grows per chunk.
- **2026-09-24-three-os-ci-headless-harness-skeleton, harness target dir.** The harness builds into `target/harness`. Why: Windows `os error 5` when relinking a running `viola-harness.exe`. This matters again if PTY-held processes outlive tests.
- **2026-09-24-fake-agent-and-test-data-fixtures, consumer-first.** The `viola_e2e::fixtures` copy lands with its first consumer, and no shape is invented before a recorded fixture. The `--exit-no-eof` mode is built. Why: operator P4 "consumer-first".
- **2026-09-24-observability-gates.** The root `booted_wrapper` is 10 s and exit-aware, and the mutants slow-timeout is 5 s × 2. Why: `wait_ready` spinning to 20 s graded mutants as Timeout. The same amendment moved ripgrep 15.2.0 to `scripts/install-ripgrep.sh`, which is the script this chunk hardens.
- **2026-09-24-quality-gates.** A two-leg mutation union was added. Why: cargo-mutants reports another OS's `#[cfg]` bodies as missed, so Windows-only PTY code is judged by the windows leg.
- **2026-09-24-workspace-tree-and-code-graph-planes.** The orphans gate runs per lib/bin target. This is the context for the CARRY hypothesis about cfg-gated modules reading as orphans.
- **2026-09-24-epoch-1-cleanup.** The `unviable-exceeds-caught` red rule was added. Why: a leaked supervisor locked `viola-harness.exe` (135/143 unviable on windows). This chunk rewrites the supervisor, so leaked PTY-owning processes are a known mutation-gate hazard.
- **2026-09-25-security-prerequisites.** The `test-only-rust-delta` verdict was added. It applies only if this chunk's diff turns out to be test-only Rust, which is unlikely given the new `viola-pty` crate.
