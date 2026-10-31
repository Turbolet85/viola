group: architecture-3
read: .claude/docs/services/viola.md · 58 · «- `.andromeda/architecture.md` · `.claude/rules/api.md` · `.»
read: .claude/docs/stack.md · 66 · «5. Commit as `chore: bump {tool} to {version}`»
read: .claude/docs/workflow.md · 67 · «This project uses a **reference-based** CLAUDE.md architectu»
- leaf: .claude/docs/services/viola.md:45   class: stale
  leaf-text: «exit 0 once initialized, 11 before `initialize`»
  source: .andromeda/architecture-detail/occupied-resources.md:29 «which ends the session as a close does (exit 0 once initialized, 1 before `initialize`, both streams empty)»
  replace-with: «exit 0 once initialized, 1 before `initialize`, both streams empty»
- leaf: .claude/docs/services/viola.md:45   class: omission
  leaf-text: «the reply shapes, exit 20 / exit 21»
  source: .andromeda/architecture.md:227 «(exit 20): `message` a fixed text and `data` a closed object or null, read through `viola_channel::sanitised_fault`»
  replace-with: «the reply shapes (a `wrapper-fault`'s `message` a fixed text and `data` a closed object or null, read through `viola_channel::sanitised_fault`; a fault code outside the five the fixed `wrapper fault`, its human line `error: wrapper fault  <code>`; a `refusal` that is no code word read as `unknown`, a string `detail` that is no code word as `null`, a non-string `detail` as no known reply), exit 20 / exit 21»
- leaf: .claude/docs/stack.md:23   class: stale
  leaf-text: «`viola-agent-claude`'s pre-existing `Refusal` beside `AgentError`, `viola-mcp`'s `ServeError` beside `McpError` and `viola-state`'s `StampsError` beside `StateError` are the three open divergences»
  source: .andromeda/architecture.md:106 «`viola-mcp`'s `ServeError` beside `McpError` and `viola-state`'s `StampsError` beside `StateError` are the two open divergences named there, with `PinError`, `StampsReadError` and `Refused` the three»
  replace-with: «`viola-mcp`'s `ServeError` beside `McpError` and `viola-state`'s `StampsError` beside `StateError` are the two open divergences, with `PinError`, `StampsReadError` and `Refused` the three the census adds»
