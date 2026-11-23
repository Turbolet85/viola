@leaf_group group=architecture-4
@leaf_read leaf=.claude/rules/events.md lines=40
last: - 2026-10-16: A snapshot field that a reader expects once an
@leaf_edit n=1 leaf=.claude/rules/events.md line=16 class=omission
leaf-text: `lineage` `{parent, role}` (each a string or `null`, `source: wrapper`).
source: ctr-xw4xo2fx «contract/event-data/launch · event data · `launch` — `{model, effort}`, both strings, with `source: wrapper`.»
replace-with: `lineage` `{parent, role}` (each a string or `null`, `source: wrapper`), `launch` `{model, effort}` (both strings, `source: wrapper`, appended once at every start, after `lineage`).
@leaf_edit n=2 leaf=.claude/rules/events.md line=17 class=omission
leaf-text: `wheel`, `lineage`, a harness-origin
source: ctr-yemjpoxs «contract/wrapper-event-appends/launch · wrapper event appends · `launch` — The wrapper appends `launch` with `source: wrapper` once at every start, after `lineage`; it is log-only and never wakes `wait`.»
replace-with: `wheel`, `lineage`, `launch`, a harness-origin
@leaf_edit n=3 leaf=.claude/rules/events.md line=30 class=stale
leaf-text: replay recovers only `links`, `agent_session_id`, `wheel`, `lineage` (`dialog_pending` reads false; the last `lineage` line replaces the held value).
source: ctr-wjadxiye «contract/instance-snapshot/launch · instance snapshot · launch — `launch` is `{model, effort}`, the pair the start handed, omitted when absent with no snapshot `v` move; a replay rebuilds it from the log, the last `launch` line replacing the held value»
replace-with: replay recovers only `links`, `agent_session_id`, `wheel`, `lineage`, `launch` (`dialog_pending` reads false; the last `lineage` line replaces the held value, as the last `launch` line does, a `launch` line with a non-string member reading none).
