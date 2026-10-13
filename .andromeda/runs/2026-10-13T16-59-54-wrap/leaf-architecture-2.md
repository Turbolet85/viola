group: architecture-2
read: .claude/docs/services/viola-state.md · 49 · «- `.andromeda/architecture.md` §Standard Contracts (snapshot»
read: .claude/docs/services/viola-ui.md · 42 · «- `.claude/rules/api.md` · `.claude/rules/frontend.md` · `.c»
read: .claude/docs/services/viola.md · 58 · «- `.andromeda/architecture.md` · `.claude/rules/api.md` · `.»
read: .claude/docs/stack.md · 70 · «5. Commit as `chore: bump {tool} to {version}`»
read: .claude/docs/workflow.md · 67 · «This project uses a **reference-based** CLAUDE.md architectu»
read: .claude/rules/api.md · 45 · «_This section is owned by `/wrap-session`. setup-project pre»
- leaf: .claude/docs/services/viola-state.md:23   class: omission
  leaf-text: «any other error returns at once. `write_snapshot` and `replace_private_shared` inherit it.»
  source: .andromeda/architecture.md:51 «The loop sits in the private `replace_private_retrying`, which takes the decision to retry a failed rename (from its raw OS error) from its caller»
  replace-with: «any other error returns at once. The loop sits in the private `replace_private_retrying`, which takes the retry decision (from the raw OS error) from its caller; `replace_private_with` passes `retry_replace(_, HOST_IS_WINDOWS)`, so the product behaves as above and a unit case exercises the attempt bound on every host. `write_snapshot` and `replace_private_shared` inherit it.»
