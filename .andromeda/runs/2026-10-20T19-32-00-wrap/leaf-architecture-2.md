group: architecture-2
read: .claude/docs/services/viola-mcp.md · 39 · «- `.andromeda/architecture.md` [MCP] · `.claude/rules/api.md»
read: .claude/docs/services/viola-pty.md · 47 · «- `.andromeda/architecture.md` [PTY] · `.claude/docs/gotchas»
read: .claude/docs/services/viola-state.md · 49 · «- `.andromeda/architecture.md` §Standard Contracts (snapshot»
read: .claude/docs/services/viola.md · 58 · «- `.andromeda/architecture.md` · `.claude/rules/api.md` · `.»

- leaf: .claude/docs/services/viola.md:6   class: stale
  leaf-text: «five interactive PTY runs (`src/cmd/verify/typed.rs`: Run A untrusted»
  source: .andromeda/architecture.md:101 «It then drives five interactive PTY runs (`src/cmd/verify/typed.rs` and its children under `src/cmd/verify/typed/`).»
  replace-with: «five interactive PTY runs (`src/cmd/verify/typed.rs` and its children under `src/cmd/verify/typed/`: Run A untrusted»

- leaf: .claude/docs/services/viola.md:6   class: stale
  leaf-text: «(`typed::agents_read`: Run B's program»
  source: .andromeda/architecture.md:101 «verify makes ONE `claude agents --json` read (`typed::turns::agents_read`: Run B's program, cwd the probe dir, the R8 strip, `VERSION_DEADLINE`, none once a stop is read)»
  replace-with: «(`typed::turns::agents_read`: Run B's program»

- leaf: .claude/docs/services/viola.md:47   class: omission
  leaf-text: «`src/cmd/verify/typed.rs`, `src/cmd/verify/stop.rs`»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:33 «verify/typed.rs: the module root (`RunDirs`, `Inputs`, `RunStep`, `measure`, `POLL`) of verify's five interactive PTY runs (Run A untrusted, Run B trusted, Run C dialogs, Run D plan, Run E resume),»
  replace-with: «`src/cmd/verify/typed.rs` (the module root) over `src/cmd/verify/typed/` (`runs.rs` Runs A–E and `trusted_args`, `turns.rs` the waits and `agents_read`, `pty.rs`, `run.rs`), `src/cmd/verify/stop.rs`»

- leaf: .claude/docs/services/viola-mcp.md:11   class: stale
  leaf-text: «(`rt`), serde, serde_json, thiserror, tracing;»
  source: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:7 «rmcp (default features off; `server` + `transport-io`), schemars, tokio (`rt`), serde_json, thiserror, tracing (no serde); `[lints] workspace = true`; `fake-agent = []`»
  replace-with: «(`rt`), serde_json, thiserror, tracing (no serde);»

- leaf: .claude/docs/services/viola-mcp.md:27   class: omission
  leaf-text: «drops every tool of a list result without them (as measured live).»
  source: .andromeda/architecture.md:152 «claude 2.1.302 on that protocol opens with `server/discover` (its JSON-RPC id a string) and then `tools/list`, each carrying `_meta` (`io.modelcontextprotocol/protocolVersion`, `…/clientInfo`,»
  replace-with: «drops every tool of a list result without them (as measured live); claude 2.1.302 on that protocol opens with `server/discover` (its JSON-RPC id a string) and then `tools/list`, each carrying `_meta`, sends no `initialize` or `notifications/initialized`, and viola's two results each carry `ttlMs` 0 and `cacheScope` `private` (as measured live, `viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup-ii/evidence/live-tap.md`).»

- leaf: .claude/docs/services/viola-mcp.md:36   class: omission
  leaf-text: «`mcp_tools_list_is_exactly_the_five_tools_and_list_matches_the_cli`)»
  source: .andromeda/architecture.md:152 «That recorded exchange, `fixtures/claude/2.1.302/Mcp.tools-list-1.json`, is replayed against `viola mcp` by `tests/mcp_recorded_exchange.rs` (`viola-e2e`), which takes the `tools/list` response by its r»
  replace-with: «`mcp_tools_list_is_exactly_the_five_tools_and_list_matches_the_cli`); `crates/viola-e2e/tests/mcp_recorded_exchange.rs` replays the recorded exchange `fixtures/claude/2.1.302/Mcp.tools-list-1.json` (recorded by the test-only `viola-mcp-tap`) against `viola mcp`, taking the `tools/list` response by its request's id and holding the served result to the recorded result's key and type shape, its tool names in order»

- leaf: .claude/docs/services/viola-mcp.md:35   class: omission
  leaf-text: «(`#[tokio::test(start_paused = true)]` for adapter-owned timers)»
  source: .andromeda/architecture.md:351 «Test-only, read by no `viola` build: `MCP_SERVE_TEST_CHILD`, a marker `viola-mcp`'s unit tests set by `Command::env` on the child they spawn, read only by that crate's `#[cfg(test)]` child entry»
  replace-with: «(`#[tokio::test(start_paused = true)]` for adapter-owned timers; the child the unit tests spawn is marked by `MCP_SERVE_TEST_CHILD` through `Command::env`, read only by the crate's `#[cfg(test)]` child entry `serve_child_entry` and by no `viola` build)»
