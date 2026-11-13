@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/viola-channel.md lines=45
last: - `doc:architecture` §Standard Contracts (Wrapper channel fr
@leaf_read leaf=.claude/docs/services/viola-core.md lines=43
last: - `.claude/docs/stack.md` · `.claude/docs/conventions.md` · 
@leaf_read leaf=.claude/docs/services/viola-mcp.md lines=39
last: - `doc:architecture` [MCP] · `.claude/rules/api.md`
@leaf_read leaf=.claude/docs/services/viola-pty.md lines=47
last: - `doc:architecture` [PTY] · `.claude/docs/gotchas.md` (ConP
@leaf_read leaf=.claude/docs/services/viola-state.md lines=49
last: - `doc:architecture` §Standard Contracts (snapshot envelope,
@leaf_edit n=1 leaf=.claude/docs/services/viola-channel.md line=22 class=omission
leaf-text: a symlink, a non-directory, another owner or any group or other bit is refused
source: dec-tzuxsd5q «It refuses a symlink, a non-directory, another owner, any group or other bit, and an unstattable path.»
replace-with: a symlink, a non-directory, another owner, any group or other bit or an unstattable path is refused
@leaf_edit n=2 leaf=.claude/docs/services/viola-mcp.md line=23 class=stale
leaf-text: (stderr JSON only if the file cannot be opened)
source: ctr-xjvsadfj «viola mcp writes only MCP frames to stdout; when its role file cannot be opened it exits 1 and prints nothing.»
replace-with: (if the file cannot be opened it exits 1 and prints nothing)
@leaf_edit n=3 leaf=.claude/docs/services/viola-mcp.md line=23 class=stale
leaf-text: (exit 0, with no self `process-exit`, as at a closed stdin)
source: ctr-v6wlxdat «viola mcp's codes-only lines, ending with a self process-exit (exit_code 0, no detail) once stdin closes.»
replace-with: (exit 0, as at a closed stdin)
