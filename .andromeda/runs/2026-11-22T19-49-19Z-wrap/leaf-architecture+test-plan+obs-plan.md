@leaf_group group=architecture+test-plan+obs-plan
@leaf_read leaf=.claude/docs/commands.md lines=90
last: - `scripts/agent-run.sh logs --instance <name> | jq -c 'selec
@leaf_edit n=1 leaf=.claude/docs/commands.md line=24 class=omission
leaf-text: (SIGKILL, a crash or a failed handler install leaves the dirs to remove by hand)
source: dec-wc3ygufa «SIGKILL, a crash or a failed handler install leave the folders to be removed by hand; a SIGKILL also leaves verify's own one-key trust record.»
replace-with: (SIGKILL, a crash or a failed handler install leaves the dirs to remove by hand; a SIGKILL also leaves verify's own one-key trust record in the Claude config file, to remove by hand)
