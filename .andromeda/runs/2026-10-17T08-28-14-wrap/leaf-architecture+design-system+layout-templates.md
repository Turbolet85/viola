group: architecture+design-system+layout-templates
read: .claude/rules/frontend.md · 33 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/rules/frontend.md:28   class: stale
  leaf-text: «As built, `list` prints its uncoloured form on every stream, a TTY included (a wrapped row joined to one `claude agents --json` read by child pid or agent session id reads STATUS `idle` or `busy`, else `unknown`; each unjoined session follows under the `-- UNWRAPPED - READ-ONLY --` line, sorted by name, NAME its `sessionId`, LIVE `live`, `n/a` in WHEEL, DIALOG and CLI, with no separator when there is none and the board as with no `claude` on a failed read): the colour layer is owed to "CLI output discipline".»
  source: .andromeda/design-system.md:441 «On a terminal that takes colour (the decision order above) every NAME value is bold; the `DIALOG` word of a wrapped row with a pending dialog takes the attention token, its kind word plain;»
  replace-with: «As built (chunk 2026-10-17-cli-output-discipline), `list` is uncoloured under any rung above; on a terminal that takes colour every NAME value is bold, the `DIALOG` word of a wrapped row with a pending dialog takes the attention token (its kind word plain), a wrapped `stale` row is dim from its first byte to its last (NAME dim and bold; on a stale row with a dialog only `DIALOG` amber and not dim), the BAY line, caption, empty-board line and separator carry no SGR, the spans are laid over the escaped, padded cells (the plain board's columns, one write), and the depth is truecolor / 256 / 16 from `COLORTERM` / `TERM` (`COLUMNS`, `CLICOLOR`, `CLICOLOR_FORCE` and `CI` are not read). A wrapped row joined to one `claude agents --json` read by child pid or agent session id reads STATUS `idle` or `busy`, else `unknown`; each unjoined session follows under the `-- UNWRAPPED - READ-ONLY --` line, sorted by name, NAME its `sessionId`, LIVE `live`, `n/a` in WHEEL, DIALOG and CLI, with no separator when there is none and the board as with no `claude` on a failed read. With the terminal's width known (`viola_pty::host_size()`, never `COLUMNS`) only an unwrapped NAME is cut, to the column less 3 characters plus `...`, never inside an escape; a wrapped NAME is never cut, and a piped board or `--json` keeps every name whole.»

- leaf: .claude/rules/frontend.md:28   class: omission
  leaf-text: «`NO_COLOR` · `TERM=dumb`»
  source: .andromeda/design-system.md:404 «4. `NO_COLOR` set (present, an empty value included)»
  replace-with: «`NO_COLOR` (present, an empty value included) · `TERM=dumb`»

- leaf: .claude/rules/frontend.md:27   class: tree
  leaf-text: «## CLI surface (`src/cmd/**`)»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:23 «board.rs: the two state refusal pairs (`instance_state_pair`, `home_state_pair`) and the board writers (`BayBudget`, `bay_line`, `board_rows`, `write_board`: with a known terminal width the unwrapped NAME cut»
  replace-with: «## CLI surface (`src/cmd/**`; the human writers and the colour layer `style.rs` in `src/human/`)»

- leaf: .claude/rules/frontend.md:29   class: omission
  leaf-text: «no spinner, no redraw, no stack traces, no "done".»
  source: .andromeda/layout-templates.md:215 «**As built** (chunk 2026-10-17-cli-output-discipline): `viola --help` (and `viola help`) prints exactly `Usage: viola [--home <dir>] <verb> [args]`, a blank line, the four rows `board:    list` ·»
  replace-with: «no spinner, no redraw, no stack traces, no "done". `viola --help` (and `viola help`) prints exactly `Usage: viola [--home <dir>] <verb> [args]`, the served-verb rows `board:` · `traffic:` · `wheel:` · `setup:`, then `Options:` (no about line, no clap `Commands:` list; pinned by the `HELP` literal in `tests/cli_output_plain.rs`); the `handoff:` row and `plugin install` join when "Explicit session links" and "Optional plugin install" serve those verbs (PROVISIONAL, `inputs#I1` Q3); per-verb `--help` is clap's usage, unchanged.»
