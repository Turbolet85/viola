@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/viola-mcp.md lines=40
last: - `doc:architecture` [MCP] · `.claude/rules/api.md`
@leaf_read leaf=.claude/docs/services/viola-pty.md lines=48
last: - `doc:architecture` [PTY] · `.claude/docs/gotchas.md` (ConP
@leaf_read leaf=.claude/docs/services/viola-state.md lines=50
last: - `doc:architecture` §Standard Contracts (snapshot envelope,
@leaf_edit n=1 leaf=.claude/docs/services/viola-mcp.md line=20 class=omission
leaf-text: `up` is CLI-only as well, with no MCP tool and no channel method.
source: dec-y7x5wcbv «decision/switch-cli-only · Switch is CLI-only — viola switch is offered on the CLI only: no MCP tool switches a session's model or effort. · provisional»
replace-with: `up` is CLI-only as well, with no MCP tool and no channel method; `switch` is CLI-only too: no MCP tool switches a session's model or effort.
