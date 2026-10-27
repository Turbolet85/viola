group: architecture-1
read: .claude/docs/conventions.md · 61 · «- For warnings and gotchas, see `.claude/docs/gotchas.md`»
read: .claude/docs/gotchas.md · 147 · «- For architectural decisions and their rationale, see `.and»
read: .claude/docs/services/viola-agent-claude.md · 47 · «- `.andromeda/architecture.md` [CLI Version Compatibility] ·»
read: .claude/docs/services/viola-channel.md · 45 · «- `.andromeda/architecture.md` §Standard Contracts (Wrapper »
read: .claude/docs/services/viola-core.md · 43 · «- `.claude/docs/stack.md` · `.claude/docs/conventions.md` ·»

- leaf: .claude/docs/conventions.md:15   class: stale
  leaf-text: «Two open divergences, not exceptions: the crate's pre-existing `Refusal` (`resolve_program`) beside `AgentError`, and `viola-mcp`'s `ServeError` (`Runtime`, `Initialize`, `Session`; fixed messages, returned by `serve`) beside `McpError` (`Internal`, Display `internal error`)»
  source: .andromeda/architecture.md:106 «`viola-mcp`'s `ServeError` beside `McpError` and `viola-state`'s `StampsError` beside `StateError` are the three open divergences named there»
  replace-with: «Three open divergences, not exceptions: the crate's pre-existing `Refusal` (`resolve_program`) beside `AgentError`, `viola-mcp`'s `ServeError` (`Runtime`, `Initialize`, `Session`; fixed messages, returned by `serve`) beside `McpError` (`Internal`, Display `internal error`), and `viola-state`'s `StampsError` (`StrictModes`, `State`; fixed messages, returned by `update_stamps`) beside `StateError`»

- leaf: .claude/docs/conventions.md:33   class: omission
  leaf-text: «CLI exit codes: 0 · 1 · 2 ·»
  source: .andromeda/architecture.md:100 «`1` internal error, and the start, revive-preflight and verify refusals (a refused home-tree check of `run`, `viola revive`'s start or `verify` among them)»
  replace-with: «CLI exit codes: 0 · 1 (also `viola run`'s and `viola revive`'s start, and `viola verify`, refusing the home tree before any log opens: the home-state `unable:` / `hint:` pair, or the instance-state pair for an instance-level refusal, no line written, no child started) · 2 ·»

- leaf: .claude/docs/conventions.md:33   class: omission
  leaf-text: «and `viola link` / `viola unlink`'s refused instance check of the driven instance (`check_instance` on the home and `instances/<driven>`, before any role file or log opens), no log opened, no line written)»
  source: .andromeda/architecture-detail/conventions.md:58 «Exit `21` also covers the refused strict-modes check of `send`, `wait`, `last`, `answer`, `pause` and `release` (`client::refuse_untrusted`: `check_instance` on the home and `instances/<name>`»
  replace-with: «and the refused instance check of `send` / `wait` / `last` / `answer` / `pause` / `release` (the target instance) and of `viola link` / `viola unlink` (the driven instance), made first by `client::refuse_untrusted` (`check_instance` on the home and `instances/<name>`, after `send`'s text read, before any connect, role file or log opens): the instance-state `unable:` / `hint:` pair, or `{"v":1,"error":"instance-unreachable","detail":null}` under `--json`, no log opened, no line written)»

- leaf: .claude/docs/conventions.md:44   class: omission
  leaf-text: «a home that exists is canonicalised)»
  source: .andromeda/architecture-detail/cross-cutting-patterns.md:10 «`viola hook` takes the `instance_of` session only when its `VIOLA_DIR` instance directory exists (`canonical`) and passes `check_instance` (`trusted`;»
  replace-with: «a home that exists is canonicalised; `viola hook`'s event, dialog and statusline paths take the `instance_of` session only when its `VIOLA_DIR` instance directory exists and passes `check_instance`, else exit 0 with no `config.json` read, no stdin read, no line and no frame)»

- leaf: .claude/docs/services/viola-channel.md:25   class: omission
  leaf-text: «`viola-mcp`'s four frame tools) takes its one snapshot read»
  source: .andromeda/architecture-detail/standard-contracts.md:63 «Before that comparison each client runs the instance's strict-modes check, and a refused check writes no frame and no line: `client::refuse_untrusted` for `send`, `wait`, `last`, `answer`, `pause`,»
  replace-with: «`viola-mcp`'s four frame tools) first runs the instance's strict-modes check (`client::refuse_untrusted` for the CLI verbs, the hook's `trusted`, `strict::check_instance` in the MCP frame tools; a refused check writes no frame and no line), then takes its one snapshot read»

- leaf: .claude/docs/services/viola-channel.md:25   class: stale
  leaf-text: «`LiveEndpoint`: endpoint, pid, start time from one snapshot read; `viola hook`'s event and dialog paths one shared `read_snapshot` before `Client::connect_by`)»
  source: .andromeda/architecture-detail/established-decisions.md:122 «share one connect: `replay::read_snapshot_or_replay` (a replayed or absent snapshot is an endpoint not reached: the hook fails open with `channel-unreachable`), then `Client::connect_by`»
  replace-with: «`LiveEndpoint`: endpoint, pid, start time from one `replay::read_snapshot_or_replay` read, only on a real snapshot; `viola hook`'s event and dialog paths one shared `replay::read_snapshot_or_replay` before `Client::connect_by`, a replayed or absent snapshot unreachable)»
