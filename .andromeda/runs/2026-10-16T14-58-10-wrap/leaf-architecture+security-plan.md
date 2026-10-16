group: architecture+security-plan
read: .claude/rules/api.md · 45 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/rules/api.md:4   class: tree
  leaf-text: «"crates/viola-ui/src/**"»
  source: .andromeda/architecture.md:96 «Members: `viola-core`, `viola-pty`, `viola-channel`, `viola-state`, `viola-agent-claude`, `viola-mcp`, plus the test-only `viola-e2e`»
  replace-with: «»

- leaf: .claude/rules/api.md:11   class: retired
  leaf-text: «the MCP tools, the CLI verbs and the loopback GUI HTTP»
  source: .andromeda/architecture.md:111 «**Four interfaces from one binary:** CLI verbs, MCP tools (stdio JSON-RPC 2.0), the hook stdin/stdout contract (owned by Claude Code), and the wrapper channel»
  replace-with: «the MCP tools and the CLI verbs»

- leaf: .claude/rules/api.md:13   class: stale
  leaf-text: «## Five interfaces, one binary»
  source: .andromeda/architecture.md:111 «**Four interfaces from one binary:** CLI verbs, MCP tools (stdio JSON-RPC 2.0), the hook stdin/stdout contract (owned by Claude Code), and the wrapper channel»
  replace-with: «## Four interfaces, one binary»

- leaf: .claude/rules/api.md:14   class: retired
  leaf-text: «on a local socket) · GUI HTTP GET + SSE.»
  source: .andromeda/architecture.md:111 «and the wrapper channel (JSON-RPC 2.0 over ndjson on a local socket). There is no REST write surface, GraphQL, gRPC or tRPC.»
  replace-with: «on a local socket).»

- leaf: .claude/rules/api.md:18   class: retired
  leaf-text: «Every channel `params`, event line, snapshot and GUI JSON body carries integer `v`»
  source: .andromeda/architecture.md:112 «an integer `v` starting at `1` in every channel `params`, ndjson event line and snapshot.»
  replace-with: «Every channel `params`, event line and snapshot carries integer `v`»

- leaf: .claude/rules/api.md:21   class: stale
  leaf-text: «its `ok` payload is `{items, budget, skipped}` in the `/api/sessions` shapes»
  source: .andromeda/architecture.md:238 «Its `ok` payload (CLI `--json` and MCP `list`) is `{items, budget, skipped}`, with the item, `budget` and `skipped` shapes of the `list` payload above.»
  replace-with: «its `ok` payload is `{items, budget, skipped}` in the shapes of the `list` payload (Standard Contracts), returned whole, with `skipped{unknown_kinds,unknown_fields,torn_lines}`»

- leaf: .claude/rules/api.md:38   class: retired
  leaf-text: «## GUI HTTP (`viola ui`)»
  source: .andromeda/architecture.md:173 «**GUI contracts.** The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «## GUI»

- leaf: .claude/rules/api.md:39   class: retired
  leaf-text: «- Bind `127.0.0.1` only. Outermost middleware: raw `Host` header exactly `127.0.0.1:<port>` or `localhost:<port>` → else 403 `urn:viola:problem:host-not-allowed` (never axum-extra's `Host` extractor). GET only; anything else 405.»
  source: .andromeda/architecture.md:291 «**No database port, no broker port, no listening TCP port.** The MCP server is stdio only.»
  replace-with: «- The frontend is deferred to 0.2.0; its technology is not chosen. There is no `ui` subcommand and no listening TCP port; the MCP server is stdio only.»

- leaf: .claude/rules/api.md:40   class: retired
  leaf-text: «- `/api/*` and SSE need the `viola_<port>` cookie (constant-time compare) → else 401 `urn:viola:problem:unauthorized`; `/`, `/assets/*`, `/health`, `/ready` ungated.»
  source: .andromeda/security-plan.md:266 «The frontend is deferred to 0.2.0; its technology is not chosen.»
  replace-with: «»

- leaf: .claude/rules/api.md:41   class: retired
  leaf-text: «- No CORS headers ever; CSP + `nosniff` + `no-referrer` + CORP `same-origin` on every response; `Cache-Control: no-store` on `/api/*` and the `?t=` exchange. `CompressionLayer` on `/assets/*` only — SSE never compressed. Assets from `include_bytes!`, never `ServeDir`.»
  source: .andromeda/security-plan.md:268 «_Applies to the MCP transport feature set. The IPC channel's controls are in Authentication & Authorization and Input Validation._»
  replace-with: «»

- leaf: .claude/rules/api.md:42   class: retired
  leaf-text: «- Errors are RFC 9457 `application/problem+json` with `v` and fixed `detail`; `/ready` 503 uses its own body, never Problem Details. Lists are returned whole with `skipped{unknown_kinds,unknown_fields,torn_lines}`.»
  source: .andromeda/security-plan.md:452 «**External responses** (to channel clients, CLI `--json` and MCP drivers): sanitised.»
  replace-with: «»
