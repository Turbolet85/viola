@leaf_group group=architecture+test-plan+obs-plan
@leaf_read leaf=.claude/docs/commands.md lines=89
last: - `scripts/agent-run.sh logs --instance <name> | jq -c 'sele
@leaf_edit n=1 leaf=.claude/docs/commands.md line=40 class=stale
leaf-text: the separator class makes the regex match Windows paths
source: fct-3mj52n2x «its separator class is kept from the retired Windows target, where llvm-cov reported paths with `\`»
replace-with: the separator class is kept from the retired Windows target, where llvm-cov reported paths with `\`
@leaf_edit n=2 leaf=.claude/docs/commands.md line=24 class=stale
leaf-text: and so on every OS does an eighteen-row stamp
source: fct-62qgonvf «A version is verified only when every row of `LedgerRow::relied()` reads `pass` (all twenty-four, on every build)»
replace-with: and so does an eighteen-row stamp
