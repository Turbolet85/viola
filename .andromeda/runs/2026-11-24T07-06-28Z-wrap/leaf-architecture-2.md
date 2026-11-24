@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/viola-mcp.md lines=39
last: - `doc:architecture` [MCP] · `.claude/rules/api.md`
@leaf_read leaf=.claude/docs/services/viola-pty.md lines=47
last: - `doc:architecture` [PTY] · `.claude/docs/gotchas.md` (ConPTY
@leaf_read leaf=.claude/docs/services/viola-state.md lines=49
last: - `doc:architecture` §Standard Contracts (snapshot envelope,
@leaf_edit n=1 leaf=.claude/docs/services/viola-mcp.md line=20 class=omission
leaf-text: `focus` is CLI-only too: no MCP tool raises a window, and adding one is a boundary widening of its own.
source: ctr-uv2g64ih «`viola up [<NAME>...] [--dry-run] [--json]` re-opens, after a reboot, each instance gone before the boot through viola revive in a new terminal; names given narrow the fleet and a name not scanned reads not-taken; CLI-only, no MCP tool and no channel method.»
replace-with: `focus` is CLI-only too: no MCP tool raises a window, and adding one is a boundary widening of its own; `up` is CLI-only as well, with no MCP tool and no channel method.
