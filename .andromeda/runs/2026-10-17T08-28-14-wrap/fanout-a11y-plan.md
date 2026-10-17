proposals:
  - detector: D-a11y-surface
    severity: warning
    section: '§6 Visual Design Verification — CLI equivalent'
    change: 'Replace the as-built clause "As built (chunk 2026-10-11-the-board-viola-list) `viola list` emits no SGR and reads no `NO_COLOR` / `TERM` / `COLORTERM` ... with no truncation); the colour layer is owed to "CLI output discipline"" with: as built (chunk 2026-10-17-cli-output-discipline) `viola list` human mode on a terminal writes bold on every NAME value (wrapped and unwrapped), the attention (amber) token on the `DIALOG` word only of a wrapped row with a pending dialog (its kind word plain), and a wrapped `stale` row dim from its first byte to its last (NAME dim and bold, `DIALOG` amber and not dim); the BAY line, caption, empty-board line and separator carry no SGR; each state is still carried by its printed word (`stale`, `DIALOG <kind>`, `expired`, `budget-paused`, `unverified-cli`, `idle` / `busy`, `n/a`); the colour decision reads `NO_COLOR` (present, an empty value included), `TERM` (exactly `dumb`), `COLORTERM` (the founder''s own word, inputs#I2) and whether stdout is a terminal, writes no SGR under `--json`, a passthrough role or any of those rungs, and picks truecolor / 256 / 16 depth (Windows: VT processing enabled on stdout and restored after; a failed enable writes none); on a terminal too narrow an unwrapped NAME is cut to `column − 3` characters + `...` (never inside a `\xHH` escape; a wrapped NAME never cut; piped and `--json` whole), the width read from `viola_pty::host_size()` (PROVISIONAL, inputs#I1 Q2); checked by the `board_row_style_*` / `board_row_width_*` units and `cross_list_rows_style_each_span_beside_its_word_on_a_terminal` / `cross_list_rows_print_no_sgr_under_no_color_dumb_piped_or_json` / `cross_list_rows_truncate_an_unwrapped_name_only_on_a_terminal_too_narrow_for_it`; dim only on stale rows proven at the unit tier only (its integration-tier witness owed at P5). The rest of the paragraph (zero SGR under non-TTY / NO_COLOR / dumb / --json / run / revive; Windows ConPTY bytes; `unable` / `fail` never coloured) stays.'
    sidecar: '§6 CLI equivalent: the colour layer and the width cut built (chunk 2026-10-17-cli-output-discipline); `viola list` reads NO_COLOR / TERM / COLORTERM (founder, inputs#I2); width source host_size PROVISIONAL (inputs#I1 Q2); stale-dim integration witness owed at P5.'
    rationale: 'Report Changes → Symbols / APIs (src/human/style.rs, src/human/board.rs, src/cmd/list.rs, Env vars) and Counts [colour-env, colour-layer, owner-title, width] name a11y-plan.md:258 changed: the colour layer the line calls owed now exists, the three env vars are read, and unwrapped NAMEs are cut on a narrow terminal; Coverage of new surfaces and Outcome (a11y) name the cases; Decisions name the P5-owed integration-tier dim witness.'
    basis: '.andromeda/a11y-plan.md:258'
  - detector: D-a11y-surface
    severity: warning
    section: '§4 ARIA Patterns & Roles — P6 CLI board, wait/last, verify'
    change: 'Replace "`viola list` emits no SGR yet (chunk 2026-10-11-the-board-viola-list), so its three spans have no check until the colour layer lands ("CLI output discipline")" with: as built (chunk 2026-10-17-cli-output-discipline) `viola list`''s spans are checked under the outer PTY by `cross_list_rows_style_each_span_beside_its_word_on_a_terminal` (bold NAME, amber only on `DIALOG`; on windows-2025 the parsed attribute set) and at the unit tier by the `board_row_style_*` cases; dim only on stale rows is proven at the unit tier only (`board_row_style_a_stale_row_is_dim_end_to_end_and_only_dialog_is_amber`), its integration-tier witness owed at P5; `viola wait` and `viola --help` carry no ESC and no lone CR on Unix (`tests/support/sgr.rs` `redraws`).'
    sidecar: '§4 P6: the list colour spans now checked (outer PTY + units), stale-dim integration witness owed at P5; no-ESC / no-CR reading extended to --help (chunk 2026-10-17-cli-output-discipline).'
    rationale: 'Report Counts [colour-layer, owner-title] name a11y-plan.md:234 changed; Outcome (a11y) rows "spans beside their words under the outer PTY; dim only on stale rows at the unit tier" and "no ESC and no lone CR on Unix for wait and --help"; Decisions "Owed at P5".'
    basis: '.andromeda/a11y-plan.md:234'
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    section: '§1 A11y Scope Summary — Surface cli, Notes'
    change: 'Replace "as built (chunk 2026-10-11-the-board-viola-list) `viola list` prints words only, with no SGR (owed to "CLI output discipline")" with: as built (chunk 2026-10-17-cli-output-discipline) `viola list` on a terminal writes amber only on `DIALOG`, dim on stale rows and bold on NAME values, each beside its word, and none under `--json`, a pipe, `NO_COLOR` (empty included) or `TERM=dumb`.'
    sidecar: '§1 cli Notes: the colour-as-second-cue rule built for viola list (chunk 2026-10-17-cli-output-discipline).'
    rationale: 'Report Counts [owner-title] names a11y-plan.md:73 changed (an "owed to CLI output discipline" now discharged); Symbols / APIs board.rs and style.rs.'
    basis: '.andromeda/a11y-plan.md:73'
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    section: '§4 ARIA Patterns & Roles — Per-surface landmark roles inventory (cli row)'
    change: 'Replace "the `--help` group labels ("CLI output discipline"; the built help is clap''s ordered list) are not built" with: the `--help` group labels built (chunk 2026-10-17-cli-output-discipline): `viola --help` and `viola help` print `Usage: viola [--home <dir>] <verb> [args]`, the rows `board:` · `traffic:` · `wheel:` · `setup:` over the served verbs only (PROVISIONAL, inputs#I1 Q3), then `Options:`, plain ASCII, pinned by the `HELP` literal in `help_is_the_grouped_verb_table` (`tests/cli_output_plain.rs`); the `handoff:` row is owned by "Explicit session links" and `plugin install` in `setup:` by "Optional plugin install".'
    sidecar: 'cli landmark row: the grouped --help table built, served verbs only (PROVISIONAL, inputs#I1 Q3); handoff row and plugin install owned by their entries (chunk 2026-10-17-cli-output-discipline).'
    rationale: 'Report Counts [help, owner-title] name a11y-plan.md:209 changed ("not built … clap''s ordered list"); Symbols / APIs `viola --help`; inputs#I3 Beyond-the-plan rows name the owners of `handoff:` and `plugin install`.'
    basis: '.andromeda/a11y-plan.md:209'
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    section: '§1 A11y Scope Summary — Critical paths, 6. CLI board, wait/last and verify output (Required ARIA roles)'
    change: 'Replace "the `--help` group labels stay owed to "CLI output discipline"" with: the `--help` group labels stand since chunk 2026-10-17-cli-output-discipline (board / traffic / wheel / setup, served verbs only, PROVISIONAL inputs#I1 Q3; `handoff:` owned by "Explicit session links").'
    sidecar: 'Critical path 6: --help group labels built (chunk 2026-10-17-cli-output-discipline).'
    rationale: 'Report Counts [owner-title] names a11y-plan.md:134 changed; Symbols / APIs `viola --help`.'
    basis: '.andromeda/a11y-plan.md:134'
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    section: '§8 Cognitive Accessibility — Plain language, Verification'
    change: 'Add `viola --help` beside `viola list` as a fixed CLI string pinned by literal, not trycmd: the grouped verb table is pinned by the `HELP` literal in `help_is_the_grouped_verb_table` and the served set by `help_lists_only_served_verbs` (`tests/cli_output_plain.rs`, chunk 2026-10-17-cli-output-discipline).'
    sidecar: '§8 Plain language: viola --help pinned by the HELP literal, not trycmd (chunk 2026-10-17-cli-output-discipline).'
    rationale: 'Report Spec claims disproved [trycmd] names a11y-plan.md:303 and states "`viola --help` is pinned by the `HELP` literal in `tests/cli_output_plain.rs` (`help_is_the_grouped_verb_table`)"; the new help surface''s pin must be stated as built.'
    basis: '.andromeda/a11y-plan.md:303'
    dependent-of: D-a11y-surface
note: 'D-a11y-obs-schema: no drift (report Schema / config: none; no log line, event or schema; structured-violation-json-schema key unchanged). The other [trycmd] hits in a11y-plan (:60, :91, :191, :192, :216, :222, :229, :232, :293, :309, :323, :358) and the key files a11y-testing-tool-pick.md:4 / keyboard-test-harness.md:3 are the pre-existing tooling claim the report sends to Validate (amend or carry); not proposed here. §1 :69 lists handoff among the --help group labels as the target set and is left standing (owned by "Explicit session links").'
