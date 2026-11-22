@leaf_group group=a11y-plan
@leaf_read leaf=.claude/docs/a11y-summary.md lines=58
last: **Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/a11y.md lines=30
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/docs/a11y-summary.md line=47 class=stale
leaf-text: past the current atomic paste
source: dec-2b576hni «A human keystroke in `viola run`, or in a start a passed `viola revive` made, is never blocked, refused or delayed past the current paste's in-flight chunk and closing marker.»
replace-with: past the current paste's in-flight chunk and closing marker
@leaf_edit n=2 leaf=.claude/rules/a11y.md line=19 class=stale
leaf-text: human keystrokes are never blocked, refused or delayed past the atomic paste
source: ctr-3n4o5u6h «Human keystrokes are never blocked, refused or dropped once the wrapped child holds the terminal: queued with the paste in arrival order, they wait behind at most its in-flight chunk and closing marker»
replace-with: human keystrokes are never blocked, refused or dropped: queued with the paste in arrival order, they wait behind at most its in-flight chunk and closing marker
