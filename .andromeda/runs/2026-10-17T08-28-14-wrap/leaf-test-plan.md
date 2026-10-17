group: test-plan
read: .claude/docs/tests-summary.md · 65 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 98 · «- 2026-10-17: Under a parsed `VIOLA_NAME`, `viola list` (and»
read: .claude/rules/verification-harness.md · 71 · «- 2026-10-14: A side effect added to a fake-agent turn (a sp»

- leaf: .claude/rules/testing.md:19   class: omission
  leaf-text: «the one complete-lines reader in `ndjson.rs` (every reader of a live-appended file parses only newline-terminated lines), and a root test file defines no copy of its own)»
  source: .andromeda/test-plan.md:589 «The SGR parser `sgr_attributes`, with `redraws` and `sets_a_foreground`, lives in `tests/support/sgr.rs`; its self-check case stays in `tests/cli_output_plain.rs`.»
  replace-with: «the one complete-lines reader in `ndjson.rs` (every reader of a live-appended file parses only newline-terminated lines), the SGR parser `sgr_attributes` (with `redraws` and `sets_a_foreground`) in `sgr.rs`, its self-check case staying in `tests/cli_output_plain.rs`, and a root test file defines no copy of its own)»
