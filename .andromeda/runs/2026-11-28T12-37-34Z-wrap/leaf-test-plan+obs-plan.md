@leaf_group group=test-plan+obs-plan
@leaf_read leaf=.claude/rules/observability.md lines=47
last: - 2026-11-28: A new `ObsEvent` moves three lists, not two: bo
@leaf_edit n=1 leaf=.claude/rules/observability.md line=21 class=omission
leaf-text: `viola console` runs its strict-modes check before the log init too: a refusal is exit 1 with the home pair, no line and the mode left as found.
source: ctr-rjgdvz3d «At its start, exit 1 with the home pair, made before any log opens: no line, the mode left as found; on a board scan refused mid-run, exit 1 with the home pair after the terminal's restore and one process-exit line, detail strict-modes-failed.»
replace-with: `viola console` runs its strict-modes check before the log init too: a refusal at its start is exit 1 with the home pair, no line and the mode left as found; a board scan refused mid-run is exit 1 with the home pair after the terminal's restore and one `process-exit{detail:"strict-modes-failed"}` line in `console.ndjson`.
