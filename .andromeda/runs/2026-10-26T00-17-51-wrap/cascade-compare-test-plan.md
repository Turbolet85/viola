# Cascade compare — test-plan · wrap · 2026-10-25-windows-endpoint-admission
E items 8 · held 4 · lost at classify 1 · lost at strong 0 · beside a placed fact 1 · cascade only 0 · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### mocking-stubbing-discipline#2.4 · lost at classify · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:19
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/mocking-stubbing-discipline.md
    near: 19
    section: "## 8. Mocking & Stubbing Discipline"
    change: "| Fake servers | interprocess 2.4.4 server API (squatter listener); `viola-fake-agent` as the fake CLI; windows-sys 0.61.2 `CreateNamedPipeW` (Windows only: test-owned control pipes in `tests/security_negatives_channel.rs`) | The squatter binds the computed endpoint before `run`; on Windows the listener-admission negatives set test-owned control pipes (one `PIPE_ACCEPT_REMOTE_CLIENTS`, one `PIPE_REJECT_REMOTE_CLIENTS`, one with null security attributes, the default descriptor) beside the real `viola_channel::Server::bind` listener, all in the `viola-test-chan-<pid>-<label>` namespace, and stand in for another principal with `ImpersonateAnonymousToken` on one spawned thread, the pipe bound before impersonating and reverted (`RevertToSelf`) inside the same call (chunk 2026-10-25-windows-endpoint-admission); no HTTP fake servers |"
    sidecar: "§8 Fake servers row: the Windows listener-admission negatives' test-owned `CreateNamedPipeW` control pipes and the anonymous-token stand-in principal added (chunk 2026-10-25-windows-endpoint-admission)."
    rationale: "The report's Changes › Symbols / APIs (first bullet) lands five Windows cases whose controls are test-owned `CreateNamedPipeW` pipes (accept-remote, reject-remote, default descriptor) beside the viola listener, and an `ImpersonateAnonymousToken` open as the foreign principal; Deviations names the bind-before-impersonate order. The Fake servers row names only the interprocess squatter and the fake agent, so the new test-owned listeners and the identity stand-in are new detail for this row."
    basis: viola-0.1.0/chunks/2026-10-25-windows-endpoint-admission/report.md:10

### test-strategy#3.1 · lost at classify · beside a placed fact F9 (near 10, test-strategy#1.1) · .andromeda/test-plan-detail/test-strategy.md:57
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 57
    section: "## 2. Test Strategy"
    change: "The trigger map's IPC row reads `| IPC vector (V1) | integration + E2E | §5 Module ↔ IPC; §6 Scenario 1 + Security control negatives (root `security_negatives_channel.rs`: the Windows listener's remote-client rejection and its DACL's denial of an anonymous-token open, each beside its controls, on `windows-2025`) |`."
    sidecar: "test-strategy trigger map: IPC vector (V1) also traced to §6 Security control negatives, where the Windows listener's remote-client rejection and DACL effect are now asserted on windows-2025 (chunk 2026-10-25-windows-endpoint-admission)."
    rationale: "The report's Symbols / APIs first bullet adds five Windows `#[test]` cases to `tests/security_negatives_channel.rs` (remote-client rejection plus accept-remote / reject-remote controls; anonymous open denied plus default-descriptor control), and Counts / qualifiers moved records the V1 Windows listener admission controls going 0 → asserted on `test (windows-2025)` (ci#40247500584); the V1 row's plan location names only §5 Module ↔ IPC and §6 Scenario 1, so the traceability map does not reach the suite that now holds those V1 controls."
    basis: .andromeda/test-plan-detail/test-strategy.md:57

## Held

- e2e-test-strategy#15.1
- integration-test-strategy#1.4
- test-scope-summary#1.1
- unit-test-strategy#2.1

## Cascade only
