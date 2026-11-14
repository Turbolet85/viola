@leaf_group group=test-plan+obs-plan
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, s
@leaf_edit n=1 leaf=.claude/rules/observability.md line=35 class=stale
leaf-text: the `run` pump and handle-wait threads
source: dec-4alyzcfr «The two run PTY pump threads each run inside catch_unwind, and the child's exit is polled inline on run's main thread under main's catch_unwind»
replace-with: the two `run` PTY pump threads (the child's exit is polled inline on run's main thread, under main's catch site)
