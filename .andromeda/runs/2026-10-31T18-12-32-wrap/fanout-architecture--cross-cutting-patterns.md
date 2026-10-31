proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/cross-cutting-patterns.md
    near: 35
    section: "## Cross-cutting Patterns"
    change: "Add after this line one item under Sanitised error surfaces: `  - The E2E table `tests/cross_exit_causes.rs` (chunk 2026-10-31-exit-cause-code-catalogue; a root `cross_` binary run under `run --e2e`, its `[[test]]` requiring the `fake-agent` feature, `binary(cross_exit_causes)` on the `ci` profile's kill line) reads each exit-21 and exit-1 cause it lists as its own code on the `--json` document, the human hint and the `process-exit` line; `no_two_codes_share_a_hint`, with its control `shared_hints_reads_a_planted_duplicate`, `no_hint_names_release` and `every_code_is_a_code_word` hold the table itself, and a cause named outside the table is named in the comment at the file's top.`"
    sidecar: "Cross-cutting Patterns, Sanitised error surfaces: the exit-cause table `tests/cross_exit_causes.rs` recorded beside the contract binary and the surface witnesses (chunk 2026-10-31-exit-cause-code-catalogue)."
    rationale: "The report's Changes name the new file `tests/cross_exit_causes.rs`, its `[[test]]` with `required-features = [\"fake-agent\"]` and the kill-line operand (Harness / gate surface), the comment at its top (Counts, `cross-table`) and its table cases (Outcome, deviation 7); this group names each binary that holds the error-body rule and has no entry for this one, and the entry leaves MCP's words to `handlers` and `mcp_tools` as the report's Coverage does."
    basis: tests/cross_exit_causes.rs:69
note: No D-arch-resources, D-arch-decisions or D-retired drift in this file, the chunk adding no crate, dependency, env var or endpoint, and no line here stating an item the report's Retired bullet names.
