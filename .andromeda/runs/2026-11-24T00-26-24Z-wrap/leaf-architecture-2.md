@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/viola-mcp.md lines=39
last: - `doc:architecture` [MCP] · `.claude/rules/api.md`
@leaf_read leaf=.claude/docs/services/viola-pty.md lines=47
last: - `doc:architecture` [PTY] · `.claude/docs/gotchas.md` (ConPTY,
@leaf_read leaf=.claude/docs/services/viola-state.md lines=49
last: - `doc:architecture` §Standard Contracts (snapshot envelope,
@leaf_edit n=1 leaf=.claude/docs/services/viola-state.md line=28 class=omission
leaf-text: a line with a non-string member reading none) and recovers `links`
source: ctr-navndiay «`window` is `{address?, workspace?, placement, origin}`, the wrapper's window record, omitted when absent; a replay rebuilds it from the log, the last `window` line after the last `launch` line winning and a»
replace-with: a line with a non-string member reading none) and `window` (`{address?, workspace?, placement, origin}`, the last `window` line after the last `launch` line winning and a later `launch` line clearing it) and recovers `links`
