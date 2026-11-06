@leaf_group group=test-plan+obs-plan
@leaf_read leaf=.claude/rules/observability.md lines=46
last: - 2026-09-24: `obs_event!` expands to `::tracing::event!`, s
@leaf_edit n=1 leaf=.claude/rules/observability.md line=37 class=stale
leaf-text: symbolising cost 351 of a 403 ms hook run on the Windows runner;
source: fct-c5mu6ru4 «`Backtrace::force_capture().to_string()` cost 351.4 ms of a 402.7 ms hook run for 72 frames on that chunk's Windows runner»
replace-with: symbolising cost 351 of a 403 ms hook run on chunk 2026-09-28-capability-ledger-and-viola-verify's retired Windows runner;
