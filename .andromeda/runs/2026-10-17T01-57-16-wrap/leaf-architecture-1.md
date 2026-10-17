group: architecture-1
read: .claude/docs/conventions.md · 61 · «- For warnings and gotchas, see `.claude/docs/gotchas.md`»
read: .claude/docs/gotchas.md · 147 · «- For architectural decisions and their rationale, see `.and»
read: .claude/docs/services/viola-agent-claude.md · 47 · «- `.andromeda/architecture.md` [CLI Version Compatibility] ·»
read: .claude/docs/services/viola-channel.md · 45 · «- `.andromeda/architecture.md` §Standard Contracts (Wrapper »
read: .claude/docs/services/viola-core.md · 43 · «- `.claude/docs/stack.md` · `.claude/docs/conventions.md` · »
read: .claude/docs/services/viola-mcp.md · 39 · «- `.andromeda/architecture.md` [MCP] · `.claude/rules/api.md»

- leaf: .claude/docs/conventions.md:13   class: omission
  leaf-text: «read by `viola-harness`, the root test chain `tests/support/home.rs` and/or viola-e2e's `tests/harness_lifecycle.rs` `Booted` guard)»
  source: .andromeda/architecture.md:328 «Read by `viola-harness`, by the root test chain (`tests/support/home.rs`) and by viola-e2e's `harness::session` guard (`HarnessSession`'s `Drop`, through `keep_home`), which keeps the session's home under `1`.»
  replace-with: «read by `viola-harness`, the root test chain `tests/support/home.rs` and/or viola-e2e's `tests/harness_lifecycle.rs` `Booted` guard and `harness::session` guard (`HarnessSession`'s `Drop`, through `keep_home`, which reads both `KEEP_*` variables)»
