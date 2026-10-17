group: architecture+security-plan
read: .claude/rules/api.md · 42 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/rules/api.md:21   class: stale
  leaf-text: «`mcp`'s `list` is not built.»
  source: .andromeda/architecture.md:237 «MCP `list` (chunk 2026-10-17-mcp-server-for-drivers) builds the same board through `list::board_document` (scan → read → document, the provider `viola mcp` hands `viola_mcp::serve`)»
  replace-with: «`mcp`'s `list` tool (chunk 2026-10-17-mcp-server-for-drivers) builds the same board through `list::board_document` (scan → read with the one `claude agents --json` read and join → document, run on a blocking thread, its `agents-probe` pair in `diagnostics/mcp.ndjson`) on each call and returns the document's `ok`, or `instance-unreachable` for the board's refusal.»

- leaf: .claude/rules/api.md:30   class: omission
  leaf-text: «`list`'s `instances/` that cannot be listed or failed log start»
  source: .andromeda/architecture.md:301 «exits 1 with nothing on stdout or stderr and no role file when its home or `diagnostics/` (each that exists) fails `viola_state::strict::check_path`»
  replace-with: «`list`'s `instances/` that cannot be listed or failed log start, the hidden `viola mcp`'s home or `diagnostics/` (each that exists) failing `viola_state::strict::check_path` (nothing on stdout or stderr, no role file)»

- leaf: .claude/rules/api.md:31   class: omission
  leaf-text: «as landed every CLI verb, `run` and `revive` included, resolves the home»
  source: .andromeda/architecture.md:322 «`viola mcp` (chunk 2026-10-17-mcp-server-for-drivers) resolves its home through the same `resolved_home` and reads `VIOLA_NAME` for its own name»
  replace-with: «as landed every CLI verb, `run`, `revive` and `mcp` included (`mcp`'s use NOT ratified either, a delegate's answer at the 2026-10-17-mcp-server-for-drivers wrap), resolves the home»

- leaf: .claude/rules/api.md:31   class: omission
  leaf-text: «`send`, `wait`, `last`, `answer`, `pause` and `release` read `snapshot.json` without strict-modes after a liveness-only pre-check.»
  source: .andromeda/security-plan.md:182 «until the same two entries land, `viola mcp`'s `send` / `wait` / `last` / `answer` tools write their frames through the Tokio client after the same liveness-only pre-check»
  replace-with: «`send`, `wait`, `last`, `answer`, `pause` and `release` read `snapshot.json` without strict-modes after a liveness-only pre-check, and so do `viola mcp`'s `send` / `wait` / `last` / `answer` tools (`viola_state::liveness::live_endpoint`, else `{"error":"instance-unreachable","detail":null}`; the eighth dated gap, the founder's own answer, `inputs#I2` Q1 of chunk 2026-10-17-mcp-server-for-drivers).»

- leaf: .claude/rules/api.md:32   class: omission
  leaf-text: «The CLI adds `from` from its own `VIOLA_NAME` when set.»
  source: .andromeda/architecture.md:58 «the server adds `from` from its own `VIOLA_NAME` only when it has a name of its own»
  replace-with: «The CLI adds `from` from its own `VIOLA_NAME` when set; the MCP server adds it from its own `VIOLA_NAME` only when it has a name of its own.»

- leaf: .claude/rules/api.md:35   class: omission
  leaf-text: «handlers re-deserialise into `ViolaName`, `u64`, closed enums and re-run `validate_paste_text`.»
  source: .andromeda/security-plan.md:202 «each handler takes the raw argument object, never rmcp's typed deserialisation (a serde failure would carry serde text)»
  replace-with: «each handler takes the raw argument object, never rmcp's typed deserialisation: `target` through `ViolaName::try_new`, `text` a string, `after` / `timeout_ms` absent, `null` or a `u64`, `dialog_id` a `u64`, `response` an object through `Response::parse`, then `validate_paste_text` on `send`'s text (then the empty-typed-text check, refused `not-delivered` / `control-character` | `empty-text` with one client-side `send-refused` line) and on every free-text answer value; a failure is `isError:true` `{"error":"invalid-params","detail":"<field>"}` (closed field words `target` · `text` · `after` · `timeout_ms` · `dialog_id` · `response`) with no frame written.»

- leaf: .claude/rules/api.md:36   class: omission
  leaf-text: «refusal/unreachable/fault: `isError:true` + codes only.»
  source: .andromeda/security-plan.md:451 «`error: instance-unreachable | wrapper-fault | invalid-params` — `instance-unreachable` with `detail: null`, `wrapper-fault` with the JSON-RPC fault's `{code,message,data}`, `invalid-params` with a closed field word)»
  replace-with: «refusal/unreachable/fault/invalid params: `isError:true` + codes only (`structuredContent` the `refusal` + `detail`, or `{"error":"instance-unreachable"|"wrapper-fault"|"invalid-params","detail":…}`), every `isError` text `refused: <code>`; JSON-RPC errors are only the connection's own faults, an unknown tool name (`-32602`) and any other handler fault, a reply of no known shape or a panicked call (`-32603` `internal error`, `data: null`); `tools/list`'s result carries `ttlMs` 0 and `cacheScope` `private` (claude 2.1.287 drops every tool of a list result without them).»

- leaf: .claude/rules/api.md:36   class: omission
  leaf-text: «every call returns below the client's tool-call timeout.»
  source: .andromeda/architecture.md:58 «MCP `wait` sends `timeout_ms` = `min(given, 50 000)` (`viola_mcp::WAIT_CAP`, `wait_timeout`), 50 000 ms when none is given»
  replace-with: «every call returns below the client's tool-call timeout: MCP `wait` sends `timeout_ms` = `min(given, 50 000)` (`viola_mcp::WAIT_CAP`), 50 000 ms when none is given, below claude 2.1.287's stdio tool-call default (over 50 s, as measured live).»
