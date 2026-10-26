proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 57
    section: "## 2. Test Strategy"
    change: "The trigger map's IPC row reads `| IPC vector (V1) | integration + E2E | §5 Module ↔ IPC; §6 Scenario 1 + Security control negatives (root `security_negatives_channel.rs`: the Windows listener's remote-client rejection and its DACL's denial of an anonymous-token open, each beside its controls, on `windows-2025`) |`."
    sidecar: "test-strategy trigger map: IPC vector (V1) also traced to §6 Security control negatives, where the Windows listener's remote-client rejection and DACL effect are now asserted on windows-2025 (chunk 2026-10-25-windows-endpoint-admission)."
    rationale: "The report's Symbols / APIs first bullet adds five Windows `#[test]` cases to `tests/security_negatives_channel.rs` (remote-client rejection plus accept-remote / reject-remote controls; anonymous open denied plus default-descriptor control), and Counts / qualifiers moved records the V1 Windows listener admission controls going 0 → asserted on `test (windows-2025)` (ci#40247500584); the V1 row's plan location names only §5 Module ↔ IPC and §6 Scenario 1, so the traceability map does not reach the suite that now holds those V1 controls."
    basis: .andromeda/test-plan-detail/test-strategy.md:57
note: D-tests-coverage, D-tests-framework, D-tests-obs-harness and D-tests-count-literals hold for this file (every new path is test code run under nextest in the existing coverage job; no harness, runner or count literal in this category moved; Retired none); the naming group's root sync-suite list (`tests/{cli,hook,tui,channel,chaos,contract}_<topic>.rs`) omits the pre-existing `security_negatives_` prefix, a gap this chunk did not open (it added cases to an existing file), left to the orchestrator.
