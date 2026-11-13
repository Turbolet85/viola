@leaf_group group=architecture+test-plan+obs-plan
@leaf_read leaf=.claude/docs/commands.md lines=89
last: - `scripts/agent-run.sh logs --instance <name> | jq -c 'sele
@leaf_edit n=1 leaf=.claude/docs/commands.md line=23 class=stale
leaf-text: (and the same pair, exit 1, when `update_stamps` refuses `ledger/stamps.json`
source: ctr-x5orad6w «A refused home tree exits 1 with the home-state unable and hint pair on stderr and no step line, summary or log line; a strict-modes refusal while measuring exits 1 with the same pair after the step lines»
replace-with: (and the same pair, exit 1, after the step lines, when `update_stamps` refuses `ledger/stamps.json`
