group: architecture+security-plan+design-system+layout-templates
read: .claude/rules/frontend.md · 42 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/rules/frontend.md:38   class: omission
  leaf-text: «the exit-1 refusals of `run` / `revive` / `verify` and `list`'s exit-21 strict-modes refusal take the fixed form `unable: <text>`»
  source: .andromeda/design-system.md:826 «or a `viola verify` stopped by Ctrl-C or TERM before its rows are checked, `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI` (stdout empty)»
  replace-with: «the exit-1 refusals of `run` / `revive` / `verify`, a `verify` stopped by Ctrl-C or TERM before its rows are checked (stdout empty, no summary), and `list`'s exit-21 strict-modes refusal take the fixed form `unable: <text>`»
