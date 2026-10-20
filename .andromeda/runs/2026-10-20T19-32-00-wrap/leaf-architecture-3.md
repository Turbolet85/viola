group: architecture-3
read: .claude/docs/stack.md · 66 · «5. Commit as `chore: bump {tool} to {version}`»
read: .claude/docs/workflow.md · 67 · «This project uses a **reference-based** CLAUDE.md architectu»
read: .claude/rules/events.md · 40 · «- 2026-10-16: A snapshot field that a reader expects once an»
read: .claude/rules/frontend.md · 29 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/docs/workflow.md:49   class: omission
  leaf-text: «`viola verify` / `verify --record` against it)»
  source: .andromeda/architecture.md:375 «The `Mcp.<variant>.json` class is recorded not by `--record` but by the test-only `viola-mcp-tap` (Binary, above): a live CLI's MCP exchange through the same scrub and refusal»
  replace-with: «`viola verify` / `verify --record` against it, and the test-only `viola-mcp-tap` recording a live CLI's MCP exchange for the `Mcp.` fixtures)»
