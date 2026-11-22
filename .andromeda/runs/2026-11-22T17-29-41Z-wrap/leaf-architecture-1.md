@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - For warnings and gotchas, see `.claude/docs/gotchas.md`
@leaf_read leaf=.claude/docs/services/viola-agent-claude.md lines=47
last: - `doc:architecture` [CLI Version Compatibility] · `.claude/
@leaf_read leaf=.claude/docs/services/viola-channel.md lines=45
last: - `doc:architecture` §Standard Contracts (Wrapper channel fr
@leaf_read leaf=.claude/docs/services/viola-core.md lines=43
last: - `.claude/docs/stack.md` · `.claude/docs/conventions.md` · 
@leaf_edit n=1 leaf=.claude/docs/services/viola-agent-claude.md line=26 class=omission
leaf-text: its probe stamped as `measured.cross_session_probe`, closed words and one number);
source: dec-fqsrsydx «The permission-request-question relied ledger row is probed by verify's Run C with a compiled prompt raising a question through PermissionRequest, answered by the closed probe id permission-question-first-option»
replace-with: its probe stamped as `measured.cross_session_probe`, closed words and one number) · `permission-request-question` (relied; probed by `viola verify`'s Run C with a compiled prompt raising a question through PermissionRequest, answered by the closed probe id `permission-question-first-option`; passes only when that request is raised and the answer reaches the tool's answers);
@leaf_edit n=2 leaf=.claude/docs/services/viola-agent-claude.md line=24 class=omission
leaf-text: backed by the ledger's four dialog rows (`question-answer` · `plan-approve-revise` · `question-notes` · `dialog-concurrency`)
source: dec-fqsrsydx «The permission-request-question relied ledger row is probed by verify's Run C with a compiled prompt raising a question through PermissionRequest, answered by the closed probe id permission-question-first-option»
replace-with: backed by the ledger's five dialog rows (`question-answer` · `plan-approve-revise` · `question-notes` · `dialog-concurrency` · `permission-request-question`, the PermissionRequest-raised question's answer probed by Run C)
