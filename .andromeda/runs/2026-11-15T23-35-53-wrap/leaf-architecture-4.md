@leaf_group group=architecture-4
@leaf_read leaf=.claude/rules/events.md lines=40
last: - 2026-10-16: A snapshot field that a reader expects once an
@leaf_edit n=1 leaf=.claude/rules/events.md line=16 class=omission
leaf-text: `session-start`, `turn-ended`, `prompt-submitted`
source: ctr-ydq6fc5q «contract/event-data/turn-ended-last-assistant-message · event data per kind · turn-ended last_assistant_message — `turn-ended` data is `{last_assistant_message, background_tasks, cause?}`; `last_assistant_message` is a string or `null` and is the source `last` rebuilds from.»
replace-with: `session-start`, `turn-ended` `{last_assistant_message, background_tasks, cause?}` (`cause` present only on an API-error turn end, as `api-error`, one word of the closed `TurnEndCause` set; absent on a Stop end), `prompt-submitted`
@leaf_edit n=2 leaf=.claude/rules/events.md line=20 class=omission
leaf-text: SessionEnd → `session-end`; Notification
source: ctr-nyxx7utw «contract/hook-to-kind-map/stopfailure · Hook to kind map · StopFailure — StopFailure (`viola hook stop-failure`) maps to `turn-ended`, not a dialog, its data exactly `last_assistant_message` null, `background_tasks` null and `cause` `api-error`.»
replace-with: StopFailure (`viola hook stop-failure`) → `turn-ended`, not a dialog (data exactly `last_assistant_message` null, `background_tasks` null and `cause` `api-error`); SessionEnd → `session-end`; Notification
