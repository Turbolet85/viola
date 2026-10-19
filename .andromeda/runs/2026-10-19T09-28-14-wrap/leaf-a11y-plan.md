group: a11y-plan
read: .claude/docs/a11y-summary.md · 58 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:»
read: .claude/rules/a11y.md · 29 · «_This section is owned by `/wrap-session`. setup-project pre»
- leaf: .claude/docs/a11y-summary.md:53   class: omission
  leaf-text: «the confirm wait takes no key and blocks, refuses or delays none, and the wheel is untouched.»
  source: .andromeda/a11y-plan.md:285 «the hold bounds the driver, never the human: any human key still hands the held dialog back at once, whatever the hold.»
  replace-with: «the confirm wait takes no key and blocks, refuses or delays none, and the wheel is untouched. A raised dialog takes a `viola answer` only within the dialog hold (`config.json`'s `dialog_hold_s`, whole seconds 60–3600, default 60, PROVISIONAL, any other value giving 60 and one `parse-rejected`; chunk 2026-10-19-dialog-hold-for-the-driver); at the hold's end the wrapper replies `null` and appends one `dialog-expired` line that wakes a parked `wait`, the dialog renders for the human in the `claude` TUI, and a later `answer` for it is refused `not-delivered` / `unknown-dialog` (exit 13): the hold bounds the driver, never the human, and any human key still hands the held dialog back at once, whatever the hold.»
