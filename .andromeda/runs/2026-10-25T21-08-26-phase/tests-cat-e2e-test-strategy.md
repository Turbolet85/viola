# tests · E2E Test Strategy — category read

## Carried
- (.andromeda/test-plan-detail/e2e-test-strategy.md:13) Any end-to-end driver of the Windows pipe (an admission or first-instance probe included) must take the endpoint only from `snapshot.json`, never a re-derived name.
  > | ipc-internal (wrapper channel) | viola-channel sync client + interprocess 2.4.4 raw stream + jsonschema 0.57.0 | direct | The endpoint is read only from `snapshot.json`. |
- (.andromeda/test-plan-detail/e2e-test-strategy.md:27) The squatted-name start refusal must stay driven as Path 1 step 4: a squatter listener bound through the interprocess server API in its own home before `viola run`, which this chunk must keep green while it hardens the listener.
  > 4. Squat: in a new home H2, create a listener on `viola_channel` endpoint name(`builder`, H2) through the interprocess server API, then run `viola run builder --home H2 -- <fake>`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:34) The squatted endpoint must still exit 1 (no exit code or refusal word changes), beside the duplicate-run and tampered-exe signals the same flow asserts.
  > - The duplicate `run` exits 1. The squatted endpoint exits 1. The tampered pinned exe exits 1.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:37) The landed Path 1 binary is the end-to-end witness of the squat refusal the scope marks "re-verify, never re-build"; it must stay green on the Windows leg with every signal asserted.
  > steps 1–6 run as one ordered flow over one stamped home in the root path binary `tests/path1_run_start.rs`, `path1_run_start_one_home_in_order(stamped_home)` (`[[test]]` `required-features = ["fake-agent"]`; in the E2E layer, `nextest-e2e`; on the `ci` profile's verify-driven 20 s kill line), every signal above asserted
- (.andromeda/test-plan-detail/e2e-test-strategy.md:38) A first-instance or squat test this chunk adds must release its squatter listener and every wrapper in its cleanup.
  > - **Cleanup:** `Drop` guard kills and waits every wrapper; TempDir drop removes H and H2; the squatter listener is dropped.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:216) Windows handle non-inheritance must stay asserted at the product unit boundary (the scope's `pipe_handles_on_both_ends_are_not_inheritable`), not by an end-to-end fd read, which is Unix's.
  > On Windows, handle non-inheritance is asserted at the product unit boundary.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:250) Every end-to-end test the chunk adds or touches must pass the sanitization check: no absolute path in a CLI `--json` or channel `error.data` from a refused start or a refused pipe open.
  > - **Error sanitization and secret scan:** after every E2E test, the `events` support helper checks:
  >   - CLI `--json`, MCP `isError` and channel `error.data` contain no absolute path (drive letter, `/home/`, `/Users/`, the temp home prefix), no serde path and no `Caused by`
- (.andromeda/test-plan-detail/e2e-test-strategy.md:261) The squatted-name exit-1 row of the exit-cause matrix must keep its own exit code, `--json` document and a cause-naming `hint:` line distinct from every other cause's.
  > Every case asserts the exit code, the `--json` document, and a last stderr `hint:` line that names that cause. A shared hint across causes fails the table, and exact strings are pinned with trycmd once the product defines them.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:270) The squat cause is a named exit-1 cause of `run`, tied to Path 1 step 4; the chunk changes neither.
  > - the endpoint name is squatted (Path 1 step 4)
- (.andromeda/test-plan-detail/e2e-test-strategy.md:288) The chunk's new Windows negatives (a remote client rejected, a second listener on a held name failing, an open outside the two ACEs denied where a runner can measure it) are security-control negatives: one negative test per control that had none.
  > - **Security control negatives** (`security_negatives_*.rs`; one negative test per security-plan control that had none):
- (.andromeda/test-plan-detail/e2e-test-strategy.md:290) The scope's CARRY 1 coordinate (`tests/security_negatives_channel.rs` named for the DACL read-back) must be re-verified against this file's own placement: the client SQOS negative belongs there and to the next entry, not to this chunk.
  > The `security_negatives_*.rs` case against the viola client lands with `viola-channel`, its first consumer.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:292) A Windows DACL test the chunk adds must create its home under `target/e2e-home`, never under `%TEMP%`.
  > A Windows DACL test creates its home where the CI homes live (`target/e2e-home`), never under `%TEMP%`, which lies inside the profile.
- (.andromeda/test-plan.md:46) Remote pipe clients and the DACL's effect against another principal are out of reach of hosted runners and must be asserted at the check boundary; item (c) runs on the runner only where it can be measured without a second user.
  > the cross-user parts (a peer of another euid, remote pipe clients, the Windows pipe DACL) cannot be driven on hosted runners and are asserted at the peer-credential check boundary.
- (.andromeda/test-plan.md:88) A second local user squatting or connecting is an untestable zone in CI; no test may claim to drive it on a hosted runner.
  > Cross-OS-user attacks (a second local user squatting or connecting to the endpoint, reading the home). Hosted runners provide one account (security excerpt, Vectors 1 and 7).
- (.andromeda/test-plan.md:135) The `run` start critical path must keep refusing a squatted endpoint with exit 1.
  > a second `run` of the same name, a squatted endpoint or a tampered pinned exe exits 1.
- (.andromeda/test-plan.md:185) The IPC security-vector trigger requires the squat negative and the no-inherited-handles assertion that this chunk brings to "fully asserted" on Windows.
  >     - squatted endpoint → `run` exit 1
  >     - Unix socket dir 0700 and file 0600 asserted
  >     - no inherited channel handles in the child
- (.andromeda/test-plan.md:250) The named-pipe and Windows DACL branches must be tested on their own runner as OS-branch tests.
  > full workspace and fake-agent E2E on all three runners. OS-branch tests cover named pipe vs Unix socket, ConPTY vs openpty, Windows DACL / strict-modes vs Unix modes, and macOS euid + dir-only server verification.
- (.andromeda/test-plan.md:376) Windows handle non-inheritance is a `viola-channel` unit-tier case.
  > the peer-credential decision with injected euids, and Windows handle non-inheritance.
- (.andromeda/test-plan.md:421) A squatter must be bound first through the server API and never through `try_overwrite`, and the pipe DACL must be read back in its canonical form, never compared with the literal it set.
  > - **Module ↔ IPC:** a real endpoint bound in a temp home with frames validated by jsonschema, the squatter created first through the server API (never `try_overwrite`), and the Unix socket modes and the Windows pipe DACL read back — the DACL compared in its canonical form, never the literal it set.
- (.andromeda/test-plan.md:476) Cross-user peers stay covered by the boundary unit test and the same-user SDDL read-back; a remote-client or other-principal case not measurable on the runner falls back to that coverage, with the live reading owed elsewhere.
  > Cross-OS-user peers. Covered by the peer-credential boundary unit test (§4) and same-user SDDL read-back (§5).
- (.andromeda/test-plan.md:488) Path 1's verification signal must still hold after the change: a squatted endpoint exits 1.
  > a duplicate run, a squatted endpoint and a tampered pinned exe each exit 1
- (.andromeda/test-plan.md:604) The exit-cause matrix must keep one negative per exit-1 cause with a cause-naming hint.
  > - **Exit-cause matrix** (`cross_exit_causes.rs`): one negative test per cause of exit 21 and of exit 1, each asserting the exit code, the `--json` document and a last `hint:` line that names its cause — a hint shared across causes fails the table; `revive`'s refusals are cases of `tests/cli_revive.rs`; driver-facing hints never contain `release`.
- (.andromeda/test-plan.md:606) The Windows endpoint negatives join the security-control negatives family.
  > - **Security control negatives** (`security_negatives_*.rs`): one negative test per security-plan control that had none
- (.andromeda/test-plan.md:767) The `windows-2025` runner is where the named pipe and the DACL / SDDL read-back must be verified.
  > `windows-2025` (ConPTY, named pipe, DACL / SDDL read-back, `.cmd` refusal)
- (.andromeda/test-plan.md:868) A pipe-mode, first-instance or DACL property a unit or integration test can hold must not be asserted at E2E.
  > - NEVER assert at E2E a case that a lower layer can hold. Parser, refusal-order, threshold and clock cases belong in §4, contract shapes in §5, and §6 asserts only cross-process behaviour (§2 pyramid).
- (.andromeda/test-plan.md:873) The chunk's Windows assertions close only on the Windows CI leg, never on the Linux dev host.
  > - NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
- (.andromeda/test-plan.md:889) A first-instance test must not use `try_overwrite` and must not reuse an endpoint name across tests.
  > - NEVER use interprocess `try_overwrite`, or reuse an endpoint name across tests. Take the endpoint only from `snapshot.json`.
- (.andromeda/test-plan.md:851) The folded CARRY's once-red budget case must be fixed at its root cause within this chunk, never retried or parked.
  > flaky tests are NOT tolerated. The nextest `retries = 0`. If a test flakes once, it gets quarantined immediately and fixed at the root cause, never with a retry-once budget. For viola, quarantine means the chunk stays red until the root cause is fixed within that chunk. `#[ignore]` is not allowed as a parking place.
- (.andromeda/test-plan.md:919) The fix for the libtest-thread log-capture red must not serialise the suite.
  > - NEVER serialise tests (`max-threads = 1`): isolation comes from the per-test home.

## Bindings
- **Path 1 step 4 squat ↔ exit-cause matrix**: the squat refusal driven by Path 1 step 4 is the same exit-1 cause the matrix lists, so one product change moves both (.andromeda/test-plan-detail/e2e-test-strategy.md:27) (.andromeda/test-plan-detail/e2e-test-strategy.md:270).
- **E2 Windows non-inheritance ↔ body §4 viola-channel**: the E2 scenario puts Windows handle non-inheritance at the unit boundary, and §4 lists it as a `viola-channel` unit case (.andromeda/test-plan-detail/e2e-test-strategy.md:216) (.andromeda/test-plan.md:376).
- **Security control negatives ↔ body §1 untestable zones**: a new Windows negative may assert only what one runner account can measure; remote clients and other principals go to the check boundary (.andromeda/test-plan-detail/e2e-test-strategy.md:288) (.andromeda/test-plan.md:46) (.andromeda/test-plan.md:88).
- **Windows DACL test home ↔ body §5 IPC read-back**: a DACL test's home sits under `target/e2e-home`, and its DACL is compared in canonical form (.andromeda/test-plan-detail/e2e-test-strategy.md:292) (.andromeda/test-plan.md:421).
- **Security control negatives ↔ security-plan §Authentication & Authorization (IPC access control, Windows)**: each control that clause names and lacks a negative gets one in `security_negatives_*.rs` (.andromeda/test-plan-detail/e2e-test-strategy.md:288).
- **Folded budget CARRY ↔ body §10 zero-flakiness and §11 CI**: the once-red case is fixed at its cause in this chunk without serialising tests (.andromeda/test-plan.md:851) (.andromeda/test-plan.md:919).
