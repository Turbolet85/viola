@leaf_group group=architecture+test-plan+obs-plan
@leaf_read leaf=.claude/docs/commands.md lines=90
last: - `scripts/agent-run.sh logs --instance <name> | jq -c 'sele
@leaf_edit n=1 leaf=.claude/docs/commands.md line=23 class=omission
leaf-text: the stdio MCP server (tools `send · wait · last · answer · list`), hidden from `viola --help`
source: dec-eljum6qo «Every MCP frame tool call (send, wait, last, answer) is bounded by a compiled 55 s call deadline, above the 50 s wait cap and below the claude client's tool-call bound»
replace-with: the stdio MCP server (tools `send · wait · last · answer · list`; each frame tool call — `send`, `wait`, `last`, `answer`, never `list` — bounded by a compiled 55 s call deadline, at whose expiry the call's work is dropped and the tool returns `instance-unreachable` with detail `call-deadline`, which no CLI document carries), hidden from `viola --help`
