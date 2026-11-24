@leaf_group group=architecture+test-plan+obs-plan
@leaf_read leaf=.claude/docs/commands.md lines=90
last: - `scripts/agent-run.sh logs --instance <name> | jq -c 'selec
@leaf_edit n=1 leaf=.claude/docs/commands.md line=23 class=omission
leaf-text: `viola mcp [--home <dir>]` — the stdio MCP server
source: ctr-d6isq7rt «contract/cli-verbs/switch · CLI verbs · switch — `viola switch <name> [--model <MODEL>] [--effort <EFFORT>] [--json]` takes at least one of `--model` and `--effort`, else a usage error, exit 2;»
replace-with: `viola switch <name> [--model <MODEL>] [--effort <EFFORT>] [--json]` — switch a running wrapped session's model and effort in place, resuming the same session id on the new pair under the same wrapper (at least one of `--model` and `--effort`, else a usage error, exit 2; prints `<name>  switched  <model> <effort>` and exits 0, 10, 12, 13, 14, 20 or 21; CLI only, no MCP tool switches a session's model or effort). `viola mcp [--home <dir>]` — the stdio MCP server
