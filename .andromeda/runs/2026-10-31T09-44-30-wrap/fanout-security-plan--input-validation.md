proposals:
  - detector: D-security-input
    severity: escalate
    file: .andromeda/security-plan-detail/input-validation.md
    near: 31
    section: "## Input Validation"
    change: "The close of the On the cut sentence reads: and returns what the session gave: exit 0 for an initialized session; when the cut lands before `initialize` completes, `serve` returns `ServeError::Initialize` and the `viola mcp` process exits 1 with both streams empty (`src/main.rs` `exit_code`: `Role::Mcp` + `Failed`); 11 is only the unit child's own code (`serve_child_entry`)."
    sidecar: "§Input Validation, MCP stdin: a cut before `initialize` completes exits the `viola mcp` process 1, not 11; `ServeError::Initialize`'s 11 is the unit child's own code (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "The report's Spec claims disproved names input-validation.md:31 under pattern `exit-eleven`: `mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line` reads exit 1, both streams empty, one `parse-rejected`; its Expected amendments carries §Input Validation (MCP stdin, the exit)."
    basis: .andromeda/security-plan-detail/input-validation.md:31
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/input-validation.md
    near: 74
    section: "## Input Validation"
    change: "`Refusal::BatchScriptChild` reads `AgentError::BatchScriptChild`; the rest of the line stands (exit 1, no child spawned, the `process-exit` line and the two fixed stderr lines)."
    sidecar: "§Input Validation, Child executable resolution: the `.cmd`/`.bat` refusal is `AgentError::BatchScriptChild`, `viola_agent_claude::Refusal` folded into `AgentError` with its messages and outputs unchanged (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "The report's Retired bullet names `viola_agent_claude::Refusal`; Symbols / APIs says `AgentError` gains `BatchScriptChild`, messages unchanged; Counts lists input-validation.md:74 changed under pattern `refusal-enum`, with :73 and :75 no-change."
    basis: .andromeda/security-plan-detail/input-validation.md:74
note: "No unvalidated boundary, no auth change and no new dependency: the near-31 proposal sits under D-security-input as the nearest detector, correcting a disproved exit rather than a missing check; the report homes the clients' screen (`is_code_word`, `sanitised_fault`) in §Error Handling, so no boundary row is proposed here for it."
