group: security-plan
read: .claude/docs/security-summary.md · 80 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/ci.md · 33 · «- 2026-10-10: In the operator pass, read the Windows-target »
read: .claude/rules/security.md · 47 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/docs/security-summary.md:73   class: omission
  leaf-text: «never the repository or an ancestor (`scratch-refused`, no fallback).»
  source: .andromeda/security-plan.md:607 «The wiped scratch is re-created owner-only through `viola_state::fs::create_private_dir` (the protected user + SYSTEM DACL on Windows)»
  replace-with: «never the repository or an ancestor (`scratch-refused`, no fallback); the wiped scratch is re-created owner-only through `viola_state::fs::create_private_dir` (the protected user + SYSTEM DACL on Windows), never a plain `create_dir_all`, so a test's instance tree inside it passes the strict-modes check, which is never loosened for it.»
