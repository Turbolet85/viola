@leaf_group group=test-plan+obs-plan
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, s
@leaf_edit n=1 leaf=.claude/rules/observability.md line=19 class=stale
leaf-text: stderr JSON only when that file cannot be opened, never stdout (it carries MCP frames)
source: dec-4bmxcdmt «decision/mcp-log-file-sink · MCP logs to a file sink, never stdout — The `mcp` process always logs to the file sink `<home>/diagnostics/mcp.ndjson`, with no stderr fallback: a file that cannot be opened exits 1 printing nothing»
replace-with: no stderr fallback (a file that cannot be opened exits 1 printing nothing), never stdout (it carries MCP frames)
