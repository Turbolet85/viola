@leaf_group group=architecture-3
@leaf_read leaf=.claude/docs/stack.md lines=67
last: 5. Commit as `chore: bump {tool} to {version}`
@leaf_read leaf=.claude/docs/workflow.md lines=68
last: This project uses a **reference-based** CLAUDE.md architectu
@leaf_read leaf=.claude/rules/events.md lines=41
last: - 2026-10-16: A snapshot field that a reader expects once an
@leaf_edit n=1 leaf=.claude/rules/events.md line=16 class=stale
leaf-text: `turn-ended` `{last_assistant_message, background_tasks, cause?}`
source: ctr-ydq6fc5q «`turn-ended` data is `{last_assistant_message, background_tasks, scheduled_wakes, cause?}`; `last_assistant_message` is a string or `null` and is the source `last` rebuilds from.»
replace-with: `turn-ended` `{last_assistant_message, background_tasks, scheduled_wakes, cause?}`
@leaf_edit n=2 leaf=.claude/rules/events.md line=17 class=omission
leaf-text: every `turn-ended` wakes whatever its `background_tasks`,
source: ctr-yns6vswp «so every `turn-ended` wakes whatever its `background_tasks` or `scheduled_wakes`; a finished or paused turn is for the driver to tell, and a parked `wait` never polls the file.»
replace-with: every `turn-ended` wakes whatever its `background_tasks` or `scheduled_wakes`,
@leaf_edit n=3 leaf=.claude/rules/events.md line=20 class=omission
leaf-text: a count only, or `null` when the key is absent, `null` or not an array);
source: ctr-fpzfqovz «`scheduled_wakes` counts the session's scheduled wakes: the Stop payload's `session_crons` array length with no element read or carried, `null` when the key is absent, `null` or not an array»
replace-with: a count only, or `null` when the key is absent, `null` or not an array; `scheduled_wakes`, the Stop payload's `session_crons` array length, a count only, or `null` when the key is absent, `null` or not an array);
@leaf_edit n=4 leaf=.claude/rules/events.md line=20 class=stale
leaf-text: `background_tasks` null and `cause` one of the nine `TurnEndCause` words
source: ctr-nyxx7utw «its data exactly `last_assistant_message` null, `background_tasks` null, `scheduled_wakes` null and `cause` one of the nine TurnEndCause words»
replace-with: `background_tasks` null, `scheduled_wakes` null and `cause` one of the nine `TurnEndCause` words
@leaf_edit n=5 leaf=.claude/docs/workflow.md line=49 class=stale
leaf-text: all twenty-two on every build
source: dec-ucvfrmxd «A PermissionRequest allow with updatedInput would satisfy it, but that body has no ledger row; adding one needs a 22nd row, its viola verify probe and a re-stamp of every stamped version.»
replace-with: all twenty-one on every build
