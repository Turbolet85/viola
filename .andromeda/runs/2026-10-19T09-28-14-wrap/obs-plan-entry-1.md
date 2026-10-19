
## 2026-10-19-dialog-hold-for-the-driver — the expiry is the wrapper's: `dialog-expired` and `dialog-answered{deadline_hit:true}`; the start reads `dialog_hold_s`
**Section:** §4 Scenario: `wait` / `last` (Required span attributes; Required log fields); §4 Scenario: Dialog → `answer` (Required span attributes; Required log fields; Cleanup); §3 → OTel SDK init (Init order steps 5 and 6).
**Change:**
- `woken_kind` and the `channel-response` `outcome` list six kinds (`…|session-end|dialog-expired`, the six of `EventKind::WAIT_WAKE`). Was five.
- `run.dialog_await`'s `deadline_ms` is the hold the slot was started with (`dialog_hold_s` in ms, 60 000–3 600 000, default 60 000).
- `dialog-answered` carries `from` / `from_trust` on an answer only; the expiry line has `deadline_hit:true` and no `from`.
- Cleanup: at the hold's end the wrapper appends one `dialog-expired` product line and writes `dialog-answered{deadline_hit:true}`, and the hook, handed `null`, writes `hook-decision{decision_emitted:false, deadline_hit:false}`. The hook's `deadline_hit:true` is only its backstop at `DIALOG_HOLD_MAX + 5 s` (3605 s). Was "On deadline the hook emits `hook-decision{decision_emitted:false, deadline_hit:true}`", a claim the wrapper's earlier reply already made false.
- OTel SDK init: `run`'s start reads `config.json` a third time, for `dialog_hold_s`, writing exactly one `parse-rejected{config-json, malformed}` on a bad value. `CONFIG_KEYS` holds six names. Was five. A clause's chunk stamp was dropped from the key file.
**Why:** The chunk moved the expiry's record to the wrapper, which knows the hold ended. The hook only sees a `null` reply. The start's new read is the configured hold, a boundary widening the founder answered himself to P4 fork Q1 (`inputs#I2`); the wrap's answer-card confirm was the Viola overseer's (delegate viola-overseer), PROVISIONAL.
**Ref:** .andromeda/runs/2026-10-19T09-28-14-wrap/
