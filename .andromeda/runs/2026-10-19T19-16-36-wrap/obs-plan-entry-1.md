
## 2026-10-19-overseer-switch-to-viola — `dialog-answered`'s `from` / `from_trust` only on an answer that carried a `from`
**Section:** §4 Scenario: Dialog → `answer` (Required log fields, wrapper line).
**Change:** `from` / `from_trust` are present on an answer that carried a `from`, absent on an answer that carried none and on the expiry line (`deadline_hit:true`). Was "present on an answer and absent on the expiry line".
**Why:** Measured false live: a product `viola answer` from a process without `VIOLA_NAME` sent no `from`, and its `dialog-answered` line held neither field. The scenario's own Must-trace text already named an answer with no `from`.
**Ref:** .andromeda/runs/2026-10-19T19-16-36-wrap/
