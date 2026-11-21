@leaf_group group=test-plan+obs-plan
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, s
@leaf_edit n=1 leaf=.claude/rules/observability.md line=40 class=omission
leaf-text: and as their own refused `check_instance` of the home and the instance directory does (made before `live_endpoint`, `detail` `strict-modes-failed`, no frame); no pid
source: dec-bamrozq7 «An MCP instance-unreachable reject, from any instance, liveness, socket-directory, euid or server-verification check or from the MCP call deadline, is logged by no line; the isError code is its only record.»
replace-with: and as their own refused `check_instance` of the home and the instance directory does (made before `live_endpoint`, `detail` `strict-modes-failed`, no frame), and as `send`, `wait`, `last` and `answer` do at the 55 s MCP call deadline (`detail` `call-deadline`, a cause the MCP frame tools alone write and the CLI never does; the frame dropped); no pid
