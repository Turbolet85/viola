group: architecture-1
read: .claude/docs/conventions.md · 61 · «- For warnings and gotchas, see `.claude/docs/gotchas.md`»
read: .claude/docs/gotchas.md · 147 · «- For architectural decisions and their rationale, see `.and»
read: .claude/docs/services/viola-agent-claude.md · 47 · «- `.andromeda/architecture.md` [CLI Version Compatibility] ·»
read: .claude/docs/services/viola-channel.md · 45 · «- `.andromeda/architecture.md` §Standard Contracts (Wrapper »
read: .claude/docs/services/viola-core.md · 43 · «- `.claude/docs/stack.md` · `.claude/docs/conventions.md` · »
read: .claude/docs/services/viola-mcp.md · 39 · «- `.andromeda/architecture.md` [MCP] · `.claude/rules/api.md»
read: .claude/docs/services/viola-pty.md · 47 · «- `.andromeda/architecture.md` [PTY] · `.claude/docs/gotchas»

- leaf: .claude/docs/services/viola-agent-claude.md:14   class: omission
  leaf-text: «`viola verify` (probes, fixture recorder)»
  source: .andromeda/architecture.md:371 «`viola verify`'s Runs C and D read the same constant, and verify's default program reads `viola_agent_claude::PROGRAM` (both moved at chunk 2026-10-13-interrupted-verify-cleanup).»
  replace-with: «`viola verify` (probes, fixture recorder, `PROGRAM` as its default program, `statusline::SETTINGS_FLAG` for Runs C and D)»
