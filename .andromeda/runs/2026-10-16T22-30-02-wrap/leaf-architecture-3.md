group: architecture-3
read: .claude/rules/events.md · 39 · «_This section is owned by `/wrap-session`. setup-project pre»
- leaf: .claude/rules/events.md:29   class: omission
  leaf-text: «only the instance's wrapper writes `instances/<name>/snapshot.json`.»
  source: .andromeda/architecture.md:90 «so a session-id join survives a `/clear` because the wrapper rewrites the snapshot's `agent_session_id` on each `session-start`.»
  replace-with: «only the instance's wrapper writes `instances/<name>/snapshot.json`, and it rewrites the snapshot's `agent_session_id` on each `session-start`, so the `claude agents --json` session-id join survives a `/clear`.»
