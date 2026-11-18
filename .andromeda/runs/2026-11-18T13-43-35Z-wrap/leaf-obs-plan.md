@leaf_group group=obs-plan
@leaf_read leaf=.claude/docs/obs-summary.md lines=76
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: `
@leaf_edit n=1 leaf=.claude/docs/obs-summary.md line=42 class=omission
leaf-text: `state-recovered` · `parse-rejected`.
source: ctr-5gjcaq4l «contract/diag-line-fields/trust-record · diagnostic line additive fields · trust-record — A fixed message and a required detail drawn from the 16 start words; trust-record stands in the event enum of both the diag-line and the detail-line schema.»
replace-with: `state-recovered` · `parse-rejected` · `trust-record` (one INFO line per wrapped `viola run` / `viola revive` start on a verified CLI, in the run role file, its required `detail` one of 16 closed start words, no path, project key or config value; it never refuses the start).
