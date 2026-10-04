# Red-before-green controls — 2026-10-04-the-wheel

Each guard was neutralised by one asserted replacement, its own tests run through
`bash scripts/agent-run.sh run --unit --filter '<filter>'`, and the file restored and re-hashed (sha256 equal
to the pre-edit bytes). Green on the restored tree: the implement run's gate entries 3 and 5 (199 and every
unit test passed).

| # | guard (neutralised to) | filter | reading | restored |
|---|---|---|---|---|
| 1 | `src/run/wheel.rs` classifier: `is_reply` answer (→ `false`) | `test(/run::wheel::tests::classifier_/)` | red, exit 1: 22 passed, 1 failed — `classifier_every_listed_reply_is_not_editing_whole_and_split` | yes |
| 2 | `src/run/send.rs` the `human-typing` step (→ never human) | `test(/run::send::tests::send_refusal_order/)` | red, exit 1: 6 passed, 2 failed — cases `human_typing_before_turn_running`, `manual_pause_before_turn_running` | yes |
| 3 | `src/run/dialog.rs` `answer`'s `human-typing` slot (→ never human) | `test(/run::dialog::tests::answer_refusal_order_under_a_human_wheel/)` | red, exit 1: 1 passed, 4 failed — every `human_typing` / `manual_pause` row; the `control-character` row still green | yes |
| 4 | `src/run/dialog.rs` the hand-back flag (→ not set) | `test(/run::dialog::tests::hook_dialog_held_is_handed_back/)` | red, exit 1: 0 passed, 1 failed — the held dialog never ended on its fixed clock and the run's kill ended the test | yes |
| 5 | `src/run/wheel.rs` the observer's move (→ never) | `test(/run::wheel::tests::observed_/)` | red, exit 1: 0 passed, 1 failed — `observed_returns_every_byte_and_moves_the_wheel_on_a_key` | yes |
| 6 | `src/run/dialog.rs` the hold's driver check (→ always hold) | `test(/run::dialog::tests::hook_dialog_under_a_human_wheel/)` | red, exit 1: 0 passed, 1 failed — the dialog was held on its fixed clock and the run's kill ended the test | yes |
| 7 | `src/run/wheel.rs` the `release` `from` refusal (→ never) | `test(/cmd::run::tests::methods_release_from_a_driver/)` | red, exit 1: 0 passed, 2 failed — both cases | yes |

The integration and outer-PTY entries (6, 7) were vacuous red at P5 (no `cli_wheel` / `tui_wheel` binary);
their first implement run read 6 green and 7 red on one case of its own design
(`tui_focus_mouse_and_resize_never_take_the_wheel`: a driver `send` after the reports could never be read back,
because the reports sit in the fake agent's prompt line; the probe became an `answer` to no pending dialog),
then green, 74 passed.
