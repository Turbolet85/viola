# tests extract

## Relevance
relevant — the chunk asserts Windows named-pipe controls (remote-client rejection, first-instance, DACL, non-inheritance) on the windows-2025 leg and carries a flake obligation (the budget send case under plain `cargo test`).

## Constraints
- test-plan §1 Test Scope Summary (Coverage scope, `viola-channel` entity; Untestable zones) classes rejecting remote pipe clients and denial through the Windows pipe DACL as cross-user parts hosted runners cannot drive (no second OS account), to be asserted at the check boundary; scope item (c) "the DACL's effect" therefore needs research to show a windows-2025 runner can measure it with one account — otherwise it stays a Windows-only live measurement, and a CI test claiming it would contradict this classification (a plan amendment, not a silent addition).
- test-plan §1 Test Scope Summary (Coverage triggers, security-vector-coverage (IPC)) requires a squatted endpoint → `run` exit 1 and no inherited channel handles in the child; the chunk's listener-level first-instance assertion (scope item (b)) sits beside, never replaces, the CLI-level squat refusal of Path 1.
- test-plan §5 Integration Test Strategy (Module ↔ IPC) requires a real endpoint bound in a temp home, the squatter created first through the interprocess server API (never `try_overwrite`), and the Windows pipe DACL read back in Windows' canonical form (`D:P(A;;FA;;;<sid>)(A;;FA;;;SY)`, the user SID possibly an alias such as `LA`), never compared against the literal it set.
- test-plan §4 Unit Test Strategy (viola-channel) requires `#[cfg(windows)]` handle non-inheritance proved by `GetHandleInformation` on both the server and client handles (`HANDLE_FLAG_INHERIT` clear); whether the code already holds this at HEAD (scope names `pipe_handles_on_both_ends_are_not_inheritable`) is research's question, to re-verify, not re-build.
- test-plan §6 E2E Test Strategy (Security control negatives) requires one negative test per security-plan control that had none, in `security_negatives_*.rs`; the remote-client rejection and the listener's first-instance property are such controls if no test asserts them today (research's question) — and scope's CARRY coordinate (`tests/security_negatives_channel.rs` for the DACL read-back) must be re-verified against what that file holds.
- test-plan §10 Quality Gates & Coverage Targets (Zero-flakiness budget) requires a test that flaked once to be fixed at its root cause within the chunk, `retries = 0`, no `#[ignore]` parking; the CARRY on `run::send::budget::tests::send_budget_paused_is_refused_with_the_window_word_and_no_cursor` case_4 (red once under plain `cargo test`, green only under nextest) is held to this, and its "log capture found no `send-refused` line" mechanism is a claim for research to measure, not a cause taken as known.
- test-plan §10 Quality Gates & Coverage Targets (Coverage thresholds) gates coverage per OS job (lines ≥ 85, functions ≥ 95, regions ≥ 80), deliberately per OS because `#[cfg(windows)]` code compiles only on its own runner — new Windows-only listener code and tests are judged on the windows-2025 job alone.

## Patterns to follow
- A test-owned same-user pipe server on a unique `\\.\pipe\viola-test-…-<pid>-<label>` name, as `tests/channel_sqos_open.rs` does for the SQOS open, with a control case proving the check fires (per test-plan §6 E2E Test Strategy, Security control negatives).
- The squatter bound first through the interprocess 2.4.4 server API as the fake server, never an endpoint reused across tests (per test-plan §8 Mocking & Stubbing Discipline, Fake servers; §11 Test Anti-Patterns, Integration).
- Windows-only cases gated `#[cfg(windows)]` and read back with windows-sys 0.61.2, run on the `windows-2025` matrix leg (named pipe, DACL / SDDL read-back) (per test-plan §5 Integration Test Strategy, Driver(s); §9 CI Integration, Matrix builds).
- Root integration binaries named `<surface>_<topic>.rs` (`channel_` / `security_negatives_`) and functions `<subject>_<condition>_<expected>`, table cases as readable rstest `#[case::label]` (per test-plan §2 Test Strategy, Test directory + naming conventions; §4 Unit Test Strategy, Conventions).
- A fresh tempfile home per test under `target/e2e-home`, wrappers in a `Drop` guard that kills and waits (per test-plan §5 Integration Test Strategy, Setup / teardown lifecycle).

## Anti-patterns to avoid
- Treating a Linux dev-host run as proof of the Windows assertions: OS-branch code is verified only on its own runner (per test-plan §11 Test Anti-Patterns, Test Strategy).
- Closing the budget-case flake by a green re-run, a retry budget or `#[ignore]` rather than its root cause (per test-plan §11 Test Anti-Patterns, Quality and CI; §10 Zero-flakiness budget).
- Using interprocess `try_overwrite`, `sleep(N)` for synchronisation, or `std::env::set_var` in the new tests (per test-plan §11 Test Anti-Patterns, Integration and E2E).

## Contract bindings
- tests ↔ security: the controls under test are security-plan §Authentication & Authorization's "IPC access control (Windows)" clause (protected SDDL, `accept_remote(false)`, first-instance, non-inheritable); test-plan §6 Security control negatives and §5 Module ↔ IPC own the assertions, security owns the control.
- tests ↔ obs: the budget CARRY's verdict reads a `send-refused` line through the crate's in-test log capture; the event is obs's (event catalog), the capture's determinism under libtest threads vs nextest process-per-test (test-plan §4 Unit Test Strategy, Framework) is tests'.
- tests ↔ CI: the Windows assertions count only as run on the `test (windows-2025)` leg's `run --coverage` and its `gate --require` (test-plan §9 CI Integration); the leg's `supply-chain` red (RUSTSEC-2026-0386) is outside this chunk per scope and is not a test-side gate to widen.

## Acceptance criteria contributions
- (tests) The chunk's new `#[cfg(windows)]` cases — listener remote-client rejection read back, a second listener on a held name failing, and any DACL-effect case research admits — run and pass on `test (windows-2025)` (not compiled out, not skipped), with the full matrix green otherwise apart from the scoped-out `supply-chain` job (per test-plan §9 CI Integration, Matrix builds; §11 Test Anti-Patterns, Test Strategy).
- (tests) The first-instance case binds its squatter first through the interprocess server API on an endpoint name unique to the test, never `try_overwrite`, and the DACL read-back compares the canonical form only (per test-plan §5 Integration Test Strategy, Module ↔ IPC).
- (tests) `send_budget_paused_is_refused_with_the_window_word_and_no_cursor` case_4 passes three consecutive plain `cargo test` runs, or its log capture is fixed at the measured root cause and then passes them; no retry, `#[ignore]` or nextest-only carve-out (per test-plan §10 Quality Gates & Coverage Targets, Zero-flakiness budget).
- (tests) The windows-2025 job's coverage gate holds at `--fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80` with the new Windows-only code (per test-plan §10 Quality Gates & Coverage Targets, Coverage thresholds).
