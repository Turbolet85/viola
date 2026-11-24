@leaf_group group=a11y-plan
@leaf_read leaf=.claude/docs/a11y-summary.md lines=58
last: **Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/a11y.md lines=30
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/docs/a11y-summary.md line=28 class=omission
leaf-text: cli (`pause`, `release`, `send` refused with `human-typing`)
source: dec-3ijhwhmf «When the human takes the wheel, `send` and `switch` are refused with `human-typing` on stderr and a typed exit code.»
replace-with: cli (`pause`, `release`, `send` and `switch` refused with `human-typing`)
@leaf_edit n=2 leaf=.claude/docs/a11y-summary.md line=53 class=omission
leaf-text: and the wheel is untouched. An MCP frame tool call
source: dec-khujfvwn «A `viola switch` waits at most 10 s (RESPAWN_REAP_BOUND) for the old child to exit after its hang-up; a child not exited is not replaced, and the switch is refused not-delivered child-not-ended, exit 13.»
replace-with: and the wheel is untouched. A `viola switch` waits at most 10 s (`RESPAWN_REAP_BOUND`) for the old child to exit after its hang-up; a child not exited is not replaced, and the switch is refused `not-delivered` / `child-not-ended`, exit 13: the bound falls on the wrapper, never on the human (provisional). An MCP frame tool call
@leaf_edit n=3 leaf=.claude/rules/a11y.md line=22 class=omission
leaf-text: plain ASCII, with no SGR, each line in one write.
source: ctr-tvudi4hs «`viola switch` prints one line, `<name>  switched  <model> <effort>`, on stdout in plain ASCII with no SGR.»
replace-with: plain ASCII, with no SGR, each line in one write, and `viola switch` prints one line, `<name>  switched  <model> <effort>`, on stdout in plain ASCII with no SGR.
