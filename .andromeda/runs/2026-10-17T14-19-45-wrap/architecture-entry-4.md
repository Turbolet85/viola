
## 2026-10-17-mcp-server-for-drivers — the plugin registers viola mcp, its log and its home
**Section:** §Established Decisions [Deployment / Distribution]; §Occupied Resources Filesystem (`diagnostics/`), Environment variables (`VIOLA_DIR`)
**Change:**
- [Deployment / Distribution]: was "until the `mcp` verb exists, `.mcp.json` ships `{"mcpServers": {}}`"; now `.mcp.json` registers `{"mcpServers": {"viola": {"command": "@@VIOLA_BIN@@", "args": ["mcp"]}}}`, the pinned path substituted at every start; whether a registered `viola mcp` that exits 1 blocks the CLI's start is unmeasured.
- Filesystem: `mcp.ndjson` has a producer — once its home and `diagnostics/` pass `check_path` (a refusal opens no role file): `process-start{subject:"self"}`, the Tokio client's `channel-*` lines, one client-side `send-refused` per refused text, the `list` tool's `agents-probe` pair and the board's `state-recovered` / `parse-rejected` lines (`config-json` included), and `process-exit` on stdin EOF — none when Claude Code ends it by a signal (measured live on 2.1.287).
- Environment variables: was "the `mcp` verb is not built"; now `viola mcp` resolves its home through the same `resolved_home` and reads `VIOLA_NAME` for its own name.
**Why:** the chunk built the verb. Its home step through `resolved_home` (the `VIOLA_DIR` step and the canonicalisation) is written as built and is NOT ratified, on the Viola overseer's answer at this wrap, a delegate's; it joins the carried CLI-verbs item for the founder's own word.
**Ref:** .andromeda/runs/2026-10-17T14-19-45-wrap/
