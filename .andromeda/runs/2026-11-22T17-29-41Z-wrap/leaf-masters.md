@leaf_group group=masters
@leaf_read leaf=.claude/docs/gotchas.md lines=149
last: - For architectural decisions and their rationale, see `doc:
@leaf_read leaf=CLAUDE.md lines=136
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=42 class=omission
leaf-text: `run`'s child screen, the CLI's `--json`
source: ctr-bjgrjczv «run writes nothing to the terminal except the child's own output and, before the child, a keyed start's automatic-verify lines and the verify child's own output, none under revive --json»
replace-with: `run`'s child screen, before it only a keyed start's automatic-verify lines and the verify child's own output, the CLI's `--json`
