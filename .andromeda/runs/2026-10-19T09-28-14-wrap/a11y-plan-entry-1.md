
## 2026-10-19-dialog-hold-for-the-driver — the driver's dialog hold is configurable, the human's key unchanged
**Section:** §8 Cognitive Accessibility → Timeout extensions (the CLI bullet).
**Change:** A dialog takes a `viola answer` within the dialog hold: `dialog_hold_s`, whole seconds 60–3600, default 60, a `DialogHold` with PROVISIONAL bounds, under the `hooks.json` dialog `timeout` of 3615 s and the hook's bound of 3605 s. At the hold's end the wrapper replies `null` and appends one `dialog-expired` line that wakes `wait`. The hold bounds the driver, never the human: any human key still hands the held dialog back at once, whatever the hold. Was `DIALOG_DEADLINE`, 60 s, under a 75 s `timeout`.
**Why:** A longer, configurable hold lets a driver relay a dialog to a person and back. The key is a boundary widening the founder answered himself to P4 fork Q1 (`inputs#I2`); the wrap's answer-card confirm was the Viola overseer's (delegate viola-overseer), PROVISIONAL. No human-facing time limit is added.
**Ref:** .andromeda/runs/2026-10-19T09-28-14-wrap/
