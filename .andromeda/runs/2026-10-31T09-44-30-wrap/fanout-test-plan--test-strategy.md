proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 42
    section: "## 2. Test Strategy"
    change: "In the `tests/support/` helpers item, after the clause on `events.rs`'s reader, wait and `boot`, add that `events.rs` also holds the error-body check `error_body_hit` and `assert_error_body_clean`, classes `absolute-path` · `caused-by` · `serde-path` · `planted`, reporting place and class, never bytes (chunk 2026-10-31-sanitised-error-surfaces)."
    sidecar: "test-strategy.md: `tests/support/events.rs` now named as the holder of the error-body check `error_body_hit` / `assert_error_body_clean` and its four classes (2026-10-31-sanitised-error-surfaces)."
    rationale: "Report Changes, Symbols / APIs, Tests' check, adds both helpers to `tests/support/events.rs`; this item lists what each support file holds and names only the reader, the wait and `boot` for it."
    basis: tests/support/events.rs:18
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 73
    section: "## 2. Test Strategy"
    change: "In the `error sanitization / secret logging` row's Plan location, after `§6 Error sanitization and secret scan`, add in parentheses the three surface witnesses, root `cli_error_bodies.rs` `cli_error_documents_hold_no_path_or_upstream_text`, root `channel_error_bodies.rs` `channel_error_bodies_hold_no_path_or_upstream_text` and `crates/viola-e2e/tests/mcp_tools.rs` `mcp_tools_error_results_hold_no_path_or_upstream_text`, and root `contract_error_bodies.rs`, the enumeration and the census of writers; `§3 logs` stays."
    sidecar: "test-strategy.md: the error-sanitization trigger row names its three surface witnesses and the `contract_error_bodies` enumeration and census (2026-10-31-sanitised-error-surfaces)."
    rationale: "Report Changes, Counts / qualifiers moved, Fact `new-binaries`, says no master names these binaries yet, and Spec claims disproved says the three surface witnesses, the enumeration and the census of writers carry this check; the neighbouring trigger rows name their root binaries the same way."
    basis: tests/cli_error_bodies.rs:115
note: D-tests-coverage, D-tests-framework, D-tests-obs-harness and D-tests-count-literals found no drift in this file; the report rules `test-strategy.md:29` no-change for `kill-line`, and no item its Retired bullet names appears here.
