group: architecture-3
read: .claude/docs/stack.md · 66 · «5. Commit as `chore: bump {tool} to {version}`»
read: .claude/docs/workflow.md · 67 · «This project uses a **reference-based** CLAUDE.md architectu»
read: .claude/rules/events.md · 40 · «- 2026-10-16: A snapshot field that a reader expects once an»

- leaf: .claude/docs/stack.md:23   class: stale
  leaf-text: «`viola-agent-claude`'s pre-existing `Refusal` beside `AgentError` and `viola-mcp`'s `ServeError` beside `McpError` are the two open divergences»
  source: .andromeda/architecture.md:106 «`viola-mcp`'s `ServeError` beside `McpError` and `viola-state`'s `StampsError` beside `StateError` are the three open divergences named there»
  replace-with: «`viola-agent-claude`'s pre-existing `Refusal` beside `AgentError`, `viola-mcp`'s `ServeError` beside `McpError` and `viola-state`'s `StampsError` beside `StateError` are the three open divergences»

- leaf: .claude/rules/events.md:30   class: stale
  leaf-text: «with two product readers, `viola revive`'s preflight and `viola list`'s `viola_state::board::read` (a replayed snapshot gives a `live` row only for a beat at most `STALE_AFTER` old, never `stale`); the other snapshot readers still call `read_snapshot`: its three product callers (`src/cmd/hook.rs` once, the one read its event and dialog paths share; `run`'s collision check; `hook statusline`) and `viola_state::liveness::live_endpoint`, inside which the CLI frame verbs and `viola mcp` read it.»
  source: .andromeda/architecture.md:153 «The replay's product readers are `viola revive`'s preflight (one read for both its collision check and its recorded directory), `viola list`, `viola_state::liveness::live_endpoint`»
  replace-with: «with six product readers, each after the instance's strict-modes check has passed: `viola revive`'s preflight (one read for both its collision check and its recorded directory), `viola list`'s `viola_state::board::read` (a replayed snapshot gives a `live` row only for a beat at most `STALE_AFTER` old, never `stale`), `viola_state::liveness::live_endpoint` (the CLI frame verbs, `viola link` / `viola unlink` and `viola mcp`'s frame tools; only a real snapshot gives an endpoint), `viola hook`'s `connect` (a replayed or absent snapshot is unreachable, so the hook fails open), `hook statusline`'s `serve` (a replayed snapshot holds no `statusline_command`, so nothing runs) and `run`'s collision check (a replayed snapshot judged by the heartbeat alone: `already-live` within `STALE_AFTER`, else free); no product code under `src/` calls `read_snapshot`.»

- leaf: .claude/rules/events.md:31   class: stale
  leaf-text: «returns them with the endpoint from one `read_snapshot` (`LiveEndpoint`)»
  source: .andromeda/architecture-detail/standard-contracts.md:127 «all from its one `replay::read_snapshot_or_replay` call and only on a real snapshot (a replayed or absent one, or a failed read, gives `None`)»
  replace-with: «returns them with the endpoint from one `replay::read_snapshot_or_replay` (`LiveEndpoint`, only on a real snapshot: a replayed or absent one, or a failed read, gives none)»

- leaf: .claude/rules/events.md:31   class: omission
  leaf-text: «takes its snapshot read first (the CLI frame verbs and the MCP tools the liveness-only pre-check through `live_endpoint`, `viola hook`'s event and dialog paths one shared `read_snapshot`, then `Client::connect_by`)»
  source: .andromeda/architecture-detail/standard-contracts.md:63 «Before that comparison each client runs the instance's strict-modes check, and a refused check writes no frame and no line: `client::refuse_untrusted` for `send`, `wait`, `last`, `answer`, `pause`»
  replace-with: «runs the instance's strict-modes check first (`client::refuse_untrusted` for the CLI verbs, the hook's `trusted` for `hook.event` / `hook.dialog`, `strict::check_instance` in the MCP frame tools; a refused check writes no frame and no line), then takes its snapshot read (the CLI frame verbs and the MCP tools the liveness-only pre-check through `live_endpoint`, `viola hook`'s event and dialog paths one shared `replay::read_snapshot_or_replay`, a replayed or absent snapshot read as unreachable, then `Client::connect_by`)»
