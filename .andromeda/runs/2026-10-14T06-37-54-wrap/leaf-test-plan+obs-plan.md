group: test-plan+obs-plan
read: .claude/rules/observability.md · 47 · «- 2026-09-24: `obs_event!` expands to `::tracing::event!`, s»

- leaf: .claude/rules/observability.md:39   class: stale
  leaf-text: «The hidden `hook --capture` arm does no obs init and writes no line (a bounded panic-line exemption, obs-plan §10).»
  source: .andromeda/obs-plan.md:1071 «5. the hidden capture arms, `hook <event> --capture` and `hook statusline --capture` (which runs no instance check and no pass-through), each doing no obs init and so having no role file;»
  replace-with: «The hidden capture arms, `hook <event> --capture` and `hook statusline --capture` (the latter runs no instance check, no shell-out and no `budget.json` write, opens no role file and prints nothing), do no obs init and write no line (a bounded panic-line exemption, obs-plan §10).»
