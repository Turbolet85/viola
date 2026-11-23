@leaf_group group=test-plan+obs-plan
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, so
@leaf_edit n=1 leaf=.claude/rules/observability.md line=41 class=omission
leaf-text: The wrapper's stdin typed mark and its held scheduled-wakes count are in-memory state
source: dec-5s63dkyk «The lineage line written at every run and revive start is log-only product audit data: it wakes no wait and writes no process-log line, span or terminal byte of its own.»
replace-with: The lineage line written at every run and revive start is events-log product audit data: it wakes no wait and writes no process-log line, span or terminal byte of its own. The wrapper's stdin typed mark and its held scheduled-wakes count are in-memory state
@leaf_edit n=2 leaf=.claude/rules/observability.md line=41 class=omission
leaf-text: `driver`, `driven` and `since` reach no home-level line.
source: dec-3tlgpfr6 «A start's parent and role names appear only in events.ndjson, the snapshot, the viola list board, list --json and the MCP list result, each parent re-parsed as a viola name and each role as a code word»
replace-with: `driver`, `driven` and `since` reach no home-level line; a start's `parent` and `role` names reach no process-log or detail line (only `events.ndjson`, the snapshot, the `viola list` board, `list --json` and the MCP list result).
@leaf_stands n=1 leaf=.claude/rules/observability.md line=25
because: The term the row lost is "in v1", while this line's v1 is only the `schemas/diag-line.v1.json` file name, which ctr-q5dspexy never bore on.
@leaf_stands n=2 leaf=.claude/rules/observability.md line=28
because: The line's v1 is the `schemas/diag-line.v1.json` schema name, not the list-columns "in v1" qualifier ctr-q5dspexy dropped, so it still states what obs-plan says.
@leaf_stands n=3 leaf=.claude/rules/observability.md line=38
because: The line's v1 appears only in the `schemas/diag-line.v1.json` schema name, unrelated to ctr-q5dspexy's dropped "in v1" about list columns, so the line still holds.
