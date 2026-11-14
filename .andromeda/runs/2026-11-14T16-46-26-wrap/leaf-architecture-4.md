@leaf_group group=architecture-4
@leaf_read leaf=.claude/rules/events.md lines=40
last: - 2026-10-16: A snapshot field that a reader expects once an
@leaf_edit n=1 leaf=.claude/rules/events.md line=16 class=retired
leaf-text: `wheel`, `budget-gate`, `send-issued`
source: ctr-zpc6ox2b «contract/normalised-event-kinds/budget-gate · normalised event kinds · budget-gate — `budget-gate` is a normalised event kind.»
replace-with: `wheel`, `send-issued`
@leaf_edit n=2 leaf=.claude/rules/events.md line=17 class=retired
leaf-text: `wheel`, `budget-gate` and the send records are log-only
source: dec-yyuuzpxg «decision/wrapper-events-log-only · Wrapper-appended events are log-only — Event kinds the wrapper appends (`wheel`, `send-issued`, `send-confirmed`, `send-refused`, `link`, `unlink`) are log-only and never wake `wait`.»
replace-with: `wheel` and the send records are log-only
@leaf_edit n=3 leaf=.claude/rules/events.md line=30 class=stale
leaf-text: replay recovers only `links`, `agent_session_id`, `wheel`, `budget_paused`, `budget_override_until` (`dialog_pending` reads false).
source: ctr-z3syrfob «contract/snapshot-envelope/unreadable-snapshot · snapshot envelope · unreadable snapshot — A snapshot whose `v` is higher than the reader supports, or that fails to parse, is ignored and state is rebuilt by re»
replace-with: replay recovers only `links`, `agent_session_id`, `wheel` (`dialog_pending` reads false).
@leaf_edit n=4 leaf=.claude/rules/events.md line=30 class=retired
leaf-text: The wrapper's budget gate writes `budget-gate` `{paused, window?, override_until?}` once at start as record two and then only when the triple (paused, window, override end) changes, and mirrors it to the snapshot's `budget_paused` / `budget_override_until` in the same step.
source: ctr-ya3pw7hs «contract/wrapper-event-appends/budget-gate · wrapper event appends · `budget-gate` — The wrapper appends `budget-gate` with `source: wrapper` once at start and on every change.»
replace-with: The wrapper appends no budget-gate line and the snapshot holds no budget field.
