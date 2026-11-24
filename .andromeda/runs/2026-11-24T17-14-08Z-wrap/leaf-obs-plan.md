@leaf_group group=obs-plan
@leaf_read leaf=.claude/docs/obs-summary.md lines=76
last: **Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: `
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, so
@leaf_edit n=1 leaf=.claude/docs/obs-summary.md line=67 class=stale
leaf-text: `unsupported` spawns nothing, `unreachable` provisional)
source: ctr-amu66bwf «viola focus ends exit 1 with one process-exit detail: window-not-found (no window of the session, nothing dispatched), unsupported (no hyprctl reachable, nothing spawned), unreachable (hyprctl reachable, no compositor answering)»
replace-with: `unsupported` (no hyprctl reachable) spawns nothing, `unreachable` hyprctl reachable but no compositor answering)
@leaf_edit n=2 leaf=.claude/rules/observability.md line=28 class=stale
leaf-text: `unsupported` (not on Hyprland, nothing spawned), `unreachable` (no compositor answering, provisional)
source: ctr-amu66bwf «viola focus ends exit 1 with one process-exit detail: window-not-found (no window of the session, nothing dispatched), unsupported (no hyprctl reachable, nothing spawned), unreachable (hyprctl reachable, no compositor answering)»
replace-with: `unsupported` (no hyprctl reachable, nothing spawned), `unreachable` (hyprctl reachable, no compositor answering)
