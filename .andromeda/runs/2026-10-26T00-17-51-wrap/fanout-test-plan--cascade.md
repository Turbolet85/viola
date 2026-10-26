proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 11
    section: '## 1. Test Scope Summary'
    change: '  - **Reason if not fully testable:** same-user behaviour is testable: framing, method dispatch, refusal order, `-32602` on a higher `v`, the pid/start-time server verification, and `run` exiting 1 on a squatted name. The Windows listener admission is asserted on CI `test (windows-2025)` since chunk 2026-10-25-windows-endpoint-admission (`tests/security_negatives_channel.rs`): rejecting remote pipe clients, by the pipe-type read-back beside an accept-remote and a reject-remote control, and denying access through the Windows pipe DACL (set from `D:P(A;;GA;;;<user-SID>)(A;;GA;;;SY)`, read back as `D:P(A;;FA;;;<sid>)(A;;FA;;;SY)`, as measured at chunk 2026-09-27-wrapper-channel), by an anonymous-token open denied on the viola listener beside the same open admitted on a default-descriptor pipe. The cross-user part cannot be driven on hosted GitHub runners, which have no second OS account: rejecting a peer with a different euid. It has to be asserted at the peer-credential check boundary. The security excerpt (data classifications) notes the Windows DACL cases are only "partially testable with stubs".'
    sidecar: 'test-scope-summary viola-channel Reason: remote pipe clients and the pipe DACL moved from cannot-be-driven to asserted on test (windows-2025); only the other-euid peer stays at the peer-credential boundary (chunk 2026-10-25-windows-endpoint-admission).'
    rationale: 'Report, Spec claims disproved, first bullet: the clause is measured false for remote pipe clients and the DACL on ci#40247500584 (JUnit passed), each witnessed with a control telling the outcomes apart; the rest of the clause (a peer with a different euid; a second OS account) stands.'
    basis: .andromeda/test-plan-detail/test-scope-summary.md:11
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 46
    section: '## 1. Test Scope Summary'
    change: '- **Entity:** `viola-channel`: JSON-RPC 2.0 over ndjson on `interprocess` local sockets — partially testable: same-user framing, dispatch, refusal order, server verification and the squatted-name refusal are testable, and the Windows listener rejecting remote pipe clients and its pipe DACL denying an anonymous open are asserted on `test (windows-2025)`; the cross-user part (a peer of another euid) cannot be driven on hosted runners and is asserted at the peer-credential check boundary.'
    sidecar: 'test-plan §1 viola-channel broad line: remote pipe clients and the Windows pipe DACL now asserted on the Windows leg; only the other-euid peer left at the boundary.'
    rationale: 'Report, Spec claims disproved, first bullet, and Counts / qualifiers moved (remote-rej standing row at .andromeda/test-plan.md:46, stale for remote pipe clients and the DACL); the body line is the broad line test-scope-summary#1.1 details.'
    basis: .andromeda/test-plan.md:46
    dependent-of: D-cascade
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 62
    section: '## 4. Unit Test Strategy'
    change: '  - Scoped log captures (chunk 2026-10-25-windows-endpoint-admission): every scoped capture in the root bin unit modules goes through `crate::test_support::with_scoped_default` (`src/main.rs`), which registers once per process a pinned always-interested `tracing::Dispatch` over a bare `tracing_subscriber::registry()` (no layer, no filter) in a `static OnceLock`, never the global default, then runs `tracing::subscriber::with_default`; so no scoped capture is ever the only dispatcher, and a line whose callsite another thread registered first is not lost. The binary one global install, `set_global_default` in `src/run/gate.rs`, is unchanged.'
    sidecar: 'unit-test-strategy root bin: the scoped-capture helper test_support::with_scoped_default added as a unit-strategy fact.'
    rationale: 'Report, Symbols / APIs, second and third bullets: the helper registers a pinned Dispatch over a bare registry, never the global default, and the nine root-bin captures now call it (census 9 to 1, the helper own); pattern scoped-cap read 0 master sites, a new fact.'
    basis: src/main.rs:281
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 62
    section: '## 4. Unit Test Strategy'
    change: '    - Its witness `tests::scoped_capture_sees_a_line_whose_callsite_another_thread_registered_first` (`src/main.rs`) runs on every OS under nextest: red on a bare `with_default` (the capture stays empty), green on the helper.'
    sidecar: 'unit-test-strategy root bin: the scoped-capture witness named under the helper.'
    rationale: 'Report, Symbols / APIs (the witness at src/main.rs 639-651) and Outcome: red left [] on a bare with_default, green 1 test run: 1 passed on the helper (evidence/capture-witness.md); Coverage of new surfaces names it tests unit, every OS.'
    basis: src/main.rs:640
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 62
    section: '## 4. Unit Test Strategy'
    change: '    - Not moved: the three crate-level scoped captures sharing the mechanism, in `crates/viola-state/src/lib.rs`, `crates/viola-channel/src/test_support.rs` and `crates/viola-channel/src/lib.rs`, stay on a bare `with_default`; never observed red, they have no owner.'
    sidecar: 'unit-test-strategy root bin: the three crate-level bare captures recorded as left out with no owner.'
    rationale: 'Report, inputs I6 Beyond-the-plan row Same-class captures left out, no owner (the founder yes on the P5 card): never observed red, no route entry proposed, recorded in the plan and the report.'
    fact: F7
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 19
    section: '## 5. Integration Test Strategy'
    change: '  - **Tools:** viola-channel client, interprocess 2.4.4, jsonschema 0.57.0, windows-sys 0.61.2 (since chunk 2026-10-25-windows-endpoint-admission the root package declares it itself as a `cfg(windows)` dev-dependency with the features `Wdk_Storage_FileSystem` and `Win32_System_IO`, both needed because windows-sys gates `NtQueryInformationFile` on `Win32_System_IO` too, through its `IO_STATUS_BLOCK` argument)'
    sidecar: 'integration-test-strategy Module to IPC Tools: windows-sys now a root cfg(windows) dev-dependency with two features.'
    rationale: 'Report, Dependencies: a new root dev-dependency table windows-sys with Wdk_Storage_FileSystem and Win32_System_IO, Cargo.lock unchanged; Spec claims disproved, second bullet: NtQueryInformationFile is gated on Win32_System_IO too.'
    basis: Cargo.toml:168
    fact: F8
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 18
    section: '## 5. Integration Test Strategy'
    change: 'append to the Test approach: Beyond the read-back, the pipe admission itself is asserted on Windows (chunk 2026-10-25-windows-endpoint-admission, the root `tests/security_negatives_channel.rs`, §6 Security control negatives): the remote-client bit read back through `NtQueryInformationFile` (`FilePipeLocalInformation`), and the DACL effect by an anonymous-token open denied on the viola listener.'
    sidecar: 'integration-test-strategy Module to IPC: the DACL effect and the remote-client bit now asserted beside the canonical read-back.'
    rationale: 'Report, Symbols / APIs, first bullet, and Outcome: both Windows listener negatives passed on test (windows-2025) (ci#40247500584); the Test approach named only the canonical read-back of the pipe DACL.'
    basis: tests/security_negatives_channel.rs:218
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 294
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '  - **Windows listener remote-client rejection** (`#[cfg(windows)]`, IPC access control (Windows), `tests/security_negatives_channel.rs`, CI `test (windows-2025)`, chunk 2026-10-25-windows-endpoint-admission). `security_negative_viola_listener_rejects_remote_clients` binds `viola_channel::Server::bind`, opens a client end with `CreateFileW(FILE_READ_ATTRIBUTES)`, reads `NtQueryInformationFile(…, FilePipeLocalInformation)` and asserts the `FILE_PIPE_REJECT_REMOTE_CLIENTS` bit (2) set in `NamedPipeType`.'
    sidecar: 'e2e security negatives: the Windows listener remote-client rejection case added.'
    rationale: 'Report, Symbols / APIs, first bullet, and Outcome (security) remote-client rejection met on ci#40247500584; pattern neg-file read 0 test-plan sites, so the naming is a new fact.'
    basis: tests/security_negatives_channel.rs:218
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 294
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '    - (under the remote-client rejection bullet) Its two controls run on test-owned `CreateNamedPipeW` pipes and tell the bit apart: `security_negative_control_accept_remote_pipe_reads_the_bit_clear` (`PIPE_ACCEPT_REMOTE_CLIENTS`) reads it clear, `security_negative_control_reject_remote_pipe_reads_the_bit_set` (`PIPE_REJECT_REMOTE_CLIENTS`) reads it set.'
    sidecar: 'e2e security negatives: the remote-client case controls (accept-remote clear, reject-remote set) added.'
    rationale: 'Report, Symbols / APIs, first bullet (the controls at 229-235 and 237-243), and Outcome: both controls passed, so nothing is owed to Windows-only live measurements.'
    basis: tests/security_negatives_channel.rs:229
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 294
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '  - **Windows listener DACL denial** (`#[cfg(windows)]`, IPC access control (Windows), `tests/security_negatives_channel.rs`, CI `test (windows-2025)`, chunk 2026-10-25-windows-endpoint-admission). `security_negative_viola_listener_denies_an_anonymous_open` opens the viola listener `GENERIC_READ` under `ImpersonateAnonymousToken` and asserts `ERROR_ACCESS_DENIED` (5); its control `security_negative_control_default_pipe_admits_an_anonymous_read` makes the same open on a pipe with the default descriptor, and that open succeeds.'
    sidecar: 'e2e security negatives: the Windows listener anonymous-open denial and its default-descriptor control added.'
    rationale: 'Report, Symbols / APIs, first bullet (245-259, 261-272), and Outcome (security) anonymous-token open denied with ERROR_ACCESS_DENIED met, the control passed (ci#40247500584).'
    basis: tests/security_negatives_channel.rs:245
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 294
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '    - (under the DACL denial bullet) Both anonymous-open cases bind their pipe before impersonating, on one spawned thread, and revert (`RevertToSelf`) within the same call; the default-descriptor control pipe also carries `PIPE_REJECT_REMOTE_CLIENTS`, so the descriptor is its only difference from the viola listener.'
    sidecar: 'e2e security negatives: the anonymous-open cases bind-before-impersonate order and the control pipe single difference recorded.'
    rationale: 'Report, Deviations from intent, second bullet (anonymous_read impersonates, opens and reverts in one call; the control pipe carries PIPE_REJECT_REMOTE_CLIENTS) and Decisions & corrections, the Windows test design candidate.'
    basis: tests/security_negatives_channel.rs:206
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 294
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '    - (after the DACL denial bullet) The five Windows listener cases bind their test pipes in the `viola-test-chan-<pid>-<label>` namespace, one label per case: `remote`, `accept-remote`, `reject-remote`, `anon` and `anon-default`.'
    sidecar: 'e2e security negatives: the Windows listener cases test-pipe namespace and labels recorded.'
    rationale: 'Report, Symbols / APIs, first bullet (test pipe names stay in the viola-test-chan-<pid>-<label> namespace) and the neg-file standing row (architecture test-only endpoint registry, true).'
    basis: tests/security_negatives_channel.rs:124
    fact: F10
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 606
    section: '## 6. E2E Test Strategy'
    change: '- **Security control negatives** (`security_negatives_*.rs`): one negative test per security-plan control that had none — the Unix socket directory refusal, Windows client SQOS at the identification level, the Windows listener rejecting remote clients and denying an anonymous open (each beside its controls), raw-channel `link` / `unlink` name validation, a Windows `--home` outside the profile given a protected DACL (its test home never under `%TEMP%`), umask-independent modes, and single writers of `stamps.json` and `snapshot.json`.'
    sidecar: 'test-plan §6 Security control negatives broad line: the two Windows listener negatives named.'
    rationale: 'Report, Expected amendments, second bullet: test-plan §6 Security control negatives names the two Windows listener negatives and their controls; the body line is the broad line e2e-test-strategy#15.1 details.'
    basis: tests/security_negatives_channel.rs:218
note: 'Not placed here: F9 (test-strategy, plain cargo test unsupported) has no picked entity of its category; the body §6 Skipped-as-untestable cross-OS-user item and security-plan authentication-authorization.md:15 (the remote-client witness) are the new-detail pass and the security cascade.'
