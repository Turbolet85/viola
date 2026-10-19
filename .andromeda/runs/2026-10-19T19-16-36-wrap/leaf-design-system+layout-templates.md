group: design-system+layout-templates
read: .claude/docs/design-summary.md · 73 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/»
- leaf: .claude/docs/design-summary.md:50   class: omission
  leaf-text: «`viola verify` step counter, the `viola run`»
  source: .andromeda/layout-templates.md:146 «A `wait` woken by `dialog-expired` (the wrapper's record of an expired hold, chunk 2026-10-19-dialog-hold-for-the-driver) prints `dialog-expired  <name>  dialog <id>  cursor <n>`, ASCII, no SGR, written once»
  replace-with: «`viola wait` / `viola last` (one static result line on stdout, no spinner; a `wait` woken by an expired dialog hold prints `dialog-expired  <name>  dialog <id>  cursor <n>`, a planning form, the founder's wording owed), `viola verify` step counter, the `viola run`»
