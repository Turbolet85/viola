@leaf_group group=architecture+obs-plan
@leaf_read leaf=.claude/docs/commands.md lines=90
last: - `scripts/agent-run.sh logs --instance <name> | jq -c 'select
@leaf_edit n=1 leaf=.claude/docs/commands.md line=23 class=stale
leaf-text: `unsupported` (not on Hyprland, nothing spawned), `unreachable` (no compositor answering)
source: ctr-amu66bwf «viola focus ends exit 1 with one process-exit detail: window-not-found (no window of the session, nothing dispatched), unsupported (no hyprctl reachable, nothing spawned), unreachable (hyprctl reachable, no»
replace-with: `unsupported` (no hyprctl reachable, nothing spawned), `unreachable` (hyprctl reachable, no compositor answering)
