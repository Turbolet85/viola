@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/viola-channel.md lines=46
last: - `doc:architecture` §Standard Contracts (Wrapper channel fra
@leaf_read leaf=.claude/docs/services/viola-core.md lines=44
last: - `.claude/docs/stack.md` · `.claude/docs/conventions.md` · `
@leaf_read leaf=.claude/docs/services/viola-mcp.md lines=40
last: - `doc:architecture` [MCP] · `.claude/rules/api.md`
@leaf_read leaf=.claude/docs/services/viola-pty.md lines=48
last: - `doc:architecture` [PTY] · `.claude/docs/gotchas.md` (ConPTY
@leaf_read leaf=.claude/docs/services/viola-state.md lines=50
last: - `doc:architecture` §Standard Contracts (snapshot envelope, 
@leaf_edit n=1 leaf=.claude/docs/services/viola-core.md line=6 class=stale
leaf-text: 3600 s, PROVISIONAL: the dialog hold's default and bounds
source: dec-xdmk4fvj «decision/dialog-hold · Dialog hold per instance — A sync dialog hook waits for the driver at most the instance's dialog hold, whole seconds from 60 to 3600, default 60, read once at the wrapper's start.»
replace-with: 3600 s: the dialog hold's default and bounds
