@leaf_group group=test-plan+obs-plan
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, so
@leaf_edit n=1 leaf=.claude/rules/observability.md line=28 class=omission
leaf-text: written by the Hyprland seam (never `unknown`).
source: ctr-uueewfdf «A StopFailure error word outside the CLI's 13 writes one WARN parse-rejected line with parser hook-stdin, detail unknown-value and count 1, carrying the code and count only, and the event is still delivered»
replace-with: written by the Hyprland seam (never `unknown`). A StopFailure error word outside the CLI's 13 writes one WARN `parse-rejected{parser:"hook-stdin", detail:"unknown-value", count:1}` (code and count only) and the event is still delivered as a turn end with cause `api-error`; a known word writes no line.
