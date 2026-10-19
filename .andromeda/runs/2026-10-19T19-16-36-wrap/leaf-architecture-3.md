group: architecture-3
read: .claude/docs/stack.md · 66 · «5. Commit as `chore: bump {tool} to {version}`»
read: .claude/docs/workflow.md · 67 · «This project uses a **reference-based** CLAUDE.md architectu»
read: .claude/rules/events.md · 40 · «- 2026-10-16: A snapshot field that a reader expects once an»

- leaf: .claude/rules/events.md:20   class: stale
  leaf-text: «Stop → `turn-ended` (`last_assistant_message` or `null`)»
  source: .andromeda/architecture.md:255 «`turn-ended`: `{last_assistant_message, background_tasks}`. `last_assistant_message` is a string or `null`, and is the source `last` rebuilds from.»
  replace-with: «Stop → `turn-ended` (`last_assistant_message` or `null`; `background_tasks`, the Stop payload's `background_tasks` array length, a count only, or `null` when the key is absent, `null` or not an array)»

- leaf: .claude/docs/stack.md:23   class: omission
  leaf-text: «`viola-agent-claude`'s pre-existing `Refusal` beside `AgentError` is an open divergence»
  source: .andromeda/architecture.md:153 «`viola-agent-claude`'s pre-existing `Refusal` beside `AgentError` and `viola-mcp`'s `ServeError` beside `McpError` are the two open divergences named there»
  replace-with: «`viola-agent-claude`'s pre-existing `Refusal` beside `AgentError` and `viola-mcp`'s `ServeError` beside `McpError` are the two open divergences»
