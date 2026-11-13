@leaf_group group=a11y-plan
@leaf_read leaf=.claude/docs/a11y-summary.md lines=58
last: **Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules: 
@leaf_read leaf=.claude/rules/a11y.md lines=30
last: _This section is owned by `/wrap-session`. setup-project pres
@leaf_edit n=1 leaf=.claude/docs/a11y-summary.md line=46 class=stale
leaf-text: NEVER parse the rendered `claude` TUI beyond its compiled input-box and modal signatures, or take screenshots as a11y evidence (R7).
source: dec-22ngmman «The rendered `claude` TUI screen is parsed only for compiled signatures (footer literals, the input box's rule and prompt rows, modal literals), the prompt row only for its opening char, never the text after it»
replace-with: NEVER parse the rendered `claude` TUI beyond its compiled signatures (footer literals, the input box's rule and prompt rows, modal literals), the prompt row only for its opening char and never the text after it, or take screenshots as a11y evidence (R7).
