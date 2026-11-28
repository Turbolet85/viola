@leaf_group group=architecture-3
@leaf_read leaf=.claude/docs/services/viola.md lines=58
last: - `doc:architecture` · `.claude/rules/api.md` · `.claude/rules
@leaf_read leaf=.claude/docs/stack.md lines=66
last: 5. Commit as `chore: bump {tool} to {version}`
@leaf_read leaf=.claude/docs/workflow.md lines=67
last: This project uses a **reference-based** CLAUDE.md architectur
@leaf_edit n=1 leaf=.claude/docs/services/viola.md line=11 class=omission
leaf-text: tempfile (`=3.27.0`, workspace-pinned: `viola verify`'s Run A temp dir only).
source: dec-ou6ppzpr «decision/unicode-width-column-widths · unicode-width for terminal column widths — unicode-width is a direct, exactly pinned dependency of the root bin, used for terminal column widths of the console's editing line.»
replace-with: tempfile (`=3.27.0`, workspace-pinned: `viola verify`'s Run A temp dir only); unicode-width (direct, exactly pinned: the terminal column widths of the console's editing line; already in the lock through vt100).
