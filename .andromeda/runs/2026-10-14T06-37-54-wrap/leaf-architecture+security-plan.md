group: architecture+security-plan
read: .claude/rules/api.md · 45 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/rules/api.md:31   class: omission
  leaf-text: «(`hook statusline` canonicalises its instance directory and runs `check_instance` before its log)»
  source: .andromeda/architecture.md:326 «With `--capture <DIR>` it takes `viola verify`'s probe capture instead, before the pass-through ([Hook Contract]): no instance check, no `budget.json` write, no command run, nothing printed»
  replace-with: «(`hook statusline` canonicalises its instance directory and runs `check_instance` before its log; its hidden `hook statusline --capture <DIR> [--shell <WORD>]` probe form, called only by Run B's probe settings, runs no instance check, reads no `VIOLA_*` and requires an absolute, existing `<DIR>` instead, its `--shell` word mapped only into the closed `ShellVerdict`)»
