group: a11y-plan
read: .claude/docs/a11y-summary.md · 58 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:»
read: .claude/rules/a11y.md · 29 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/docs/a11y-summary.md:29   class: stale
  leaf-text: «(the `--help` group labels stay owed)»
  source: .andromeda/a11y-plan.md:134 «the `--help` group labels stand since chunk 2026-10-17-cli-output-discipline (board / traffic / wheel / setup over the served verbs, `inputs#I1` Q3 of that chunk, PROVISIONAL;»
  replace-with: «(the `--help` group labels stand too: board / traffic / wheel / setup over the served verbs, PROVISIONAL; the `handoff:` row joins with "Explicit session links")»

- leaf: .claude/rules/a11y.md:21   class: stale
  leaf-text: «the unwrapped rows' `n/a` cells, never truncated)»
  source: .andromeda/a11y-plan.md:258 «On a terminal too narrow, an unwrapped NAME is cut to the column less 3 characters plus `...`, never inside a `\xHH` escape»
  replace-with: «the unwrapped rows' `n/a` cells; on a terminal too narrow only an unwrapped NAME is cut, to the column less 3 characters plus `...` and never inside a `\xHH` escape, a wrapped NAME never, and piped output and `--json` keep every name whole)»

- leaf: .claude/rules/a11y.md:21   class: omission
  leaf-text: «viola emits no SGR of its own under non-TTY / `NO_COLOR` / `TERM=dumb`»
  source: .andromeda/a11y-plan.md:73 «and no SGR under `--json`, a pipe, `NO_COLOR` (an empty value included) or `TERM=dumb`;»
  replace-with: «viola emits no SGR of its own under non-TTY / `NO_COLOR` (an empty value included) / `TERM=dumb`»
