proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 10
    section: "## 6. E2E Test Strategy"
    change: "Append to the cli row's Notes: As landed (chunk 2026-10-31-sanitised-error-surfaces), `tests/cli_error_bodies.rs` `cli_error_documents_hold_no_path_or_upstream_text` holds each driven CLI `--json` failure to one document, a literal exit, empty stderr, no ESC and a clean body, across dead, refused and live rows."
    sidecar: "e2e-test-strategy: the cli driver row names the CLI error-body witness tests/cli_error_bodies.rs."
    rationale: "Report Changes names the new file and Outcome reads the case met: each CLI --json failure the witness drives gives one document, literal exit, stderr empty, no ESC, clean."
    basis: tests/cli_error_bodies.rs:115
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 13
    section: "## 6. E2E Test Strategy"
    change: "Append to the wrapper-channel row's Notes: As landed (chunk 2026-10-31-sanitised-error-surfaces), `tests/channel_error_bodies.rs` `channel_error_bodies_hold_no_path_or_upstream_text` writes seven raw-channel fault rows to a wrapper booted with a planted value (through `OuterPty::spawn_without`'s `env_set`, as `tests/tui_env_strip.rs` does), its bodies clean and the three `detail` words read on the lines."
    sidecar: "e2e-test-strategy: the wrapper-channel driver row names the channel error-body witness tests/channel_error_bodies.rs."
    rationale: "Report Changes names the new file, Deviations item 1 states how the planted value reaches the child, and Outcome reads the seven wire bodies unchanged with the case green."
    basis: tests/channel_error_bodies.rs:100
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 14
    section: "## 6. E2E Test Strategy"
    change: "Append to the MCP stdio row's Notes: As landed (chunk 2026-10-31-sanitised-error-surfaces), `mcp_tools_error_results_hold_no_path_or_upstream_text` (an unstamped session, no row reaching a stamped path), `mcp_tools_a_wrong_shaped_call_answers_no_caller_text` and `mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line` (exit 1, both streams empty, one `parse-rejected` `mcp-stdin` / `oversize` line, count 1) stand in the same file."
    sidecar: "e2e-test-strategy: the MCP stdio driver row names the three new mcp_tools cases."
    rationale: "Report New text lists the three cases in crates/viola-e2e/tests/mcp_tools.rs, Deviations item 4 states the unstamped session, and Spec claims disproved states the measured exit 1 with one line."
    basis: crates/viola-e2e/tests/mcp_tools.rs:522
    fact: F6
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 14
    section: "## 6. E2E Test Strategy"
    change: "Append to the MCP stdio row's Notes: rmcp's client reads the wire's null `data` as `None`, so the live unknown-tool case asserts `None`; on the wire it is the product's `Some(Value::Null)`."
    sidecar: "e2e-test-strategy: the MCP stdio driver row records that rmcp's client reads a null data as None."
    rationale: "Report Deviations item 5 and Decisions & corrections state the oracle's red and the reading through rmcp's client."
    basis: crates/viola-e2e/tests/mcp_tools.rs:739
    fact: F7
note: No picked entity of group 3 carries F1, F2, F5, F8, F9 or F10 (the contract and error-sanitization entities are outside this group), so the new-detail pass or another group places them; #2.3's add-to class drew no proposal because the Path 1 CI timeout watch is P5's to retire; F6 and F7 together bring line 14 near the 2 000-char line limit, so check its length on apply.
