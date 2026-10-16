group: a11y-plan
read: .claude/docs/a11y-summary.md · 58 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:»
read: .claude/rules/a11y.md · 29 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/rules/a11y.md:21   class: omission
  leaf-text: «`expired`, `budget-paused`, `unverified-cli`); colour»
  source: .andromeda/a11y-plan.md:258 «and since chunk 2026-10-16-unwrapped-sessions-on-the-board the STATUS words `idle` / `busy` on joined wrapped rows and the unwrapped rack's `n/a` cells, with no truncation»
  replace-with: «`expired`, `budget-paused`, `unverified-cli`, the STATUS words `idle` / `busy` on joined wrapped rows and the unwrapped rows' `n/a` cells, never truncated); colour»

- leaf: .claude/docs/a11y-summary.md:29   class: omission
  leaf-text: «`viola list`'s `CAPTION` literal pins the caption order the deferred 0.2.0 page must share»
  source: .andromeda/a11y-plan.md:134 «As built the caption line and the `-- UNWRAPPED - READ-ONLY --` separator stand (the separator since chunk 2026-10-16-unwrapped-sessions-on-the-board, printed only when unwrapped rows exist)»
  replace-with: «`viola list`'s `CAPTION` literal pins the caption order the deferred 0.2.0 page must share; the `-- UNWRAPPED - READ-ONLY --` separator stands too, printed only when unwrapped rows exist (the `--help` group labels stay owed)»

- leaf: .claude/docs/a11y-summary.md:53   class: omission
  leaf-text: «falls on the user's own command, never on a keystroke or the human.»
  source: .andromeda/a11y-plan.md:285 «`viola list` and `viola revive` each make one `claude agents --json` read (a direct spawn, stdout through `take(MAX_FRAME + 1)`, killed at 5 s; chunk 2026-10-16-unwrapped-sessions-on-the-board)»
  replace-with: «falls on the user's own command, never on a keystroke or the human. The one `claude agents --json` read `viola list` and `viola revive` each make (chunk 2026-10-16-unwrapped-sessions-on-the-board) is killed at 5 s, a bound on viola's own child only: a failed, oversize, malformed or timed-out read fails open, and no key is blocked, refused or delayed by it.»
