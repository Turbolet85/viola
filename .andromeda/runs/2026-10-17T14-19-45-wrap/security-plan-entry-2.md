
## 2026-10-17-mcp-server-for-drivers — the mcp process spawns claude agents --json
**Section:** Threat Model Summary (the `claude agents --json` spawn bullet); Input Validation (Child executable resolution row)
**Change:** was "`mcp` spawns none (its `list` is not built)" and "`mcp`'s `list` is not built"; now the `mcp` process's `list` tool makes one `claude agents --json` read per call through CLI `list`'s board path (`list::board_document`, run on a blocking thread), its `agents-probe` pair logged in `diagnostics/mcp.ndjson` under the same no-row-text, no-id, no-`cwd`, no-path rule; its output stays untrusted and `MAX_FRAME`-bounded, a row's `name` never a `ViolaName` or `target` in the MCP `list` tool.
**Why:** a boundary widening (a new process spawning `claude`) ratified by the founder's own answer to P4 fork Q1, «Восьмой gap + agents» (MCP `list` reads agents as CLI `list` does), relayed verbatim by the overseer. The wrap's answer-card confirm is a delegate's (the Viola overseer) — PROVISIONAL, superseded by his own word at the Epoch 5 boundary batch.
**Ref:** .andromeda/runs/2026-10-17T14-19-45-wrap/
