
## 2026-10-19-dialog-hold-for-the-driver — the hold's unit cases, sixteen kinds, six wake kinds, the controls table at ten rows
**Section:** §1 Test Scope Summary (Path `wait` / `last`); §4 Unit Test Strategy (Conventions → Test grouping; What unit tests cover → viola-core, root bin, the channel `wait` handler); §5 Integration Test Strategy (Cross-module patterns → On-disk; CLI, `tests/cli_controls_not_disableable.rs`).
**Change:**
- The wake set is six (rule: `EventKind::WAIT_WAKE`). Was five; `dialog-expired` is added, at §1 Path 3 and at §4's channel `wait` handler.
- `state_events.rs` round-trips 16 kinds (rule: the length of `EventKind::ALL`). Was 15; `dialog-expired` carries `dialog_id` and `dialog_kind`.
- The controls table's `SETTINGS` holds ten rows (rule: `SETTINGS` in the file). Was nine; the new row is `dialog_hold_s`, with the booted case `setting_does_not_unbound_the_dialog_hold` (`0`, `3601`, `-1`, `"3600"`, `false`, each one `parse-rejected{config-json, malformed}`).
- §4 viola-core gains the `DialogHold` bounds case and the counts `ALL` 16 / `WAIT_WAKE` 6. §4 root bin gains the dialog-hold cases on the injected clock: `read_hold`, the 120 s hold, the expiry's one `dialog-expired` and `dialog-answered{deadline_hit:true}`, no record on a hand-back or an answer, `pending_dialog` before the line, the hook bound, the timeout margin and the `wait` line form.
- §4 Test grouping: `viola-core` (no rstest dev-dependency) writes table-driven cases as literal tables, beside `viola-e2e`.
**Why:** The chunk made the dialog hold configurable and recorded its expiry as a wrapper event that wakes `wait`. These are the tiers that prove it: the clock-driven unit case §11 Universal requires for the hold, the on-disk round trip, and the control row for the new key. The `dialog_hold_s` row records a boundary widening the founder answered himself to P4 fork Q1 (`inputs#I2`). The wrap's answer-card confirm was the Viola overseer's (delegate viola-overseer), PROVISIONAL.
**Ref:** .andromeda/runs/2026-10-19T09-28-14-wrap/
