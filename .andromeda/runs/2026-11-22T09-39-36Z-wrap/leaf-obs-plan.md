@leaf_group group=obs-plan
@leaf_read leaf=.claude/docs/obs-summary.md lines=76
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: 
@leaf_edit n=1 leaf=.claude/docs/obs-summary.md line=67 class=omission
leaf-text: exit 21 `instance-dead|unwrapped-name|endpoint-lost|strict-modes-failed|server-verify-failed|socket-dir-refused`).
source: ctr-7mzj6yuo «The instance-unreachable detail is one closed cause: strict-modes-failed, instance-dead, unwrapped-name, endpoint-lost, server-verify-failed, socket-dir-refused or call-deadline»
replace-with: exit 21 `instance-dead|unwrapped-name|endpoint-lost|strict-modes-failed|server-verify-failed|socket-dir-refused`; `viola mcp`'s `instance-unreachable` isError detail takes those six causes plus `call-deadline`, the last written by the frame tools alone on a frame call past the MCP call deadline, and no MCP unreachable reject writes a line, the isError code its only record).
