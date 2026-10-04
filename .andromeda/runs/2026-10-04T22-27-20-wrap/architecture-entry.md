
## 2026-10-04-running-turn-refusal — the running-turn state as built
**Section:** Established Decisions → [Human Takeover / Wheel]
**Change:**
- Was "as built, no running-turn state exists beyond the in-flight `send` slot … so `release` returns the wheel alone", per "2026-10-04-the-wheel — the wheel as landed: the closed non-editing list and the Windows platform fact"; now the `viola release` that returns the wheel clears the running-turn state, as measured at this chunk.
- The state lives in memory in `WheelSlot` beside holder and cause ("holder, cause and the running turn under one lock") and records no event, snapshot field, span or obs line; a restarted wrapper starts with none.
- `run::send::append_hook_event` marks it before the hook line is appended, keyed on kind alone: a `prompt-submitted` of any origin (the claimed driver prompt included) starts it, `turn-ended` / `session-start` / `session-end` end it, `activity` leaves it; the turn is marked ahead of a human prompt's wheel move.
- The `send` rung decides at arrival (`turn-running`: a turn running or another `send` in flight). Only a `release` returning a human-held wheel clears it, in the same lock hold; a driver-held `release`, `release --budget`, a refused `release`, `pause` and human keys leave it — recovery is `pause` then `release`. An unsent human prompt takes the wheel first, so the next `send` reads `human-typing`, never `turn-running`.
- Residuals: a turn starting during the readiness gate's wait (up to `GATE_MAX_WAIT`, 5 s provisional) is not refused; a SessionEnd the hook appends directly leaves the turn marked.
**Why:** the founder's live ruling minting the entry (relayed by the overseer at the wheel's wrap) asked for the running-turn refusal; the hypothesis that a driver `send` is typed into a turn it did not start was measured true before the fix and refused after it on three CI OSes. The rung-at-arrival residual was accepted at P5 by the overseer and recorded on the operator's directive at this wrap.
**Ref:** .andromeda/runs/2026-10-04T22-27-20-wrap/
