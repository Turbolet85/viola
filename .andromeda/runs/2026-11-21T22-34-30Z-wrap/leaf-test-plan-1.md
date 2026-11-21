@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=65
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules: 
@leaf_read leaf=.claude/rules/testing.md lines=129
last: - 2026-11-21: A test helper that rebuilds a `Command` from `ge
@leaf_edit n=1 leaf=.claude/rules/testing.md line=28 class=stale
leaf-text: `viola-mcp` runs no tokio timer, so no paused test clock
source: dec-d6bwecvy «decision/time-faked-through-seams · Time faked through injected clocks — Time is faked only through injected seams, with no mock_instant and no paused Tokio clock: the viola_core Clock, the MCP adapter's injected deadline provider»
replace-with: no paused Tokio clock; `viola-mcp`'s 55 s MCP call deadline comes through its injected deadline provider, which a test fires by a signal, never by waiting out the 55 s
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=27 class=omission
leaf-text: `mcp_tools_a_wrapper_killed_under_a_parked_wait_reads_endpoint_lost`)
source: ctr-k4p56hxt «contract/mcp-unreachable-detail/call-deadline · MCP frame tools instance-unreachable detail · call-deadline — Each of send, wait, last and answer ends with detail call-deadline at the MCP call deadline, the frame dropped»
replace-with: `mcp_tools_a_wrapper_killed_under_a_parked_wait_reads_endpoint_lost`; the MCP `send`, `wait`, `last` and `answer` alone end with `detail` `call-deadline` at the 55 s MCP call deadline, the frame dropped, a body with no CLI document, and a reply arriving before the deadline is returned as the reply)
