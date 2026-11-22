@leaf_group group=architecture-4
@leaf_read leaf=.claude/rules/api.md lines=41
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_read leaf=.claude/rules/events.md lines=40
last: - 2026-10-16: A snapshot field that a reader expects once an
@leaf_edit n=1 leaf=.claude/rules/events.md line=20 class=omission
leaf-text: PermissionRequest → `permission`, or `question` for an AskUserQuestion call with no matching armed dialog (
source: ctr-zl7x6xdx «PermissionRequest maps to `permission`, to `question` for an AskUserQuestion call with no matching armed dialog, or to `plan` for such an ExitPlanMode call (unmeasured live)»
replace-with: PermissionRequest → `permission`, `question` for an AskUserQuestion call with no matching armed dialog, or `plan` for such an ExitPlanMode call (unmeasured live) (
