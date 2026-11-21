@leaf_group group=a11y-plan
@leaf_read leaf=.claude/docs/a11y-summary.md lines=58
last: **Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules: 
@leaf_read leaf=.claude/rules/a11y.md lines=30
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/docs/a11y-summary.md line=53 class=stale
leaf-text: Every time bound viola holds (wait timeout and cap, dialog hold, send readiness gate, confirmation windows, agents read, the start's Claude config lock wait) falls on the driver, viola's child or viola's own start; no key is blocked, refused or delayed by it.
source: dec-3cppjeep «Every time bound viola holds (wait timeout and cap, MCP call deadline, dialog hold, send readiness gate, confirmation windows, agents read, the start's Claude config lock wait) falls on the driver»
replace-with: Every time bound viola holds (wait timeout and cap, MCP call deadline, dialog hold, send readiness gate, confirmation windows, agents read, the start's Claude config lock wait) falls on the driver, viola's child or viola's own start; no key is blocked, refused or delayed by it. An MCP frame tool call (send, wait, last, answer) that outlives the MCP call deadline (55 s) ends `instance-unreachable` / `call-deadline`, the outcome of a send unknown; only the MCP frame tools write that detail, never the CLI, whose exit-21 cause set stays six.
