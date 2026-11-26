@leaf_group group=test-plan+obs-plan
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, s
@leaf_edit n=1 leaf=.claude/rules/observability.md line=41 class=omission
leaf-text: follows the trust step, before `state.snapshot_write`.
source: dec-kkpxti4a «its restore of the earlier active workspace when the move changed it, add no placement word, span field or log line of their own»
replace-with: follows the trust step, before `state.snapshot_write`; the window step's reads of the active workspace before and after its move, and its restore of the earlier active workspace when the move changed it, add no placement word, span field or log line of their own (a failed read or restore leaves the placement word as the move set it).
@leaf_edit n=2 leaf=.claude/rules/observability.md line=39 class=omission
leaf-text: ends that log with `process-exit{exit_code:1, detail:"strict-modes-failed"}`.
source: ctr-pecbg5rc «viola up ends exit 1 with process-exit detail not-live when any instance reads not-live, else with no detail (an exit 1 from no-session, cwd-missing, session-live or not-taken alone)»
replace-with: ends that log with `process-exit{exit_code:1, detail:"strict-modes-failed"}`; any other exit 1 of `viola up` carries `process-exit` detail `not-live` when an instance reads not-live, else no detail (no-session, cwd-missing, session-live or not-taken alone), and each of its refusals writes its own detail word.
