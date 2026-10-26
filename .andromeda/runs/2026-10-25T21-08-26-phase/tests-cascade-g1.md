# tests · cascade read · group 1 of 3

## Carried
- (.andromeda/test-plan-detail/test-scope-summary.md:11) The remote-client rejection and the DACL's effect cannot be driven on a hosted runner, so the chunk must assert them at a boundary (the pipe mode and the canonical DACL read back) and land the DACL-effect open in CI only where one account can measure it, else leave it a Windows-only live measurement.
  > **Reason if not fully testable:** same-user behaviour is testable: framing, method dispatch, refusal order, `-32602` on a higher `v`, the pid/start-time server verification, and `run` exiting 1 on a squatted name. The cross-user parts cannot be driven on hosted GitHub runners, which have no second OS account: rejecting a peer with a different euid, rejecting remote pipe clients, and denying access through the Windows pipe DACL (set from `D:P(A;;GA;;;<user-SID>)(A;;GA;;;SY)`, read back as `D:P(A;;FA;;;<sid>)(A;;FA;;;SY)`, as measured at chunk 2026-09-27-wrapper-channel). They have to be asserted at the peer-credential check boundary. The security excerpt (data classifications) notes the Windows DACL cases are only "partially testable with stubs".
  (.andromeda/test-plan.md:46)
  > - **Entity:** `viola-channel`: JSON-RPC 2.0 over ndjson on `interprocess` local sockets — partially testable: same-user framing, dispatch, refusal order, server verification and the squatted-name refusal are testable; the cross-user parts (a peer of another euid, remote pipe clients, the Windows pipe DACL) cannot be driven on hosted runners and are asserted at the peer-credential check boundary.
  (.andromeda/test-plan.md:88)
  > Cross-OS-user attacks (a second local user squatting or connecting to the endpoint, reading the home). Hosted runners provide one account (security excerpt, Vectors 1 and 7).
- (.andromeda/test-plan.md:11) Every listener assertion the chunk adds must run agent-driven to an exit code; a DACL-effect case no runner can drive is handled by the boundary read-back, never by a manual step.
  > _**Agent-driven invariant:** every test layer described in this plan must
  > be runnable end-to-end by an agent reading exit codes and structured
  > output. No human-in-the-loop checkpoints, no manual verification, no
  > visual regression with human review. If a surface cannot be agent-driven,
  > it is flagged "untestable" in test-scope.md Section 1 and handled via
  > boundary tests / mocks._
  (.andromeda/test-plan.md:939)
  > - NEVER include manual smoke step ("developer verifies before deploy")
- (.andromeda/test-plan.md:942) The remote-client rejection must be asserted by reading the listener's pipe mode back, never by a real network open of the pipe.
  > - NEVER use real network calls in tests (use mocks / localhost stubs).
- (.andromeda/test-plan-detail/test-scope-summary.md:32) The re-verified squatted-name refusal stays an exit 1 inside this exit-code set, which the chunk must leave unchanged.
  > CLI verbs and the `--json` / exit-code contract (`run · revive · send · wait · last · list · answer · pause · release · link · unlink · verify · plugin install`, global `--home`, exits 0/1/2/10–14/20/21;
  (.andromeda/test-plan.md:62)
  > - **Entity:** CLI verbs and the `--json` / exit-code contract (the verbs, global `--home`, exits 0/1/2/10–14/20/21, `revive`'s `--json` documents on both arms), plus the human TTY output (`viola list`, the readback mirror, C0/C1 escaping) — testable.
- (.andromeda/test-plan-detail/test-scope-summary.md:41) The squat refusal's re-verification (and any edit to `refuse_squatted`) must be asserted through the subprocess driver on its exit code and its `unable` line with a last-line `hint:`.
  > - **Driver:** subprocess + `assert_cmd` / `Command::cargo_bin("viola")`, with `--home <per-test temp>` and `--json`
  >   - **Signal:** exit code (0, 1, 2, 10–14, 20, 21) plus one JSON document on stdout (`{"v":1,"ok":…}` / `{"v":1,"refusal":…,"detail":…}` / `{"v":1,"error":…}`). Human mode is checked by stdout/stderr line match: `[RB] read back … cursor <n>`, `unable  <name>  <reason>  <detail>` + last-line `hint:`, `stamped <ver>  <n> pass  <n> fail`. There must be no SGR under non-TTY / `NO_COLOR` / `TERM=dumb`.
  (.andromeda/test-plan.md:94)
  > - **Surface:** cli (`viola` verbs) — driven as a subprocess by `assert_cmd` with a per-test `--home` and `--json`; its signal is the exit code plus one JSON document on stdout, human mode checked by line match with no SGR under non-TTY / `NO_COLOR` / `TERM=dumb`; the TTY rendering goes through the outer-PTY driver, never by inspecting the child agent's screen.
- (.andromeda/test-plan-detail/test-scope-summary.md:51) `tests/path1_run_start.rs`, reached whole, drives Path 1's tui surface: its squatted-endpoint leg must read exit on the process handle and never parse the child screen.
  > - **Driver:** PEXPECT-class outer-PTY driver. The test spawns `viola run` inside a PTY it owns, writes human keystrokes and bracketed pastes, and resizes. The concrete crate is Phase 2's choice.
  >   - **Signal:** `events.ndjson` lines (`wheel {holder:"human", cause:"human-input"}`, `prompt-submitted {origin:"human"}`), the fake agent's own report of what it received (one prompt, newlines inside, no Esc), and process exit code / exit detection on the process handle
  (.andromeda/test-plan-detail/test-scope-summary.md:54)
  > Assertions must never parse the rendered child screen for content (creator excerpt, Test Anti-Patterns: "Nobody parses the rendered screen for content").
  (.andromeda/test-plan.md:102)
  > - **Surface:** tui (`viola run` terminal passthrough, the human's side of the PTY) — driven by an outer-PTY driver that writes keystrokes and bracketed pastes and resizes; its signal is `events.ndjson`, the fake agent's receipt and exit detection on the process handle; assertions never parse the rendered child screen for content, and the readiness gate is tested through its refusal outputs.
- (.andromeda/test-plan-detail/test-scope-summary.md:57) The new listener tests (a second listener on a held name, the remote-rejecting mode) must drive the pipe through the `viola-channel` sync API or a raw `interprocess` client, with cross-user rejection held to the section 1 flag.
  > **Driver:** language-native IPC test client (the `viola-channel` sync client, or a raw `interprocess` `local_socket` client for malformed frames) + JSON schema assertion
  (.andromeda/test-plan-detail/test-scope-summary.md:60)
  > **Notes:** traced to arch excerpt, Wrapper channel and security excerpt, Vector 1. Cross-user peer rejection is flagged in section 1.
  (.andromeda/test-plan.md:104)
  > - **Surface:** ipc-internal (the wrapper channel, JSON-RPC 2.0 over ndjson) — driven by a language-native IPC client with JSON schema assertion; its signal is the success, refusal or protocol-error payload, cross-checked against the `events.ndjson` cursor; cross-user peer rejection is flagged in section 1.
- (.andromeda/test-plan-detail/test-scope-summary.md:105) Every listener, squat and case_4 test the plan adds or re-runs must build its own fresh home and seed no state by hand.
  > Self-bootstrapping per test: a fresh `--home` temp dir.
  (.andromeda/test-plan-detail/test-scope-summary.md:112)
  > No developer-seeded state.
  (.andromeda/test-plan.md:131)
  > - **Test data strategy:** self-bootstrapping per test in a fresh `--home`
  (.andromeda/test-plan.md:309)
  > every test creates its own `--home` via tempfile 3.27.0
- (.andromeda/test-plan.md:905) The squat tests must take their state from the wrapper and the writers, never a hand-written `snapshot.json` or `budget.json`, and the case_4 fix may plant a state file only through its writer's bytes.
  > - NEVER hand-write `ledger/stamps.json`, `snapshot.json` or `budget.json`. Stamps come from `viola verify`, snapshots from the wrapper and budget from `viola hook statusline`. The ban binds every booted-wrapper test. A reader's own unit cases may plant a malformed state file only by altering the writer's own bytes: write through the writer, then pad, replace or re-version (the operator's answer at the chunk 2026-10-10-budget-governor wrap, where `budget_read_*` landed in that form).
- (.andromeda/test-plan-detail/test-scope-summary.md:118) Path 1's squatted-endpoint exit 1 after the exclusive endpoint bind is the behaviour the chunk re-verifies against `tests/path1_run_start.rs`, and it must stand.
  > - **Path:** `run` start sequence. Collision check, pinned copy and plugin folder, the ConPTY sideload (Windows x64, fail-open: any failure leaves the child on the inbox ConPTY), version gate, exclusive endpoint bind, first snapshot and heartbeat, start `wheel` and `budget-gate` events, and only then the child spawn. A second `run` of the same name, or a squatted endpoint, exits 1.
  (.andromeda/test-plan.md:135)
  > - **Path:** `run` start sequence — the collision check, pinned copy and plugin folder, the fail-open ConPTY sideload, version gate, exclusive endpoint bind, first snapshot and heartbeat, and the start `wheel` and `budget-gate` events all precede the child spawn; a second `run` of the same name, a squatted endpoint or a tampered pinned exe exits 1.
- (.andromeda/test-plan-detail/test-scope-summary.md:152) case_4 re-asserts the `budget-paused` refusal carrying its window word, so its fix must keep that outcome while it repairs only the log capture.
  > Crossing `five_hour` ≥ 90 % or `seven_day` ≥ 85 % refuses automated `send` with `budget-paused`, and `release --budget` overrides it.
  (.andromeda/test-plan-detail/test-scope-summary.md:158)
  > `send` exits 11 with detail `five-hour` / `seven-day`.
  (.andromeda/test-plan.md:159)
  > crossing `five_hour` ≥ 90 % or `seven_day` ≥ 85 % refuses automated `send` with `budget-paused`, and `release --budget` overrides it.
- (.andromeda/test-plan-detail/unit-test-strategy.md:50) The carried case_4 is one of the `send_budget_paused_*` unit cases in `src/run/send/budget.rs`, and it must pass under plain `cargo test` (libtest threads), not only under nextest's process-per-test.
  > - Budget governor thresholds (`five_hour` 89 → open, 90 → paused; `seven_day` 84 → open, 85 → paused) and `budget_override_until` expiry. As landed (chunk 2026-10-10-budget-governor) both are unit cases of the pure `decide(reading, thresholds, override_until, now)`, which takes the wall-clock instant as an argument (no clock seam, no mock_instant): `gate_decide_*`, with the expiry in `gate_decide_an_expired_override_is_dropped` and `gate_release_override_ends_at_its_instant`. Beside them: `percent_*`, `thresholds_*`, `gate_eval_*` and `gate_release_*` (`src/run/budget.rs`, `src/cmd/run.rs`, `src/run/wheel.rs`), `send_budget_paused_*` (`src/run/send/budget.rs`) and `budget_read_*` (`crates/viola-state/src/budget.rs`).
  (.andromeda/test-plan.md:386)
  > the budget governor's pure decision and override expiry
  (.andromeda/test-plan.md:349)
  > **Framework:** cargo test (libtest, current Rust stable pinned in `rust-toolchain.toml`, ≥ 1.96; MSRV 1.96, edition 2024), executed through cargo-nextest 0.9.146 (process-per-test, JUnit).
- (.andromeda/test-plan-detail/test-strategy.md:74) The case_4 flake is a red the chunk owns: its root cause must be fixed so the case passes deterministically, never parked, retried or ignored.
  > | discipline (no screen parsing / no blanket approval / no flakes) | all | §6 Scenario 4 negative; §11 |
  (.andromeda/test-plan.md:271)
  > Waits must key on logged events and byte offsets, not sleeps.
  (.andromeda/test-plan.md:305)
  > no nextest retries (`retries = 0`) and no `std::thread::sleep` / `tokio::time::sleep` used for synchronisation
- (.andromeda/test-plan.md:922) The case_4 flake must be fixed at its cause inside this chunk, not left to pass on a re-run.
  > - NEVER ignore flaky tests; quarantine and fix immediately
  (.andromeda/test-plan.md:925)
  > - NEVER set nextest `retries` above 0.
  (.andromeda/test-plan.md:929)
  > - NEVER add retry-once policies for flaky tests (masks real failures)
  (.andromeda/test-plan.md:948)
  > - NEVER add retry-once policies or any retry budget for flaky tests (§10 Zero-flakiness budget).
- (.andromeda/test-plan.md:919) The case_4 fix must not rest on single-threading the suite: it must hold under libtest's parallel threads.
  > - NEVER serialise tests (`max-threads = 1`): isolation comes from the per-test home. The fuzz stages are single jobs, by design (§9).
- (.andromeda/test-plan.md:903) A log capture shared across libtest threads is the suspected case_4 cause, so the fix must leave no mutable state shared between parallel tests.
  > - NEVER share mutable state across parallel tests
- (.andromeda/test-plan.md:943) The case_4 log capture must wait on a signal, with no test-owned timer, sleep or elapsed-time verdict.
  > - NEVER use real time without injection (use deterministic clocks) in test-side code: no test-owned timer, sleep or elapsed-time verdict.
- (.andromeda/test-plan.md:895) The listener and squat tests must wait on explicit signals, read exit on the process handle, and hold any long-lived wrapper in a `Drop` guard.
  > - NEVER use `sleep(N)` for synchronization. Wait for an explicit signal: an `events.ndjson` offset, `viola wait --after` or a status field.
  > - NEVER parse the rendered child screen (the vt100 grid, PTY master output) for content. Verdicts come from hook events, `events.ndjson`, channel payloads and the fake-agent receipt (R7).
  > - NEVER detect child exit by reading master EOF; use `child.wait()` on the handle (ConPTY).
  > - NEVER use `assert_cmd` `.assert()` on the long-lived `viola run` wrapper. It blocks until exit; use std::process + a `Drop` guard.
- (.andromeda/test-plan-detail/test-strategy.md:10) The `win.rs` unit cases and the case_4 fix sit in the unit tier: inline `mod tests`, rstest cases with readable labels, run on every OS the code compiles for.
  > **Tier-specific depth (Comprehensive, viola):** Inline `#[cfg(test)] mod tests` in all 6 crates plus the root bin. rstest 0.27.0 `#[case]` tables cover the refusal-order matrices, the exit-code table (0/1/2/10–14/20/21) and the per-`<cli-version>` fixture replay. Runs on all 3 OSes, except the two real-cargo-mutants harness tests, compiled out on macOS (§10 Mutation gate).
  (.andromeda/test-plan.md:285)
  > - **Unit:** inline `#[cfg(test)] mod tests` in all six crates and the root bin, with rstest `#[case]` tables for the refusal-order, exit-code and fixture-replay matrices, on all three OSes except the two real-cargo-mutants harness tests, compiled out on macOS.
  (.andromeda/test-plan.md:355)
  > - Test file location: inline `#[cfg(test)] mod tests` in the same source file, in every `crates/viola-<area>` and in root `src/`.
  > - Test function naming: `<subject>_<condition>_<expected>` in snake_case.
  > - Test grouping: flat modules per source file. Table-driven cases use rstest `#[case]`, and each case name must be a readable label (for example `#[case::esc_refused]`).
- (.andromeda/test-plan-detail/test-strategy.md:13) A listener property the chunk asserts across a process or endpoint boundary (a second listener on a held name, the squat) must use a real `interprocess` endpoint and a real tempfile home.
  > **Tier-specific depth (Comprehensive, viola):** Required. Every arch Standard Contract (Wrapper channel, On-disk, Hooks, CLI, MCP) has a crate-level `tests/` suite, using a real `interprocess` endpoint, a real tempfile home, and assert_cmd against `CARGO_BIN_EXE_viola`.
  (.andromeda/test-plan.md:287)
  > - **Integration:** required — every arch Standard Contract has a crate-level `tests/` suite over a real `interprocess` endpoint, a real tempfile home and `assert_cmd`.
- (.andromeda/test-plan.md:886) The new listener tests must keep the integration bans: no mocked disk state, cleanup between tests, never `try_overwrite`, a fresh endpoint name per test taken from `snapshot.json`, no `set_var`, `LLVM_PROFILE_FILE` re-added after `env_clear()`.
  > - NEVER mock the on-disk state (the arch has no DB; this is the DB-equivalent rule). Use a real tempfile home.
  > - NEVER skip cleanup between integration tests: order-dependent tests
  >   hide bugs
  > - NEVER use interprocess `try_overwrite`, or reuse an endpoint name across tests. Take the endpoint only from `snapshot.json`.
  > - NEVER use `std::env::set_var` in tests. It is `unsafe` in edition 2024 and races across threads; use `Command::env` per child.
  > - NEVER call `.env_clear()` without re-adding `LLVM_PROFILE_FILE`, because that run silently drops out of coverage.
- (.andromeda/test-plan.md:868) The remote-rejection and first-instance properties belong at the unit or integration layer (the listener itself), never asserted only through an E2E run.
  > - NEVER assert at E2E a case that a lower layer can hold. Parser, refusal-order, threshold and clock cases belong in §4, contract shapes in §5, and §6 asserts only cross-process behaviour (§2 pyramid).
- (.andromeda/test-plan-detail/test-strategy.md:16) `tests/path1_run_start.rs`, reached whole, is an E2E Path binary run against a fake-agent wrapper, and its squatted-endpoint leg stays there.
  > **Tier-specific depth (Comprehensive, viola):** Required for all 7 critical paths plus 4 expansion scenarios (Section 6). They run against harness-booted `viola run … -- viola-fake-agent`, using the outer PTY (portable-pty) and the rmcp client.
  (.andromeda/test-plan.md:289)
  > - **E2E:** required for all seven critical paths and the expansion scenarios, against harness-booted wrappers over the fake agent, through the outer PTY and the rmcp client.
- (.andromeda/test-plan-detail/test-strategy.md:39) The new tests must sit where the plan homes them (inline unit tests, `crates/viola-channel/tests/<topic>.rs`, root `tests/` sharing `tests/support/`), named `<subject>_<condition>_<expected>`, and `tests/path1_run_start.rs` keeps its `fake-agent` feature gate.
  > **Unit tests:** inline `#[cfg(test)] mod tests` at the bottom of each source file.
  >   - **Crate-level integration tests:** `crates/viola-<area>/tests/<topic>.rs`.
  >   - **Sync (tokio-free) E2E suites:** root `tests/{cli,hook,tui,channel,chaos,contract}_<topic>.rs`, sharing helpers in `tests/support/{home.rs,outer_pty.rs,fake.rs,events.rs,cli.rs,ndjson.rs}`.
  (.andromeda/test-plan-detail/test-strategy.md:41)
  > No root test file outside `tests/support/` defines its own copy of the reader, the wait, the guard or the result struct.
  (.andromeda/test-plan-detail/test-strategy.md:42)
  > The root package carries numbered Path binaries too (`tests/path1_run_start.rs`, sync, `[[test]]` `required-features = ["fake-agent"]`); both homes report in the E2E layer, whose regex is `^(path[0-9]*|tui|mcp|cross|chaos|contract)_` (§3 5-command `run` step 2).
  (.andromeda/test-plan.md:315)
  > unit tests are inline, crate integration tests sit in each crate's `tests/`, and the sync E2E suites in root `tests/` share one set of helpers in `tests/support/`, which no root test file copies.
  (.andromeda/test-plan.md:323)
  > General pattern: snake_case `<subject>_<condition>_<expected>`, for example `send_with_esc_refuses_control_character` or `hook_oversize_stdin_exits_zero_silent`.
- (.andromeda/test-plan-detail/test-strategy.md:57) The squatted-endpoint and no-inherited-handle triggers the chunk re-asserts are the IPC vector's, held at integration and E2E.
  > | IPC vector (V1) | integration + E2E | §5 Module ↔ IPC; §6 Scenario 1 |
  (.andromeda/test-plan.md:185)
  > - squatted endpoint → `run` exit 1
  (.andromeda/test-plan.md:187)
  > - no inherited channel handles in the child
- (.andromeda/test-plan-detail/unit-test-strategy.md:13) The non-inheritable handles on both ends are re-verified, never rebuilt, through this `GetHandleInformation` unit case in `win.rs`; and `endpoint.rs`, reached whole, must keep its golden vectors, the Windows `\\.\pipe\` form among them.
  > `#[cfg(windows)]` handle non-inheritance, which backs E2 and the §12 open question. After the `viola-channel` server creates a pipe instance and a client connects, `GetHandleInformation` (windows-sys) on both handles (via `AsRawHandle`) reports `HANDLE_FLAG_INHERIT` clear. This is the only Windows evidence; there is no child-side probe.
  (.andromeda/test-plan-detail/unit-test-strategy.md:11)
  > FNV-1a endpoint golden vectors: `viola-<12 hex>` of `ViolaName + "\0" + abs home`, with the Windows `\\.\pipe\` form
  (.andromeda/test-plan.md:376)
  > - **viola-channel:** the ndjson frame codec at the `MAX_FRAME` boundary, the JSON-RPC error mapping, the FNV-1a endpoint golden vectors, the peer-credential decision with injected euids, and Windows handle non-inheritance.
- (.andromeda/test-plan.md:881) The DACL read-back, the squat exit code and the refusal word must be checked against literals written in the test, the exit code as an integer, never the product's own constant or a `Debug` string.
  > - NEVER import the product's own `IDENTITY_FLOOR`, ledger rows or refusal ordering as the test oracle. The expected values (the 11 identity-floor names, the refusal order, exit codes) are written as literals in the test.
  > - NEVER compare `RefusalReason` or exit codes by `Debug` string. Assert on the serde kebab-case value and the integer exit code.
- (.andromeda/test-plan-detail/test-strategy.md:69) The chunk's `#[cfg(windows)]` listener code and tests are proven only by the windows-2025 matrix leg.
  > | multi-os-compat | CI matrix | §9 Matrix builds |
  (.andromeda/test-plan.md:250)
  > OS-branch tests cover named pipe vs Unix socket, ConPTY vs openpty, Windows DACL / strict-modes vs Unix modes, and macOS euid + dir-only server verification.
- (.andromeda/test-plan.md:873) A Linux dev-host run proves none of the chunk's Windows assertions; each is verified only on its own runner.
  > - NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
- (.andromeda/test-plan-detail/test-strategy.md:64) The chunk's CI runs read `supply-chain` red on RUSTSEC-2026-0386, owned by the next route entry, and the chunk must not widen the `cargo deny` ignore list to reach green.
  > | Supply chain (V9) | CI gates | §9 Lint stage (sync-crate `cargo check`, `scripts/orphans-check.sh`) + Supply-chain stage (cargo deny incl. the `fuzz/Cargo.lock` audit, sole-root tokio ban, ban probes, zizmor) + Release build stage (`scripts/release-check.sh`) + `nightly.yml` weekly advisories (root and fuzz lockfiles) |
  (.andromeda/test-plan.md:226)
  > **Required test type:** CI gates on exit code: `cargo deny check` (only ignore RUSTSEC-2017-0008),
  (.andromeda/test-plan.md:931)
  > - NEVER skip quality gates "just this once"

## Bindings
- **test-scope-summary (viola-channel testability) ↔ the body's agent-driven invariant and §11**: the cross-user parts the scope names (remote clients, the DACL's effect) are asserted at a boundary (the pipe mode and canonical DACL read back), never by a real network open or a manual step, and the DACL-effect open lands in CI only where one runner account can drive it (.andromeda/test-plan-detail/test-scope-summary.md:11), (.andromeda/test-plan.md:11), (.andromeda/test-plan.md:942)
- **test-scope-summary (Path 1, cli surface, exit contract) ↔ test-strategy (V1 row)**: the squatted-endpoint exit 1 is one fact held at three places, the Path 1 leg, the cli driver's exit-and-`hint:` signal and the IPC trigger, and the chunk re-verifies it without changing any of them (.andromeda/test-plan-detail/test-scope-summary.md:118), (.andromeda/test-plan-detail/test-scope-summary.md:41), (.andromeda/test-plan.md:185)
- **unit-test-strategy (handle non-inheritance) ↔ test-strategy (multi-os-compat, unit tier)**: the `#[cfg(windows)]` unit case is inline in `win.rs` and counts only on the windows-2025 leg, never a Linux run (.andromeda/test-plan-detail/unit-test-strategy.md:13), (.andromeda/test-plan-detail/test-strategy.md:69), (.andromeda/test-plan.md:873)
- **unit-test-strategy (root bin `send_budget_paused_*`) ↔ test-strategy (no-flakes discipline) ↔ §11 CI and Universal**: case_4 must pass under plain `cargo test`'s threads as under nextest's process-per-test, by a fix to its capture that shares no state, waits on a signal and neither serialises nor retries (.andromeda/test-plan-detail/unit-test-strategy.md:50), (.andromeda/test-plan-detail/test-strategy.md:74), (.andromeda/test-plan.md:349), (.andromeda/test-plan.md:919)
- **test-strategy (integration tier) ↔ test-scope-summary (ipc-internal driver, test data)**: the second-listener and squat tests use a real `interprocess` endpoint through the server API, a fresh per-test home, and never `try_overwrite` or a reused name (.andromeda/test-plan-detail/test-strategy.md:13), (.andromeda/test-plan-detail/test-scope-summary.md:57), (.andromeda/test-plan.md:886)
- **test-strategy (supply chain V9) ↔ ci-integration**: the red `supply-chain` job is read, not silenced, while its owning route entry is pending (.andromeda/test-plan-detail/test-strategy.md:64), (.andromeda/test-plan.md:226)
