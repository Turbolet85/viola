@leaf_group group=architecture+security-plan+test-plan+obs-plan
@leaf_read leaf=.claude/docs/gotchas.md lines=150
last: - For architectural decisions and their rationale, see `doc:a
@leaf_read leaf=CLAUDE.md lines=137
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=.claude/docs/gotchas.md line=93 class=retired
leaf-text: (11 names measured on the Windows host)
source: dec-x56wogur «decision/r8-identity-floor · R8 parent-identity floor — The identity-floor CLAUDE* parent-identity environment names are stripped from the child even when the persistent environment defines them»
replace-with: (the identity-floor names)
@leaf_edit n=2 leaf=.claude/docs/gotchas.md line=94 class=stale
leaf-text: always the 11-name `IDENTITY_FLOOR`
source: dec-ysshfvlo «decision/claude-env-stripped-from-child · Inherited CLAUDE names stripped from the child — Every inherited CLAUDE* environment name, matched exactly, is removed from the wrapped child's environment»
replace-with: always every `IDENTITY_FLOOR` name (beside the floor stands a compiled list of known non-identity hook names, which the floor never strips)
