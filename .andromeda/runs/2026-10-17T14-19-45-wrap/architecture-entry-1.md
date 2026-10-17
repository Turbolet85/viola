
## 2026-10-17-mcp-server-for-drivers — the MCP tools' contract as landed
**Section:** §Established Decisions [MCP], [Delivery Confirmation]; §Stack and Technologies (MCP server row); §Conventions Error handling schema (MCP bullet), CLI exit codes; §Occupied Resources Claude Code integration names, Binary, subcommands and exit codes
**Change:**
- [MCP]: was "the default `timeout_ms` for MCP `wait` is an open item"; now `wait` sends `timeout_ms` = `min(given, 50 000)` (`WAIT_CAP`, `wait_timeout`), 50 000 ms by default, `after` as given; the bound sits below claude 2.1.287's stdio tool-call default, which exceeds 50 s (measured live; the exact default unmeasured). `from` only when the server has a name. Each handler re-validates its raw arguments: a schema violation is `invalid-params`, an internal handler fault JSON-RPC `-32603` `internal error`, `data: null`.
- [Delivery Confirmation]: the MCP client's tool-call timeout exceeds 50 s on 2.1.287, above the 18.5 s send total.
- Conventions, MCP bullet: was "only faults on the MCP connection itself are JSON-RPC errors"; now `instance-unreachable` carries `detail: null`, `wrapper-fault` the fault's `{code,message,data}`, `invalid-params` a closed field word (`target` · `text` · `after` · `timeout_ms` · `dialog_id` · `response`) with no frame; every `isError` text `refused: <code>`; JSON-RPC errors are the connection's faults, an unknown tool (`-32602`) and an internal handler fault (`-32603`); `tools/list` carries `ttlMs` 0 and `cacheScope` `private`, without which 2.1.287 (protocol `2026-07-28`) drops every tool.
- CLI exit codes: `role_of` excludes `mcp` from `cli`; `Role::Mcp` exits like `Other` and never prints `error: internal error`.
- Integration names: tools reach a session as `mcp__plugin_viola_viola__<tool>`; `--allowedTools mcp__plugin_viola_viola` allowed them with no dialog; they load deferred, through `ToolSearch`.
- Subcommands: `mcp` hidden from `--help`, no argument but `--home`, exit 1 printing nothing when its home or `diagnostics/` fails `check_path`.
- MCP server row: release graph rmcp exactly `server` + `transport-io` (`scripts/rmcp-features.sh`); `viola-e2e` takes rmcp `client` alone, never `transport-child-process`.
**Why:** the chunk built `viola mcp`; the `ttlMs` / `cacheScope` requirement and the over-50 s client default were measured at its two live starts (the second on the founder's own raised cap).
**Ref:** .andromeda/runs/2026-10-17T14-19-45-wrap/
