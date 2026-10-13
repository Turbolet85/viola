group: architecture+security-plan
read: .claude/rules/api.md · 45 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/rules/api.md:30   class: omission
  leaf-text: «a `verify` failing row or refusal,»
  source: .andromeda/architecture.md:139 «`viola verify` stopped by Ctrl-C or TERM before its rows are checked (no stamp written, stdout empty, stderr exactly `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp»
  replace-with: «a `verify` failing row, refusal or Ctrl-C / TERM stop (no stamp written, `unable: verify was stopped before it finished`),»
