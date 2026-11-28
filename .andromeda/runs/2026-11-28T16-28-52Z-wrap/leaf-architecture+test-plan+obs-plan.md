@leaf_group group=architecture+test-plan+obs-plan
@leaf_read leaf=.claude/docs/commands.md lines=90
last: - `scripts/agent-run.sh logs --instance <name> | jq -c 'selec
@leaf_edit n=1 leaf=.claude/docs/commands.md line=23 class=stale
leaf-text: in the alternate screen, plain ASCII with no colour, its refusals
source: ctr-krgnhdj4 «viola console writes only its screen to stdout, in the alternate screen, with no colour and plain ASCII bar the editing line, which echoes the typed text as typed with every control escaped»
replace-with: in the alternate screen, with no colour and plain ASCII bar the editing line, which echoes the typed text as typed with every control escaped, its refusals
