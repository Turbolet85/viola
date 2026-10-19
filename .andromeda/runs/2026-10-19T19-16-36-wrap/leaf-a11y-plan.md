group: a11y-plan
read: .claude/docs/a11y-summary.md · 58 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:»
read: .claude/rules/a11y.md · 29 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/docs/a11y-summary.md:47   class: omission
  leaf-text: «and never count focus / mouse / resize sequences or terminal replies as editing.»
  source: .andromeda/a11y-plan.md:362 «— a mouse report the Windows ConPTY already turned into win32 key-down records is typing (F-W3).»
  replace-with: «and never count focus / mouse / resize sequences or terminal replies as editing — a mouse report the Windows ConPTY already turned into win32 key-down records is typing (F-W3).»

- leaf: .claude/docs/a11y-summary.md:53   class: omission
  leaf-text: «Once typed, a confirmed `viola send` waits for its confirmation»
  source: .andromeda/a11y-plan.md:285 «A raised dialog takes a `viola answer` only within the dialog hold (`config.json`'s `dialog_hold_s`, whole seconds 60–3600, default 60, read once at the wrapper's start into `DialogHold`»
  replace-with: «A raised dialog takes a `viola answer` only within the dialog hold (`config.json`'s `dialog_hold_s`, whole seconds 60–3600, default 60, PROVISIONAL bounds; chunk 2026-10-19-dialog-hold-for-the-driver); at the hold's end the wrapper replies `null` and appends one `dialog-expired` line, the dialog renders for the human in the `claude` TUI, and a later `answer` for it is refused `not-delivered` / `unknown-dialog` (exit 13) — the hold bounds the driver, never the human: any human key still hands the held dialog back at once, whatever the hold. A `viola send` on a verified CLI waits in the readiness gate for at most `GATE_MAX_WAIT` (8.5 s) and is then refused `not-delivered` / `input-not-ready` with nothing typed; a key pressed during the wait goes to the child and takes the wheel at once. Once typed, a confirmed `viola send` waits for its confirmation»

- leaf: .claude/rules/a11y.md:3   class: tree
  leaf-text: «"crates/viola-ui/assets/**"»
  source: .andromeda/a11y-plan.md:232 «the `CAPTION` literal in the root case `list_human_board_is_the_literal_rows` (`tests/cli_list.rs`) and the unit case `board_row_caption_and_default_row_are_the_literals` (`src/human/board.rs`)»
  replace-with: «"src/human/**"»

- leaf: .claude/rules/a11y.md:4   class: tree
  leaf-text: «"e2e-web/fixtures/a11y.ts"»
  source: .andromeda/a11y-plan.md:232 «the `CAPTION` literal in the root case `list_human_board_is_the_literal_rows` (`tests/cli_list.rs`)»
  replace-with: «"tests/cli_*.rs"»

- leaf: .claude/rules/a11y.md:5   class: tree
  leaf-text: «"e2e-web/a11y/**"»
  source: .andromeda/a11y-plan.md:236 «held by the `board_row_*` units and by `tests/cross_list_rows.rs` in three modes»
  replace-with: «"tests/cross_list_rows.rs"»

- leaf: .claude/rules/a11y.md:6   class: tree
  leaf-text: «"e2e-web/.htmlvalidate.json"»
  source: .andromeda/registries/contracts/a11y-plan/keyboard-test-harness.md:3 «`passthrough_with_a_statusline_source_adds_no_viola_bytes` in `tests/tui_passthrough.rs`»
  replace-with: «"tests/tui_*.rs"»

- leaf: .claude/rules/a11y.md:7   class: tree
  leaf-text: «"e2e-web/eslint.config.js"»
  source: .andromeda/a11y-plan.md:234 «`viola wait` and `viola --help` carry no ESC byte and no lone CR on Unix (`tests/support/sgr.rs` `redraws`)»
  replace-with: «"tests/support/sgr.rs"»

- leaf: .claude/rules/a11y.md:12   class: retired
  leaf-text: «Path-scoped rules for the `viola ui` page and the a11y harness.»
  source: .andromeda/a11y-plan.md:23 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «Path-scoped rules for the cli output discipline, the tui boundary and the a11y harness.»

- leaf: .claude/rules/a11y.md:12   class: retired
  leaf-text: «the frontend (the `viola ui` page) is deferred to 0.2.0»
  source: .andromeda/a11y-plan.md:23 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «the frontend is deferred to 0.2.0»
