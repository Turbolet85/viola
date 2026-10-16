group: test-plan+obs-plan
read: .claude/rules/observability.md · 47 · «- 2026-09-24: `obs_event!` expands to `::tracing::event!`, s»

- leaf: .claude/rules/observability.md:39   class: stale
  leaf-text: «(a child killed at the stop closes with `duration_ms` and no `child_exit_status`)»
  source: .andromeda/obs-plan.md:739 «a child of an interactive PTY run killed at the stop closes with `duration_ms` and a `child_exit_status`, the PTY pump's exit code, since portable-pty reports a code for a signalled child»
  replace-with: «(a child of an interactive PTY run killed at the stop closes with `duration_ms` and a `child_exit_status`, the PTY pump's exit code; a print-mode or `--version` probe child killed at the stop is not measured)»
