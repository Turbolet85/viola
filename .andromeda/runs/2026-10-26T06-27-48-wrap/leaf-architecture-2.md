group: architecture-2
read: .claude/docs/services/viola-mcp.md · 39 · «- `.andromeda/architecture.md` [MCP] · `.claude/rules/api.md»
read: .claude/docs/services/viola-pty.md · 47 · «- `.andromeda/architecture.md` [PTY] · `.claude/docs/gotchas»
read: .claude/docs/services/viola-state.md · 49 · «- `.andromeda/architecture.md` §Standard Contracts (snapshot»
read: .claude/docs/services/viola.md · 58 · «- `.andromeda/architecture.md` · `.claude/rules/api.md` · `.»

- leaf: .claude/docs/services/viola-mcp.md:11   class: stale
  leaf-text: «tokio 1.53.1 (`rt`)»
  source: .andromeda/architecture.md:442 «A sync std-thread core; Tokio 1.53.3 only in `mcp`; rmcp 3.4.1 for MCP; clap 4.6.7 for the CLI.»
  replace-with: «tokio 1.53.3 (`rt`)»
