@leaf_group group=architecture-4
@leaf_read leaf=.claude/rules/events.md lines=40
last: - 2026-10-16: A snapshot field that a reader expects once an
@leaf_edit n=1 leaf=.claude/rules/events.md line=16 class=omission
leaf-text: `launch` `{model, effort}` (both strings, `source: wrapper`, appended once at every start, after `lineage`).
source: ctr-t3m4inxx «The wrapper appends one `window` line with `source: wrapper` after the `launch` line, only when started under --origin; it is log-only and never wakes `wait`.»
replace-with: `launch` `{model, effort}` (both strings, `source: wrapper`, appended once at every start, after `lineage`), `window` `{address, workspace, placement, origin}` (`source: wrapper`, placement and origin closed words, appended once after `launch` only when started under --origin).
@leaf_edit n=2 leaf=.claude/rules/events.md line=17 class=omission
leaf-text: `activity`, `link`, `unlink`, `wheel`, `lineage`, `launch`, a harness-origin
source: dec-yyuuzpxg «Event kinds the wrapper appends (`wheel`, `lineage`, `launch`, `window`, `send-issued`, `send-confirmed`, `send-refused`, `link`, `unlink`) are log-only and never wake `wait`.»
replace-with: `activity`, `link`, `unlink`, `wheel`, `lineage`, `launch`, `window`, a harness-origin
@leaf_edit n=3 leaf=.claude/rules/events.md line=30 class=omission
leaf-text: replay recovers only `links`, `agent_session_id`, `wheel`, `lineage`, `launch` (`dialog_pending` reads false;
source: ctr-navndiay «`window` is `{address?, workspace?, placement, origin}`, the wrapper's window record, omitted when absent; a replay rebuilds it from the log, the last `window` line after the last `launch` line winning and a»
replace-with: replay recovers only `links`, `agent_session_id`, `wheel`, `lineage`, `launch`, `window` (`dialog_pending` reads false; `window` is the last `window` line after the last `launch` line, a later `launch` line clearing it;
