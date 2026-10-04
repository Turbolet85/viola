# Red-before-green controls (the plan's baselines: "its control is implement red-before-green")

Each guard was neutralised in place, the chunk's new tests run through the harness, the guard restored, and the
restore confirmed with a re-read of the line. Readings from the harness's nextest lines.

## 1. The rung's running-turn read (`src/run/send.rs`, `send`: `flight.is_some() || slot.wheel.turn_running()`)

Neutralised to `flight.is_some()`:
- `run --unit --filter 'test(/run::wheel::tests::|run::send::tests::|cmd::run::tests::|human::tests::/)'`: exit 1,
  150 run, 146 passed, 3 failed, 1 timed out:
  - FAIL `run::send::tests::send_refusal_order::case_09_a_running_turn_before_input_not_ready`
  - FAIL `run::send::tests::send_a_prompt_of_any_origin_starts_a_turn::case_1_harness`
  - FAIL `run::send::tests::send_a_turn_end_lets_the_next_send_past_the_rung::case_4_activity`
  - TIMEOUT (120 s) `run::send::tests::send_after_a_confirmed_send_is_turn_running_until_turn_ended`: under a fixed
    clock the wrongly-pasted second send parked on its confirmation window. The test was reworked (a clock that
    jumps only while the refused send runs), and the control re-run reads FAIL in 0.186 s, no timeout.
- `run --integration --filter 'binary(cli_send) | binary(cli_wheel) | binary(cli_controls_not_disableable) |
  binary(tui_wheel)'`: exit 1, 31 run, 27 passed, 4 failed:
  - `cli_wheel::path5_a_turn_left_running_is_cleared_by_pause_then_release`
  - `tui_wheel::path5_harness_turns_never_take_the_wheel` (the hypothesis witness)
  - `cli_send::send_after_a_confirmed_send_is_turn_running_until_turn_ended`
  - `cli_controls_not_disableable::setting_does_not_disable_the_send_turn_running_control`

Restored: the reworked unit test PASS (0.006 s).

## 2. The release clear (`src/run/wheel.rs`, `apply`: `held.turn = Turn::Idle` on `Input::Release`)

Neutralised to `let _ = Turn::Idle;`:
- `run --unit --filter 'test(/run::wheel::tests::/)'`: exit 1, 42 run, 41 passed, 1 failed:
  `run::wheel::tests::wheel_only_a_release_returning_the_wheel_clears_the_turn::case_1_release_returning_the_wheel`.
- `run --integration --filter 'binary(cli_wheel) | binary(tui_wheel)'`: exit 1, 7 run, 5 passed, 2 failed:
  `tui_wheel::path5_human_takes_the_wheel_and_release_returns_it` (its post-release `send` after the unended human
  prompt, the plan's wheel-move witness) and `cli_wheel::path5_a_turn_left_running_is_cleared_by_pause_then_release`.

Restored: line re-read as `held.turn = Turn::Idle;`.
