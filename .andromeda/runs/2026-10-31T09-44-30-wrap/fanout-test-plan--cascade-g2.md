proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 59
    section: "## 5. Integration Test Strategy"
    change: "Replace the item's first sentence so it reads: oversize `answer` (Vector 6): `viola answer` stdin over `MAX_FRAME` exits 2 before connecting, with the fixed `UNUSABLE_RESPONSE` / `UNUSABLE_TEXT` line on stderr whatever `--json` says and no document (named in `NOT_ERROR_BODIES`, `tests/contract_error_bodies.rs`). The raw-client sentence and the dialog-pending sentence stay as written."
    sidecar: "integration-test-strategy: the oversize `viola answer` case exits 2 with the fixed stderr line and no `--json` error document, as measured at chunk 2026-10-31-sanitised-error-surfaces."
    rationale: "Report, Spec claims disproved by measurement: the claim of one `--json` error document at integration-test-strategy.md:59 is false; the exit-2 paths write the fixed line to stderr whatever `--json` says, no document, named in `NOT_ERROR_BODIES` (pattern `answer-oversize`, this the one changed site)."
    basis: "src/cmd/answer.rs:91"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 57
    section: "## 5. Integration Test Strategy"
    change: "Add after this item, as a sibling item: the MCP screen case `handlers_a_refusal_of_free_text_reads_as_unknown_with_no_detail` (`crates/viola-mcp/tests/handlers.rs`, chunk 2026-10-31-sanitised-error-surfaces) holds `of_reply`'s screen against a test wrapper: a `refusal` of free text reads `unknown`, with no detail."
    sidecar: "integration-test-strategy: the MCP refusal screen's handler case added beside the other `handlers.rs` cases."
    rationale: "Report, Coverage of new surfaces: the integration test of the `reply_of` / `of_reply` screen is this case; Symbols / APIs states that a `refusal` that is no code word reads `unknown`."
    basis: "crates/viola-mcp/tests/handlers.rs:246"
    fact: F11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 89
    section: "## 5. Integration Test Strategy"
    change: "Add under this item, as a sub-item: since chunk 2026-10-31-sanitised-error-surfaces a `channel-response` line's `detail` is one of `unsupported-version` · `invalid-params` · `release-from-driver` and absent on every other such line, written on both the server's and the client's line, level WARN unchanged, no name or param on the line, held by `link_verbs_log_the_channel_pair_and_no_names` and `schemas/diag-line.v1.json`."
    sidecar: "integration-test-strategy: the closed `channel-response` `detail` and its two sides added beside the link verbs' log pin."
    rationale: "Report, Symbols / APIs and Schema / config: `ResponseLine`'s `Drop` writes `ProtocolError::cause` on both lines, level unchanged, and the schema closes `detail` to the three words; Outcome: no name or param on the line, met with `link_verbs_log_the_channel_pair_and_no_names` green."
    basis: "schemas/diag-line.v1.json:42"
    fact: F12
note: "`verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1` (integration-test-strategy.md:82) went red once at the 7 s `WITHIN` and no bound moved, so no edit is proposed here; its owner or note is P5's by the report."
