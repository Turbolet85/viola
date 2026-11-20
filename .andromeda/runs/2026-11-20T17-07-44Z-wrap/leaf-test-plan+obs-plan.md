@leaf_group group=test-plan+obs-plan
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, s
@leaf_edit n=1 leaf=.claude/rules/observability.md line=38 class=stale
leaf-text: `run`'s version gate observes no stop.
source: ctr-4c6fbnjp «A SIGINT or SIGTERM during the start's automatic verify TERMs the verify child, KILLs it after a 30 s grace, writes the auto-verify process-exit, then re-raises the signal»
replace-with: `run`'s version gate observes no stop, save during the start's automatic verify: a SIGINT or SIGTERM there TERMs the verify child, KILLs it after a 30 s grace, writes the `auto-verify` `process-exit`, then re-raises the signal, the start ending with no `process-exit` of its own, no stamp, no snapshot and no session.
@leaf_edit n=2 leaf=.claude/rules/observability.md line=42 class=omission
leaf-text: around each hyprctl spawn, at viola's call site.
source: ctr-e32v362q «Subject of the process-start and process-exit pair of the viola verify child that a viola run start spawns, the exit carrying child_exit_status and duration_ms, written to the run log.»
replace-with: around each hyprctl spawn, at viola's call site. A `viola run` start that spawns its automatic `viola verify` child writes one `process-start` / `process-exit{subject:"auto-verify"}` pair to the run log, the exit carrying `child_exit_status` and `duration_ms`.
