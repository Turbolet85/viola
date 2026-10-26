# Cascade compare — test-plan · wrap · 2026-10-26-explicit-session-links
E items 25 · held 14 · lost at classify 3 · lost at strong 0 · beside a placed fact 0 · cascade only 1 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### integration-test-strategy#3.2 · lost at classify · .andromeda/test-plan-detail/integration-test-strategy.md:57
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 57
    section: '## 5. Integration Test Strategy'
    change: '  - `viola link` / `viola unlink` run `check_instance` on the driven instance before `client::start` (the founder form, chunk 2026-10-26-explicit-session-links), so a refusal is exit 21 with the fixed instance-state pair (or `{"v":1,"error":"instance-unreachable","detail":null}` under `--json`, stderr empty), no role file opened, no line in any log and the directory mode as found: `tests/cli_links.rs` `link_verbs_refuse_a_widened_instance_before_any_log` (Unix) reads the mode after the refusal, because opening a role file narrows an existing home to 0700 and a check placed after it would pass a 0770 home'
    sidecar: 'integration-test-strategy On-disk: the link verbs strict-modes refusal before any log and its test'
    rationale: 'Report Symbols / APIs (`link_call` step 1); Decisions & corrections (the order trap held by a test; exit 21 as `list` / `plugin install`); Outcome (security group-writable driven instance refused before any log, mode as found)'

### test-data-fixtures#1.8 · lost at classify · .andromeda/test-plan-detail/test-data-fixtures.md:17
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 17
    section: "## 7. Test Data & Fixtures"
    change: "In the Human-mode expected output row's Strategy cell, after the `viola list` board clause, add: and the three `viola link` / `viola unlink` result lines (`link    <driver> -> <driven>  since <HH:MM:SSZ>` · `link    <driver> -> <driven>  already linked` · `unlink  <driver> -> <driven>`, plain in every mode) are pinned byte for byte in their test files, `src/human/verbs.rs` (`link_verb_lines_are_the_design_lines_in_one_write`, `link_since_is_the_utc_time_alone`) and `tests/cli_links.rs` (`link_verbs_print_the_design_lines`), the printed time checked by shape (chunk 2026-10-26-explicit-session-links)."
    sidecar: "§7 Human-mode expected output: the `link` / `unlink` result lines join `viola --help` and the `viola list` board as literals pinned in their test files; chunk 2026-10-26-explicit-session-links."
    rationale: "The report's Symbols / APIs adds three result lines (`write_linked`, `write_already_linked`, `write_unlinked`) and its Outcome (design) records them met byte for byte by the verbs.rs cases and `link_verbs_print_the_design_lines`; the row names only `viola --help` and the `viola list` board as literal-pinned human output."
    basis: "viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:310"

### test-strategy#3.1 · lost at classify · .andromeda/test-plan-detail/test-strategy.md:58
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 58
    section: "## 2. Test Strategy"
    change: "In the IPC vector (V1) row's Security control negatives parenthetical, add: root `tests/channel_links.rs` `channel_link_names_are_validated_by_the_wrapper` (all OSes): raw `link` / `unlink` frames with a missing, non-string or invalid name, a `driven` that is not the serving wrapper's own name, or a `driver` equal to `driven`, each answered `-32602` with `data: null` and no line written, beside its control."
    sidecar: "test-strategy trigger map V1 row: the raw-channel link / unlink name negative, landed in tests/channel_links.rs (chunk 2026-10-26-explicit-session-links)."
    rationale: "The report's Changes (Files: new tests/channel_links.rs; Symbols: the served link / unlink methods' -32602 causes) and Outcome (security: raw bad-name / foreign-driven / self-pair frames -> -32602, no line, met by channel_link_names_are_validated_by_the_wrapper, 14 passed) land the raw-channel negative the V1 row's plan location names only by file for the other security control negatives."
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:301

## Held

- e2e-test-strategy#10.1
- e2e-test-strategy#11.1
- e2e-test-strategy#14.1
- e2e-test-strategy#15.1
- integration-test-strategy#3.1
- integration-test-strategy#4.1
- mocking-stubbing-discipline#1.1
- test-data-fixtures#1.12
- test-scope-summary#1.1
- test-scope-summary#3.1
- test-scope-summary#5.2
- test-strategy#1.1
- unit-test-strategy#1.2
- unit-test-strategy#2.1

## Cascade only

- e2e-test-strategy#1.1
