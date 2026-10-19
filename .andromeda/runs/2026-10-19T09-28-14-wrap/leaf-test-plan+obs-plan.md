group: test-plan+obs-plan
read: .claude/rules/observability.md · 46 · «- 2026-09-24: `obs_event!` expands to `::tracing::event!`, s»

- leaf: .claude/rules/observability.md:41   class: omission
  leaf-text: «`used_percentage`, `resets_at` and `paused:true`.»
  source: .andromeda/obs-plan.md:615 «At the hold's end the wrapper appends one `dialog-expired{dialog_id, dialog_kind}` product line (`source: wrapper`, no question, plan or tool text; it wakes a parked `wait`) and writes one `dialog-answered»
  replace-with: «`used_percentage`, `resets_at` and `paused:true`. At a dialog hold's end (`run.dialog_await`'s `deadline_ms`, `config.json`'s `dialog_hold_s` in ms, 60 000–3 600 000, default 60 000) the wrapper appends one `dialog-expired{dialog_id, dialog_kind}` product line and writes one `dialog-answered{corr:<dialog_id>, dialog_kind, deadline_hit:true, duration_ms}` with no `from` / `from_trust`; a hand-back or an answer writes neither; the hook then writes `hook-decision{decision_emitted:false, deadline_hit:false}`, `deadline_hit:true` only when its own read times out at its backstop bound `DIALOG_HOLD_MAX + 5 s` (3605 s).»
