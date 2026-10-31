# Cascade compare — test-plan · wrap · 2026-10-31-sanitised-error-surfaces
E items 31 · held 11 · lost at classify 3 · lost at strong 1 · beside a placed fact 0 · cascade only 5 · listed - · body 0 · unplaced 1

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### test-data-fixtures#1.14 · lost at strong · .andromeda/test-plan-detail/test-data-fixtures.md:23
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 23
    section: "## 7. Test Data & Fixtures"
    change: "Add one table row after the Time-sensitive data row, reading: | The channel error-body witness's planted value | `tests/channel_error_bodies.rs` `booted_with_the_value` starts from `stamped_home` and boots through `OuterPty::spawn_without`, not `Wrapper` or `Command::env` (`Wrapper` takes no environment); the value reaches that one child through the PTY spawn's `env_set`, as `tests/tui_env_strip.rs` does. The check `tests/support/events.rs` `assert_error_body_clean` takes the planted values and reads a hit as class `planted`, reporting place and class, never bytes (chunk 2026-10-31-sanitised-error-surfaces). | per-test (rstest `stamped_home`) |"
    sidecar: "test-data-fixtures: the channel error-body witness's planted value recorded as a data row: planted through the PTY spawn's env_set, read by assert_error_body_clean as class planted (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "The report's Deviations item 1 names how step 7's planted value reaches the child, and its Symbols / APIs names the check and its class `planted`; the §7 table states a strategy per data type and holds no row for this value."
    basis: tests/channel_error_bodies.rs:56

### test-scope-summary#5.2 · lost at classify · .andromeda/test-plan-detail/test-scope-summary.md:63
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 63
    section: "## 1. Test Scope Summary"
    change: Add one record under this surface, reading - **As landed (the error bodies):** the seven `ProtocolError` wire bodies are unchanged and `v` stays 1, a too-new request answered `-32602` with the wrapper's version (`protocol_error_body_is_the_literal_wire_object`, `channel_error_bodies_hold_no_path_or_upstream_text`). `channel-response`'s `detail`, on the server's and the client's line at WARN, is closed to `unsupported-version` · `invalid-params` · `release-from-driver` for the three `-32602` faults and absent on every other line, with no name or param on it (`answer_invalid_params_names_its_cause_and_no_param`, `answer_faults_other_than_invalid_params_carry_no_detail`, `reply_from_logs_the_cause_of_each_invalid_params`). No `-32603` is driven in a scanned home; the unit layer holds it (`answer_a_panicking_dispatch_is_internal_error_and_closes`, `links_set_method_an_append_that_fails_is_internal_with_the_set_unchanged`).
    sidecar: test-scope-summary wrapper-channel surface gains the unchanged wire bodies' witness, the closed `channel-response` detail and the unit-held `-32603`.
    rationale: The report's Schema / config and Reverted / negative API facts close `detail` and place `-32603` at the unit layer, and its Outcome names the channel witness; the wrapper-channel surface's detail states none of them.

### test-scope-summary#5.3 · lost at classify · .andromeda/test-plan-detail/test-scope-summary.md:70
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 70
    section: "## 1. Test Scope Summary"
    change: Add one record under this surface, reading - **As landed (a peer's reply):** `of_reply` (`crates/viola-mcp/src/result.rs`) applies the screen the CLI's `reply_of` does. A `refusal` that is no code word reads `unknown`, a string `detail` that is no code word reads `null` and a non-string `detail` is no known reply. `wrapper-fault`'s `detail` is `viola_channel::sanitised_fault`'s, its `message` the fault's fixed text, a code outside the five reading `wrapper fault` with `data` null (`of_reply_maps_each_reply_shape`, `handlers_a_refusal_of_free_text_reads_as_unknown_with_no_detail`, `mcp_tools_error_results_hold_no_path_or_upstream_text`).
    sidecar: test-scope-summary MCP surface gains the `of_reply` screen and the sanitised `wrapper-fault` detail with their witnesses.
    rationale: The report's Symbols / APIs and Coverage of new surfaces name the `of_reply` screen, `sanitised_fault` and their tests; the MCP surface's Signal gives the `wrapper-fault` shape but not what fills it.

### test-scope-summary#5.3 · lost at classify · .andromeda/test-plan-detail/test-scope-summary.md:71
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 71
    section: "## 1. Test Scope Summary"
    change: Add one record after this one, reading - **As landed (the line before `initialize`):** a stdin line past `MAX_FRAME` before `initialize` completes ends `viola mcp` with exit 1, both streams empty and one `parse-rejected{mcp-stdin, oversize, count:1}` line (`crates/viola-e2e/tests/mcp_tools.rs` `mcp_tools_a_line_past_max_frame_before_initialize_exits_1_with_one_line`); `serve` returns its initialize error (`serve_at_a_line_past_max_frame_before_initialize_is_the_initialize_error`), whose 11 is the unit child's own code and not the process's exit.
    sidecar: test-scope-summary MCP surface gains the measured exit 1 for a stdin line past `MAX_FRAME` before `initialize`.
    rationale: The stdin line cap record states only the initialized session's exit 0; the report's Outcome and Spec claims disproved by measurement give the case before `initialize` and its exit 1.

### test-strategy#3.16 · lost at classify · .andromeda/test-plan-detail/test-strategy.md:73
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 73
    section: "## 2. Test Strategy"
    change: "In the `error sanitization / secret logging` row's Plan location, after `§6 Error sanitization and secret scan`, add in parentheses the three surface witnesses, root `cli_error_bodies.rs` `cli_error_documents_hold_no_path_or_upstream_text`, root `channel_error_bodies.rs` `channel_error_bodies_hold_no_path_or_upstream_text` and `crates/viola-e2e/tests/mcp_tools.rs` `mcp_tools_error_results_hold_no_path_or_upstream_text`, and root `contract_error_bodies.rs`, the enumeration and the census of writers; `§3 logs` stays."
    sidecar: "test-strategy.md: the error-sanitization trigger row names its three surface witnesses and the `contract_error_bodies` enumeration and census (2026-10-31-sanitised-error-surfaces)."
    rationale: "Report Changes, Counts / qualifiers moved, Fact `new-binaries`, says no master names these binaries yet, and Spec claims disproved says the three surface witnesses, the enumeration and the census of writers carry this check; the neighbouring trigger rows name their root binaries the same way."
    basis: tests/cli_error_bodies.rs:115

## Held

- e2e-test-strategy#12.1
- e2e-test-strategy#13.1
- e2e-test-strategy#19.1
- integration-test-strategy#4.1
- integration-test-strategy#5.1
- test-scope-summary#4.1
- test-strategy#2.1
- unit-test-strategy#1.1
- unit-test-strategy#1.3
- unit-test-strategy#1.4
- unit-test-strategy#2.1

## Cascade only

- e2e-test-strategy#1.1
- e2e-test-strategy#1.4
- e2e-test-strategy#1.5
- test-scope-summary#1.1
- test-scope-summary#9.2
