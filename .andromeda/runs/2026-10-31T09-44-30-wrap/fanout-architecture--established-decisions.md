proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 267
    section: "## Established Decisions"
    change: In the [Error Handling] entry replace everything from «One open divergence, not an exception» through the third «its fold or ratification is open» with «`AgentError` also holds `resolve_program`'s `BatchScriptChild` and `ProgramNotFound`, the enum `Refusal` folded into it by chunk 2026-10-31-sanitised-error-surfaces). Two named open divergences, not exceptions. `viola-mcp` holds `ServeError` (`Runtime`, `Initialize`, `Session`; fixed messages, returned by `serve`) beside `McpError` (`Internal`, Display `internal error`) (chunk 2026-10-17-mcp-server-for-drivers). `viola-state` holds `StampsError` (`StrictModes`, `State`; fixed messages, returned by `update_stamps`) beside `StateError`. The 2026-10-31 census of the 9 `thiserror` derives adds `PinError`, `StampsReadError` and `Refused`. The rule stands, and the fold or ratification of all five is open, owner "Version done-check" (PROVISIONAL)», the text before and the «; anyhow 1.0.104» after it unchanged.
    sidecar: established-decisions [Error Handling] - `Refusal` retired and folded into `AgentError` (`BatchScriptChild`, `ProgramNotFound`); `ServeError` and `StampsError` the two named open divergences, the census adding `PinError`, `StampsReadError` and `Refused`, 9 derives, owner a CARRY on "Version done-check".
    rationale: The report's Retired bullet names `viola_agent_claude::Refusal` and its Symbols / APIs bullet folds it into `AgentError`; its Counts bullets move the open divergences from three named to two named plus three the census adds and the derives from 10 to 9, patterns `refusal-enum` and `enum-rule` listing this line; inputs#I2 names the owner, the delegate overseer's word and PROVISIONAL.
    basis: .andromeda/architecture-detail/established-decisions.md:267
  - detector: D-arch-decisions
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 130
    section: "## Established Decisions"
    change: In the [MCP] entry replace «or `ServeError::Initialize` (exit 11) when the cut lands before `initialize` completes.» with «or `ServeError::Initialize` when the cut lands before `initialize` completes, and `viola mcp` then exits 1 (`src/main.rs` `exit_code`, `Role::Mcp` + `Failed`) with both streams empty, as `mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line` reads; 11 is only the code of `viola-mcp`'s unit child (`serve_child_entry`).»
    sidecar: established-decisions [MCP] - a stdin cut before `initialize` exits `viola mcp` 1, not 11; 11 is the unit child's own code.
    rationale: The report's Spec claims disproved lists «`ServeError::Initialize` (exit 11)» as `viola mcp`'s exit at this line, measured exit 1 with both streams empty and one `parse-rejected` line; pattern `exit-eleven` and the Expected amendments for §Established Decisions [MCP] carry the correction, and the report's Decisions keep the process exit at 1 with the masters moving.
    basis: .andromeda/architecture-detail/established-decisions.md:130
note: This file took two Write calls, not one, the second shortening the line 267 text so the amended entry stays under 2 000 chars (about 1 904); that owner clause reads on the CARRY P5 writes on "Version done-check"; no D-new-detail is proposed because the report expects `is_code_word`, `sanitised_fault` and the `channel-response` words under Conventions, Standard Contracts and the obs and security masters, and no new resource or dependency violates D-arch-resources or the stack half of D-arch-decisions.
