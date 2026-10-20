group: security-plan
read: .claude/docs/security-summary.md · 77 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/ci.md · 35 · «- 2026-10-13: A CI job that reads `cancelled` with no steps»
read: .claude/rules/security.md · 44 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/rules/security.md:17   class: omission
  leaf-text: «The `--record` refusal names the fixture file and a closed code (`home-path` · `absolute-path` · `username` · `email`), never the content;»
  source: .andromeda/security-plan.md:258 «applies the same scrub, and refuses through `ledger::screen_fault` with the file name and the same closed codes, never the content»
  replace-with: «The `--record` refusal names the fixture file and a closed code (`home-path` · `absolute-path` · `username` · `email`), never the content, and so does the test-only `viola-mcp-tap`'s `fixture` step for the `Mcp.<variant>.json` class (the same scrub, through `ledger::screen_fault`; its `record` capture a 0600 file in a private directory, never committed, its `viola mcp` a direct spawn with no shell and no PATH lookup);»

- leaf: .claude/docs/security-summary.md:72   class: omission
  leaf-text: «the `--record` refusal names file + closed code (`home-path` · `absolute-path` · `username` · `email`), never content (founder-ratified live at the chunk's wrap);»
  source: .andromeda/security-plan.md:258 «applies the same scrub, and refuses through `ledger::screen_fault` with the file name and the same closed codes, never the content»
  replace-with: «the `--record` refusal names file + closed code (`home-path` · `absolute-path` · `username` · `email`), never content (founder-ratified live at the chunk's wrap), and the test-only `viola-mcp-tap`'s `fixture` step refuses an `Mcp.<variant>.json` exchange the same way after the same scrub (its `record` capture 0600 in a private directory, never committed);»
