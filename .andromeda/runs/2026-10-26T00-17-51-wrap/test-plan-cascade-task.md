# Task — test-plan · 2026-10-25-windows-endpoint-admission

## The session
The chunk added five Windows-only integration cases to `tests/security_negatives_channel.rs`. They assert on CI `test (windows-2025)` (ci#40247500584, JUnit `passed`) that the viola pipe listener rejects remote clients and that its DACL denies an anonymous-token open, and each assertion has a control that tells the two outcomes apart.
It added the test helper `test_support::with_scoped_default` in `src/main.rs` and its witness test, and moved the root bin's nine scoped log captures off a bare `tracing::subscriber::with_default` onto that helper. This removes, at its root cause, the folded case_4 CARRY: a lost `send-refused` capture line.
It added a Windows-only root dev-dependency table, `windows-sys` with the features `Wdk_Storage_FileSystem` and `Win32_System_IO`. `Cargo.lock` is unchanged, the release graph is unchanged and no ignore was added.
No product file changed: no product behaviour, wire contract, exit code, refusal word, harness command, script or CI step moved, and nothing was retired.
It measured false the test-plan claim that remote pipe clients and the Windows pipe DACL cannot be driven on hosted runners. The rest of that claim stands: a peer of another euid and a second OS account still cannot be driven there.

## Reached files
- tests/security_negatives_channel.rs
- Cargo.toml
- src/main.rs
- src/run/send/test_support.rs
- src/run/dialog.rs
- src/run/budget.rs
- src/run/gate.rs
- src/obs.rs
- src/cmd/run.rs
- src/cmd/hook/statusline.rs

## Touches
- §1 Test Scope Summary, the `viola-channel` entity (`.andromeda/test-plan.md:46`, test-scope-summary scope-entities, its `:11` entity). The clause saying that remote pipe clients and the Windows pipe DACL cannot be driven on hosted runners is now stale for those two: both are asserted on `test (windows-2025)`. Only a peer of another euid and a second OS account stay at the peer-credential boundary.
- §6 "Skipped as untestable", the cross-OS-user peers item (e2e values). Today it is covered by the peer-credential unit test and the same-user SDDL read-back. The DACL's effect and the remote-client rejection are now driven directly on the Windows leg.
- §6 Security control negatives suite (`security_negatives_*.rs`, suite-security-negatives). Its list of covered controls now also includes the Windows listener's remote-client rejection and its anonymous-open denial, together with their three controls.
- §1 coverage trigger security-vector-coverage (IPC), the required negative list (triggers). Two Windows listener negatives now stand beside the squatted-endpoint and handle-inheritance items.
- §1 multi-os-compat trigger, the Windows DACL OS-branch tests. Before, only the DACL read-back was asserted on the Windows runner. Now the DACL's effect is asserted as well.
- §9 Matrix builds, the `windows-2025` row (ConPTY, named pipe, DACL / SDDL read-back, `.cmd` refusal) (ci). That runner now also carries the remote-client rejection and the anonymous-open denial.
- §5 Module ↔ IPC (integration). Before, the Windows pipe DACL was only read back in its canonical form. Now its effect is also asserted (an anonymous open is denied), and the pipe's remote-client bit is read back through `NtQueryInformationFile` (`FilePipeLocalInformation`).
- §5 Driver "Windows DACL read-back: windows-sys 0.61.2" (integration). The root package now declares `windows-sys` itself, as a `cfg(windows)` dev-dependency with two features.
- §4 Unit Test Strategy, root bin (unit). Every scoped log capture in the root bin's unit modules now goes through `crate::test_support::with_scoped_default`. That covers `hook/statusline.rs`, `cmd/run.rs`, `run/gate.rs`, `run/budget.rs`, `run/send/test_support.rs`, `run/dialog.rs` and `obs.rs`. The census `grep -rn 'subscriber::with_default' src` went from 9 to 1, the helper's own call. The single `set_global_default` in `src/run/gate.rs` is unchanged.
- §10 Zero-flakiness budget and the §11 Quality bans (gate-coverage). The folded case_4 CARRY, a capture that lost its `send-refused` line, is now explained and fixed at its root cause inside the chunk, with no retry and no `#[ignore]`.
- §4 Framework and the §2 agent-runnable invariants (strategy). Plain `cargo test --bin viola` is not a supported runner, and its known-cause reds are recorded with no owner. nextest stays the runner.
- §1 dependency and boundary policy entity and the supply-chain trigger. A new root Windows dev-dependency table landed. `cargo deny check bans licenses sources` and `scripts/channel-tokio-free.sh` are green, `Cargo.lock` is unchanged, and a dev-dependency feature reaches no release build under `resolver = "3"`.
- §11 Test Strategy ban "NEVER treat a Linux-only local run as proof for Windows". The new Windows cases are proven on their own runner, ci#40247500584.

## New facts
- F1 · e2e-test-strategy · `security_negative_viola_listener_rejects_remote_clients` (Windows only, `tests/security_negatives_channel.rs`) binds `viola_channel::Server::bind`. It opens a client end with `CreateFileW(FILE_READ_ATTRIBUTES)` and asserts that the `FILE_PIPE_REJECT_REMOTE_CLIENTS` bit (2) is set in `NamedPipeType`.
- F2 · e2e-test-strategy · The two controls for the remote-client case run on test-owned `CreateNamedPipeW` pipes. `security_negative_control_accept_remote_pipe_reads_the_bit_clear` reads the bit clear, and `security_negative_control_reject_remote_pipe_reads_the_bit_set` reads it set.
- F3 · e2e-test-strategy · `security_negative_viola_listener_denies_an_anonymous_open` opens the viola listener `GENERIC_READ` under `ImpersonateAnonymousToken` and asserts `ERROR_ACCESS_DENIED` (5). Its control, `security_negative_control_default_pipe_admits_an_anonymous_read`, makes the same open on a pipe with the default descriptor, and that open succeeds.
- F4 · e2e-test-strategy · The anonymous-open cases bind their pipe before impersonating, on one spawned thread, and revert (`RevertToSelf`) within the same call. The default-descriptor control pipe also carries `PIPE_REJECT_REMOTE_CLIENTS`, so the descriptor is its only difference from the viola listener.
- F5 · unit-test-strategy · The root bin's scoped log captures go through `test_support::with_scoped_default` in `src/main.rs`. It registers, once per process, a pinned always-interested `Dispatch` over a bare registry, never the global default. That way no scoped capture is ever the only dispatcher, and a line whose callsite another thread registered first is not lost.
- F6 · unit-test-strategy · The witness `tests::scoped_capture_sees_a_line_whose_callsite_another_thread_registered_first` in `src/main.rs` runs on every OS under nextest. It is red on a bare `with_default` (the capture stays empty) and green on the helper.
- F7 · unit-test-strategy · Three crate-level scoped captures share the same mechanism: the ones in `crates/viola-state/src/lib.rs`, `crates/viola-channel/src/test_support.rs` and `crates/viola-channel/src/lib.rs`. They have never been observed red and stay on a bare `with_default`, with no owner.
- F8 · integration-test-strategy · The root package carries `[target.'cfg(windows)'.dev-dependencies] windows-sys` (the workspace pin `=0.61.2`) with features `Wdk_Storage_FileSystem` and `Win32_System_IO`. Both are needed because windows-sys gates `NtQueryInformationFile` on `Win32_System_IO` too, through its `IO_STATUS_BLOCK` argument.
- F9 · test-strategy · Plain `cargo test` is not a supported runner, by the founder's fork. Under it, `run::gate::tests::feed_panic_writes_one_parse_rejected_line_until_the_size_changes`, `obs::tests::log_config_rejection_writes_one_warn_line` and `case_2_sigterm` are known-cause reds, recorded with no owner.
- F10 · e2e-test-strategy · The new cases' test pipes stay in the `viola-test-chan-<pid>-<label>` namespace, one label per case: `remote`, `accept-remote`, `reject-remote`, `anon` and `anon-default`.

## Untouched
- §1 Untestable zones, items: the real `claude` CLI, cross-OS-user attacks, volumes without persistent ACLs and foreign-owner SIDs, and macOS local runs. The label was re-read and no item moved.
- The `viola-state` entity and the config and code-bearing state entity, which are only partially testable because of foreign-owner SIDs and FAT/exFAT volumes. These Windows strict-modes stubs are unchanged.
- §3 Test Harness Contract: the five commands, exit codes, session record and usage details. No harness, script or xtask was edited.
- §9 pipeline structure, the per-OS test job, its artifacts and the secret scan. The new cases run inside the existing `test (windows-2025)` coverage run and land in `junit-windows-2025`, and no step was edited.
- §10 coverage thresholds, the ignore regex, the perf budgets and the mutation gate. There was no mutation run, no perf row and no threshold change.
- §7 fixtures, the fake agent and fixture hygiene. No fixture, script or recorded exchange was added.
- §6 Paths 1–7, E1–E5 and the exit-cause matrix. No product behaviour moved. The Path 1 squatted-endpoint case and the DACL read-back cases were re-asserted green without being edited.
- The supply-chain ban list: the only ignore is still RUSTSEC-2017-0008. The `supply-chain` red (tokio RUSTSEC-2026-0386) belongs to the route entry "tokio RUSTSEC-2026-0386 lock bump" and was not caused by this chunk.
- §8 mocking discipline. No double or seam was added, and the synthetic SDDL descriptor stubs are unchanged.
- Test-case counts. test-plan states none, and the JUnit total of 2988 is a CI reading, not a plan figure.
- §5 readiness stall report and the macOS readiness watch. The watch got one green run and did not recur, and nothing in the plan moved.
