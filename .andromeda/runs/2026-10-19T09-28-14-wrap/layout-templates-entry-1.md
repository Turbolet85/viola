
## 2026-10-19-dialog-hold-for-the-driver — the `viola wait` wireframe gains the `dialog-expired` line
**Section:** §Surface: cli → Output structure — `viola wait` / `viola last` (the wireframe and the prose under it).
**Change:**
- The wireframe gains `dialog-expired  builder  dialog 7  cursor 49400`, annotated as the dialog form.
- The `session-end` annotation reads "every kind without a dialog_id takes the turn-ended form". Was "every non-dialog kind".
- The prose says a `wait` woken by `dialog-expired` prints `dialog-expired  <name>  dialog <id>  cursor <n>`, ASCII, no SGR, written once.
**Why:** The wrapper now records an expired hold as `dialog-expired` `{dialog_id, dialog_kind}`, which wakes `wait`. The line is a planning form, and the founder's wording for the kind word and the line is owed to his Epoch 5 boundary batch (the P5 card's row 3, approved by the operator's word, `inputs#I3`).
**Ref:** .andromeda/runs/2026-10-19T09-28-14-wrap/
