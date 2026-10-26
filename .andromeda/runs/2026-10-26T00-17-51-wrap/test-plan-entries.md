## 2026-10-25-windows-endpoint-admission — the Windows listener's admission negatives
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: Security control negatives, two new cases; the body's Security control negatives line) · §5 Integration Test Strategy (integration-test-strategy.md: Module ↔ IPC, Test approach) · §2 Test Strategy (test-strategy.md: the trigger map's IPC vector (V1) row)
**Change:**
- §6 gains **Windows listener remote-client rejection** (`#[cfg(windows)]`, IPC access control (Windows)): `tests/security_negatives_channel.rs` `security_negative_viola_listener_rejects_remote_clients`, on the windows-2025 CI `test` leg, binds `viola_channel::Server::bind`, opens a client end with `CreateFileW(FILE_READ_ATTRIBUTES)`, reads `NtQueryInformationFile(…, FilePipeLocalInformation)` and asserts the `FILE_PIPE_REJECT_REMOTE_CLIENTS` bit (2) set in `NamedPipeType`; its two controls, test-owned `CreateNamedPipeW` pipes read through the same client-end instrument, tell the bit apart: `security_negative_control_accept_remote_pipe_reads_the_bit_clear` (`PIPE_ACCEPT_REMOTE_CLIENTS`) clear, `security_negative_control_reject_remote_pipe_reads_the_bit_set` (`PIPE_REJECT_REMOTE_CLIENTS`) set.
- §6 gains **Windows listener DACL denial**: `security_negative_viola_listener_denies_an_anonymous_open` opens the viola listener `GENERIC_READ` under `ImpersonateAnonymousToken` and asserts `ERROR_ACCESS_DENIED` (5); its control `security_negative_control_default_pipe_admits_an_anonymous_read` makes the same open of a test-owned default-descriptor pipe, and it succeeds.
- The body's Security control negatives line names the Windows listener rejecting remote clients and denying an anonymous open, each beside its controls (was: neither named).
- §5 Module ↔ IPC: after the canonical SDDL read-back, the pipe's admission itself (remote clients rejected, an anonymous open denied) is asserted in §6 Security control negatives (was: the read-back alone).
- §2 the IPC vector (V1) row's plan location adds Security control negatives (root `security_negatives_channel.rs`: the Windows listener's remote-client rejection and DACL denial) (was: §5 Module ↔ IPC; §6 Scenario 1).
**Why:** the chunk landed the five Windows cases and every one passed on CI's `test (windows-2025)`; the controls tell the two outcomes apart, so nothing is owed to "Windows-only live measurements". §6's naming was the plan's expected amendment.
**Kept:** the Module ↔ IPC Tools line (windows-sys 0.61.2) is unchanged: the root's `cfg(windows)` dev-dependency features are architecture's record (Crate dependency direction). The cases' bind-before-impersonate order stays the test code's own, and their `viola-test-chan-<pid>-<label>` pipe names stay architecture's registered namespace.
**Ref:** .andromeda/runs/2026-10-26T00-17-51-wrap/

## 2026-10-25-windows-endpoint-admission — the control pipes and the anonymous principal as fake servers
**Section:** §8 Mocking & Stubbing Discipline (mocking-stubbing-discipline.md: the Fake servers row)
**Change:**
- The tool cell adds windows-sys 0.61.2 `CreateNamedPipeW` (Windows only: test-owned control pipes) (was: the interprocess 2.4.4 squatter listener and `viola-fake-agent` as the fake CLI alone).
- The pattern cell adds: in the listener-admission negatives the test-owned control pipes (accept-remote, reject-remote, the default descriptor) stand beside the real `viola_channel::Server::bind` listener, and `ImpersonateAnonymousToken` stands in for a foreign principal; "no HTTP fake servers" stays.
**Why:** the chunk's five Windows cases take test-owned `CreateNamedPipeW` pipes as their controls and an anonymous-token open as the foreign principal; the row named neither.
**Ref:** .andromeda/runs/2026-10-26T00-17-51-wrap/

## 2026-10-25-windows-endpoint-admission — the viola-channel untestable zone narrowed to the other-euid peer
**Section:** §1 Test Scope Summary (the body's `viola-channel` entity line; test-scope-summary.md: the `viola-channel` entity, Reason if not fully testable)
**Change:**
- Body: was "the cross-user parts (a peer of another euid, remote pipe clients, the Windows pipe DACL) cannot be driven on hosted runners"; now the Windows listener rejecting remote pipe clients and its pipe DACL denying an anonymous open are asserted on `test (windows-2025)`, and only the cross-user part (a peer of another euid) cannot be driven on hosted runners and is asserted at the peer-credential check boundary.
- Detail: was rejecting remote pipe clients and denying access through the Windows pipe DACL among the parts hosted GitHub runners cannot drive; now the Windows listener's admission is asserted on CI `test (windows-2025)` (`tests/security_negatives_channel.rs`): remote clients rejected by the pipe-type read-back beside an accept-remote and a reject-remote control, and the DACL (set from `D:P(A;;GA;;;<user-SID>)(A;;GA;;;SY)`, read back as `D:P(A;;FA;;;<sid>)(A;;FA;;;SY)`) by an anonymous-token open denied on the viola listener beside the same open admitted on a default-descriptor pipe. The cross-user part that cannot be driven (no second OS account) is rejecting a peer with a different euid, asserted at the peer-credential check boundary.
**Why:** the clause was disproved by measurement: false for remote pipe clients and the DACL on CI's `test (windows-2025)`, each witnessed beside a control telling the outcomes apart; the rest of it (a peer of another euid; a second OS account) stands.
**Kept:** the entity stays partially testable; the security excerpt's note that the Windows DACL cases are only "partially testable with stubs" stands as a quotation of the excerpt, whose stub cases are the home strict-modes ones (§1, the `viola-state` entity: a foreign-owner SID, a volume without persistent ACLs).
**Ref:** .andromeda/runs/2026-10-26T00-17-51-wrap/

## 2026-10-25-windows-endpoint-admission — nextest the only supported runner, three plain-run reds with no owner
**Section:** §2 Test Strategy (test-strategy.md: the Unit level, a new Runner item)
**Change:**
- The Unit level gains Runner: nextest is the runner; a plain `cargo test` run (`cargo test --bin viola`) is not supported, and under it the root-bin cases `run::gate::tests::feed_panic_writes_one_parse_rejected_line_until_the_size_changes`, `obs::tests::log_config_rejection_writes_one_warn_line` and `case_2_sigterm` are known-cause reds, recorded with no owner.
**Why:** the founder's own answer to a P4 fork, live by dialog in the overseer session: plain `cargo test` is unsupported and its known-cause reds are recorded with no owner, so no route entry is owed for them.
**Ref:** .andromeda/runs/2026-10-26T00-17-51-wrap/

## 2026-10-25-windows-endpoint-admission — scoped log captures through with_scoped_default
**Section:** §4 Unit Test Strategy (unit-test-strategy.md: a new Scoped log captures item and its two sub-items)
**Change:**
- Every scoped capture in the root bin's unit modules goes through `crate::test_support::with_scoped_default` (`src/main.rs`): it registers once per process a pinned, always-interested `tracing::Dispatch` over a bare `tracing_subscriber::registry()` (no layer, no filter) in a `static OnceLock`, never the global default, then runs `tracing::subscriber::with_default`; so no scoped capture is ever the only registered dispatcher, and a line whose callsite another, subscriber-less thread registered first is not lost. The binary's one global install is `set_global_default` in `src/run/gate.rs`.
- Its witness `tests::scoped_capture_sees_a_line_whose_callsite_another_thread_registered_first` (`src/main.rs`) runs on every OS under nextest: red on a bare `with_default` (the capture stays empty), green on the helper.
- The three crate-level scoped captures sharing the mechanism, in `crates/viola-state/src/lib.rs`, `crates/viola-channel/src/test_support.rs` and `crates/viola-channel/src/lib.rs`, run on a bare `with_default`; never observed red, they have no owner.
**Why:** the carried capture loss is explained: a callsite first registered on a subscriber-less thread while the scoped capture is the only registered dispatcher loses the line; the witness reproduces it and the helper removes it. The three crate captures are left out with no owner on the founder's own yes to the P5 review card's Beyond-the-plan row, live by dialog in the overseer session.
**Ref:** .andromeda/runs/2026-10-26T00-17-51-wrap/
