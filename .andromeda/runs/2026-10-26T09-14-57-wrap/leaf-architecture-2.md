group: architecture-2
read: .claude/docs/services/viola-mcp.md · 39 · «- `.andromeda/architecture.md` [MCP] · `.claude/rules/api.md»
read: .claude/docs/services/viola-pty.md · 47 · «- `.andromeda/architecture.md` [PTY] · `.claude/docs/gotchas»
read: .claude/docs/services/viola-state.md · 49 · «- `.andromeda/architecture.md` §Standard Contracts (snapshot»
read: .claude/docs/services/viola.md · 58 · «- `.andromeda/architecture.md` · `.claude/rules/api.md` · `.»

- leaf: .claude/docs/services/viola.md:45   class: omission
  leaf-text: «exit 20 / exit 21 — the latter at WARN, a failed server check logged `server-verify-failed` with `during:"connect"` and no `channel-request` line»
  source: .andromeda/architecture-detail/conventions.md:40 «the `unable  <name>  instance-unreachable` line with the server-verify hint last on stderr (its wording: design-system, surface cli-send)»
  replace-with: «exit 20 / exit 21 — the latter at WARN, a failed server check logged `server-verify-failed` with `during:"connect"` and no `channel-request` line, and printed as the `unable  <name>  instance-unreachable` line with the server-verify hint last on stderr, or `{"v":1,"error":"instance-unreachable","detail":null}` under `--json` with stderr empty»

- leaf: .claude/docs/services/viola-state.md:28   class: omission
  leaf-text: «whose two product callers are `viola revive`'s preflight and `viola list`'s `board::read`;»
  source: .andromeda/architecture-detail/established-decisions.md:79 «A verified client meets an unreadable or newer snapshot as an absent one: every reader stays on `read_snapshot` (§Standard Contracts, Snapshot envelope)»
  replace-with: «whose two product callers are `viola revive`'s preflight and `viola list`'s `board::read` (every other reader, `read_snapshot`'s three product callers and `liveness::live_endpoint` for the CLI frame verbs and `viola mcp`, reads such a snapshot as absent);»
