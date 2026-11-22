@leaf_group group=architecture-4
@leaf_read leaf=.claude/rules/events.md lines=41
last: - 2026-10-16: A snapshot field that a reader expects once an
@leaf_edit n=1 leaf=.claude/rules/events.md line=20 class=omission
leaf-text: PermissionRequest → `permission` (a PermissionRequest repeating
source: ctr-zl7x6xdx «PermissionRequest maps to `permission`, or to `question` for an AskUserQuestion call with no matching armed dialog, except one that repeats the armed `question` / `plan` dialog's tool with an equal `tool_input`»
replace-with: PermissionRequest → `permission`, or `question` for an AskUserQuestion call with no matching armed dialog (a PermissionRequest repeating
