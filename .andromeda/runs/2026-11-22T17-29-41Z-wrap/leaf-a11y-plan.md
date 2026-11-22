@leaf_group group=a11y-plan
@leaf_read leaf=.claude/docs/a11y-summary.md lines=58
last: **Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:
@leaf_read leaf=.claude/rules/a11y.md lines=30
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/docs/a11y-summary.md line=53 class=stale
leaf-text: send readiness gate, confirmation windows
source: dec-3cppjeep «Every time bound viola holds (wait timeout and cap, MCP call deadline, dialog hold, send readiness gate, paste write, confirmation windows, agents read, the start's Claude config lock wait) falls on the driver»
replace-with: send readiness gate, paste write, confirmation windows
@leaf_edit n=2 leaf=.claude/docs/a11y-summary.md line=27 class=omission
leaf-text: cli (`viola answer`, `unverified-cli` refusal)
source: dec-3o2t22ez «A pending dialog answered with viola answer (question, permission, plan, and a question first raised by PermissionRequest), including the unverified-cli refusal that withholds answers»
replace-with: cli (`viola answer` for a question, permission, plan, or a question first raised by PermissionRequest; `unverified-cli` refusal)
