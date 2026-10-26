proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 21
    section: "## 7. Test Data & Fixtures"
    change: "Append to the Time-sensitive data row's Strategy cell: one stamp stays outside the clock seam, `LinkSet::line` (`src/run/links.rs`), which stamps a link line's `ts` with `Utc::now()` directly; that `ts` is the `since` a served `link` answers (`{since, already_linked}`), so no case injects it and the root cases of `tests/cli_links.rs` assert `since` by shape only (`is_clock_time`, held by `is_clock_time_reads_the_shape_only`) (as landed at chunk 2026-10-26-explicit-session-links)."
    sidecar: "§7 Time-sensitive data: the link line's `ts` (the `link` reply's `since`) is stamped by `Utc::now()` in `LinkSet::line`, outside the clock seam, and asserted by shape only (`is_clock_time`); chunk 2026-10-26-explicit-session-links."
    rationale: "The report's Decisions & corrections states `LinkSet::line` stamps `since` with `Utc::now()` directly, outside the clock seam, and root cases assert `since` by shape only (`is_clock_time`); its Expected amendments name §7 for this clock-seam fact with no pattern, for the test-plan detector to read; the row today says time is injected per test behind `viola_core::Clock` with only the budget gate's override expiry as a carve-out."
    basis: "viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:278"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 17
    section: "## 7. Test Data & Fixtures"
    change: "In the Human-mode expected output row's Strategy cell, after the `viola list` board clause, add: and the three `viola link` / `viola unlink` result lines (`link    <driver> -> <driven>  since <HH:MM:SSZ>` · `link    <driver> -> <driven>  already linked` · `unlink  <driver> -> <driven>`, plain in every mode) are pinned byte for byte in their test files, `src/human/verbs.rs` (`link_verb_lines_are_the_design_lines_in_one_write`, `link_since_is_the_utc_time_alone`) and `tests/cli_links.rs` (`link_verbs_print_the_design_lines`), the printed time checked by shape (chunk 2026-10-26-explicit-session-links)."
    sidecar: "§7 Human-mode expected output: the `link` / `unlink` result lines join `viola --help` and the `viola list` board as literals pinned in their test files; chunk 2026-10-26-explicit-session-links."
    rationale: "The report's Symbols / APIs adds three result lines (`write_linked`, `write_already_linked`, `write_unlinked`) and its Outcome (design) records them met byte for byte by the verbs.rs cases and `link_verbs_print_the_design_lines`; the row names only `viola --help` and the `viola list` board as literal-pinned human output."
    basis: "viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:310"
note: "Outside this file: body §7 (test-plan.md:653, Time-sensitive data) carries the broad clock line the first proposal details, and §8's `chrono::Utc::now()` only-through-the-clock-seam claim (test-plan.md:705) reads stale beside `LinkSet::line`'s stamp (the mocking category's); the report's `src/run/links.rs:92` falls inside its own New-text row 85-99 (`driver_of`), so the first proposal names the symbol, not the line."
