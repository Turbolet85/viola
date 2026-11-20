@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/viola-pty.md lines=47
last: - `doc:architecture` [PTY] · `.claude/docs/gotchas.md` (ConPTY
@leaf_read leaf=.claude/docs/services/viola-state.md lines=49
last: - `doc:architecture` §Standard Contracts (snapshot envelope, 
@leaf_read leaf=.claude/docs/services/viola.md lines=58
last: - `doc:architecture` · `.claude/rules/api.md` · `.claude/rule
@leaf_edit n=1 leaf=.claude/docs/services/viola.md line=41 class=omission
leaf-text: the per-tool armed continuation matched on equal `tool_input`
source: dec-nwjqcqs4 «A question first raised by PermissionRequest with no matching armed dialog is held for the driver; the answer returns as PermissionRequest allow with updatedInput, the tool input plus answers and annotations»
replace-with: the per-tool armed continuation matched on equal `tool_input`; a question first raised by PermissionRequest with no matching armed dialog held for the driver, its answer returned as PermissionRequest allow with `updatedInput` (the tool input plus answers and annotations), a human key, an expired hold or an unverified CLI leaving it to the human
