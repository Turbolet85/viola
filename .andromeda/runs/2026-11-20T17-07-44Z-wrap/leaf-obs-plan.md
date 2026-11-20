@leaf_group group=obs-plan
@leaf_read leaf=.claude/docs/obs-summary.md lines=76
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: `
@leaf_read leaf=.claude/rules/ci.md lines=36
last: - 2026-11-20: A WATCH's green run read from CI counts only on
@leaf_edit n=1 leaf=.claude/docs/obs-summary.md line=72 class=stale
leaf-text: `config.json` has four known keys (`v`, `diagnostics_level`, `claude_env_keep`, `dialog_hold_s`)
source: ctr-4hxbzvo4 «config.json has five known keys: v, diagnostics_level, claude_env_keep, dialog_hold_s, verify_new_cli»
replace-with: `config.json` has five known keys (`v`, `diagnostics_level`, `claude_env_keep`, `dialog_hold_s`, `verify_new_cli`)
