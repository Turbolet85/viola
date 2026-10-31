proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 39
    section: "## 7. Error Capture & Reporting"
    change: "Replace the clause «or `ServeError::Initialize` (exit 11) when the cut lands before it» with «or exit 1 with both streams empty when the cut lands before it (`ServeError::Initialize` reaches `src/main.rs` `exit_code`, `Role::Mcp` + `Failed`; 11 is the unit child's own code)»."
    sidecar: "§7 stdin clause: a `viola mcp` stdin line past `MAX_FRAME` before `initialize` exits 1, both streams empty; 11 is the unit child's own code (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "The report's Spec claims disproved by measurement names error-capture-reporting.md:39 under pattern `exit-eleven`: `mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line` reads exit 1, both streams empty, one `parse-rejected`; its Expected amendments carry the stdin clause's exit for obs-plan §7."
    basis: crates/viola-e2e/tests/mcp_tools.rs:522
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 39
    section: "## 7. Error Capture & Reporting"
    change: "Replace the clause «with `process-exit` 0 once `initialize` has completed» with «exit 0 and no self `process-exit` line once `initialize` has completed (the `mcp` role writes none)»."
    sidecar: "§7 stdin clause: the `mcp` role writes no self `process-exit` at a stdin cut after `initialize`; the session's exit stays 0 (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "The report's Spec claims disproved by measurement names error-capture-reporting.md:39 under pattern `mcp-exit-line` as the one changed site: its grep for `ObsEvent::ProcessExit` shows no self `process-exit` written on the `mcp` role, and the witness asserts exactly one `parse-rejected`."
    basis: src/obs.rs:232
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 17
    section: "## 7. Error Capture & Reporting"
    change: "Add after this line the sub-bullet «  - **`mcp` at the main-thread catch site:** `obs::internal_error_exit_line` writes for `Run` and `Cli` only, so `mcp`'s catch site writes no `process-exit` line; this binds the body's §7 `mcp` per-role line (chunk 2026-10-31-sanitised-error-surfaces).»"
    sidecar: "§7 catch site: `mcp`'s main-thread catch site writes no `process-exit`, `internal_error_exit_line` covering `Run` and `Cli` only (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "The report's Expected amendments carry for obs-plan §7 that `mcp`'s catch site writes no `process-exit`, measured under Spec claims disproved: `src/obs.rs:232` `internal_error_exit_line` writes for `Run` and `Cli` only."
    basis: src/obs.rs:232
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 39
    section: "## 7. Error Capture & Reporting"
    change: "Add after this line the item «- A client screens a peer's reply before any `--json` document or MCP result, and the screen adds no line: `reply_of` (`src/cmd/client.rs`) and `of_reply` (`crates/viola-mcp/src/result.rs`) read a `refusal` that is no code word (`viola_core::is_code_word`: 1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first) as `unknown` and a string `detail` that is no code word as null, and read a `wrapper-fault` through `viola_channel::sanitised_fault`, a code outside the five giving the fixed message `wrapper fault` with `data` null. A code word outside the closed list stays visible, its exit `unknown`'s 14. This binds security-plan §Error Handling (chunk 2026-10-31-sanitised-error-surfaces).»"
    sidecar: "§7 new detail: the clients' screen of a peer's reply, `is_code_word` and `sanitised_fault`, no line added (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "The report's Symbols / APIs and Coverage of new surfaces bring the `reply_of` / `of_reply` screen, instrumentation n/a and PII redacted; it serves this category's error-reporting surfaces (body §7 Scrubbing layers) and the file does not hold it."
    basis: crates/viola-channel/src/lib.rs:117
note: "The body's §7 `mcp` per-role bullet still says a main-thread panic writes `process-exit` with `internal-error`; the report's Expected amendments for §7 retire it, but its `mcp-exit-line` census lists no changed body site, so the body pass must carry it, and D-obs-instrumentation, D-obs-stack and D-obs-pii found no violation here."
