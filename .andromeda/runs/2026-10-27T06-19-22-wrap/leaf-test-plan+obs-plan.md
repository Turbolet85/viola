group: test-plan+obs-plan
read: .claude/rules/observability.md · 46 · «- 2026-09-24: `obs_event!` expands to `::tracing::event!`, s»
- leaf: .claude/rules/observability.md:19   class: omission
  leaf-text: «`mcp` always writes `mcp.ndjson` (D-09), stderr JSON only when that file cannot be opened, never stdout (it carries MCP frames).»
  source: .andromeda/obs-plan-detail/log-coverage.md:35 «is written at `WARN` into `mcp.ndjson` by `viola_mcp::serve` through `obs_event!`, once per session, after the session ends, only when the stdin reader cut a line past `MAX_FRAME`»
  replace-with: «`mcp` always writes `mcp.ndjson` (D-09), stderr JSON only when that file cannot be opened, never stdout (it carries MCP frames); a `viola mcp` stdin line past `MAX_FRAME` (`CappedLines`) is refused whole and ends the session as a closed stdin does, with no `isError` result and no rmcp line, its one record `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` at WARN written into `mcp.ndjson` by `viola_mcp::serve` through `obs_event!` once per session after the session ends, holding no byte of the cut line.»
