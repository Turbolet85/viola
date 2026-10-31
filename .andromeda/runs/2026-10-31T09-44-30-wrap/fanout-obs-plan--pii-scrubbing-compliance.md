proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 20
    section: "## 8. PII Scrubbing & Compliance › ### PII Scrubbing"
    change: Add this row after line 20 | A channel peer's reply (`refusal`, `detail`, an error's `message` and `data`) | Untrusted; may carry a path or upstream text, so the Medium rule applies | no process-log line carries it; reaches a CLI `--json` document, an MCP error or human text only screened | `viola_core::is_code_word` (1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first) and `viola_channel::sanitised_fault`, read by `reply_of` (`src/cmd/client.rs`) and `of_reply` (`crates/viola-mcp/src/result.rs`) — a `refusal` that is no code word reads `unknown`, a string `detail` that is no code word reads null, an error keeps its integer `code`, its `message` is that code's fixed text (`wrapper fault` for a code outside the five) and its `data` holds only closed values or null; a code word outside the closed list stays visible by design; asserted by `reply_of_reads_the_three_shapes`, `of_reply_maps_each_reply_shape`, `handlers_a_refusal_of_free_text_reads_as_unknown_with_no_detail` and `sanitised_fault_carries_nothing_of_the_peer_but_closed_values` |
    sidecar: §8 PII table gains a peer's reply as a data type, screened by `is_code_word` and `sanitised_fault` before any document or text (chunk 2026-10-31-sanitised-error-surfaces).
    rationale: The report's Symbols / APIs and Coverage of new surfaces add the screen on `reply_of` and `of_reply` (PII redacted, no line added), and its Outcome states a peer's fields reach no document or text; the table names no such data type. The report states no classification, so "Untrusted" and the Medium rule are this proposal's reading, taken from the `claude agents --json` row and the path-valued clause of the table.
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 22
    section: "## 8. PII Scrubbing & Compliance › ### PII Scrubbing"
    change: Line 22's Scrubbing mechanism cell reads Closed kebab-case enums; `channel-response`'s `detail` is one of `unsupported-version` · `invalid-params` · `release-from-driver` for the three `-32602` faults, on both the server's and the client's line, absent on every other `channel-response` line, closed by the enum in `schemas/diag-line.v1.json` and naming no param or value; asserted by `answer_invalid_params_names_its_cause_and_no_param`, `reply_from_logs_the_cause_of_each_invalid_params` and `link_verbs_log_the_channel_pair_and_no_names`
    sidecar: §8 PII table's protocol-codes row names `channel-response`'s three closed `detail` words and their no-param rule (chunk 2026-10-31-sanitised-error-surfaces).
    rationale: The report's Schema / config closes the field to the enum, its Symbols / APIs has `ResponseLine`'s `Drop` write `ProtocolError::cause`, and its Coverage reads PII no param value or name; this is the one log field the chunk adds near user data, and the row is where its class stands. D-obs-pii is not violated, since the words are fixed.
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 41
    section: "## 8. PII Scrubbing & Compliance › ### PII Scrubbing"
    change: Add this Verification item after line 41 — An error body (a CLI `--json` document, an MCP error result, a channel error) carries no absolute path, chain text, serde path or planted value. `error_body_hit` and `assert_error_body_clean` in `tests/support/events.rs` read the classes `absolute-path` · `caused-by` · `serde-path` · `planted` and report place and class, never bytes. The witnesses are `cli_error_documents_hold_no_path_or_upstream_text` (`tests/cli_error_bodies.rs`), `channel_error_bodies_hold_no_path_or_upstream_text` (`tests/channel_error_bodies.rs`) and `mcp_tools_error_results_hold_no_path_or_upstream_text` (`crates/viola-e2e/tests/mcp_tools.rs`); the census in `tests/contract_error_bodies.rs` (`error_bodies_each_name_a_writer_and_a_witness_that_stand`, `product_error_writers_are_each_named`, `thiserror_messages_interpolate_nothing`) holds every other writer. These replace a check after every end-to-end test, which is not built. No `-32603` is driven in a scanned home; the unit layer holds it.
    sidecar: §8 Verification gains the error-body check, its three surface witnesses and its census of writers (chunk 2026-10-31-sanitised-error-surfaces).
    rationale: The report's Symbols / APIs names the check and its classes, Spec claims disproved states the witnesses, enumeration and census replace the unbuilt after-every-test check, and Reverted / negative API facts records the `-32603` limit; item 5 lists each output-scan guarantee of this kind and does not hold this one.
