@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - For warnings and gotchas, see `.claude/docs/gotchas.md`
@leaf_read leaf=.claude/docs/services/viola-agent-claude.md lines=47
last: - `doc:architecture` [CLI Version Compatibility] · `.claude/
@leaf_read leaf=.claude/docs/services/viola-channel.md lines=45
last: - `doc:architecture` §Standard Contracts (Wrapper channel fr
@leaf_read leaf=.claude/docs/services/viola-core.md lines=43
last: - `.claude/docs/stack.md` · `.claude/docs/conventions.md` · 
@leaf_read leaf=.claude/docs/services/viola-mcp.md lines=39
last: - `doc:architecture` [MCP] · `.claude/rules/api.md`
@leaf_edit n=1 leaf=.claude/docs/services/viola-agent-claude.md line=25 class=stale
leaf-text: its data exactly `last_assistant_message` null, `background_tasks` null and `cause` one of the nine TurnEndCause words
source: ctr-nyxx7utw «StopFailure (`viola hook stop-failure`) maps to `turn-ended`, not a dialog, its data exactly `last_assistant_message` null, `background_tasks` null, `scheduled_wakes` null and `cause` one of the nine»
replace-with: its data exactly `last_assistant_message` null, `background_tasks` null, `scheduled_wakes` null and `cause` one of the nine TurnEndCause words
@leaf_edit n=2 leaf=.claude/docs/services/viola-agent-claude.md line=25 class=stale
leaf-text: Stop's `turn-ended` data is `{last_assistant_message, background_tasks}`, with no `cause`
source: ctr-ydq6fc5q «`turn-ended` data is `{last_assistant_message, background_tasks, scheduled_wakes, cause?}`; `last_assistant_message` is a string or `null` and is the source `last` rebuilds from.»
replace-with: Stop's `turn-ended` data is `{last_assistant_message, background_tasks, scheduled_wakes}`, with no `cause`
@leaf_edit n=3 leaf=.claude/docs/services/viola-agent-claude.md line=25 class=omission
leaf-text: `null` when the key is absent, `null` or not an array).
source: ctr-fpzfqovz «`scheduled_wakes` counts the session's scheduled wakes: the Stop payload's `session_crons` array length with no element read or carried, `null` when the key is absent, `null` or not an array, and on»
replace-with: `null` when the key is absent, `null` or not an array; `scheduled_wakes` the Stop payload's `session_crons` array length only, no element read or carried, `null` when the key is absent, `null` or not an array, and on an API-error turn end).
