proposals:
  - detector: D-retired
    severity: warning
    section: "## 7. Screen Reader Support → ## 8. Cognitive Accessibility → Plain language → Verification"
    change: "In the trycmd bullet, re-cite the `viola list` unit cases `bay_line_*`, `board_row_*` and `instance_and_home_state_pairs_are_the_fixed_texts` as the `src/human/board/` units (the module root `src/human/board.rs` over its children `pairs.rs` · `bay.rs` · `cells.rs` · `rows.rs`, chunk 2026-10-20-epoch-5-cleanup-ii) in place of \"the `src/human/board.rs` units\"; unit names and the rest of the bullet unchanged."
    sidecar: "§8 Plain language → Verification: the `viola list` literal-pinning units re-cited under `src/human/board/` (the single-file `src/human/board.rs` retired into a module root, chunk 2026-10-20-epoch-5-cleanup-ii)."
    rationale: "The report's Retired bullet names the single-file `src/human/board.rs` (now a module root); Symbols / APIs → Moved, paths kept: `src/human/board.rs` re-exports `BayBudget` (`bay.rs`), `home_state_pair` / `instance_state_pair` (`pairs.rs`) and `write_board` (`rows.rs`), and `cells.rs` holds the cells, spans, `board_line`, `cut`; Expected amendments → `board-split` names `a11y-plan.md:303` \"the `src/human/board.rs` units\" — now under `src/human/board/`. Every case kept its name (Counts / qualifiers moved: 0 removed)."
    basis: "a11y-plan.md:303 (the report's `board-split` site)"
  - detector: D-retired
    severity: warning
    section: "## 4. ARIA Patterns & Roles → Must-be-accessible path scenarios → P6 → cli mirror"
    change: "Re-cite the unit case `board_row_caption_and_default_row_are_the_literals` as under `src/human/board/` (the module root `src/human/board.rs` and its children, chunk 2026-10-20-epoch-5-cleanup-ii) in place of \"(`src/human/board.rs`)\"; the `CAPTION` literal, the root case and the chunk citation unchanged."
    sidecar: "§4 P6 cli mirror: the caption unit case re-cited under `src/human/board/` (the single-file `src/human/board.rs` retired into a module root, chunk 2026-10-20-epoch-5-cleanup-ii)."
    rationale: "Same retired claim, second occurrence: this bullet cites the unit case by the file `src/human/board.rs`, which the report's Retired bullet retires as a single file (Symbols / APIs → Moved, paths kept). The report's `board-split` sweep classed `a11y-plan.md:232` as naming units only (no-change), but the line also carries the file cite `(src/human/board.rs)`, the same file-as-holder claim as `:303`; the unit name stands (0 removed)."
    basis: "a11y-plan.md:232"
    dependent-of: D-retired
note: "D-a11y-surface and D-a11y-obs-schema hold — the report adds no interactive UI element (Coverage of new surfaces: a11y n/a ×3; Outcome: board and verify output unchanged, no literal edited) and changes neither the a11y violation schema nor the obs log schema (its one new schema is `schemas/claude-mcp-fixture.v1.json`, the `Mcp.` fixture class). The report's own sweep classed `a11y-plan.md:232` no-change; the second proposal reads it otherwise because the line carries the file cite, so the orchestrator may confirm or drop it. Lines `:234`, `:236`, `:258` name units only and stand."
