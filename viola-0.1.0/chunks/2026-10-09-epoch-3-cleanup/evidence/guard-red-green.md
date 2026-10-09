# The three new guards, each removed and restored (testing.md 2026-09-25)

One-shot readings at implement, 2026-10-09 (UTC). Each guard was neutralised by a scratch edit marked
`GUARD-OFF-E3`, the cases run, the guard restored. After the third restore the marker counts 0 under `src/`,
`crates/`, `tests/` and `.config/`, and `cargo fmt --all --check` exits 0.

Every run is the same command, the form the harness's `run` builds, over the new cases and every newline case:

```
CARGO_TARGET_DIR=target/harness cargo nextest run --workspace --features viola/fake-agent --profile ci \
  --no-fail-fast -E 'test(/empty_text/) | test(/typed_text_drops/) | test(/trailing_newline/) | test(/ending_in_newlines/)'
```

It selects 70 cases. On the tree with all three guards in place they pass: the gate block's own entries read them
green both before these runs (16:08Z) and after the restores (the block's run recorded in implement's report).

| guard | neutralised as | started | exit | summary | red |
|---|---|---|---|---|---|
| 1. the wrapper's rung, `src/run/send.rs` `send` | the `if text.is_empty()` test made false | 16:12:35Z | 100 | `70 tests run: 58 passed, 12 failed, 1730 skipped` | 12 |
| 2. the client's check, `src/cmd/send.rs` `refusal_of` | the empty-typed-text test made false | 16:13:02Z | 100 | `70 tests run: 64 passed, 6 failed, 1730 skipped` | 6 |
| 3. the CR half of the strip, `crates/viola-agent-claude/src/hook.rs` `typed_text` | the body put back to `trim_end_matches('\n')` | 16:13:17Z | 100 | `70 tests run: 39 passed, 31 failed, 1730 skipped` | 31 |

## What went red

**Guard 1 (12).** Each removes the guard inside the function the cases call (testing.md 2026-10-07).
- `run::send::tests::send_empty_text_is_refused_first_after_control_character`, all four cases (the empty text, one
  LF, one CRLF, two CRs);
- `run::send::tests::send_empty_text_under_a_human_wheel_is_still_empty_text` and
  `…send_empty_text_while_a_turn_runs_is_still_empty_text`;
- `run::send::tests::send_refusal_order`, its five `empty_text_before_…` cases (human typing, manual pause, turn
  running, a running turn, input not ready);
- `channel_paste_validation send_empty_text_straight_to_the_wrapper_is_refused`, the cross-process wrapper half.

The client's cases stayed green, as they must: the client refuses before any frame, so they never reach the wrapper.

**Guard 2 (6).**
- `cmd::send::tests::deliver_empty_text_is_refused_before_any_frame`, all three cases: the call ends 21 where it
  must end 13;
- `cli_send send_empty_text_is_refused_at_once`, all three cases.

The wrapper's cases stayed green: with the client's check off the wrapper still refuses, which is why the
cross-process case asserts that no record and no wrapper line exists, not only the exit code.

**Guard 3 (31).** Every red case holds a CR; no LF-only case went red.
- `hook::tests::typed_text_drops_every_trailing_newline_and_nothing_else`: one CR, one CRLF, two CRs, CRLF twice,
  an LF before a CR, only CRs and LFs (6);
- `run::send::tests`: the five CR-bearing tails of `send_trailing_newlines_are_not_typed_and_the_send_is_confirmed`;
  the CR and CRLF cases of `send_a_prompt_that_keeps_the_trailing_newline_claims_nothing`, of
  `send_local_command_decision_reads_the_text_without_its_trailing_newline`, of
  `send_local_command_without_a_post_condition_and_a_trailing_newline_is_unconfirmable` and of
  `send_local_command_clear_with_a_trailing_newline_is_confirmed_by_a_new_session` (8); the CRLF and two-CR cases of
  `send_empty_text_is_refused_first_after_control_character`, `send_empty_text_under_a_human_wheel_is_still_empty_text`,
  and three `send_refusal_order` cases whose text holds a CR (6);
- `cmd::send::tests::deliver_empty_text_is_refused_before_any_frame`, its CRLF case;
- `channel_paste_validation send_empty_text_straight_to_the_wrapper_is_refused`;
- `cli_send send_empty_text_is_refused_at_once`, its CRLF case, and
  `cli_send send_text_ending_in_newlines_is_typed_without_them_and_confirmed`, its one-CR, one-CRLF and two-CR
  cases: the receipt's last typed byte is a CR.

## The order case

The fourth new guard test, the override order in `.config/nextest.toml`, has its two readings in
`planted-hang.md`.
