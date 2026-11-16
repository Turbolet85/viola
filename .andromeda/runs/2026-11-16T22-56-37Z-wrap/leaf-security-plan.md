@leaf_group group=security-plan
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_read leaf=.claude/rules/security.md lines=45
last: - 2026-11-16: Write a bounded read as `take(reader, MAX_FRAME).r
@leaf_edit n=1 leaf=.claude/rules/security.md line=25 class=stale
leaf-text: its turn-ended data holding only `null` `last_assistant_message` and `background_tasks` and one word of the closed nine-word turn-end cause set
source: dec-woerghls «an API-error end's turn-ended data holds only null last_assistant_message, background_tasks and scheduled_wakes and one nine-word cause set word, crossing into events.ndjson, wait --json and MCP wait.»
replace-with: its turn-ended data holding only `null` `last_assistant_message`, `background_tasks` and `scheduled_wakes` and one word of the closed nine-word turn-end cause set
@leaf_edit n=2 leaf=.claude/rules/security.md line=25 class=omission
leaf-text: and the Stop's `background_tasks` is read as `IgnoredAny` elements and carried only as its length (`null` when absent, `null` or not an array);
source: dec-cbbijivg «Stop's session_crons is carried only as its length, scheduled_wakes on turn-ended, no element read or kept; null when absent, null or not an array, a non-array also writing one drift entry to the diagnostics»
replace-with: and the Stop's `background_tasks` is read as `IgnoredAny` elements and carried only as its length (`null` when absent, `null` or not an array), and its `session_crons` likewise only as its length, `scheduled_wakes` on turn-ended, no element read or kept (`null` when absent, `null` or not an array, a non-array also writing one drift entry to the diagnostics detail file; `null` on an API-error turn end);
