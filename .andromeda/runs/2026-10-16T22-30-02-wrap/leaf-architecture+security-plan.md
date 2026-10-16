group: architecture+security-plan
read: .claude/rules/api.md · 42 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/rules/api.md:21   class: stale
  leaf-text: «As landed CLI `list` reads `<home>/instances/` and `budget.json` only and spawns no `claude agents --json`: every item is `wrapped: true` with `status: "unknown"` and no unwrapped row is listed (owed to "Unwrapped sessions on the board").»
  source: .andromeda/architecture.md:237 «Since chunk 2026-10-16-unwrapped-sessions-on-the-board it then makes one `claude agents --json` read (`run::agents::read_here`, `claude` resolved by `run::resolve_program` from its current directory)»
  replace-with: «As landed CLI `list` reads `<home>/instances/` and `budget.json`, then makes one `claude agents --json` read joined by `viola_state::board::join`: a row matching a wrapped item's `child_pid` or `agent_session_id` sets that item's `status` to `idle` or `busy`, and every unjoined row is an unwrapped item (`wrapped: false`, its name the row's `sessionId`, never a `ViolaName` or a valid `target`); a failed, oversize or malformed read leaves every `status` `unknown` and no unwrapped item. `mcp`'s `list` is not built.»
