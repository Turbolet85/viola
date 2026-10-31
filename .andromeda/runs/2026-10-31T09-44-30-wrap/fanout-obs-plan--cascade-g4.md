proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 40
    section: "## 8. PII Scrubbing & Compliance › ### PII Scrubbing"
    change: "   - **Error-body census:** `tests/contract_error_bodies.rs` (carrying `andromeda:walks-tree`) holds the error bodies of CLI `--json`, MCP results and the seven channel faults in tables: each body names a writer and a witness that stand (`error_bodies_each_name_a_writer_and_a_witness_that_stand`), every product error writer must be named (`product_error_writers_are_each_named`), and `thiserror_messages_interpolate_nothing` holds thiserror messages free of interpolation."
    sidecar: "§8 Verification: the error-body census `tests/contract_error_bodies.rs` recorded beside the secret scan (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "The report's Changes (Harness / gate surface) and Deviations from intent name the census and its `andromeda:walks-tree`, and its New text, by line lists the three checks; no §8 verification item carries it."
    basis: tests/contract_error_bodies.rs:652
    fact: F12
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 40
    section: "## 8. PII Scrubbing & Compliance › ### PII Scrubbing"
    change: "   - **Error-body check:** `tests/support/events.rs` holds `error_body_hit` and `assert_error_body_clean`, classes `absolute-path`, `caused-by`, `serde-path` and `planted`, reporting place and class, never bytes. It does not run after every test: test-plan §6's after-every-test check was not built, and the witnesses, the enumeration and the census replace it. It sits beside the secret scan and does not change the scan."
    sidecar: "§8 Verification: the error-body check in `tests/support/events.rs` recorded, not run after every test, the scan unchanged (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "The report's Symbols / APIs name the check and its classes; its Spec claims disproved records the after-every-test helper as measured absent and replaced; its Harness / gate surface says no CI step moved."
    basis: tests/support/events.rs:18
    fact: F13
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/pii-scrubbing-compliance.md
    near: 40
    section: "## 8. PII Scrubbing & Compliance › ### PII Scrubbing"
    change: "   - **Live witnesses of clean error bodies:** `channel_error_bodies_hold_no_path_or_upstream_text` (`tests/channel_error_bodies.rs`), `cli_error_documents_hold_no_path_or_upstream_text` (`tests/cli_error_bodies.rs`) and `mcp_tools_error_results_hold_no_path_or_upstream_text` (`crates/viola-e2e/tests/mcp_tools.rs`)."
    sidecar: "§8 Verification: the three live witnesses of clean error bodies recorded (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "The report's Outcome reads the channel and CLI witnesses green, and its New text, by line lists all three cases; no §8 verification item names them."
    basis: tests/channel_error_bodies.rs:100
    fact: F14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/ci-integration.md
    near: 59
    section: "## 9. CI Integration"
    change: "   - `v1-47`'s secret-scan reading is after each job's end-to-end run, not after every test: `AGENT_RUN_KEEP_HOMES` is `1` on the job, the scan is `if: always()`, and the `viola-chaos-*` home stays out by the founder's ruling of 2026-10-04. That reading is accepted by the delegate overseer's word of 2026-10-31, PROVISIONAL, owed the founder's own."
    sidecar: "§9 step 1: `v1-47`'s secret-scan reading recorded as after each job's end-to-end run, on the delegate overseer's PROVISIONAL word (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "The report's Decisions & corrections concretizes `v1-47` to after each job's end-to-end run, measured at `.github/workflows/ci.yml:20` and `:104`, on the delegate word it marks PROVISIONAL and owed the founder's own."
    basis: .github/workflows/ci.yml:20
    fact: F15
note: "slo-invariants-telemetry-budgets#2.2 labels `max 2.4–3.3 ms`, while the report labels the same range of values (2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms before) `mean` and states no `max` for this run: left unedited because one label looks wrong and the report cannot settle which."
