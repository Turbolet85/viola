# tests · Test Strategy — category read

## Carried
- (.andromeda/test-plan-detail/test-strategy.md:10) The `pin_bytes` / `pin_exe` concurrency fix in `crates/viola-state/src/pin.rs` needs inline unit cases in its own crate, and they must run on all three OSes.
  > Inline `#[cfg(test)] mod tests` in all 6 crates plus the root bin. rstest 0.27.0 `#[case]` tables cover the refusal-order matrices, the exit-code table (0/1/2/10–14/20/21) and the per-`<cli-version>` fixture replay. Runs on all 3 OSes,
- (.andromeda/test-plan-detail/test-strategy.md:14) Any integration witness of two concurrent first starts must use a real tempfile home and assert_cmd against the built binary, never a mocked disk.
  > Every arch Standard Contract (Wrapper channel, On-disk, Hooks, CLI, MCP) has a crate-level `tests/` suite, using a real `interprocess` endpoint, a real tempfile home, and assert_cmd against `CARGO_BIN_EXE_viola`.
- (.andromeda/test-plan-detail/test-strategy.md:17) A two-instance boot witness of the race must run against harness-booted wrappers over the fake agent, because that is the E2E layer's only rig.
  > Required for all 7 critical paths plus 4 expansion scenarios (Section 6). They run against harness-booted `viola run … -- viola-fake-agent`, using the outer PTY (portable-pty) and the rmcp client.
- (.andromeda/test-plan-detail/test-strategy.md:26) A narrower concurrent-pin control must inject its fault only through real process and file operations or existing mockall seams.
  > Faults come from sysinfo 0.39.6 / `std` process and file operations (`File::set_len` truncation of a stopped wrapper's log, the torn append as landed in place of a kill mid-write; snapshot corruption, SIGSTOP heartbeat freeze, a no-EOF child), plus mockall 0.15.0 faults on the `viola-pty` seam.
- (.andromeda/test-plan-detail/test-strategy.md:29) The two-sided witness is a remove-the-guard control that starts product processes, so it must be bounded through `Running` / `WITHIN`, never left unbounded.
  > Beside it stands the per-chunk remove-the-guard rule (`.claude/rules/testing.md`): a control whose neutralised guard lets a product process start needs a bounded witness; the G4 / G10 witnesses spawn through `tests/support/cli.rs` `Running`, bounded by `WITHIN`, since nextest's default profile has no kill line (an unbounded witness hung 27 minutes, as measured at `viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md`).
- (.andromeda/test-plan-detail/test-strategy.md:42) Concurrent `viola run` children in a root test must be started through the one guard and runner in `tests/support/cli.rs`.
  > `cli.rs` holds the one started-child guard and runner (`Running`, `Ran`, `spawn`, `viola`), with stdout and stderr as two captures beside the exit code,
- (.andromeda/test-plan-detail/test-strategy.md:42) A new race witness must not define its own copy of the reader, the wait, the guard or the result struct.
  > No root test file outside `tests/support/` defines its own copy of the reader, the wait, the guard or the result struct.
- (.andromeda/test-plan-detail/test-strategy.md:43) A witness written as a Path binary must follow the `path<N>_<topic>.rs` naming so it reports in the E2E layer.
  > a numbered Path binary `path<N>_<topic>.rs`, e.g. `path6_budget_harness.rs`). The root package carries numbered Path binaries too (`tests/path1_run_start.rs`, sync, `[[test]]` `required-features = ["fake-agent"]`); both homes report in the E2E layer, whose regex is `^(path[0-9]*|tui|mcp|cross|chaos|contract)_` (§3 5-command `run` step 2).
- (.andromeda/test-plan-detail/test-strategy.md:65) The reproducible-build measurement reads the build of the release stage, so `scripts/release-check.sh` stays the supply-chain trigger's gate.
  > Release build stage (`scripts/release-check.sh`)
- (.andromeda/test-plan.md:51) The pinned-exe hash-mismatch refusal stays testable in a temp `--home`, and the fix must keep it so.
  > Unix modes and strict-modes, torn lines, snapshot replay, heartbeat staleness and pinned-exe hash mismatch are testable in a temp `--home`.
- (.andromeda/test-plan.md:67) The pinned exe and the `plugin/` files are code-bearing state under test; the chunk touches both writes.
  > config and code-bearing state: `snapshot.json`, `config.json`, `settings.json`, `budget.json`, `ledger/stamps.json`, the `plugin/` files (`hooks.json` / `.mcp.json`), the pinned exe (SHA-256 16-hex re-hash), the `heartbeat` file
- (.andromeda/test-plan.md:135) Path 1's start sequence order (pinned copy and plugin folder before the child spawn) and its tampered-exe exit 1 must hold after the concurrency change.
  > the collision check, pinned copy and plugin folder, the fail-open ConPTY sideload, version gate, exclusive endpoint bind, first snapshot and heartbeat, and the start `wheel` and `budget-gate` events all precede the child spawn; a second `run` of the same name, a squatted endpoint, a tampered pinned exe or a refused home tree exits 1.
- (.andromeda/test-plan.md:214) The filesystem vector's pinned-exe mismatch negative stays green: a present copy is never overwritten.
  > pinned exe hash mismatch → exit 1
- (.andromeda/test-plan.md:215) The per-start `plugin/` rewrite stays a tested filesystem negative while its concurrency is read.
  > `plugin/` rewritten on start, and on Unix the instance's `settings.json` (none is written on Windows)
- (.andromeda/test-plan.md:309) The race witness must not use a retry or a sleep to line two starts up.
  > no nextest retries (`retries = 0`) and no `std::thread::sleep` / `tokio::time::sleep` used for synchronisation
- (.andromeda/test-plan.md:313) Each concurrent-start witness creates its own fresh home.
  > every test creates its own `--home` via tempfile 3.27.0
- (.andromeda/test-plan.md:382) viola-state's unit cases already hold the pin strict-modes refusals; the concurrent absent-copy arm adds to them without changing those.
  > Unix strict-modes and Windows strict-modes (the write and the read rule) over synthetic SDDL descriptors and, on `windows-2025`, real grants, the instance check, the home-tree and diagnostics checks, `live_endpoint` over a replayed snapshot, and the stamps and pin strict-modes refusals.
- (.andromeda/test-plan.md:421) A concurrent pin write is tested over real files in a tempfile home, as the concurrent-append check already is.
  > module ↔ on-disk state is tested instead over real ndjson, snapshots and `.lock` siblings in a tempfile home, with a concurrent-append check that every line of every shared diagnostics file parses as one JSON object.
- (.andromeda/test-plan.md:431) A two-start witness gets a fresh home under `target/e2e-home` and `Drop`-guarded wrappers; on Windows x64 the `seed_conpty` carve-out touches the `pin_companions` path this chunk reads.
  > a fresh tempfile home per test, at a not-yet-existing path under `target/e2e-home` that viola creates, so obs-plan's CI gates, the secret scan and the upload cover integration runs; started wrappers sit in a `Drop` guard that kills and waits, and nothing is shared between tests. Two carve-outs: Windows x64 starts seed the ConPTY companions before viola runs, into a home `seed_conpty` makes private
- (.andromeda/test-plan.md:493) Path 1's signal holds the pinned path and the tampered-exe refusal; both must stay green.
  > the snapshot holds the pinned path, the pids and the endpoint; `list --json` reads the instance live; a duplicate run, a squatted endpoint and a tampered pinned exe each exit 1;
- (.andromeda/test-plan.md:541) Path 7 is the scenario whose one-instance boot is the standing workaround; its signal must hold whichever instance count P4 decides.
  > on an unverified CLI `send` still confirms, the `question` event is logged once with an empty hook stdout, and `answer` exits 12 `unverified-cli` (MCP alike);
- (.andromeda/test-plan.md:620) The pinned-exe content hash stays pinned by literal vectors; a reproducibility finding changes no vector.
  > and the pinned-exe content hash is pinned to its library by literal vectors.
- (.andromeda/test-plan.md:634) A witness needing a viola-created home starts from `stamped_home`, and a test-made home goes through `create_private_dir`.
  > a case that needs a home viola created starts from `stamped_home`, and a test that makes a home itself makes it through `viola_state::fs::create_private_dir`, never `create_dir_all`:
- (.andromeda/test-plan.md:689) `boot` stamps through `viola verify` (which pins first), the test-side workaround the CARRY keeps standing.
  > The `boot` command creates the home, stamps it via `viola verify`, and starts the wrappers automatically.
- (.andromeda/test-plan.md:692) Two concurrent starts in one test share only the home under test; env is set per child.
  > Each test has its own home, so there is no shared endpoint, log or `budget.json`. Env is set per child with `Command::env`, never `std::env::set_var`, which is `unsafe` in edition 2024.
- (.andromeda/test-plan.md:710) The race is a filesystem behaviour, so the filesystem is not mocked in its witness.
  > Filesystem: not mocked. A real tempfile home is used everywhere, because modes, `File::lock`, torn lines and symlink handling are the behaviour under test.
- (.andromeda/test-plan.md:760) The release-build stage judges `viola` alone; the reproducibility measurement through its build must not change that verdict.
  > **Release build:** `scripts/release-check.sh`, with its probe, judges a `--release` build of `viola` alone and fails on a test-only feature or binary; no upload and no gate step.
- (.andromeda/test-plan.md:795) Any change to the release build on the measurement's basis must keep both release-check endings.
  > Release-build errors: `scripts/release-check.sh` not ending `release-check: viola only`, or its probe not ending `release-check probes: 5/5 refused, control clean`
- (.andromeda/test-plan.md:816) New code in the pin path counts toward the per-OS coverage gates.
  > Gates are enforced per OS job with `cargo llvm-cov nextest … --fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80`.
- (.andromeda/test-plan.md:854) The measured 6-of-24 boot failure is a flake-shaped red: it is fixed at its root cause in this chunk, never retried or ignored.
  > flaky tests are NOT tolerated. The nextest `retries = 0`. If a test flakes once, it gets quarantined immediately and fixed at the root cause, never with a retry-once budget. For viola, quarantine means the chunk stays red until the root cause is fixed within that chunk. `#[ignore]` is not allowed as a parking place.
- (.andromeda/test-plan.md:876) A Linux reproduction of the race or a Linux build comparison proves nothing for Windows or macOS; their legs read on their own runners.
  > NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
- (.andromeda/test-plan.md:898) The witness waits on an explicit signal (the `session-start` record), never a sleep.
  > NEVER use `sleep(N)` for synchronization. Wait for an explicit signal: an `events.ndjson` offset, `viola wait --after` or a status field.
- (.andromeda/test-plan.md:901) The concurrent `viola run` wrappers are started with std::process and a `Drop` guard.
  > NEVER use `assert_cmd` `.assert()` on the long-lived `viola run` wrapper. It blocks until exit; use std::process + a `Drop` guard.
- (.andromeda/test-plan.md:908) An unstamped or stamped home in the witness gets its stamps only from `viola verify` and its snapshot only from the wrapper.
  > NEVER hand-write `ledger/stamps.json`, `snapshot.json` or `budget.json`. Stamps come from `viola verify`, snapshots from the wrapper and budget from `viola hook statusline`. The ban binds every booted-wrapper test.
- (.andromeda/test-plan.md:918) The race's failure is a hook spawn of the pinned binary; the witness must let the fake agent run the real pinned binary.
  > NEVER stub `viola hook` in E2E. The fake agent must invoke the real pinned binary through the plugin files `run` wrote.
- (.andromeda/test-plan.md:922) The fix must not close the race by serialising tests.
  > NEVER serialise tests (`max-threads = 1`): isolation comes from the per-test home.
- (.andromeda/test-plan.md:946) The concurrent-pin control holds no test-owned timer, sleep or elapsed-time verdict.
  > NEVER use real time without injection (use deterministic clocks) in test-side code: no test-owned timer, sleep or elapsed-time verdict.

## Bindings
- **Bounded remove-the-guard witness ↔ `.claude/rules/testing.md`**: the two-sided race witness that starts product processes is bounded through `tests/support/cli.rs` `Running` / `WITHIN`, the rule file naming the per-chunk control (.andromeda/test-plan-detail/test-strategy.md:29)
- **Release build stage ↔ security-plan §Dependencies and CI (the release build carries `viola` only)**: the reproducible-build measurement reads `scripts/release-check.sh`'s build, whose two endings stay the gate (.andromeda/test-plan-detail/test-strategy.md:65), (.andromeda/test-plan.md:760), (.andromeda/test-plan.md:795)
- **Pinned-exe re-hash ↔ security-plan Home rule (re-hash before reuse, never overwritten)**: the tampered-exe exit 1 and the literal hash vectors stay green under the concurrency change (.andromeda/test-plan.md:135), (.andromeda/test-plan.md:214), (.andromeda/test-plan.md:620)
- **Path 7 one-instance boot ↔ e2e-test-strategy detail (Path 7)**: the harness workaround stands; whether Path 7 returns to two instances is P4's (.andromeda/test-plan.md:541), (.andromeda/test-plan.md:689)
- **Windows `seed_conpty` carve-out ↔ integration-test-strategy detail**: the Windows x64 seeded companions sit on `pin_companions`' path that this chunk reads (.andromeda/test-plan.md:431)
- **Zero-flakiness ↔ quality-gates-coverage-targets detail**: the race's intermittent red is fixed at its root in this chunk, no retry (.andromeda/test-plan.md:854)
