group: architecture+design-system+layout-templates
read: .claude/rules/frontend.md · 33 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/rules/frontend.md:28   class: stale
  leaf-text: «(wrapped rows only, STATUS always `unknown`): the colour layer is owed to "CLI output discipline", the unwrapped rack to "Unwrapped sessions on the board".»
  source: .andromeda/design-system.md:441 «A wrapped row joined to a `claude agents --json` row (by child pid or agent session id) reads STATUS `idle` or `busy`, else `unknown`.»
  replace-with: «(a wrapped row joined to one `claude agents --json` read by child pid or agent session id reads STATUS `idle` or `busy`, else `unknown`; each unjoined session follows under the `-- UNWRAPPED - READ-ONLY --` line, sorted by name, NAME its `sessionId`, LIVE `live`, `n/a` in WHEEL, DIALOG and CLI, with no separator when there is none and the board as with no `claude` on a failed read): the colour layer is owed to "CLI output discipline".»
