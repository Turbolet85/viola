
## 2026-10-20-epoch-5-cleanup-ii — the measured 2.1.302 MCP opening and its recorded replay
**Section:** §Standard Contracts (the MCP result shape: the `tools/list` result)
**Change:** claude 2.1.302 on protocol `2026-07-28` opens with `server/discover` (its JSON-RPC id a string) and then `tools/list`, each carrying `_meta` (`io.modelcontextprotocol/protocolVersion`, `…/clientInfo`, `…/clientCapabilities`), and sends no `initialize` or `notifications/initialized`; viola's two results each carry `ttlMs` 0 and `cacheScope` `private`, as measured live (start 2). The recorded exchange `Mcp.tools-list-1.json` is replayed against `viola mcp` by `tests/mcp_recorded_exchange.rs`, which takes the `tools/list` response by its request's id and holds the served result to the recorded result's key and type shape, its tool names in order. Was: only the 2.1.287 reading (the two keys required).
**Why:** the plan assumed an `initialize`-flavoured handshake; start 2's capture disproved it for 2.1.302, so the body states the measured opening.
**Ref:** .andromeda/runs/2026-10-20T19-32-00-wrap/
