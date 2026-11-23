@leaf_group group=obs-plan
@leaf_read leaf=.claude/docs/obs-summary.md lines=76
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: 
@leaf_edit n=1 leaf=.claude/docs/obs-summary.md line=72 class=stale
leaf-text: `config.json` has five known keys (`v`, `diagnostics_level`, `claude_env_keep`, `dialog_hold_s`, `verify_new_cli`)
source: ctr-4hxbzvo4 «contract/config-json/known-keys · config.json · known keys — config.json has seven known keys: v, diagnostics_level, claude_env_keep, dialog_hold_s, verify_new_cli, default_model, default_effort;»
replace-with: `config.json` has seven known keys (`v`, `diagnostics_level`, `claude_env_keep`, `dialog_hold_s`, `verify_new_cli`, `default_model`, `default_effort`)
