group: obs-plan
read: .claude/docs/obs-summary.md · 76 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »

- leaf: .claude/docs/obs-summary.md:72   class: stale
  leaf-text: «Each bad threshold key is one `parse-rejected{parser:"config-json", detail:"malformed"}` at start; `CONFIG_KEYS` holds five names.»
  source: .andromeda/registries/contracts/obs-plan/otel-sdk-init.md:17 «`CONFIG_KEYS` holds six names (`v`, `diagnostics_level`, `claude_env_keep`, `five_hour`, `seven_day`, `dialog_hold_s`; the array in `src/obs.rs`)»
  replace-with: «Each bad threshold key is one `parse-rejected{parser:"config-json", detail:"malformed"}` at start, and a `dialog_hold_s` that is no whole number of seconds from 60 to 3600 one more (`run::dialog::read_hold`, the hold then 60); `CONFIG_KEYS` holds six names.»

- leaf: .claude/docs/obs-summary.md:42   class: omission
  leaf-text: «`dialog-raised` · `dialog-answered` (corr = dialog_id)»
  source: .andromeda/obs-plan.md:615 «the reply stays `null`, so the hook writes `hook-decision{decision_emitted:false, deadline_hit:false}`, exits 0 with empty stdout, and the dialog renders for the human.»
  replace-with: «`dialog-raised` · `dialog-answered` (corr = dialog_id; at the dialog hold's end, `config.json`'s `dialog_hold_s`, default 60 s, the wrapper writes `dialog-answered{deadline_hit:true}` with no `from` / `from_trust` beside one `dialog-expired` product line that wakes a parked `wait`, and the hook's `hook-decision` reads `deadline_hit:false`, `true` only at its 3605 s backstop)»
