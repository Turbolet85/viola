@leaf_group group=test-plan+obs-plan
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, s
@leaf_edit n=1 leaf=.claude/rules/observability.md line=38 class=omission
leaf-text: the outcome rides only the stamps.
source: dec-k5mswhye «The reading of the CLI's browser-tools prompt as a modal adds no log line, span, event or detail code of its own; verify's reading of it reaches only the ledger stamp and verify's stdout.»
replace-with: the outcome rides only the stamps. The reading of the CLI's browser-tools prompt as a modal adds no log line, span, event or detail code of its own; verify's reading of it (its `absent` step word included) reaches only the ledger stamp and verify's stdout.
