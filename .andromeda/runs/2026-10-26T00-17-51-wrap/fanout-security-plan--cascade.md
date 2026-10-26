proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 15
    section: '## Authentication & Authorization'
    change: 'Append to the IPC access control (Windows) Implementation: The remote-client rejection is asserted on CI `test (windows-2025)` by `security_negative_viola_listener_rejects_remote_clients` (`tests/security_negatives_channel.rs`): it opens a client end of a `viola_channel::Server::bind` listener and reads its pipe type (`NtQueryInformationFile`, `FilePipeLocalInformation`, `NamedPipeType & 0x2`, `FILE_PIPE_REJECT_REMOTE_CLIENTS`), beside the controls `security_negative_control_accept_remote_pipe_reads_the_bit_clear` and `security_negative_control_reject_remote_pipe_reads_the_bit_set` on test-owned pipes, which tell the bit apart.'
    sidecar: 'authentication-authorization IPC access control (Windows): the remote-client rejection gains its CI witness on test (windows-2025) (chunk 2026-10-25-windows-endpoint-admission).'
    rationale: 'Report Changes, Symbols / APIs first bullet and Counts / qualifiers moved (remote-client rejection 0 → asserted on test (windows-2025), ci#40247500584); the remote-rej row reads line 15 true, now asserted, the clause gaining its witness.'
    basis: .andromeda/security-plan-detail/authentication-authorization.md:15
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/authentication-authorization.md
    near: 15
    section: '## Authentication & Authorization'
    change: 'Append to the IPC access control (Windows) Implementation: The DACL''s effect, not only its text read-back, is asserted on CI `test (windows-2025)` by `security_negative_viola_listener_denies_an_anonymous_open`: the listener is bound first, then a `GENERIC_READ` open under `ImpersonateAnonymousToken` fails with `ERROR_ACCESS_DENIED` (5); its control `security_negative_control_default_pipe_admits_an_anonymous_read` shows the same open admitted by a pipe with null security attributes (the default descriptor).'
    sidecar: 'authentication-authorization IPC access control (Windows): the protected user + SYSTEM DACL gains a witness for its effect, an anonymous open denied (chunk 2026-10-25-windows-endpoint-admission).'
    rationale: 'Report Changes, Symbols / APIs (the case at tests/security_negatives_channel.rs 245-259 and its control at 261-272) and Outcome (both passed, ci#40247500584); Touches name the DACL''s effect as newly witnessed.'
    basis: tests/security_negatives_channel.rs:245
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 48
    section: '## Threat Model Summary'
    change: 'Add under Trust boundary, after the per-OS enforcement line: - For the Windows pipe, remote-client rejection and the DACL''s effect are witnessed on hosted CI (`test (windows-2025)`, chunk 2026-10-25-windows-endpoint-admission), so nothing about those two is owed to "Windows-only live measurements"; the cross-user parts stay unexercised: a peer with a different euid, and a second OS account.'
    sidecar: 'threat-model-summary boundary-ipc: the Windows pipe''s remote-client rejection and DACL effect are witnessed on hosted CI; the cross-user parts stay unexercised (chunk 2026-10-25-windows-endpoint-admission).'
    rationale: 'Report Outcome (security acceptance: nothing owed to "Windows-only live measurements", ci#40247500584) and Spec claims disproved (the hosted-runner claim false for those two; a peer with a different euid and a second OS account stand).'
    fact: F3
  - detector: D-cascade
    severity: warning
    file: .andromeda/security-plan-detail/bootstrap-phases.md
    near: 14
    section: '## Bootstrap phases (derive for route / setup-project)'
    change: 'Append to the `viola-channel` per-OS listener hardening item: On Windows, the remote-client rejection and the DACL''s effect have CI witnesses on `test (windows-2025)` since chunk 2026-10-25-windows-endpoint-admission (the pipe-type read-back and the anonymous-token open, each beside its controls); first-instance and non-inheritance were re-read as passing there.'
    sidecar: 'bootstrap-phases auth-scaffolding-baseline: the Windows listener hardening names its CI witnesses (chunk 2026-10-25-windows-endpoint-admission).'
    rationale: 'Report Outcome (security acceptance, all three rows met on test (windows-2025), ci#40247500584); the Touches name auth-scaffolding-baseline''s per-OS listener hardening as gaining the Windows CI witnesses.'
    dependent-of: D-new-detail
note: F4 and F5 (dependency-security) fit no picked entity and go to the new-detail pass; the Threat Model Summary is headed a verbatim copy of threat-assessment.md, so F3's placement there is the orchestrator's call; D-security-input/-auth/-deps do not fire (no new input surface, auth flow or dependency: two dev-dependency features on the pinned windows-sys =0.61.2, lock unmoved).
