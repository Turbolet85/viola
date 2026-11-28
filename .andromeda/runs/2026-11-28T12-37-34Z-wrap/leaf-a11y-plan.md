@leaf_group group=a11y-plan
@leaf_read leaf=.claude/docs/a11y-summary.md lines=58
last: **Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules
@leaf_read leaf=.claude/rules/a11y.md lines=30
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/rules/a11y.md line=19 class=stale
leaf-text: reads keys from stdin in raw mode and redraws on a resize;
source: dec-3f4o5u3u «except: the terminal fleet console (`viola console`), its own surface, which reads keys from stdin in raw mode and redraws on a resize or a change in the fleet it shows»
replace-with: reads keys from stdin in raw mode and redraws on a resize or a change in the fleet it shows;
