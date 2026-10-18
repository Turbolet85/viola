group: a11y-plan
read: .claude/docs/a11y-summary.md · 58 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules: »
read: .claude/rules/a11y.md · 29 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/docs/a11y-summary.md:53   class: omission
  leaf-text: «never a person), no key is blocked, refused or delayed by it, and the wheel is untouched.»
  source: .andromeda/a11y-plan.md:285 «Once typed, a confirmed `viola send` waits for its confirmation within `CONFIRM_WINDOW_FALLBACK` (10 s), and a `/compact` send on a verified CLI within its own `COMPACT_CONFIRM_WINDOW` (40 s, PROVISIONAL»
  replace-with: «never a person), no key is blocked, refused or delayed by it, and the wheel is untouched. Once typed, a confirmed `viola send` waits for its confirmation within `CONFIRM_WINDOW_FALLBACK` (10 s), and a `/compact` send on a verified CLI within its own `COMPACT_CONFIRM_WINDOW` (40 s, PROVISIONAL, architecture [Delivery Confirmation]); a send not confirmed inside its window is `not-delivered` / `no-prompt-submitted`: a bound on the driver only, the confirm wait takes no key and blocks, refuses or delays none, and the wheel is untouched.»
