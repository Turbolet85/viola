proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 59
    section: "## 5. Integration Test Strategy"
    change: "In the oversize `answer` item, replace the first sentence with: `viola answer` stdin over `MAX_FRAME` exits 2 before connecting, with the fixed `UNUSABLE_RESPONSE` line on stderr whatever `--json` says and no document (named in `NOT_ERROR_BODIES`, `tests/contract_error_bodies.rs`). The raw-client sentence and the dialog-stays-pending sentence stand unchanged."
    sidecar: "§5 Wrapper channel, oversize `answer`: the claim of one `--json` error document is corrected to exit 2 with the fixed stderr line and no document, as measured at chunk 2026-10-31-sanitised-error-surfaces."
    rationale: "Report, Spec claims disproved, names this sentence as disproved (pattern `answer-oversize`: 2 standing, 1 changed), and Expected amendments carries §5's `viola answer` oversize sentence."
    basis: src/cmd/answer.rs:91
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 57
    section: "## 5. Integration Test Strategy"
    change: "New item after the MCP widened-instance case: the MCP free-text refusal case `handlers_a_refusal_of_free_text_reads_as_unknown_with_no_detail` (`crates/viola-mcp/tests/handlers.rs`, chunk 2026-10-31-sanitised-error-surfaces): a `refusal` from the peer that is no code word (`viola_core::is_code_word`) reads `unknown` with `detail` null, and the peer's text reaches no document or text; a code word outside the closed list stays visible by design."
    sidecar: "§5 Wrapper channel: added the MCP case for a peer's free-text refusal reading `unknown` with no detail."
    rationale: "Report, Coverage of new surfaces, names this test as the integration tier of the `reply_of` / `of_reply` screen; Symbols / APIs and Outcome state the behaviour it holds."
    basis: crates/viola-mcp/tests/handlers.rs:246
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 62
    section: "## 5. Integration Test Strategy"
    change: "New item at the end of Wrapper channel: the channel error-body witness `tests/channel_error_bodies.rs` (`[[test]]`, `required-features = [fake-agent]`) `channel_error_bodies_hold_no_path_or_upstream_text`, from `stamped_home` (chunk 2026-10-31-sanitised-error-surfaces): seven labelled rows of faulting frames against a live wrapper, each error body clean by `assert_error_body_clean` (`tests/support/events.rs`), the wire bodies literal, a too-new request answered -32602 with the wrapper's version, and `channel-response`'s `detail` one of `unsupported-version`, `invalid-params` or `release-from-driver`. The wrapper starts through `OuterPty::spawn_without`, the planted value reaching it through the PTY spawn's `env_set` as in `tests/tui_env_strip.rs`, because `Wrapper` takes no environment. No -32603 is driven in a scanned home: no failpoint or counted panic was added, and the unit layer holds it (`answer_a_panicking_dispatch_is_internal_error_and_closes`, `links_set_method_an_append_that_fails_is_internal_with_the_set_unchanged`)."
    sidecar: "§5 Wrapper channel: added the channel error-body witness and the statement that -32603 is held at the unit layer, not in a scanned home."
    rationale: "Report, Fact `new-binaries`: no master names this binary yet; Coverage of new surfaces names it the integration tier of `channel-response.detail`; Deviation 1 states the boot; Reverted / negative API facts states the -32603 limit."
    basis: tests/channel_error_bodies.rs:100
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 97
    section: "## 5. Integration Test Strategy"
    change: "New item after the machine contract: the CLI error-body witness `tests/cli_error_bodies.rs` (`[[test]]`, `required-features = [fake-agent]`, chunk 2026-10-31-sanitised-error-surfaces): `cli_error_documents_hold_no_path_or_upstream_text`, from `stamped_home`, reads each `--json` failure it drives as one document, its literal exit, stderr empty, no ESC, clean by `assert_error_body_clean` (`tests/support/events.rs`, classes `absolute-path`, `caused-by`, `serde-path`, `planted`, reporting place and class, never bytes). The check's self-checks stand beside it: `error_body_check_reads_each_class_as_a_hit` and `error_body_check_reports_no_byte_of_what_it_read`."
    sidecar: "§5 CLI: added the CLI error-body witness and the self-checks of the shared error-body check."
    rationale: "Report, Fact `new-binaries`: no master names this binary yet; Outcome states what the witness holds; Symbols / APIs states the check and its classes; New text, by line lists the two self-checks."
    basis: tests/cli_error_bodies.rs:115
note: "The line-59 correction is filed as D-new-detail because no D-tests-* invariant covers a disproved spec claim; this file holds no drift for D-tests-coverage, D-tests-framework, D-tests-obs-harness, D-tests-count-literals or D-retired."
