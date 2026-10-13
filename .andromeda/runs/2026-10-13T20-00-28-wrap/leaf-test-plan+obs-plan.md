group: test-plan+obs-plan
read: .claude/rules/observability.md · 47 · «- 2026-09-24: `obs_event!` expands to `::tracing::event!`, s»

- leaf: .claude/rules/observability.md:39   class: omission
  leaf-text: «The hidden `hook --capture` arm does no obs init and writes no line (a bounded panic-line exemption, obs-plan §10).»
  source: .andromeda/obs-plan.md:739 «A `verify` stopped by Ctrl-C or TERM (Unix `SIGINT` / `SIGTERM`, Windows Ctrl-C / Ctrl-Break) starts no further spawn, kills and reaps the child it is running, closes every pair it opened»
  replace-with: «The hidden `hook --capture` arm does no obs init and writes no line (a bounded panic-line exemption, obs-plan §10). A `viola verify` stopped by Ctrl-C or TERM (Unix `SIGINT` / `SIGTERM`, Windows Ctrl-C / Ctrl-Break) starts no further spawn, kills and reaps the child it is running, closes every spawn pair it opened (a child killed at the stop closes with `duration_ms` and no `child_exit_status`) and ends `process-exit{subject:"self", exit_code:1, detail:"interrupted"}` (a twelfth `detail` value in `schemas/diag-line.v1.json`); `run`'s version gate observes no stop.»
