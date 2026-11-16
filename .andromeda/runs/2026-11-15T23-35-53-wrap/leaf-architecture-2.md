@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/viola-channel.md lines=45
last: - `doc:architecture` §Standard Contracts (Wrapper channel fr
@leaf_read leaf=.claude/docs/services/viola-core.md lines=43
last: - `.claude/docs/stack.md` · `.claude/docs/conventions.md` · 
@leaf_read leaf=.claude/docs/services/viola-mcp.md lines=39
last: - `doc:architecture` [MCP] · `.claude/rules/api.md`
@leaf_read leaf=.claude/docs/services/viola-pty.md lines=47
last: - `doc:architecture` [PTY] · `.claude/docs/gotchas.md` (ConP
@leaf_read leaf=.claude/docs/services/viola-state.md lines=49
last: - `doc:architecture` §Standard Contracts (snapshot envelope,
@leaf_edit n=1 leaf=.claude/docs/services/viola-core.md line=6 class=omission
leaf-text: `UnreachableCause` (`StrictModes` · `InstanceDead`
source: ctr-4lunasqf «`turn-ended` has a closed cause set, TurnEndCause, in viola-core beside UnreachableCause, with one member `api-error` and no Claude-specific name; a `turn-ended` cause is one of its words.»
replace-with: `TurnEndCause` (the closed cause set of `turn-ended`, one member `api-error`, no Claude-specific name), `UnreachableCause` (`StrictModes` · `InstanceDead`
