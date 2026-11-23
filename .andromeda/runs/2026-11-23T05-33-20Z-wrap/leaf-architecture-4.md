@leaf_group group=architecture-4
@leaf_read leaf=.claude/rules/events.md lines=40
last: - 2026-10-16: A snapshot field that a reader expects once an e
@leaf_edit n=1 leaf=.claude/rules/events.md line=30 class=stale
leaf-text: replay recovers only `links`, `agent_session_id`, `wheel` (`dialog_pending` reads false)
source: ctr-z3syrfob «A snapshot whose `v` is higher than the reader supports, or that fails to parse, is ignored and state is rebuilt by replaying `events.ndjson`, recovering only `links`, `agent_session_id`, `wheel`, `lineage`; the replay writes no file.»
replace-with: replay recovers only `links`, `agent_session_id`, `wheel`, `lineage` (`dialog_pending` reads false; the last `lineage` line replaces the held value)
@leaf_edit n=2 leaf=.claude/rules/events.md line=16 class=omission
leaf-text: `dialog-expired` `{dialog_id, dialog_kind}` (`source: wrapper`, appended once when a held dialog's hold ends; it carries no question, plan or tool text).
source: ctr-zkohdqkb «contract/normalised-event-kinds/lineage · normalised event kinds · lineage — `lineage` is a normalised event kind.»
replace-with: `dialog-expired` `{dialog_id, dialog_kind}` (`source: wrapper`, appended once when a held dialog's hold ends; it carries no question, plan or tool text), `lineage` `{parent, role}` (each a string or `null`, `source: wrapper`).
@leaf_edit n=3 leaf=.claude/rules/events.md line=17 class=omission
leaf-text: `unlink`, `wheel`, a harness-origin `prompt-submitted`
source: dec-yyuuzpxg «Event kinds the wrapper appends (`wheel`, `lineage`, `send-issued`, `send-confirmed`, `send-refused`, `link`, `unlink`) are log-only and never wake `wait`.»
replace-with: `unlink`, `wheel`, `lineage`, a harness-origin `prompt-submitted`
@leaf_stands n=1 leaf=.claude/rules/events.md line=11
because: The line names only the log's scope and that the on-disk log is the single source of truth, which the revised revive-in-place and unreadable-snapshot rows still hold (the log appended to, never truncated, replay from `events.ndjson`).
@leaf_stands n=2 leaf=.claude/rules/events.md line=25
because: The revised revive-in-place row still says `events.ndjson` is appended to and never truncated and instance dirs are resumed in place, exactly as the line says.
