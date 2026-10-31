# Cascade compare — architecture · wrap · 2026-10-31-sanitised-error-surfaces
E items 13 · held 8 · lost at classify 0 · lost at strong 0 · beside a placed fact 1 · cascade only 1 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### cross-cutting-patterns#3.1 · lost at classify · beside a placed fact F1 (near 29, cross-cutting-patterns#3.1) · .andromeda/architecture-detail/cross-cutting-patterns.md:29
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 29
    section: "## Cross-cutting Patterns"
    change: "- **Sanitised error bodies.** A CLI `--json` document, an MCP error result and a channel fault's `data` carry codes and fixed messages only, no path and no upstream text; a peer's `message`, `data`, `refusal` or `detail` reaches no document or text. A client reads a peer's reply through two shared screens, both called by `reply_of` (`src/cmd/client.rs`) and `of_reply` (`crates/viola-mcp/src/result.rs`): `viola_channel::sanitised_fault` and `viola_core::is_code_word`; a code word outside the closed list stays visible by design. The E2E contract binary `tests/contract_error_bodies.rs` (run under `run --e2e`) holds it: `CLI_DOCUMENTS` and `MCP_RESULTS` name each body's writer and witness, `CHANNEL_FAULTS` the seven faults' literal wire bodies and witnesses, `NOT_ERROR_BODIES` the lines that are no error body, `PLUMBING` the lines that build or route a document a row names; `error_bodies_each_name_a_writer_and_a_witness_that_stand` checks each still stands. A census, `product_error_writers_are_each_named`, walks the product sources, `thiserror_messages_interpolate_nothing` reads each `thiserror` message, and `census_finds_an_unnamed_writer_and_an_interpolated_message` is their self-check. Three surface witnesses drive the bodies: `cli_error_documents_hold_no_path_or_upstream_text`, `channel_error_bodies_hold_no_path_or_upstream_text`, `mcp_tools_error_results_hold_no_path_or_upstream_text`. The check `assert_error_body_clean` (`tests/support/events.rs`) reports place and class (`absolute-path` · `caused-by` · `serde-path` · `planted`), never bytes. They replace test-plan §6's after-every-test helper, which was not built. A new error writer therefore lands named in a table, or the census goes red."
    sidecar: "Cross-cutting Patterns gains the sanitised-error-bodies pattern and its contract binary, beside bounded readers (chunk 2026-10-31-sanitised-error-surfaces)."
    rationale: "The report's Changes add `tests/contract_error_bodies.rs`, `tests/channel_error_bodies.rs` and `tests/cli_error_bodies.rs` and the two shared screens, its fact `new-binaries` says no master names the three binaries yet, and its Spec claims disproved says the witnesses, the enumeration and the census replace the unbuilt after-every-test check; this file holds the like pattern for bounded readers."
    basis: tests/contract_error_bodies.rs:720

## Held

- conventions#1.1
- conventions#7.1
- established-decisions#17.1
- established-decisions#30.1
- occupied-resources#18.1
- occupied-resources#31.1
- occupied-resources#4.1
- standard-contracts#15.1

## Cascade only

- occupied-resources#35.1
