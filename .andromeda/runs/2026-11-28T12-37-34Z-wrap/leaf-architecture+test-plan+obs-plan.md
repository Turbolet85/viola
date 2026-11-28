@leaf_group group=architecture+test-plan+obs-plan
@leaf_read leaf=.claude/docs/commands.md lines=90
last: - `scripts/agent-run.sh logs --instance <name> | jq -c 'sele
@leaf_edit n=1 leaf=.claude/docs/commands.md line=23 class=stale
leaf-text: a home another user may read exits 1 with the home pair, writing no line;
source: dec-gvguve6w «viola console exits 1 with its home pair, writing no line at its start and one process-exit line when refused mid-run.»
replace-with: a home another user may read exits 1 with the home pair, writing no line at its start and one `process-exit` line of detail `strict-modes-failed` when its board scan is refused mid-run, after the terminal's restore;
