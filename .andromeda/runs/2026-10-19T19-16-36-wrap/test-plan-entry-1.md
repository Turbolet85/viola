
## 2026-10-19-overseer-switch-to-viola — the `option_details` and `background_tasks` cases named, with the switch's live readings
**Section:** §4 Unit Test Strategy (viola-agent-claude); §5 Integration Test Strategy (On-disk, `state_events.rs`); §6 Scenario: Path 3 — `wait` / `last`; §6 Scenario: Path 4 — dialog → `answer`.
**Change:**
- §4 names `question_option_details_carry_description_and_preview`, `question_option_details_are_null_when_absent_or_not_a_string` (`dialog.rs`), `turn_ended_background_tasks_counts_the_array` and `turn_ended_background_tasks_is_null_when_absent_or_not_an_array` (`hook.rs`).
- §5 On-disk names `state_events_background_tasks_and_option_details_are_known_keys` and `EventKind::TurnEnded`'s data keys `last_assistant_message`, `background_tasks`.
- Path 3 names `hook_a_recorded_stop_carries_its_background_task_count` (the 2.1.287 fixture's empty array reads 0; non-zero and `null` held at the unit tier) and the live reading on 2.1.296, a local measurement and no suite: `wait --json` woke on `turn-ended` with `background_tasks` 0, and `last` read the turn.
- Path 4 names `hook_a_recorded_question_carries_its_option_details` (the 2.1.287 fixture holds no `preview`, so previews read `null`) and the live reading on 2.1.296: a `question` with descriptions and previews answered by a product `viola answer`, `dialog-answered{deadline_hit:false}` with no `from`.
**Why:** The chunk's two additive data keys carry tests at the unit and integration tiers, and the plan asked for the switch's live readings stated as a local measurement, not a suite.
**Ref:** .andromeda/runs/2026-10-19T19-16-36-wrap/
