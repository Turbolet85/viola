proposals:
  - detector: D-new-detail
    severity: warning
    fact: F20
    file: .andromeda/security-plan-detail/error-handling.md
    near: 11
    section: "## Error Handling"
    change: "- `viola_state::stamps::StampsError` (`StrictModes(Refused)` · `State(StateError)`, the error `update_stamps` now returns, `From<io::Error>`) and `viola_state::pin::PinError::StrictModes` use fixed `Display` messages, no path or payload: `StampsError` reads «the capability stamps failed the strict-modes check» or «… could not be written», and `PinError::StrictModes` reads «the pinned copy failed the strict-modes check»; `verify` maps `StampsError::StrictModes` to the home pair, and `run` maps `PinError::StrictModes` to the `sideload_fallback` outcome `strict-modes-failed`."
    sidecar: "error-handling: new item, the fixed Display messages of StampsError and PinError::StrictModes (chunk 2026-10-26-home-and-code-bearing-file-integrity)"
    rationale: "The report's Changes name the new `StampsError` with its two fixed messages (report.md:11) and `PinError::StrictModes` with its fixed message (report.md:13), and their mappings at verify (report.md:19) and run's `sideload_outcome` (report.md:18); the category file holds no record of either."
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:11
note: The body's broad Display line (security-plan.md Error Handling, the fixed-message list naming PtyError, ChannelError, StateError, AgentError, McpError, CoreError) does not yet name StampsError or PinError; that edit is the strong readers', not proposed here.
