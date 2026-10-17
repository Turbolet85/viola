group: architecture-1
read: .claude/docs/conventions.md · 61 · «- For warnings and gotchas, see `.claude/docs/gotchas.md`»
read: .claude/docs/gotchas.md · 147 · «- For architectural decisions and their rationale, see `.and»
read: .claude/docs/services/viola-agent-claude.md · 47 · «- `.andromeda/architecture.md` [CLI Version Compatibility] ·»
read: .claude/docs/services/viola-channel.md · 45 · «- `.andromeda/architecture.md` §Standard Contracts (Wrapper »
read: .claude/docs/services/viola-core.md · 43 · «- `.claude/docs/stack.md` · `.claude/docs/conventions.md` · »
read: .claude/docs/services/viola-mcp.md · 39 · «- `.andromeda/architecture.md` [MCP] · `.claude/rules/api.md»

- leaf: .claude/docs/conventions.md:15   class: stale
  leaf-text: «One open divergence, not an exception: the crate's pre-existing `Refusal` (`resolve_program`) beside `AgentError` (arch [Error Handling]).»
  source: .andromeda/architecture.md:94 «`viola-mcp` holds `ServeError` (`Runtime`, `Initialize`, `Session`; fixed messages, returned by `serve`) beside `McpError` (`Internal`, Display `internal error`)»
  replace-with: «Two open divergences, not exceptions: the crate's pre-existing `Refusal` (`resolve_program`) beside `AgentError`, and `viola-mcp`'s `ServeError` (`Runtime`, `Initialize`, `Session`; fixed messages, returned by `serve`) beside `McpError` (`Internal`, Display `internal error`) (arch [Error Handling]).»

- leaf: .claude/docs/conventions.md:54   class: stale
  leaf-text: «only `viola-mcp` lists tokio»
  source: .andromeda/architecture.md:95 «Among the product crates only `viola-mcp` enables `tokio` (`rt`); `viola-channel` lists it optional behind its `tokio` feature»
  replace-with: «among the product crates only `viola-mcp` enables tokio (`rt`), and `viola-channel` lists it optional behind its `tokio` feature (the `TokioClient`), which `viola-mcp` enables»

- leaf: .claude/docs/services/viola-agent-claude.md:14   class: stale
  leaf-text: «`mcp`'s `list` is not built»
  source: .andromeda/architecture.md:281 «The `mcp` process's `list` tool makes the same one read on each call»
  replace-with: «and the `mcp` process's `list` tool, which makes the same one read on each call through `list::board_document`»

- leaf: .claude/docs/services/viola-channel.md:6   class: stale
  leaf-text: «as landed it is sync and tokio-free, and the Tokio client arrives behind the `tokio` feature with `viola-mcp`, the one crate that enables it.»
  source: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:4 «behind its `tokio` feature (`tokio = ["dep:tokio", "interprocess/tokio"]`; tokio `io-util`, its dev-dependency `rt`) the Tokio client `tokio_client::TokioClient`»
  replace-with: «behind its `tokio` feature (`tokio = ["dep:tokio", "interprocess/tokio"]`) it holds the Tokio client `tokio_client::TokioClient` (`connect`, `conn`, `request`; landed at chunk 2026-10-17-mcp-server-for-drivers), enabled by `viola-mcp` and, in a package-scoped test build, by the crate's own `fake-agent = ["tokio"]`; without the feature its normal graph stays tokio-free (`scripts/channel-tokio-free.sh`).»

- leaf: .claude/docs/services/viola-channel.md:11   class: omission
  leaf-text: «libc (Unix: the uid for the socket dir).»
  source: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:4 «behind its `tokio` feature (`tokio = ["dep:tokio", "interprocess/tokio"]`; tokio `io-util`, its dev-dependency `rt`) the Tokio client `tokio_client::TokioClient`»
  replace-with: «libc (Unix: the uid for the socket dir); tokio `io-util` and `interprocess/tokio` behind the optional `tokio` feature (dev-dependency tokio `rt`).»

- leaf: .claude/docs/services/viola-channel.md:17   class: stale
  leaf-text: «the sync build must pass `cargo check` without it on all three OSes.»
  source: .andromeda/architecture.md:43 «`viola-channel`, whose optional `tokio` feature `viola-mcp` enables, is not on that list (since chunk 2026-10-17-mcp-server-for-drivers)»
  replace-with: «`viola-channel` is not in `scripts/sync-crates.txt` (since chunk 2026-10-17-mcp-server-for-drivers): its normal graph without the feature is proven tokio-free by `scripts/channel-tokio-free.sh` in CI's supply-chain step.»

- leaf: .claude/docs/services/viola-channel.md:35   class: omission
  leaf-text: «**Framing / codec / shapes:** `crates/viola-channel/src/`»
  source: .andromeda/registries/contracts/architecture/project-directory-structure.md:61 «tokio_client.rs: the Tokio client behind the `tokio` feature»
  replace-with: «**Framing / codec / shapes:** `crates/viola-channel/src/`; the Tokio client is `tokio_client.rs` (behind the `tokio` feature), sharing the sync client's crate-private helpers (`next_conn`, `request_frame`, `reply_from`, `busy_deadline`, `frame::framed`, and on Windows `client::open_handle`)»

- leaf: .claude/docs/services/viola-core.md:21   class: stale
  leaf-text: «(and MCP when it lands)»
  source: .andromeda/architecture.md:343 «one client-side `send-refused` per refused `send` text»
  replace-with: «(and by `viola mcp`, which logs one client-side `send-refused` per refused `send` text)»

- leaf: .claude/docs/services/viola-mcp.md:6   class: stale
  leaf-text: «It adds `from` from its own `VIOLA_NAME`.»
  source: .andromeda/architecture.md:58 «`target` is a `ViolaName`, and the server adds `from` from its own `VIOLA_NAME` only when it has a name of its own.»
  replace-with: «It adds `from` from its own `VIOLA_NAME` only when it has a name of its own.»

- leaf: .claude/docs/services/viola-mcp.md:6   class: omission
  leaf-text: «onto the caller-side `list` read»
  source: .andromeda/architecture.md:237 «MCP `list` (chunk 2026-10-17-mcp-server-for-drivers) builds the same board through `list::board_document` (scan → read → document, the provider `viola mcp` hands `viola_mcp::serve`)»
  replace-with: «onto the caller-side `list` board (`list::board_document`, the provider `viola mcp` hands `viola_mcp::serve`, one `claude agents --json` read per call, the board's refusal returned as `instance-unreachable`)»

- leaf: .claude/docs/services/viola-mcp.md:11   class: omission
  leaf-text: «rmcp, schemars 1.2.2, tokio 1.53.1.»
  source: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:7 «rmcp (default features off; `server` + `transport-io`), schemars, tokio (`rt`), serde, serde_json, thiserror, tracing; `[lints] workspace = true`; `fake-agent = []`»
  replace-with: «rmcp (default features off; `server` + `transport-io`), schemars 1.2.2, tokio 1.53.1 (`rt`), serde, serde_json, thiserror, tracing; `[lints] workspace = true`; `fake-agent = []`.»

- leaf: .claude/docs/services/viola-mcp.md:14   class: omission
  leaf-text: «via the embedded `.mcp.json` whose command is the absolute pinned `viola` path»
  source: .andromeda/architecture.md:96 «`.mcp.json` registers the server `viola` as `{"mcpServers": {"viola": {"command": "@@VIOLA_BIN@@", "args": ["mcp"]}}}`, the pinned copy's absolute path substituted at every start»
  replace-with: «via the embedded `.mcp.json`, which registers the server `viola` as `{"mcpServers": {"viola": {"command": "@@VIOLA_BIN@@", "args": ["mcp"]}}}`, the pinned copy's absolute path substituted at every start; the tools reach a session as `mcp__plugin_viola_viola__<tool>` (as measured live on claude 2.1.287)»

- leaf: .claude/docs/services/viola-mcp.md:21   class: stale
  leaf-text: «handlers deserialise `target` into `ViolaName`, `dialog_id` into `u64`, `response` into the closed channel enums,»
  source: .andromeda/architecture.md:139 «each handler re-validates its raw argument object (`target` through `ViolaName::try_new`, `text` a string, `after` / `timeout_ms` absent, `null` or a `u64`, `dialog_id` a `u64`,»
  replace-with: «each handler re-validates its raw argument object (`target` through `ViolaName::try_new`, `text` a string, `after` / `timeout_ms` absent, `null` or a `u64`, `dialog_id` a `u64`, `response` an object through `Response::parse`), a failure being an `isError: true` result `{"error":"invalid-params","detail":"<field>"}` (closed field words `target` · `text` · `after` · `timeout_ms` · `dialog_id` · `response`) with no frame written,»

- leaf: .claude/docs/services/viola-mcp.md:22   class: stale
  leaf-text: «Only faults on the MCP connection itself are JSON-RPC errors.»
  source: .andromeda/architecture.md:139 «JSON-RPC errors are the MCP connection's own faults, an unknown tool name (`-32602`) and an internal handler fault»
  replace-with: «Every `isError` text is `refused: <code>`. JSON-RPC errors are the MCP connection's own faults, an unknown tool name (`-32602`) and an internal handler fault (any other channel error, a reply of no known shape, a `wait` / `last` refusal, a panicked call), which is `-32603` `internal error` with `data: null`.»

- leaf: .claude/docs/services/viola-mcp.md:23   class: omission
  leaf-text: «logs go to `<home>/diagnostics/mcp.ndjson` (stderr JSON only if the file cannot be opened)»
  source: .andromeda/architecture.md:301 «exits 1 with nothing on stdout or stderr and no role file when its home or `diagnostics/` (each that exists) fails `viola_state::strict::check_path`»
  replace-with: «logs go to `<home>/diagnostics/mcp.ndjson` (stderr JSON only if the file cannot be opened) once its home and `diagnostics/` (each that exists) pass `viola_state::strict::check_path`; a refusal exits 1 with nothing on stdout or stderr and no role file»

- leaf: .claude/docs/services/viola-mcp.md:24   class: stale
  leaf-text: «Every call returns below the MCP client's tool-call timeout (the MCP `wait` default is an arch open item).»
  source: .andromeda/architecture.md:58 «MCP `wait` sends `timeout_ms` = `min(given, 50 000)` (`viola_mcp::WAIT_CAP`, `wait_timeout`), 50 000 ms when none is given, and `after` as given.»
  replace-with: «MCP `wait` sends `timeout_ms` = `min(given, 50 000)` (`viola_mcp::WAIT_CAP`, `wait_timeout`), 50 000 ms when none is given, and `after` as given: below claude 2.1.287's stdio tool-call default, which exceeds 50 s (as measured live; the exact default unmeasured).»

- leaf: .claude/docs/services/viola-mcp.md:27   class: omission
  leaf-text: «`tools/list` must be exactly the five tools — the MCP E2E asserts it.»
  source: .andromeda/architecture.md:139 «`tools/list`'s result carries `ttlMs` 0 and `cacheScope` `private`: claude 2.1.287 negotiates protocol `2026-07-28` and drops every tool of a list result without them»
  replace-with: «`tools/list` must be exactly the five tools — the MCP E2E asserts it — and its result carries `ttlMs` 0 and `cacheScope` `private`: claude 2.1.287 negotiates protocol `2026-07-28` and drops every tool of a list result without them (as measured live).»

- leaf: .claude/docs/services/viola-mcp.md:31   class: stale
  leaf-text: «(not in the tree yet: the crate lands with the route entry "MCP server for drivers")»
  source: .andromeda/architecture.md:316 «Landed so far: every member — `viola`, `viola-core`, `viola-pty`, `viola-channel`, `viola-agent-claude`, `viola-state`, `viola-mcp`, `viola-e2e`.»
  replace-with: «(landed at chunk 2026-10-17-mcp-server-for-drivers); the hidden `viola mcp` verb is `src/cmd/mcp.rs` (the home and `diagnostics/` through `check_path` before anything opens, exit 1 on a refusal; then `viola_mcp::serve`)»
