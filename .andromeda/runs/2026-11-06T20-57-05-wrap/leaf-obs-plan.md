@leaf_group group=obs-plan
@leaf_read leaf=.claude/docs/obs-summary.md lines=76
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: 
@leaf_edit n=1 leaf=.claude/docs/obs-summary.md line=22 class=stale
leaf-text: `scripts/agent-run.* logs` merges
source: fct-q2zh72zo «The agent-readable consumption path is the tests harness `logs` command (`scripts/agent-run.sh logs`, via `viola-harness`).»
replace-with: `scripts/agent-run.sh logs` (via `viola-harness`) merges
