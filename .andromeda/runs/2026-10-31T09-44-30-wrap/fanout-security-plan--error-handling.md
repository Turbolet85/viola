proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 18
    section: "## Error Handling"
    change: "- `viola_channel::sanitised_fault` is the one reader of a peer's JSON-RPC fault for `wrapper-fault`'s detail, called by the CLI's `reply_of` and MCP's `of_reply`: `None` without an integer `code`; for the five known codes `message` is the fault's fixed text; `-32602` with an integer `data.supported` and a version-text `data.wrapper` (1 to 64 ASCII letters, digits, `.`, `-` or `+`) reads the unsupported-version body with those two values, `data.reason` exactly `release-from-driver` reads that body, and any other `-32602` reads `invalid params` with `data: null`; every other known code reads `data: null`; a code outside the five keeps its `code` and reads `message` `wrapper fault` with `data: null`. A peer's `message` or `data` otherwise reaches no document or text."
    sidecar: "Error Handling detail gains sanitised_fault's closed reading of a peer's fault for wrapper-fault (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "Report Changes, Symbols / APIs names sanitised_fault and its table; Expected amendments carries security-plan Error Handling (sanitised_fault); Outcome records a peer's message or data reaching no document or text."
    basis: crates/viola-channel/src/lib.rs:117
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 18
    section: "## Error Handling"
    change: "- The clients screen a peer's refusal reply through `viola_core::is_code_word` (1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first), the CLI's `reply_of` and MCP's `of_reply` alike: a `refusal` that is no code word reads `unknown`, a string `detail` that is no code word reads `None` / `null`, and a non-string `detail` is still no known reply. A code word outside the closed list stays visible by design, in the document and in `refused: <word>`, and its exit stays `unknown`'s 14 by `reason_of`."
    sidecar: "Error Handling detail gains the clients' code-word screen on a peer's refusal and detail (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "Report Changes, Symbols / APIs states the reply_of and of_reply screen and is_code_word; Expected amendments carries security-plan Error Handling (the clients' screen); Coverage of new surfaces lists the screen."
    basis: src/cmd/client.rs:77
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 18
    section: "## Error Handling"
    change: "- The `thiserror::Error` derives over product sources are nine, `AgentError` · `ChannelError` · `StateError` · `PinError` · `StampsError` · `StampsReadError` · `Refused` · `McpError` · `ServeError`, and `thiserror_messages_interpolate_nothing` (`tests/contract_error_bodies.rs`) holds their messages to interpolating nothing. `AgentError`'s `BatchScriptChild` and `ProgramNotFound` carry the two fixed messages unchanged, outputs unchanged."
    sidecar: "Error Handling detail gains the census of nine thiserror enums and the two AgentError variants that fold viola_agent_claude::Refusal (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "Report Changes, Counts / qualifiers moved gives the nine derives; Symbols / APIs folds Refusal into the two AgentError variants; Expected amendments carries security-plan Error Handling (the enum list); New text, by line lists the census test."
    basis: tests/contract_error_bodies.rs:743
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 18
    section: "## Error Handling"
    change: "- `channel-response`'s `detail`, written by `ResponseLine`'s `Drop` from `ProtocolError::cause` on both the server's and the client's line, level unchanged (WARN), is the closed enum `unsupported-version` · `invalid-params` · `release-from-driver` for the three `-32602` faults and absent on every other `channel-response` line; the line holds no param value or name, and the seven wire bodies and `v` 1 are unchanged (binds with obs-plan §6, the three `detail` words)."
    sidecar: "Error Handling detail gains the closed cause word on channel-response's detail, binding obs-plan section 6 (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "Report Changes, Symbols / APIs and Schema / config close the detail; Coverage of new surfaces states closed enum and no param value or name; Outcome records the three words on both sides, level unchanged."
    basis: schemas/diag-line.v1.json:42
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/error-handling.md
    near: 18
    section: "## Error Handling"
    change: "- Error bodies are read by `error_body_hit` and `assert_error_body_clean` (`tests/support/events.rs`), classes `absolute-path` · `caused-by` · `serde-path` · `planted`, reporting place and class, never bytes. The three surface witnesses (`cli_error_documents_hold_no_path_or_upstream_text`, `channel_error_bodies_hold_no_path_or_upstream_text`, `mcp_tools_error_results_hold_no_path_or_upstream_text`), the enumeration of bodies and the census of writers (`tests/contract_error_bodies.rs`) stand in place of an after-every-test check, which is not built (binds with test-plan §6). No `-32603` is driven in a scanned home: the unit layer holds it (`answer_a_panicking_dispatch_is_internal_error_and_closes`, `links_set_method_an_append_that_fails_is_internal_with_the_set_unchanged`)."
    sidecar: "Error Handling detail gains the error-body check, its witnesses and census, and the unit-held -32603 limit (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "Report Changes, Symbols / APIs names the check and its classes; Spec claims disproved states the witnesses, enumeration and census replace the unbuilt after-every-test check; Reverted / negative API facts states the -32603 limit."
    basis: tests/support/events.rs:18
note: "D-security-input, D-security-auth and D-security-deps find no violation and this file names no retired item (Refusal, fault_detail, CoreError); the report's standing edits at .andromeda/security-plan.md:411 and :413 are the body's, outside this file's proposals."
