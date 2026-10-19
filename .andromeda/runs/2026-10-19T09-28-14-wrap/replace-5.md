- line: .andromeda/obs-plan.md:607
old: «  - `run.dialog_await`: `deadline_ms`, `deadline_hit`;»
new: «  - `run.dialog_await`: `deadline_ms` (the hold the slot was started with: `config.json`'s `dialog_hold_s` in ms, 60 000–3 600 000, default 60 000), `deadline_hit`;»
- line: .andromeda/obs-plan.md:610
old: «then `dialog-answered{corr:<dialog_id>, dialog_kind, from, from_trust, deadline_hit, duration_ms}`;»
new: «then `dialog-answered{corr:<dialog_id>, dialog_kind, from, from_trust, deadline_hit, duration_ms}`, `from` / `from_trust` present on an answer and absent on the expiry line (`deadline_hit:true`);»
- line: .andromeda/obs-plan.md:615
old: «`run.dialog_await` closes on answer or deadline. On deadline the hook emits `hook-decision{decision_emitted:false, deadline_hit:true}`, exits 0 with empty stdout, and the dialog renders for the human.»
new: «`run.dialog_await` closes on an answer, a hand-back or the hold's end. At the hold's end the wrapper appends one `dialog-expired{dialog_id, dialog_kind}` product line (`source: wrapper`, no question, plan or tool text; it wakes a parked `wait`) and writes one `dialog-answered{corr:<dialog_id>, dialog_kind, deadline_hit:true, duration_ms}` with no `from`; the reply stays `null`, so the hook writes `hook-decision{decision_emitted:false, deadline_hit:false}`, exits 0 with empty stdout, and the dialog renders for the human. A hand-back or an answer writes neither, and a failed append changes nothing else. The hook writes `deadline_hit:true` only when its own read times out at its backstop bound, `DIALOG_HOLD_MAX + 5 s` (3605 s).»
- line: .andromeda/registries/contracts/obs-plan/otel-sdk-init.md:16
old: «read the file a second time for the two budget thresholds, `read_thresholds`, step 6).»
new: «read the file a second time for the two budget thresholds, `read_thresholds`, step 6; `run` reads it a third time for the dialog hold, `read_hold`, step 6).»
- line: .andromeda/registries/contracts/obs-plan/otel-sdk-init.md:17
old: «gives both defaults and no further line. `CONFIG_KEYS` holds five names (`v`, `diagnostics_level`, `claude_env_keep`, `five_hour`, `seven_day`; the array in `src/obs.rs`), so neither threshold key counts as unknown (chunk 2026-10-10-budget-governor).»
new: «gives both defaults and no further line. Then `run` reads `config.json` once more for `dialog_hold_s` (`run::dialog::read_hold`, through `take(MAX_FRAME)`, `v` = 1) and writes exactly one `parse-rejected{parser:"config-json", detail:"malformed", count}` WARN line when the key holds anything but a whole number of seconds from 60 to 3600, the hold then taking 60; a document that is no object, or whose `v` is not 1, gives 60 and no further line. `CONFIG_KEYS` holds six names (`v`, `diagnostics_level`, `claude_env_keep`, `five_hour`, `seven_day`, `dialog_hold_s`; the array in `src/obs.rs`), so neither threshold key nor the hold key counts as unknown (chunk 2026-10-10-budget-governor).»
