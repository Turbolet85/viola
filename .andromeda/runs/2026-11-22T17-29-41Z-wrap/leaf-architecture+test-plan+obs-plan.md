@leaf_group group=architecture+test-plan+obs-plan
@leaf_read leaf=.claude/docs/commands.md lines=90
last: - `scripts/agent-run.sh logs --instance <name> | jq -c 'select
@leaf_edit n=1 leaf=.claude/docs/commands.md line=24 class=omission
leaf-text: Run C's dialogs in `-dialogs/`
source: dec-fqsrsydx «The permission-request-question relied ledger row is probed by verify's Run C with a compiled prompt raising a question through PermissionRequest, answered by the closed probe id permission-question-first-option;»
replace-with: Run C's dialogs in `-dialogs/` (among them a compiled prompt raising a question through PermissionRequest, answered by the closed probe id `permission-question-first-option`: the permission-request-question row passes only when that request is raised and the answer reaches the tool's answers)
