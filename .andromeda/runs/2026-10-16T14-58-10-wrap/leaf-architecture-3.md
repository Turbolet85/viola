group: architecture-3
read: .claude/docs/workflow.md · 67 · «This project uses a **reference-based** CLAUDE.md architectu»
read: .claude/rules/events.md · 39 · «_This section is owned by `/wrap-session`. setup-project pre»
- leaf: .claude/rules/events.md:25   class: retired
  leaf-text: «- `wait after`, the `send` `cursor` and SSE ids are byte offsets into the instance's `events.ndjson`»
  source: .andromeda/architecture.md:344 «v1 never rotates or truncates `events.ndjson`, because `wait`'s `after` and `send`'s `cursor` are byte offsets into it.»
  replace-with: «- `wait after` and the `send` `cursor` are byte offsets into the instance's `events.ndjson`»
