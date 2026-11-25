@leaf_group group=masters
@leaf_read leaf=.claude/docs/gotchas.md lines=149
last: - For architectural decisions and their rationale, see `doc:
@leaf_read leaf=CLAUDE.md lines=136
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=.claude/docs/gotchas.md line=18 class=stale
leaf-text: detect exit only by `wait()` on the process handle (a dedicated handle-wait thread)
source: dec-4653gh2b «the child's exit is polled on main, not a handle-wait thread»
replace-with: detect exit only on the process handle, polled on main (never a dedicated handle-wait thread)
@leaf_edit n=2 leaf=.claude/docs/gotchas.md line=79 class=omission
leaf-text: is a quote and does not;
source: dec-xsqsrc2s «A modal literal's row is a quote only when it is not option-shaped and the box's own rows stand below it; a quote alone on its own line, or a code-block line holding only the label, still refuses.»
replace-with: is a quote and does not, save a quote alone on its own line or a code-block line holding only the label, which still refuses;
