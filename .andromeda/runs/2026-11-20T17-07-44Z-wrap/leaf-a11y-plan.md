@leaf_group group=a11y-plan
@leaf_read leaf=.claude/docs/a11y-summary.md lines=58
last: **Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/a11y.md lines=30
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/docs/a11y-summary.md line=47 class=omission
leaf-text: past the current atomic paste, and never count focus
source: dec-2b576hni «except: The start's automatic verify window, before any child exists: a key typed then waits in the terminal's own buffer until the child starts.»
replace-with: past the current atomic paste (the one exception: a start's automatic verify window, before any child exists, where a typed key waits in the terminal's own buffer until the child starts), and never count focus
@leaf_edit n=2 leaf=.claude/rules/a11y.md line=19 class=omission
leaf-text: human keystrokes are never blocked, refused or delayed past the atomic paste,
source: ctr-3n4o5u6h «the one exception is a start's automatic verify window, before the child starts.»
replace-with: once the wrapped child holds the terminal, human keystrokes are never blocked, refused or delayed past the atomic paste (the one exception is a start's automatic verify window, before the child starts: a key typed then waits in the terminal's own buffer),
@leaf_edit n=3 leaf=.claude/rules/a11y.md line=19 class=omission
leaf-text: tui: the terminal belongs to the child (
source: ctr-3ngwvju6 «Once the wrapped child holds the terminal viola writes zero bytes of its own to it; only a start's automatic verify lines L1 to L5 and verify's own output precede the child's bytes.»
replace-with: tui: once the child starts the terminal belongs to the child, and viola writes zero bytes of its own to it (only a start's automatic verify lines L1 to L5 and verify's own output precede the child's bytes) (
