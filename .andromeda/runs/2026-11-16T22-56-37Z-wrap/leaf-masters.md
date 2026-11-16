@leaf_group group=masters
@leaf_read leaf=.claude/docs/gotchas.md lines=149
last: - For architectural decisions and their rationale, see `doc:
@leaf_read leaf=CLAUDE.md lines=134
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=21 class=omission
leaf-text: the refusal reason type, `obs_event!` and the service identity constants
source: ctr-4lunasqf «`turn-ended` has a closed cause set, TurnEndCause, in viola-core beside UnreachableCause, of nine words»
replace-with: the refusal reason type, the closed cause sets (`TurnEndCause`, nine words, beside `UnreachableCause`), `obs_event!` and the service identity constants
