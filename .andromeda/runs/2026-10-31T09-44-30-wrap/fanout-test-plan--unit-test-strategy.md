proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 7
    section: "## 4. Unit Test Strategy"
    change: Add a new group `- **viola-core:**` ahead of viola-channel, with one item — `is_code_word` accepts 1 to 64 bytes of `[a-z0-9-]`, a lower-case letter first, pinned by the literal table `code_word_literal_table` (`crates/viola-core/src/lib.rs`); its callers are the two reply screens, `reply_of` and `of_reply`.
    sidecar: Unit Test Strategy gains a viola-core group for `is_code_word` and its literal table (chunk 2026-10-31-sanitised-error-surfaces).
    rationale: The report's Symbols / APIs name the new `viola_core::is_code_word` and New text lists `code_word_literal_table`; the file holds no case for it.
    basis: viola-0.1.0/chunks/2026-10-31-sanitised-error-surfaces/report.md:10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 10
    section: "## 4. Unit Test Strategy"
    change: Append to this item — no `-32603` is driven in a scanned home (no failpoint or counted panic was added), so the unit layer holds it, through `answer_a_panicking_dispatch_is_internal_error_and_closes` and `links_set_method_an_append_that_fails_is_internal_with_the_set_unchanged`.
    sidecar: The JSON-RPC mapping item records that the unit layer alone holds `-32603` (chunk 2026-10-31-sanitised-error-surfaces).
    rationale: The report's Reverted / negative API facts state that no `-32603` is driven in a scanned home and name the two unit cases that hold it.
    basis: viola-0.1.0/chunks/2026-10-31-sanitised-error-surfaces/report.md:41
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 11
    section: "## 4. Unit Test Strategy"
    change: Add a viola-channel item — `ProtocolError::cause` gives `unsupported-version` · `invalid-params` · `release-from-driver` for the three `-32602` faults and `None` for the other four (`protocol_error_cause_tells_the_three_invalid_params_apart`, `crates/viola-channel/src/lib.rs`); the response line writes that word as `detail` on the server's side (`answer_invalid_params_names_its_cause_and_no_param`, `answer_faults_other_than_invalid_params_carry_no_detail`, `crates/viola-channel/src/server.rs`) and on the client's, where `fault_of(&Value)` reads `data` for `-32602` (`reply_from_logs_the_cause_of_each_invalid_params`).
    sidecar: viola-channel gains the item for the `channel-response` cause words and their unit cases (chunk 2026-10-31-sanitised-error-surfaces).
    rationale: The report's Symbols / APIs and Coverage of new surfaces add `cause`, the changed `fault_of` and the `detail` write with these unit cases; the file names none of them.
    basis: viola-0.1.0/chunks/2026-10-31-sanitised-error-surfaces/report.md:61
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 14
    section: "## 4. Unit Test Strategy"
    change: Add a viola-channel item — `sanitised_fault` (`crates/viola-channel/src/lib.rs`) is `None` without an integer `code` (`sanitised_fault_without_an_integer_code_is_nothing`), keeps each wire body (`sanitised_fault_keeps_each_wire_body`) and carries nothing of the peer but closed values (`sanitised_fault_carries_nothing_of_the_peer_but_closed_values`), a code outside the five reading the message `wrapper fault` with null `data`; a `-32602` keeps an integer `data.supported` only beside a version-text `data.wrapper`, 1 to 64 ASCII letters, digits, `.`, `-` or `+` (`sanitised_fault_keeps_a_wrapper_of_sixty_four_version_characters`).
    sidecar: viola-channel gains the `sanitised_fault` item and its four unit cases (chunk 2026-10-31-sanitised-error-surfaces).
    rationale: The report's Symbols / APIs add `sanitised_fault` and `is_version_text`, and New text lists these four cases; the file holds no record of them.
    basis: viola-0.1.0/chunks/2026-10-31-sanitised-error-surfaces/report.md:12
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 36
    section: "## 4. Unit Test Strategy"
    change: Add a viola-agent-claude item — `resolve_program` returns `Result<PathBuf, AgentError>`, its refusals the variants `AgentError::BatchScriptChild` and `AgentError::ProgramNotFound`, their two messages unchanged and held by `refusal_messages_are_fixed`.
    sidecar: viola-agent-claude records the program resolution refusals as `AgentError` variants (chunk 2026-10-31-sanitised-error-surfaces).
    rationale: The report's Symbols / APIs fold the retired `Refusal` into two `AgentError` variants and Coverage of new surfaces names `refusal_messages_are_fixed` as their unit case.
    basis: viola-0.1.0/chunks/2026-10-31-sanitised-error-surfaces/report.md:63
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 50
    section: "## 4. Unit Test Strategy"
    change: Add a viola-mcp item — `of_reply` (`crates/viola-mcp/src/result.rs`) screens a peer's reply as the root bin's `reply_of` does, and `of_reply_maps_each_reply_shape` holds it — a `refusal` that is no code word reads `unknown`, a string `detail` that is no code word reads null, a non-string `detail` is no known reply, and a fault's body comes through `sanitised_fault`.
    sidecar: viola-mcp gains the item for the `of_reply` screen and its unit case (chunk 2026-10-31-sanitised-error-surfaces).
    rationale: The report's Symbols / APIs change `of_reply` this way and Coverage of new surfaces names `of_reply_maps_each_reply_shape` as its unit case.
    basis: viola-0.1.0/chunks/2026-10-31-sanitised-error-surfaces/report.md:13
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 52
    section: "## 4. Unit Test Strategy"
    change: Add a viola-mcp item — `serve_at_a_line_past_max_frame_before_initialize_is_the_initialize_error` (`crates/viola-mcp/src/lib.rs`) drives a stdin line past `MAX_FRAME` before `initialize` to `ServeError::Initialize`; the 11 is the unit child's own code (`serve_child_entry`), and the `viola mcp` process exits 1, held end to end by `mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line`.
    sidecar: viola-mcp records the driven stdin line before `initialize`, separating the unit child's 11 from the process's exit 1 (chunk 2026-10-31-sanitised-error-surfaces).
    rationale: The report's Spec claims disproved and Outcome name this new case and measure exit 1 for the process; line 52 itself states no exit and stands under pattern `exit-eleven`.
    basis: viola-0.1.0/chunks/2026-10-31-sanitised-error-surfaces/report.md:44
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 59
    section: "## 4. Unit Test Strategy"
    change: Add a root bin item — the client's reply screen `reply_of` (`src/cmd/client.rs`), held by `reply_of_reads_the_three_shapes` — a `refusal` that is no code word reads `unknown`, a string `detail` that is no code word reads `None`, a non-string `detail` is no known reply, and a fault's body comes through `sanitised_fault`; a code word outside the closed list stays visible in the document and in the `refused` text, its exit still 14 by `reason_of`.
    sidecar: Root bin gains the item for the `reply_of` screen and its unit case (chunk 2026-10-31-sanitised-error-surfaces).
    rationale: The report's Symbols / APIs change `reply_of` this way and Coverage of new surfaces names `reply_of_reads_the_three_shapes`; line 59's exit 14 mapping stands and this adds the screen ahead of it.
    basis: viola-0.1.0/chunks/2026-10-31-sanitised-error-surfaces/report.md:13
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 81
    section: "## 4. Unit Test Strategy"
    change: Add a root bin item — the cap-driving cases behind the `BOUNDARIES` rows of `tests/contract_bounded_readers.rs` — `statusline_shell_out_past_the_frame_cap_keeps_exactly_the_cap` and `statusline_shell_out_a_command_the_cut_ends_is_a_failed_command` (`src/cmd/hook/statusline.rs`, `cfg(unix)`), `drain_keeps_cap_bytes_of_an_endless_pipe` (`src/run/version_gate.rs`), `read_thresholds_keeps_no_key_past_the_frame_cap` (`src/run/budget.rs`), `read_hold_keeps_no_key_past_the_frame_cap` (`src/run/dialog.rs`) and `env_keep_keeps_no_key_past_the_frame_cap` (`src/run/env.rs`).
    sidecar: Root bin gains the item naming the six cap-driving cases behind five `BOUNDARIES` rows (chunk 2026-10-31-sanitised-error-surfaces).
    rationale: The report's Harness / gate surface and Outcome rename five `BOUNDARIES` witnesses to these cases, which New text places in the root bin's source files; the file names none of them.
    basis: viola-0.1.0/chunks/2026-10-31-sanitised-error-surfaces/report.md:99
note: No D-retired hit in this file (`Refusal`, `fault_detail` and `CoreError` appear nowhere in it); the framework, harness and count detectors read clean here; the retired exit 11 and the discharged owed clause at test-plan.md:390 are in the body, outside this file.
