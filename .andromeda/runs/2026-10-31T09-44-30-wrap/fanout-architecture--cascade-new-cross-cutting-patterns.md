proposals:
  - detector: D-new-detail
    severity: warning
    fact: F1
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 29
    section: "## Cross-cutting Patterns"
    change: "- **Sanitised error surfaces.** CLI `--json` documents, MCP error results and channel error data carry codes and fixed messages only, no path and no upstream text; a peer's `message`, `data`, `refusal` or `detail` reaches no document or text, and a code word outside the closed list stays visible by design."
    sidecar: "Cross-cutting Patterns gains the sanitised error surfaces rule, chunk 2026-10-31-sanitised-error-surfaces."
    rationale: "The report's Chunk line and Outcome state the rule and the code word's visibility by design; no standing group of this category holds it."
  - detector: D-new-detail
    severity: warning
    fact: F11
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 29
    section: "## Cross-cutting Patterns"
    change: "  - The E2E contract binary `tests/contract_error_bodies.rs` (run under `run --e2e`, carrying `andromeda:walks-tree`) holds it: its tables `CLI_DOCUMENTS`, `MCP_RESULTS` and `CHANNEL_FAULTS` enumerate the error bodies, each body naming a writer and a witness that stand (`error_bodies_each_name_a_writer_and_a_witness_that_stand`); `NOT_ERROR_BODIES` names what is no error body and `PLUMBING` the census lines that build or route a document a row names, so a row stays one body. A census, `product_error_writers_are_each_named`, covers the product's error writers, and `thiserror_messages_interpolate_nothing` holds that `thiserror` messages interpolate nothing."
    sidecar: "Cross-cutting Patterns names the error-body enumeration and census of tests/contract_error_bodies.rs, chunk 2026-10-31-sanitised-error-surfaces."
    rationale: "The report's Changes (Harness / gate surface, Spec claims disproved) and Deviations 2 and 3 state the binary, its tables and the census; New text, by line lists the table and case names. NOT_ERROR_BODIES and PLUMBING are stated apart from the enumeration because the report gives them those other roles."
  - detector: D-new-detail
    severity: warning
    fact: F12
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 29
    section: "## Cross-cutting Patterns"
    change: "  - Three surface witnesses read the bodies: `cli_error_documents_hold_no_path_or_upstream_text` in `tests/cli_error_bodies.rs`, `channel_error_bodies_hold_no_path_or_upstream_text` in `tests/channel_error_bodies.rs` and `mcp_tools_error_results_hold_no_path_or_upstream_text` in `crates/viola-e2e/tests/mcp_tools.rs`; the two root binaries' `[[test]]` tables require the `fake-agent` feature, and both sit in the `ci` profile's kill-line filter."
    sidecar: "Cross-cutting Patterns names the three surface witnesses of the sanitised error surfaces, chunk 2026-10-31-sanitised-error-surfaces."
    rationale: "The report's Outcome names the CLI and channel witnesses, New text, by line lists the MCP one, and Changes (Harness / gate surface, Counts) states the required feature and the kill-line filter's two added terms."
  - detector: D-new-detail
    severity: warning
    fact: F13
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 29
    section: "## Cross-cutting Patterns"
    change: "  - No `-32603` is driven in a scanned home and no failpoint or counted panic was added for it: the unit layer holds it, through `answer_a_panicking_dispatch_is_internal_error_and_closes` and `links_set_method_an_append_that_fails_is_internal_with_the_set_unchanged`."
    sidecar: "Cross-cutting Patterns states that the unit layer holds -32603 for the sanitised error surfaces, chunk 2026-10-31-sanitised-error-surfaces."
    rationale: "The report's Changes, Reverted / negative API facts, states the limit and names both unit cases."
note: "F1, F11, F12 and F13 are one new group after line 29, in that order, F1 its head and the other three its sub-items; the group's detail marker pair and key are left to the orchestrator."
