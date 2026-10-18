group: architecture+design-system+layout-templates
read: .claude/rules/frontend.md · 33 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/rules/frontend.md:29   class: stale
  leaf-text: «the served-verb rows `board:` · `traffic:` · `wheel:` · `setup:`, then `Options:` (no about line, no clap `Commands:` list; pinned by the `HELP` literal in `tests/cli_output_plain.rs`); the `handoff:` row and `plugin install` join when "Explicit session links" and "Optional plugin install" serve those verbs (PROVISIONAL, `inputs#I1` Q3)»
  source: .andromeda/layout-templates.md:217 «the four rows `board:    list` · `traffic:  send, wait, last, answer` · `wheel:    pause, release` · `setup:    run, revive, verify, plugin install`, a blank line, `Options:`»
  replace-with: «the served-verb rows `board:` · `traffic:` · `wheel:` · `setup:` (`setup:` holding `plugin install`), then `Options:` (no about line, no clap `Commands:` list; pinned by the `HELP` literal in `tests/cli_output_plain.rs`); the `handoff:` row joins when "Explicit session links" serves `link` and `unlink` (PROVISIONAL, `inputs#I1` Q3)»

- leaf: .claude/rules/frontend.md:29   class: omission
  leaf-text: «and `list`'s exit-21 strict-modes refusal take the fixed form `unable: <text>`»
  source: .andromeda/design-system.md:524 «`viola plugin install`'s refused strict-modes check (exit 21) prints the home-state `unable:` / `hint:` pair as `viola list`'s does, or under `--json` the `instance-unreachable` document, stderr empty.»
  replace-with: «`list`'s exit-21 strict-modes refusal and `plugin install`'s exit-21 strict-modes refusal (the home pair, stdout empty; under `--json` the `instance-unreachable` document, stderr empty) take the fixed form `unable: <text>`»
