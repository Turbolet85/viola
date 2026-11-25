@leaf_group group=architecture+test-plan+obs-plan
@leaf_read leaf=.claude/docs/commands.md lines=90
last: - `scripts/agent-run.sh logs --instance <name> | jq -c 'sele
@leaf_edit n=1 leaf=.claude/docs/commands.md line=20 class=omission
leaf-text: A verify that cannot start, or is stopped, writes nothing and is no attempt;
source: dec-ixmg7xtr «no step line, no summary, no stamp, probe folders removed; a start's automatic verify then records nothing and leaves the version un-attempted»
replace-with: A verify that cannot start, is stopped, or refuses `untrusted-folder` (any of Runs B to E reading the trust or imports form on its first screen) writes nothing and is no attempt;
@leaf_edit n=2 leaf=.claude/docs/commands.md line=24 class=omission
leaf-text: Run it from a folder you trust in Claude Code whose external CLAUDE.md import is answered.
source: dec-ixmg7xtr «viola verify refuses with untrusted-folder, exit 1, when any of Runs B to E reads the trust or imports form on its first screen (Runs A and F never read): no step line, no summary, no stamp»
replace-with: Run it from a folder you trust in Claude Code whose external CLAUDE.md import is answered: when any of Runs B to E reads the trust or imports form on its first screen, verify refuses `untrusted-folder`, exit 1, with no step line, no summary and no stamp, its probe folders removed.
