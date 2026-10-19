group: test-plan
read: .claude/docs/tests-summary.md · 65 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 101 · «- 2026-10-17: A harness-session case never assumes the fake »
read: .claude/rules/verification-harness.md · 71 · «- 2026-10-14: A side effect added to a fake-agent turn (a sp»

- leaf: .claude/docs/tests-summary.md:27   class: omission
  leaf-text: «where a PostToolUse `activity` line wakes no `wait`)»
  source: .andromeda/test-plan.md:676 «A Stop's `turn-ended` data carries `background_tasks`, the Stop array's length or `null`: `tests/hook_events.rs` `hook_a_recorded_stop_carries_its_background_task_count`»
  replace-with: «where a PostToolUse `activity` line wakes no `wait`); a Stop's `turn-ended` data carries `background_tasks`, the Stop array's length or `null` (`tests/hook_events.rs` `hook_a_recorded_stop_carries_its_background_task_count` over the recorded 2.1.287 `Stop.default.json`, so 0, with `state_events_background_tasks_and_option_details_are_known_keys`; a non-zero and a `null` count unit-tier; live on 2.1.296 on the Linux dev host, a local measurement and no suite, a product `wait --json` woke on `turn-ended` with `background_tasks` 0 and `last` read the turn)»

- leaf: .claude/docs/tests-summary.md:28   class: omission
  leaf-text: «its result and refusal compared with the CLI's `--json` document less `v`)»
  source: .andromeda/test-plan.md:696 «Each question's `option_details` (one `{description, preview}` per option, a string or `null`) is read by `tests/hook_events.rs` `hook_a_recorded_question_carries_its_option_details`»
  replace-with: «its result and refusal compared with the CLI's `--json` document less `v`); each question's `option_details` (one `{description, preview}` per option, a string or `null`) is read by `tests/hook_events.rs` `hook_a_recorded_question_carries_its_option_details` over the recorded 2.1.287 question (no `preview`, so the previews read `null`) and by the `state_events` case of §5 On-disk, non-null previews unit-tier; live on 2.1.296 on the Linux dev host, a local measurement and no suite, a `question` carried descriptions and previews and a product `viola answer` answered it with `dialog-answered{deadline_hit:false}` and no `from`»
