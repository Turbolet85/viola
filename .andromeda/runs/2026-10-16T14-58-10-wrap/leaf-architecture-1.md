group: architecture-1
read: .claude/docs/conventions.md · 61 · «- For warnings and gotchas, see `.claude/docs/gotchas.md`»
read: .claude/docs/gotchas.md · 147 · «- For architectural decisions and their rationale, see `.and»
read: .claude/docs/services/viola-agent-claude.md · 47 · «- `.andromeda/architecture.md` [CLI Version Compatibility] ·»
read: .claude/docs/services/viola-channel.md · 45 · «- `.andromeda/architecture.md` §Standard Contracts (Wrapper »
read: .claude/docs/services/viola-core.md · 43 · «- `.claude/docs/stack.md` · `.claude/docs/conventions.md` · »
read: .claude/docs/services/viola-mcp.md · 39 · «- `.andromeda/architecture.md` [MCP] · `.claude/rules/api.md»

- leaf: .claude/docs/conventions.md:15   class: stale
  leaf-text: «`AgentError`, `McpError`, `UiError`, `CoreError`), each `thiserror`»
  source: .andromeda/architecture.md:141 «- **Rust error types:** one error enum per crate, named `<Crate>Error` (`PtyError`, `ChannelError`, `StateError`, `AgentError`, `McpError`, `CoreError`), each a `thiserror` enum except `PtyError`»
  replace-with: «`AgentError`, `McpError`, `CoreError`), each `thiserror`»

- leaf: .claude/docs/conventions.md:27   class: stale
  leaf-text: «Integer `v` in every channel `params`, event line, snapshot and GUI JSON body;»
  source: .andromeda/architecture.md:112 «- **Protocol versioning:** an integer `v` starting at `1` in every channel `params`, ndjson event line and snapshot.»
  replace-with: «Integer `v` in every channel `params`, event line and snapshot;»

- leaf: .claude/docs/conventions.md:29   class: retired
  leaf-text: «- GUI routes unversioned under `/api/`; `application/json`, `application/problem+json` (with `v`), `text/event-stream`.»
  source: .andromeda/architecture.md:173 «**GUI contracts.** The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «- GUI contracts: none in v1; the frontend is deferred to 0.2.0 and its technology is not chosen.»

- leaf: .claude/docs/conventions.md:35   class: retired
  leaf-text: «- GUI: RFC 9457 Problem Details with `urn:viola:problem:*` types (`host-not-allowed` 403, `method-not-allowed` 405, `not-found` 404, `state-unreadable` 503, `unauthorized` 401, `cross-origin-forbidden` 403 v1.x).»
  source: .andromeda/architecture.md:173 «**GUI contracts.** The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «- GUI: no HTTP error contract in v1; the frontend is deferred to 0.2.0.»

- leaf: .claude/docs/conventions.md:39   class: stale
  leaf-text: «(`--json`, `--file`, `--budget`, `--port`, global `--home`)»
  source: .andromeda/architecture.md:160 «- **CLI:** flags are kebab-case long options (`--json`, `--file`, `--budget`, and the global `--home`).»
  replace-with: «(`--json`, `--file`, `--budget`, global `--home`)»

- leaf: .claude/docs/conventions.md:45   class: stale
  leaf-text: «switches the gate off), GUI port (47319), `diagnostics_level`»
  source: .andromeda/architecture.md:404 «  - Settings: budget thresholds as `Percent` (defaults `five_hour` 90, `seven_day` 85), `diagnostics_level` (`info` by default, or `debug`) and `claude_env_keep`»
  replace-with: «switches the gate off), `diagnostics_level`»

- leaf: .claude/docs/conventions.md:54   class: stale
  leaf-text: «only `viola-mcp` / `viola-ui` list tokio;»
  source: .andromeda/architecture.md:96 «Only `viola-mcp` lists `tokio`, and `viola-channel`'s Tokio client sits behind a `tokio` feature only `viola-mcp` enables.»
  replace-with: «only `viola-mcp` lists tokio;»

- leaf: .claude/docs/services/viola-agent-claude.md:14   class: stale
  leaf-text: «`list` / `mcp` / `ui` (`claude agents --json` rows;»
  source: .andromeda/architecture.md:281 «Its readers (`list` and `mcp`, through `viola-agent-claude`) spawn it directly (none does yet: CLI `list` landed without it, chunk 2026-10-11-the-board-viola-list»
  replace-with: «`list` / `mcp` (`claude agents --json` rows;»

- leaf: .claude/docs/services/viola-mcp.md:6   class: stale
  leaf-text: «It is one of the two crates allowed to build a Tokio runtime.»
  source: .andromeda/architecture.md:411 «- **Tokio containment.** New code on the `run`, `hook`, `send` or channel-server paths uses std threads and blocking I/O. Only `viola-mcp` may build a runtime.»
  replace-with: «It is the only crate allowed to build a Tokio runtime.»
