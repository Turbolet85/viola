group: design-system+layout-templates
read: .claude/docs/design-summary.md · 73 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/»

- leaf: .claude/docs/design-summary.md:50   class: stale
  leaf-text: «as built: wrapped rows only, uncoloured on every stream, STATUS always `unknown`, and a failed strict-modes check refuses the whole board with exit 21»
  source: .andromeda/design-system.md:441 «A wrapped row joined to a `claude agents --json` row (by child pid or agent session id) reads STATUS `idle` or `busy`, else `unknown`. When the read lists a session no wrapped row joins, the `-- UNWRAPP»
  replace-with: «as built: uncoloured on every stream; the wrapped rows, then, when one `claude agents --json` read lists a session no wrapped row joins, the `-- UNWRAPPED - READ-ONLY --` line and one row per unwrapped session (NAME its `sessionId`, `n/a` in WHEEL, DIALOG and CLI); STATUS `idle` / `busy` on a row the read joins, else `unknown`; a failed read prints the board as with no `claude`; and a failed strict-modes check refuses the whole board with exit 21»
