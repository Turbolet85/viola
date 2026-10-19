- line: .andromeda/test-plan.md:574
old: «`state_events.rs`: the 15 kinds (the length of `EventKind::ALL`; 13 until `link` and `unlink` joined at chunk 2026-10-10-session-links, each with the `data` keys `driver` and `driven`)»
new: «`state_events.rs`: the 16 kinds (the length of `EventKind::ALL`; 13 until `link` and `unlink` joined at chunk 2026-10-10-session-links, each with the `data` keys `driver` and `driven`, 15 until `dialog-expired` joined at chunk 2026-10-19-dialog-hold-for-the-driver with the `data` keys `dialog_id` and `dialog_kind`)»
- line: .andromeda/test-plan.md:594
old: «whose helper removes the three inherited values before setting the case's own; the other verb negatives»
new: «whose helper removes the three inherited values before setting the case's own; chunk 2026-10-19-dialog-hold-for-the-driver added the `config.json` key `dialog_hold_s` as the row `setting_does_not_unbound_the_dialog_hold` (booted on a stamped home, five cases — `0`, `3601`, `-1`, `"3600"`, `false` — each giving exactly one `parse-rejected{parser:"config-json", detail:"malformed"}` line in `run-builder.ndjson`; the value each falls back to is the `src/run/dialog.rs` unit `read_hold_falls_back_to_sixty_with_one_rejection`), so `SETTINGS` holds ten rows (rule: `SETTINGS` in `tests/cli_controls_not_disableable.rs`); the other verb negatives»
- line: .andromeda/test-plan.md:206
old: «`wait` wakes only on `turn-ended` / `question` / `permission` / `plan` / `session-end`,»
new: «`wait` wakes only on `turn-ended` / `question` / `permission` / `plan` / `session-end` / `dialog-expired` (the six of `EventKind::WAIT_WAKE`),»
- line: .andromeda/test-plan.md:517
old: «    - it wakes only on `turn-ended` / `question` / `permission` / `plan` / `session-end`»
new: «    - it wakes only on `turn-ended` / `question` / `permission` / `plan` / `session-end` / `dialog-expired` (the six of `EventKind::WAIT_WAKE`; `wait_each_wake_kind_wakes` holds one case per kind)»
- line: .andromeda/test-plan.md:461
old: «so no manifest edge is added for rstest alone.»
new: «so no manifest edge is added for rstest alone. `viola-core` carries no rstest dev-dependency either, and its table-driven cases are literal tables in one `#[test]` (for example `dialog_hold_takes_whole_seconds_from_sixty_to_thirty_six_hundred`).»
