@leaf_group group=obs-plan
@leaf_read leaf=.claude/docs/obs-summary.md lines=76
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: 
@leaf_edit n=1 leaf=.claude/docs/obs-summary.md line=69 class=stale
leaf-text: `hook_event` has nine closed names
source: ctr-ahargk56 «contract/diag-line-fields/hook-events · diagnostic line additive fields · hook-invoked / hook-decision — hook_event (one of ten closed hook names), invoked_at, decision_emitted, deadline_hit, detail, duration_ms.»
replace-with: `hook_event` has ten closed names
