proposals:
  - detector: D-tests-framework
    severity: warning
    section: '§5 Integration Test Strategy (CLI bullet, test-plan.md:585)'
    change: 'drop `viola-help` from the trycmd `tests/cmd/*.toml` list and state that `viola --help` (the grouped verb table) is pinned by the `HELP` literal in `tests/cli_output_plain.rs` (`help_is_the_grouped_verb_table`), not by a trycmd case (chunk 2026-10-17-cli-output-discipline)'
    sidecar: 'test-plan §5 CLI: `viola-help` pinned by the `HELP` literal in tests/cli_output_plain.rs, no trycmd case (2026-10-17-cli-output-discipline)'
    rationale: 'report Spec claims disproved [trycmd]: no manifest holds trycmd or assert_cmd and no `tests/cmd/` exists; measured this chunk, `viola --help` is pinned by the `HELP` literal (`help_is_the_grouped_verb_table`); Expected amendments names test-plan §5 / §6 / §7 `viola-help` pinned by literal, no trycmd case'
    basis: 'tests/cli_output_plain.rs:54-68'
  - detector: D-tests-framework
    severity: warning
    section: '§5 Integration Test Strategy (Human-mode layouts bullet, test-plan.md:542)'
    change: 'Human-mode layouts: trycmd case files except the `viola list` board (literal asserts in `tests/cli_list.rs`) and `viola --help` (the `HELP` literal in `tests/cli_output_plain.rs`, chunk 2026-10-17-cli-output-discipline)'
    sidecar: 'test-plan §5 Human-mode layouts: `viola --help` joins the list board as a literal-pinned exception to trycmd'
    rationale: 'same retired claim as the §5 CLI bullet: the help is a human-mode layout and the line routes every one but the board to trycmd; report Spec claims disproved [trycmd] lists test-plan.md:542'
    basis: 'tests/cli_output_plain.rs:70-76'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: '§6 E2E Test Strategy (surface table, cli row, test-plan.md:614)'
    change: 'the cli row tool cell reads: trycmd 1.2.1 for human layouts other than the `viola list` board and `viola --help`, pinned by literal in `tests/cli_list.rs` and `tests/cli_output_plain.rs` respectively'
    sidecar: 'test-plan §6 cli row: `viola --help` pinned by literal beside the list board'
    rationale: 'same retired claim restated in the §6 surface table; report Spec claims disproved [trycmd] lists test-plan.md:614 and the Expected amendments name the §6 `viola-help` site'
    basis: 'tests/cli_output_plain.rs:54-68'
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: '§7 Test Data & Fixtures (Human-mode expected output row, test-plan.md:931)'
    change: 'the Human-mode expected output row adds that `viola --help` (the `HELP` literal, `tests/cli_output_plain.rs`) and the `viola list` board (`tests/cli_list.rs`) are literals in their test files, not trycmd case files'
    sidecar: 'test-plan §7 Human-mode expected output: the help and the board are literal-pinned, not `tests/cmd/*.toml`'
    rationale: 'same retired claim in the §7 fixtures table (the help is human-mode expected output); report Spec claims disproved [trycmd] lists test-plan.md:931 and the Expected amendments name the §7 `viola-help` site'
    basis: 'tests/cli_output_plain.rs:54-68'
    dependent-of: D-tests-framework
  - detector: D-tests-coverage
    severity: warning
    section: '§5 Integration Test Strategy (CLI bullet, `tests/cli_output_plain.rs`, test-plan.md:589)'
    change: 'the witness also pins the grouped help: `help_is_the_grouped_verb_table` (the `HELP` literal) and `help_lists_only_served_verbs` (each served verb answers `--help` exit 0, its usage program read as `viola` or `viola.exe`; no `hook` / `help` / `link` / `unlink` / `plugin` in the table), plus the Unix no-lone-CR reading (`redraws`); the SGR parser `sgr_attributes` (with `redraws`, `sets_a_foreground`) lives in `tests/support/sgr.rs`, its self-check case staying in `tests/cli_output_plain.rs`'
    sidecar: 'test-plan §5 cli_output_plain: grouped-help and served-verbs cases, Unix CR reading; SGR parser moved to tests/support/sgr.rs'
    rationale: 'report Counts [sgr-parser]: test-plan.md:589 changed (parser in `tests/support/sgr.rs`; the file gains the grouped-help and served-verbs cases and the Unix CR reading); Deviations 1, 4 and 6; Coverage of new surfaces names the help tests'
    basis: 'tests/support/sgr.rs:34-53'
  - detector: D-tests-coverage
    severity: escalate
    section: '§6 E2E Test Strategy (surface table, cli row, test-plan.md:614)'
    change: 'the cli row verdict cell reads: bold NAME and amber `DIALOG` are checked inside the outer PTY on the SGR sequences (`cross_list_rows_style_each_span_beside_its_word_on_a_terminal`); dim `stale` is proven at the unit tier only (`board_row_style_a_stale_row_is_dim_end_to_end_and_only_dialog_is_amber`), its outer-PTY witness owed to the owner P5 names (`inputs#I4`)'
    sidecar: 'test-plan §6 cli row: dim stale witnessed at unit tier only; integration witness owed to the P5-named owner'
    rationale: 'the §6 cli row mandates TTY rendering incl. dim `stale` be checked by running `viola list` inside the outer PTY; the report (Outcome a11y bullet, inputs#I3 card row 3, Decisions) states the dim span on a real stale row is proven at the unit tier only and has no owner, and inputs#I4 directs P5 to give it one; escalated because the owner (a CARRY or the armed orphan halt) is not yet named'
    basis: 'src/human/board.rs:931-959'
  - detector: D-tests-coverage
    severity: warning
    section: '§5 Integration Test Strategy (CLI bullet, `tests/cli_controls_not_disableable.rs`)'
    change: 'the Vector 6 table also holds the three `setting_does_not_disable_the_send_paste_control` cases for `NO_COLOR`, `TERM` and `COLORTERM` (chunk 2026-10-17-cli-output-discipline), and `assert_send_paste_control_holds` removes the three inherited values before setting the case''s own'
    sidecar: 'test-plan §5 cli_controls_not_disableable: the colour env vars join the table as send-paste-control rows'
    rationale: 'report Env vars: `NO_COLOR` / `TERM` / `COLORTERM` are new env reads in every build (founder, inputs#I2) that must disable no control; Outcome (security) names the three cases of `setting_does_not_disable_the_send_paste_control`; Deviation 3 and the Decisions measurement record the inherited-value removal; the bullet scopes the table to `VIOLA_*` vars, flags and config keys only'
    basis: 'tests/cli_controls_not_disableable.rs:137-140'
note: 'D-tests-obs-harness: no drift (report Harness / gate surface: none). The [trycmd] measurement also disproves the pre-existing trycmd / assert_cmd claims this chunk did not move (test-plan.md:383, :403, :586 tail, :666, :850, :1211, and the `viola-send` mirror half of :585); the report asks Validate to dispose them (amend or carry) - not proposed here. Bootstrap-phases key line 22 already reads "in no manifest yet": no change.'
