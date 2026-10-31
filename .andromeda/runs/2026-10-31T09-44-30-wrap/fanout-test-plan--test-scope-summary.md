proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 44
    section: "## 1. Test Scope Summary"
    change: In the Signal, after the one JSON document's three shapes, add the clause that `answer`'s and `send`'s exit-2 paths print no document, writing the fixed `UNUSABLE_RESPONSE` / `UNUSABLE_TEXT` line to stderr whatever `--json` says (both named in `NOT_ERROR_BODIES`).
    sidecar: test-scope-summary cli surface Signal now states that the exit-2 paths of `answer` and `send` print no document under `--json`.
    rationale: The Signal pairs exit 2 with one JSON document on stdout; the report's Spec claims disproved by measurement reads these exit-2 paths as a fixed stderr line with no document. The report's `answer-oversize` pattern did not reach this wording.
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 46
    section: "## 1. Test Scope Summary"
    change: Add one record under this surface, reading - **As landed (a peer's reply):** `reply_of` (`src/cmd/client.rs`) screens the wrapper's reply. A `refusal` that is no code word (`viola_core::is_code_word`, 1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first) reads `unknown`, a string `detail` that is no code word reads `None`, and a non-string `detail` is no known reply. A code word outside the closed list stays visible in the document and its exit stays `unknown`'s 14. A `wrapper-fault` document's `detail` is `viola_channel::sanitised_fault`'s, a code outside the five reading `{"code":<code>,"message":"wrapper fault","data":null}`. Each driven `--json` failure is one document, its literal exit, stderr empty, no ESC and clean by `assert_error_body_clean` (`reply_of_reads_the_three_shapes`, `cli_error_documents_hold_no_path_or_upstream_text`).
    sidecar: test-scope-summary cli surface gains the `reply_of` screen, the sanitised `wrapper-fault` detail and the CLI error-document witness.
    rationale: The report's Symbols / APIs names the `reply_of` screen and `sanitised_fault`, and its Outcome names the CLI witness; the cli surface's detail states neither.
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 63
    section: "## 1. Test Scope Summary"
    change: Add one record under this surface, reading - **As landed (the error bodies):** the seven `ProtocolError` wire bodies are unchanged and `v` stays 1, a too-new request answered `-32602` with the wrapper's version (`protocol_error_body_is_the_literal_wire_object`, `channel_error_bodies_hold_no_path_or_upstream_text`). `channel-response`'s `detail`, on the server's and the client's line at WARN, is closed to `unsupported-version` · `invalid-params` · `release-from-driver` for the three `-32602` faults and absent on every other line, with no name or param on it (`answer_invalid_params_names_its_cause_and_no_param`, `answer_faults_other_than_invalid_params_carry_no_detail`, `reply_from_logs_the_cause_of_each_invalid_params`). No `-32603` is driven in a scanned home; the unit layer holds it (`answer_a_panicking_dispatch_is_internal_error_and_closes`, `links_set_method_an_append_that_fails_is_internal_with_the_set_unchanged`).
    sidecar: test-scope-summary wrapper-channel surface gains the unchanged wire bodies' witness, the closed `channel-response` detail and the unit-held `-32603`.
    rationale: The report's Schema / config and Reverted / negative API facts close `detail` and place `-32603` at the unit layer, and its Outcome names the channel witness; the wrapper-channel surface's detail states none of them.
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 70
    section: "## 1. Test Scope Summary"
    change: Add one record under this surface, reading - **As landed (a peer's reply):** `of_reply` (`crates/viola-mcp/src/result.rs`) applies the screen the CLI's `reply_of` does. A `refusal` that is no code word reads `unknown`, a string `detail` that is no code word reads `null` and a non-string `detail` is no known reply. `wrapper-fault`'s `detail` is `viola_channel::sanitised_fault`'s, its `message` the fault's fixed text, a code outside the five reading `wrapper fault` with `data` null (`of_reply_maps_each_reply_shape`, `handlers_a_refusal_of_free_text_reads_as_unknown_with_no_detail`, `mcp_tools_error_results_hold_no_path_or_upstream_text`).
    sidecar: test-scope-summary MCP surface gains the `of_reply` screen and the sanitised `wrapper-fault` detail with their witnesses.
    rationale: The report's Symbols / APIs and Coverage of new surfaces name the `of_reply` screen, `sanitised_fault` and their tests; the MCP surface's Signal gives the `wrapper-fault` shape but not what fills it.
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 71
    section: "## 1. Test Scope Summary"
    change: Add one record after this one, reading - **As landed (the line before `initialize`):** a stdin line past `MAX_FRAME` before `initialize` completes ends `viola mcp` with exit 1, both streams empty and one `parse-rejected{mcp-stdin, oversize, count:1}` line (`crates/viola-e2e/tests/mcp_tools.rs` `mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line`); `serve` returns its initialize error (`serve_at_a_line_past_max_frame_before_initialize_is_the_initialize_error`), whose 11 is the unit child's own code and not the process's exit.
    sidecar: test-scope-summary MCP surface gains the measured exit 1 for a stdin line past `MAX_FRAME` before `initialize`.
    rationale: The stdin line cap record states only the initialized session's exit 0; the report's Outcome and Spec claims disproved by measurement give the case before `initialize` and its exit 1.
note: The §1 body's security-vector-coverage (MCP) trigger in test-plan.md also states only the initialized session's exit 0 for a stdin line past `MAX_FRAME`, and this category's proposals cannot edit the body.
