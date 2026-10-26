group: architecture+design-system+layout-templates
read: .claude/rules/frontend.md · 29 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/rules/frontend.md:25   class: stale
  leaf-text: «the served-verb rows `board:` · `traffic:` · `wheel:` · `setup:` (`setup:` holding `plugin install`), then `Options:` (no about line, no clap `Commands:` list; pinned by the `HELP` literal in `tests/cli_output_plain.rs`); the `handoff:` row joins when "Explicit session links" serves `link` and `unlink`»
  source: .andromeda/design-system.md:350 «`viola --help` lists the served verbs as `board: list` / `traffic: send, wait, last, answer` / `wheel: pause, release` / `handoff: link, unlink` / `setup: run, revive, verify, plugin install`, then `Options:`.»
  replace-with: «the served-verb rows `board:` · `traffic:` · `wheel:` · `handoff:` · `setup:` (`handoff:` holding `link, unlink`, `setup:` holding `plugin install`), then `Options:` (no about line, no clap `Commands:` list; pinned by the `HELP` literal in `tests/cli_output_plain.rs`)»

- leaf: .claude/rules/frontend.md:25   class: omission
  leaf-text: «`list`'s exit-21 strict-modes refusal and `plugin install`'s exit-21 strict-modes refusal (the home pair, stdout empty; under `--json` the `instance-unreachable` document, stderr empty) take the fixed form `unable: <text>`»
  source: .andromeda/design-system.md:261 «or the fixed `unable: <text>` form for the exit-1 refusals of `run`, `revive` and `verify` and the exit-21 strict-modes refusals of `list`, `link` and `unlink`»
  replace-with: «`list`'s exit-21 strict-modes refusal, `plugin install`'s exit-21 strict-modes refusal (the home pair, stdout empty; under `--json` the `instance-unreachable` document, stderr empty) and `link` / `unlink`'s exit-21 strict-modes refusal of the driven instance (`revive`'s instance pair naming it, stdout empty; under `--json` the `instance-unreachable` document, stderr empty) take the fixed form `unable: <text>`»

- leaf: .claude/rules/frontend.md:26   class: omission
  leaf-text: «`wait`/`last` text escapes C0, DEL and C1 the same way but keeps `\n`/`\t`»
  source: .andromeda/design-system.md:315 «Message mode, the escaper `wait` and `last` share and `viola link` uses for a `since` that is no RFC 3339 instant: over decoded characters every control character (C0, DEL, C1) except `\n` and `\t`»
  replace-with: «`wait`/`last` text, and a `link` line's `since` that is no RFC 3339 instant, escape C0, DEL and C1 the same way but keep `\n`/`\t`»
