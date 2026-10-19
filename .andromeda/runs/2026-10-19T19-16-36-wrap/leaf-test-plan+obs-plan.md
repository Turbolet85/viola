group: test-plan+obs-plan
read: .claude/rules/observability.md · 46 · «- 2026-09-24: `obs_event!` expands to `::tracing::event!`, s»

- leaf: .claude/rules/observability.md:27   class: omission
  leaf-text: «`from` is logged with `from_trust:"self-reported"`.»
  source: .andromeda/obs-plan.md:610 «`from` / `from_trust` present on an answer that carried a `from`, absent on an answer that carried none and on the expiry line (`deadline_hit:true`);»
  replace-with: «`from` is logged with `from_trust:"self-reported"`, both absent on a line whose call carried no `from` (a `dialog-answered` for an answer that carried none, as on the hold's expiry line).»
