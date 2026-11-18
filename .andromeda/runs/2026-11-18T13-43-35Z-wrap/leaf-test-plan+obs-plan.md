@leaf_group group=test-plan+obs-plan
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, s
@leaf_edit n=1 leaf=.claude/rules/observability.md line=38 class=omission
leaf-text: the outcome rides only the stamps, and no path, key or value of the claude config reaches any line, stamp or summary.
source: dec-r6ir5lzb «viola verify's trust-record write and removal emit no log line and no span, the outcome riding only the stamps; a start's trust step opens no span and writes only its one trust-record line; no path, key or value»
replace-with: the outcome rides only the stamps. Each wrapped start of `viola run` and `viola revive` on a verified CLI opens no span for its trust step and writes exactly one INFO `trust-record` line to the run role file, its required `detail` one of a closed set of 16 start words (`trust-record` in the event enum of both the diag-line and the detail-line schema), and the word never refuses the start; no path, key or value of the claude config reaches any line, stamp or summary.
