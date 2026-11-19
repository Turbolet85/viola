@leaf_group group=obs-plan
@leaf_read leaf=.claude/docs/obs-summary.md lines=76
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: `.
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, so
@leaf_edit n=1 leaf=.claude/rules/observability.md line=38 class=omission
leaf-text: verify's reading of it (its `absent` step word included) reaches only the ledger stamp and verify's stdout.
source: dec-k5mswhye «verify's reading of it reaches only the ledger stamp and verify's stdout, whose one fixed line on a met prompt is the only record that the prompt stood at start.»
replace-with: verify's reading of it (its `absent` step word included) reaches only the ledger stamp and verify's stdout, whose one fixed line on a met prompt is the only record that the prompt stood at start.
