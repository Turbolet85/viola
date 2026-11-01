group: architecture-2
read: .claude/docs/services/viola-core.md · 43 · «- `.claude/docs/stack.md` · `.claude/docs/conventions.md` · »
read: .claude/docs/services/viola-mcp.md · 39 · «- `.andromeda/architecture.md` [MCP] · `.claude/rules/api.md»
read: .claude/docs/services/viola-pty.md · 47 · «- `.andromeda/architecture.md` [PTY] · `.claude/docs/gotchas»
read: .claude/docs/services/viola-state.md · 49 · «- `.andromeda/architecture.md` §Standard Contracts (snapshot»
- leaf: .claude/docs/services/viola-state.md:6   class: retired
  leaf-text: «`client::Unreachable::without_endpoint` and `viola-mcp`'s `frame` (PROVISIONAL)»
  source: .andromeda/architecture-detail/established-decisions.md:263 «It opens nothing, writes no line and stays behind the strict-modes check at every caller (`client::Unreachable::without_endpoint`, `viola mcp`'s `frame`).»
  replace-with: «`client::Unreachable::without_endpoint` and `viola-mcp`'s `frame`»
