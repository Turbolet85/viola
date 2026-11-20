@leaf_group group=architecture+test-plan+obs-plan
@leaf_read leaf=.claude/docs/commands.md lines=89
last: - `scripts/agent-run.sh logs --instance <name> | jq -c 'selec
@leaf_edit n=1 leaf=.claude/docs/commands.md line=19 class=omission
leaf-text: on a verified CLI the start, and a revive's, writes the trust record of an untrusted spawn folder into the Claude config before spawning the child
source: dec-zrumetff «Start verifies a new CLI version when keyed — With config.json verify_new_cli on, a viola run or revive start meeting a claude version with no attempted stamps entry runs viola verify once as a child after the endpoint bind»
replace-with: with `config.json` `verify_new_cli` on (off by default), the start, and a revive's, meeting a claude version with no attempted stamps entry runs `viola verify` once as a child after the endpoint bind and gates on its final verdict; on a verified verdict it writes the trust record of an untrusted spawn folder into the Claude config before spawning the child
