
## 2026-10-19-dialog-hold-for-the-driver — `viola wait`'s `dialog-expired` line takes the dialog form
**Section:** §Surface: cli §Component Patterns 3 (`viola wait` / `viola last`).
**Change:**
- A new example line: `dialog-expired  builder  dialog 7  cursor 49400`. An expired hold takes the dialog form, as `question`, `permission` and `plan` do.
- The time form is now for "every other kind without a `dialog_id`". Was "every other non-dialog kind", which would have put `dialog-expired` in the time form.
**Why:** The wrapper's new record of an expired hold carries `dialog_id`, so its `wait` line names the dialog. The line and the kind word are a planning form: the founder's wording is owed to his Epoch 5 boundary batch (the P5 card's row 3, approved by the operator's word, `inputs#I3`). The Domain status colors table gains no row: a colour for the kind is a 0.2.0 frontend decision this chunk did not make.
**Ref:** .andromeda/runs/2026-10-19T09-28-14-wrap/
