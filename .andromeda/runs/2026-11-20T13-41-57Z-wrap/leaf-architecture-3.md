@leaf_group group=architecture-3
@leaf_read leaf=.claude/docs/stack.md lines=67
last: 5. Commit as `chore: bump {tool} to {version}`
@leaf_read leaf=.claude/docs/workflow.md lines=68
last: This project uses a **reference-based** CLAUDE.md architectu
@leaf_read leaf=.claude/rules/events.md lines=41
last: - 2026-10-16: A snapshot field that a reader expects once an
@leaf_read leaf=.claude/rules/frontend.md lines=30
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/rules/events.md line=20 class=omission
leaf-text: is that dialog's continuation and raises no event); Stop → `turn-ended`
source: dec-nwjqcqs4 «A question first raised by PermissionRequest with no matching armed dialog is held for the driver; the answer returns as PermissionRequest allow with updatedInput, the tool input plus answers and annotations»
replace-with: is that dialog's continuation and raises no event; a question first raised by PermissionRequest with no matching armed dialog is held for the driver and its answer returns as PermissionRequest allow with `updatedInput`, the tool input plus answers and annotations, while a human key, an expired hold or an unverified CLI leave it to the human); Stop → `turn-ended`
