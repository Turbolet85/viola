@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/viola-mcp.md lines=39
last: - `doc:architecture` [MCP] · `.claude/rules/api.md`
@leaf_read leaf=.claude/docs/services/viola-pty.md lines=47
last: - `doc:architecture` [PTY] · `.claude/docs/gotchas.md` (ConP
@leaf_read leaf=.claude/docs/services/viola-state.md lines=49
last: - `doc:architecture` §Standard Contracts (snapshot envelope,
@leaf_edit n=1 leaf=.claude/docs/services/viola-state.md line=6 class=omission
leaf-text: called by `run`'s and `revive`'s starts and by `verify` with no name and no key
source: dec-twgnxsmy «A start reads its own exe once, keys the home-tree check by those bytes and pins those same bytes, so the copy it runs is the copy whose folders it checked.»
replace-with: called by `run`'s, `revive`'s and `adopt`'s starts and by `verify` with no name and no key
@leaf_edit n=2 leaf=.claude/docs/services/viola-state.md line=6 class=omission
leaf-text: `check_diagnostics` (a symbolic link refused;
source: dec-yngykanm «Every open of a process-log or detail file first checks the existing diagnostics folder and file: a symbolic link is refused and both must meet the read rule; a refusal is a failed open»
replace-with: `check_diagnostics` (a symbolic link refused, the existing folder and file each held to the read rule;
