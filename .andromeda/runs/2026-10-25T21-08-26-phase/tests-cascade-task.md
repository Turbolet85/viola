# Task — tests · 2026-10-25-windows-endpoint-admission

## The chunk
Asserts the Windows named-pipe listener's admission on the windows-2025 CI leg: a remote (network) pipe client rejected, with a test that reads the pipe's mode back; the first-pipe-instance behaviour as a property of the listener (a second listener on a held name fails); and, only where a hosted Windows runner can measure it without a second user, an open by a principal outside the two ACEs denied.
Re-verifies, never rebuilds, what chunk 2026-09-27-wrapper-channel landed: the protected owner-and-SYSTEM DACL read back as two allow ACEs, the non-inheritable handles on both ends, and the squatted-name exit-1 start refusal; it also re-verifies the test-file coordinate the CARRY names for the DACL read-back.
Takes up a carried flake: the root unit case `send_budget_paused_is_refused_with_the_window_word_and_no_cursor` case_4 went red once under plain `cargo test` (libtest threads, no `send-refused` line found by its log capture); it runs three times under plain `cargo test`, or its log capture is fixed. The capture cause is re-verified, not taken as measured.
No wire contract, exit code or refusal word changes (`squatted-name`, exit 1 stand); no boundary is widened.

## Reached files
- crates/viola-channel/src/server.rs · edits the remote-client rejection (today a doc comment naming `PIPE_REJECT_REMOTE_CLIENTS`)
- crates/viola-channel/src/server/win.rs · edits its unit tests `owner_only_sddl_is_protected_user_and_system`, `bound_pipe_dacl_reads_back_protected_user_and_system`, `pipe_handles_on_both_ends_are_not_inheritable`
- crates/viola-channel/src/endpoint.rs
- tests/security_negatives_channel.rs
- src/cmd/run.rs · edits `refuse_squatted`
- tests/path1_run_start.rs
- tests/cli_instance_state.rs
- src/run/send/budget.rs · edits the test `send_budget_paused_is_refused_with_the_window_word_and_no_cursor` and its log capture helper

## Needs
- The `viola-channel` entity's testability split: same-user framing and the squatted-name refusal testable; remote pipe clients and the Windows pipe DACL's effect flagged as not drivable on hosted runners, asserted at a boundary instead.
- The untestable zone of cross-OS-user attacks (hosted runners give one account), which decides whether the DACL-effect assertion lands in CI or stays a Windows-only live measurement.
- §6's skipped list: cross-OS-user peers covered by the boundary unit test and the same-user SDDL read-back.
- The IPC security-vector trigger rows this chunk asserts or re-asserts: squatted endpoint gives `run` exit 1, and no inherited channel handles in the child.
- The `viola-channel` unit coverage list: Windows handle non-inheritance, the peer-credential decision with injected identities.
- The Module ↔ IPC integration boundary: the squatter bound first through the server API, never `try_overwrite`, and the Windows pipe DACL compared in its canonical form, never the literal it set.
- The integration driver for Windows DACL read-back (windows-sys 0.61.2), and for a pipe mode read-back if one is added.
- The fake-server rule: the interprocess server API binds the squatter before `run`; no other fake.
- Integration anti-patterns the new listener tests must keep: never `try_overwrite`, never reuse an endpoint name across tests, take the endpoint only from `snapshot.json`, no `std::env::set_var`, re-add `LLVM_PROFILE_FILE` after any `env_clear()`.
- Integration lifecycle: a fresh tempfile home under `target/e2e-home`, wrappers in a `Drop` guard; a test-started `viola` drops `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN`.
- Path 1 scenario's squatted-endpoint leg (exit 1), re-verified against `tests/path1_run_start.rs`.
- The exit-cause matrix's exit-1 squatted-name case: its `--json` document and its own `hint:` line stand unchanged.
- The security control negatives suite (`security_negatives_*.rs`): one negative per control that had none, where the remote-client and first-instance negatives would sit.
- The multi-os-compat trigger and the windows-2025 matrix row (named pipe, DACL / SDDL read-back): OS-branch code is verified only on its own runner, never by a Linux-only local run.
- Unit conventions: inline `#[cfg(test)] mod tests`, `<subject>_<condition>_<expected>` names, rstest `#[case]` with readable labels.
- The per-OS coverage gate (lines 85, functions 95, regions 80): new `#[cfg(windows)]` code counts only on the windows-2025 job.
- The zero-flakiness budget: `retries = 0`, no `#[ignore]` as a parking place, the chunk stays red until the case_4 flake's root cause is fixed within it.
- The ban on serialising tests (`max-threads = 1`): the case_4 fix cannot rest on single-threading.
- The ban on `sleep` for synchronisation and on test-owned timers: the case_4 log capture waits on a signal, not a delay.
- The root-bin unit list (the budget governor's pure decision, the `send` refusal ordering), where case_4 lives.
- The supply-chain build failure condition: this chunk's CI runs read `supply-chain` red on the tokio advisory, owned by a later route entry; no ignore is added to reach green.
- The readiness stall report `wait_ready` writes (`tests/support/home.rs`), the witness of the macOS readiness WATCH this chunk carries.

## Not needed
- Unix socket directory 0700 / socket 0600 and `peer_creds` euid assertions: the "Unix endpoint and home hardening" entry.
- Server verification before any frame: the pid / start-time mismatch exits, the Windows client SQOS identification-level negative, the dated exceptions: the next entry.
- E5 snapshot corruption and replay recovery: owed to "Server verification before any frame".
- Windows strict-modes over synthetic SDDL descriptors for the home (foreign-owner SID, non-persistent-ACL volume) and the `--home` outside-profile DACL negative: home integrity, not the pipe.
- Paste-injection negatives, `validate_paste_text` and the property / fuzz suite: no parser changes.
- The hook fail-open matrix and the hook perf hyperfine gates: no hook path changes.
- The MCP surface (handler re-validation, `tools/list`, the recorded MCP exchange).
- Path 6 budget governor E2E: the carried case is a root unit test only.
- Paths 2 to 5 and 7, E1 to E3, and the cross-surface parity suite.
- Recorded fixtures, fixture hygiene, the fake agent's modes and the contract suite: no fixture or fake-agent change.
- The `list` board and row-integrity suite.
- The ConPTY sideload, its vendor verification and the `.cmd` child refusal.
- The mutation gate: run only at the epoch boundary or on demand, never per chunk.
- The harness 5-command contract, session record and release-check gates.
