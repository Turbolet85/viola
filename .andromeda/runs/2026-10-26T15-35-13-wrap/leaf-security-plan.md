group: security-plan
read: .claude/docs/security-summary.md · 77 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 44 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/docs/security-summary.md:15   class: stale
  leaf-text: «Interim, ratified by the founder (Decisions Log `2026-09-28` and the 2026-10-04 / 2026-10-17 rulings): the snapshot is read without strict-modes, owed to "Home and code-bearing file integrity";»
  source: .andromeda/security-plan.md:114 «a dated gap owed to "Home and code-bearing file integrity", except for CLI `link` / `unlink`, which read it only after the instance's strict-modes check and take no gap»
  replace-with: «Interim, ratified by the founder (Decisions Log `2026-09-28` and the 2026-10-04 / 2026-10-17 rulings): the snapshot is read without strict-modes, owed to "Home and code-bearing file integrity", except by CLI `link` / `unlink`, which read it only after `check_instance` passed on the driven instance and take no gap;»
