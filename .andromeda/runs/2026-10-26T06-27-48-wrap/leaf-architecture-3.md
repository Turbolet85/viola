group: architecture-3
read: .claude/docs/stack.md · 66 · «5. Commit as `chore: bump {tool} to {version}`»
read: .claude/docs/workflow.md · 67 · «This project uses a **reference-based** CLAUDE.md architectu»
read: .claude/rules/events.md · 40 · «- 2026-10-16: A snapshot field that a reader expects once an»
read: .claude/rules/frontend.md · 29 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/docs/stack.md:10   class: stale
  leaf-text: «Tokio 1.53.1 built only inside `mcp`»
  source: .andromeda/architecture-detail/stack-and-technologies.md:11 «Hybrid: std threads for `run`, `hook`, `send`, `wait` and every other CLI verb; Tokio 1.53.3 built only inside `mcp`»
  replace-with: «Tokio 1.53.3 built only inside `mcp`»

- leaf: .claude/docs/stack.md:47   class: stale
  leaf-text: «tokio 1.53.1 (`rt` + `process`»
  source: .andromeda/architecture.md:442 «A sync std-thread core; Tokio 1.53.3 only in `mcp`; rmcp 3.4.1 for MCP; clap 4.6.7 for the CLI.»
  replace-with: «tokio 1.53.3 (`rt` + `process`»
