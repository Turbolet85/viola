@leaf_group group=test-plan+obs-plan
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, s
@leaf_edit n=1 leaf=.claude/rules/observability.md line=38 class=omission
leaf-text: and the fixture's `<file>` enters no code.
source: dec-r6ir5lzb «viola verify's trust-record write and removal emit no log line and no span; the outcome rides only the stamps, as the trust-record row's word and the trust probe's write and remove words, and no path, key or value»
replace-with: and the fixture's `<file>` enters no code. `viola verify`'s trust-record write and removal emit no log line and no span (a refused symlinked, non-regular or foreign file included); the outcome rides only the stamps, and no path, key or value of the claude config reaches any line, stamp or summary.
