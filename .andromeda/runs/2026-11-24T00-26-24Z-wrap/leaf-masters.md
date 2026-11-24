@leaf_group group=masters
@leaf_read leaf=.claude/docs/gotchas.md lines=149
last: - For architectural decisions and their rationale, see `doc:ar
@leaf_read leaf=CLAUDE.md lines=136
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=36 class=omission
leaf-text: are dropped at the parse and reach no log line and no error.
source: dec-4rj6v4ng «An instance's window record (address, workspace, placement and origin) is kept only in the snapshot and the event log's window line and leaves only through list --json and the MCP list item; no address,»
replace-with: are dropped at the parse and reach no log line and no error; an instance's window record (address, workspace, placement, origin) is kept only in the snapshot and the events log's `window` line, leaves only through `list --json` and the MCP list item, and no address, pid or workspace enters a process-log line.
