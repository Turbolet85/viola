@leaf_group group=obs-plan
@leaf_read leaf=.claude/docs/obs-summary.md lines=77
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: `
@leaf_edit n=1 leaf=.claude/docs/obs-summary.md line=47 class=stale
leaf-text: veil `#[derive(Redact)]` on payload types (`toggle` banned)
source: dec-4agjnrnu «Every channel, hook and MCP payload type that carries content derives veil Redact, so Debug never reveals content; a payload type with no content field derives none.»
replace-with: veil `#[derive(Redact)]` on every payload type that carries content (`toggle` banned); a payload type with no content field derives neither `Redact` nor `Debug`
