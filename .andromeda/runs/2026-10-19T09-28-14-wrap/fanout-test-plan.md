proposals:
  - detector: D-tests-coverage
    severity: warning
    section: §4 Unit Test Strategy → What unit tests cover → root bin (the channel `wait` handler)
    change: "it wakes only on `turn-ended` / `question` / `permission` / `plan` / `session-end` / `dialog-expired` (the six kinds of `EventKind::WAIT_WAKE`; `dialog-expired` joined at chunk 2026-10-19-dialog-hold-for-the-driver, the first wrapper-appended kind that wakes `wait`; the `src/run/wait.rs` case `wait_each_wake_kind_wakes::case_6_dialog_expired`)"
    sidecar: "2026-10-19 (chunk 2026-10-19-dialog-hold-for-the-driver): §4 channel `wait` wake set 5 → 6 kinds (rule: `EventKind::WAIT_WAKE`), `dialog-expired` added."
    rationale: "Report Changes → Symbols / APIs: `EventKind::WAIT_WAKE: [Self; 6]` (was 5) adds `dialog-expired`, and `src/run/wait.rs`'s scan derives from it; Counts moved, pattern `wake5`, `test-plan.md:517` stale; Outcome cites `wait_each_wake_kind_wakes::case_6_dialog_expired`."
    basis: .andromeda/test-plan.md:517
  - detector: D-tests-coverage
    severity: warning
    section: §1 Test Scope Summary → Critical paths (Path `wait` / `last` event-driven readback)
    change: "`wait` wakes only on `turn-ended` / `question` / `permission` / `plan` / `session-end` / `dialog-expired` (the six kinds of `EventKind::WAIT_WAKE`, `dialog-expired` since chunk 2026-10-19-dialog-hold-for-the-driver), returns at once if the event is already logged at or after `after`, and times out with `{timed_out:true}`."
    sidecar: "2026-10-19 (chunk 2026-10-19-dialog-hold-for-the-driver): §1 Path 3 wake set 5 → 6 kinds, `dialog-expired` added."
    rationale: "Report Counts / qualifiers moved, pattern `wake5`: `test-plan.md:206` stale; `WAIT_WAKE` is now six kinds (Symbols / APIs)."
    basis: .andromeda/test-plan.md:206
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: §5 Integration Test Strategy → Cross-module patterns covered → On-disk
    change: "`state_events.rs`: the 16 kinds (the length of `EventKind::ALL`; 13 until `link` and `unlink` joined at chunk 2026-10-10-session-links, 15 until `dialog-expired` joined at chunk 2026-10-19-dialog-hold-for-the-driver with the `data` keys `dialog_id` and `dialog_kind`, `source` wrapper) round-trip through the crate's writer and reader with zero on all three counts; the rest of the bullet unchanged."
    sidecar: "2026-10-19 (chunk 2026-10-19-dialog-hold-for-the-driver): §5 On-disk round-trip 15 → 16 kinds (rule: the length of `EventKind::ALL`), `dialog-expired` {dialog_id, dialog_kind}."
    rationale: "Report Counts moved, pattern `kinds15`: `test-plan.md:574` ×2 stale; Symbols / APIs: `EventKind::ALL: [Self; 16]`, `dialog-expired` data_keys `[dialog_id, dialog_kind]`; Outcome: `state_events.rs` round-trips 16 with zero counts (crates/viola-state/tests/state_events.rs 125-129)."
    basis: .andromeda/test-plan.md:574
  - detector: D-tests-coverage
    severity: warning
    section: §5 Integration Test Strategy → Cross-module patterns covered → CLI (`tests/cli_controls_not_disableable.rs`)
    change: "Add to the table's history: chunk 2026-10-19-dialog-hold-for-the-driver added the `config.json` key `dialog_hold_s` (whole seconds 60–3600, default 60) as the row `setting_does_not_unbound_the_dialog_hold` (booted, stamped home, five cases: each out-of-range or non-integer value gives the 60 s hold and exactly one `parse-rejected{parser:\"config-json\", detail:\"malformed\"}` line), so the table's `SETTINGS` holds ten rows (rule: `SETTINGS` in `tests/cli_controls_not_disableable.rs`; nine before), and `CONFIG_KEYS` gains `dialog_hold_s` so the completeness case still holds; the wider value table (0, 59, 3601, -1, 600.5, `\"600\"`, `null`, `true`) is the `src/run/dialog.rs` unit `read_hold_falls_back_to_sixty_with_one_rejection`."
    sidecar: "2026-10-19 (chunk 2026-10-19-dialog-hold-for-the-driver): §5 controls table `SETTINGS` 9 → 10 rows (rule: `SETTINGS` in the file), the `dialog_hold_s` row and its booted case."
    rationale: "Report Counts moved, pattern `ctrltable`: `test-plan.md:594` stale by omission (gains the hold row and its booted case), and `cfgkeys` `test-plan.md:594` stale by omission; Schema / config: `dialog_hold_s` 60–3600, `CONFIG_KEYS` 5 → 6; Outcome: `setting_does_not_unbound_the_dialog_hold`, 5 cases, `every_flag_variable_and_config_key_is_a_row_of_the_table`."
    basis: tests/cli_controls_not_disableable.rs:394-418
  - detector: D-tests-coverage
    severity: warning
    section: §4 Unit Test Strategy → What unit tests cover → root bin
    change: "Add a bullet: the dialog hold (chunk 2026-10-19-dialog-hold-for-the-driver, `src/run/dialog.rs`): `read_hold` takes `config.json` `dialog_hold_s` once at start through `take(MAX_FRAME)` and `v` = 1 (`read_hold_takes_a_whole_number_of_seconds_in_range`, `read_hold_falls_back_to_sixty_with_one_rejection`); the hold is clock-driven on the injected `Clock` — pending one step before a configured 120 s hold and `null` at it (`hook_dialog_holds_until_the_configured_hold_on_the_injected_clock`); an expiry appends exactly one `dialog-expired` line and logs one `dialog-answered{deadline_hit:true}` under `corr`, and a late `answer` is `unknown-dialog` (`hook_dialog_expiry_writes_one_dialog_expired_line_and_one_answered_line`); a hand-back or an answer writes none (`hook_dialog_hand_back_writes_no_dialog_expired_line`); `pending_dialog` is on the snapshot before the dialog's line is logged and cleared when the append fails (`hook_dialog_held_is_on_the_snapshot_when_its_line_is_logged`, `hook_dialog_append_that_failed_clears_the_pending_dialog`); the hook's bound is `DIALOG_HOLD_MAX` + the reply grace (`dialog_deadline_is_the_longest_hold_plus_the_reply_grace`, `src/cmd/hook.rs`) and the embedded `hooks.json` dialog timeouts outlast the longest hold (`hooks_json_dialog_timeouts_outlast_the_longest_hold`); `viola wait`'s `dialog-expired` line takes the dialog form (`wait_line_form_dialog_expired_takes_the_dialog_form`, `src/cmd/wait.rs`). This is the §4 clock-driven case §11 Universal requires for the hold."
    sidecar: "2026-10-19 (chunk 2026-10-19-dialog-hold-for-the-driver): §4 root bin gains the dialog-hold unit cases (config read, injected-clock expiry, `dialog-expired`, snapshot-before-line order, hook bound, wait line form)."
    rationale: "Report Coverage of new surfaces names these unit cases for `dialog_hold_s`, the `dialog-expired` line, the `wait` line and the hook bound / timeouts; Deviation 4 adds the two order cases; §4's root bin list names none of the new paths, and §11 Universal requires a product deadline to be clock-driven in a §4 unit case."
    basis: src/run/dialog.rs:849-879
  - detector: D-tests-coverage
    severity: warning
    section: §4 Unit Test Strategy → What unit tests cover → viola-core
    change: "Add a bullet: `DialogHold` (chunk 2026-10-19-dialog-hold-for-the-driver) accepts whole seconds 60–3600 and rejects below and above, its `Default` the 60 s `DIALOG_HOLD_DEFAULT` (`dialog_hold_takes_whole_seconds_from_sixty_to_thirty_six_hundred`, a literal table of eight values); `EventKind::ALL` holds 16 kinds and `WAIT_WAKE` 6."
    sidecar: "2026-10-19 (chunk 2026-10-19-dialog-hold-for-the-driver): §4 viola-core gains the `DialogHold` bounds case and the `ALL` 16 / `WAIT_WAKE` 6 counts."
    rationale: "Report Symbols / APIs: `viola_core::DialogHold` nutype `greater_or_equal = 60, less_or_equal = 3600` with `Default`; Deviation 2 names the test and its 8 values; Outcome: `ALL` holds 16 and `WAIT_WAKE` 6. §4's viola-core list carries `Percent`'s bounds but no hold."
    basis: crates/viola-core/src/lib.rs:248-271
  - detector: D-tests-framework
    severity: warning
    section: §4 Unit Test Strategy → Conventions (Test grouping)
    change: "The exceptions to rstest `#[case]` tables are `viola-e2e` (no `[dev-dependencies]`) and `viola-core` (no rstest dev-dependency): their table-driven cases are a labelled literal case table in one `#[test]`, each assertion naming its label (in `viola-core`, e.g. `dialog_hold_takes_whole_seconds_from_sixty_to_thirty_six_hundred`, chunk 2026-10-19-dialog-hold-for-the-driver), so no manifest edge is added for rstest alone."
    sidecar: "2026-10-19 (chunk 2026-10-19-dialog-hold-for-the-driver): §4 Test grouping names `viola-core` beside `viola-e2e` as a crate whose table-driven cases are literal tables (no rstest dev-dependency)."
    rationale: "Report Deviation 2: the plan's rstest was written as a literal-table `#[test]` because `viola-core` has no rstest dev-dependency and the guard forbids a manifest edit; Decisions & corrections: that crate's tests are literal tables. §4 names `viola-e2e` as the only exception, so the runner/framework the chunk used is off the stated spec."
note: "D-tests-obs-harness: no drift (report Harness / gate surface: none; no harness verb, status shape or log format changed). D-retired: `DIALOG_DEADLINE` stands nowhere in test-plan (no occurrence of the constant, a 60 s dialog deadline, the 65 s hook bound or the `timeout` 75). `test-plan.md:571` and `:596` are the report's no-change rows."
