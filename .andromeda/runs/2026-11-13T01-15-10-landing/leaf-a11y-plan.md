@leaf_group group=a11y-plan
@leaf_read leaf=.claude/docs/a11y-summary.md lines=58
last: **Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/a11y.md lines=30
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/docs/a11y-summary.md line=46 class=stale
leaf-text: NEVER parse the rendered `claude` TUI or take screenshots as a11y evidence.
source: dec-22ngmman «No screen-content parsing beyond signatures — The rendered `claude` TUI screen is parsed only for compiled input-box and modal signatures, never for other content, and screenshots are never taken as a11y evidence (R7).»
replace-with: NEVER parse the rendered `claude` TUI beyond its compiled input-box and modal signatures, or take screenshots as a11y evidence (R7).
