group: architecture+security-plan
read: .claude/rules/api.md · 41 · «_This section is owned by `/wrap-session`. setup-project pre»
- leaf: .claude/rules/api.md:35   class: omission
  leaf-text: «`tools/list` carries `ttlMs` 0 and `cacheScope` `private` (claude 2.1.287 drops every tool without them);»
  source: .andromeda/architecture.md:152 «and sends no `initialize` or `notifications/initialized`; viola's two results each carry `ttlMs` 0 and `cacheScope` `private`»
  replace-with: «`tools/list` carries `ttlMs` 0 and `cacheScope` `private` (claude 2.1.287 drops every tool without them); claude 2.1.302 opens with `server/discover` (its JSON-RPC id a string) and then `tools/list`, sending no `initialize` or `notifications/initialized`, and both results carry `ttlMs` 0 and `cacheScope` `private` (replayed from `fixtures/claude/2.1.302/Mcp.tools-list-1.json` by `tests/mcp_recorded_exchange.rs`);»
