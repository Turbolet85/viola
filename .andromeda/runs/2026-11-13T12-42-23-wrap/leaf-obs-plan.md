@leaf_group group=obs-plan
@leaf_read leaf=.claude/docs/obs-summary.md lines=76
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: `.
@leaf_edit n=1 leaf=.claude/docs/obs-summary.md line=29 class=stale
leaf-text: 0 (`Cookie` header, `CLAUDE*` values, canary in home-level files)
source: dec-4irii5aq «CI runs the secret scan over every diagnostics/ file before any artifact upload, its classes being CLAUDE* values, the content canary, mode 0600 and an unreadable file; a Cookie header line is no class and no hit.»
replace-with: 0 (`CLAUDE*` values, the content canary, mode 0600 and an unreadable file, over every `diagnostics/` file; a `Cookie` header line is no class and no hit)
