
## 2026-10-19-dialog-hold-for-the-driver — the configurable dialog hold and the dialog-expired record
**Section:** §Established Decisions [Hook Contract] (with two detail groups below it), [Human Takeover / Wheel], [Message Broker / IPC], [Deployment / Distribution], [Validation]; §Stack and Technologies (Domain newtypes); §Conventions (Normalised event kinds); §Standard Contracts (`pending_dialog`, `wait`, Event `data` per kind, wrapper-appended records); §Occupied Resources (`config.json`, `schemas/config.v1.json`, Registered hook events); §Cross-cutting Patterns (Config management, Settings); the subject line (new: `dec-hook-contract`).
**Change:**
- The hold is `config.json`'s `dialog_hold_s`, whole seconds from 60 to 3600, default 60 (`DIALOG_HOLD_DEFAULT` · `DIALOG_HOLD_MIN` · `DIALOG_HOLD_MAX`, PROVISIONAL), a `DialogHold` read once at start. Was `viola_core::DIALOG_DEADLINE` = 60 s, retired.
- The hook's reply bound is `DIALOG_HOLD_MAX + 5 s` (3605 s), the ceiling (was `DIALOG_DEADLINE + 5 s`). The dialog hooks' `hooks.json` `timeout` is 3615 (was 75), so the rendered plugin stays byte-identical across starts.
- At the hold's end the reply stays `null`, and the wrapper appends one `dialog-expired` line (`source: wrapper`, `{dialog_id, dialog_kind}`) that wakes `wait`. It is the one wrapper-appended kind that wakes; "only hook lines signal" now excepts it. A hand-back or an answer appends none.
- The normalised kinds gain `dialog-expired`. The wake set is six: `turn-ended` · `question` · `permission` · `plan` · `session-end` · `dialog-expired`.
- `pending_dialog` is written before the dialog's event line and written back to none on a failed append. It was written after the line.
- `DialogHold` joins `ViolaName` and `Percent` as a nutype.
- "dialog deadline" reads "dialog hold" in [Human Takeover / Wheel].
- The live reading of a 671 s hold on claude 2.1.287, Linux, `as measured at` the chunk's `evidence/live-round.md`. A pending group owes the near-ceiling reading to "Overseer switch to viola" and the Windows leg to "Windows-only live measurements".
**Why:** The founder answered the P4 forks himself, relayed verbatim by the operator (`inputs#I2`). Q1 ratified the key as a boundary widening, Q2 kept `null` at expiry, and Q3 capped the live round. The wrap's answer-card confirm of these proposals was the Viola overseer's (delegate viola-overseer), PROVISIONAL, superseded by his own word at the Epoch 5 boundary. The order fix answers a macOS CI red whose cause predates the chunk, on the operator's word (`inputs#I4`). The hook's bound and the timeout follow the ceiling, not each start's hold, because the wrapper replies at its own hold's end.
**Ref:** .andromeda/runs/2026-10-19T09-28-14-wrap/
