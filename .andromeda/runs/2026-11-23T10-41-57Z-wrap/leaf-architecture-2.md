@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/viola-mcp.md lines=40
last: - `doc:architecture` [MCP] · `.claude/rules/api.md`
@leaf_read leaf=.claude/docs/services/viola-pty.md lines=48
last: - `doc:architecture` [PTY] · `.claude/docs/gotchas.md` (ConPTY
@leaf_read leaf=.claude/docs/services/viola-state.md lines=50
last: - `doc:architecture` §Standard Contracts (snapshot envelope,
@leaf_edit n=1 leaf=.claude/docs/services/viola-state.md line=28 class=omission
leaf-text: rebuilds `lineage` from the log (the last `lineage` line replacing the held value)
source: ctr-wjadxiye «`launch` is `{model, effort}`, the pair the start handed, omitted when absent with no snapshot `v` move; a replay rebuilds it from the log, the last `launch` line replacing the held value and a line with a non-string member reading none.»
replace-with: rebuilds `lineage` from the log (the last `lineage` line replacing the held value) and `launch` (`{model, effort}`, the last `launch` line replacing the held value, a line with a non-string member reading none)
