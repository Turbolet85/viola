# Step 7 — the measured shapes as unit cases: red before, green after

A one-shot control at implement (it mutates source and reverts, so it is no gate entry). Every reading is
`bash scripts/agent-run.sh run --unit --filter 'test(/live_shape/)'` unless it says otherwise; the times are read
from `date -u` before each run.

The cases live in `crates/viola-agent-claude/src/hook.rs`, `mod tests`:
- `prompt_text_live_shape_normalises_as_measured`, five cases (shapes 1 to 5 of the scratch session);
- `prompt_origin_live_shape_files_as_harness`, two cases (the typed `<task-notification>` at a prompt's start, and
  the cross-session raw start with its three attribute values replaced).

Each raw prompt is rebuilt from literals: the measured frame under the measured id `eec9`, and the 1 500-byte text
from the same head, filler and tail the driver pasted. A reading script checked each rebuilt string equal to its
captured `prompt` byte for byte (five of five), and each case asserts the measured prompt's length.

## Reading 1 — the cases against the base commit's unwrap, every expected `text` the bytes sent (10:18:17Z)
Exit 1, `"ok":false`: 7 run, 4 passed, 3 failed.

| case | reading |
|---|---|
| `case_1_typed_then_paste` | pass |
| `case_2_paste_then_typed` | **FAIL** |
| `case_3_two_pastes` | pass |
| `case_4_long_ending_newline` | **FAIL** |
| `case_5_short_ending_newline` | **FAIL** |
| the two `prompt_origin` cases | pass |

So HEAD's unwrap is falsified by three measured shapes.

## Reading 2 — with the fix inside `unwrap_pastes` (10:18:34Z)
The fix: after a pair's close tag and its one newline, a second newline goes too when text follows it that is not
the next pair's own two-newline frame. It removes the one byte the paste-then-typed measurement shows, inside
`unwrap_pastes`; `paste_pair` and every other file are untouched.

Exit 1, `"ok":false`: 7 run, 5 passed, 2 failed. `case_2_paste_then_typed` now passes. The two that still fail:

| case | why no fix inside `hook.rs` exists |
|---|---|
| `case_4_long_ending_newline` | the raw prompt of a wrapped text ending in a newline is byte for byte the raw prompt of the same text without that newline: the CLI adds no newline before the close tag for it |
| `case_5_short_ending_newline` | the CLI drops an unwrapped text's last newline before the hook is called: the prompt is 95 chars for 96 bytes sent |

**STOP 5.** A fix for these two leaves `hook.rs` (the send side, or the claim's exact match). Nothing was built for
it. The failing cases are recorded here, and the end-to-end reading is in `hint-window.md` (step 7 of each hint
run: `not-delivered` / `no-prompt-submitted`, the prompt filed `human`, the wheel moved).

## Reading 3 — the cases as committed (10:19:19Z)
Cases 4 and 5 now pin what the measured prompt normalises to: the text sent less its last newline, with the loss
named in the test's own comment. Exit 0, `"ok":true`: 7 run, 7 passed.

## The fix's two conditions, each removed once
`bash scripts/agent-run.sh run --unit --filter 'test(/prompt_text/)'`, 25 cases, the guard edited inside
`unwrap_pastes` itself and confirmed by line before each run.

| run | the guard as run | reading |
|---|---|---|
| 10:19:35Z | `!text.is_empty()` alone (the next-frame condition removed) | exit 1: 23 passed, 2 failed — `case_7_two_framed_pairs`, `case_3_two_pastes` |
| 10:19:48Z | `!tail.starts_with(NEXT_FRAME)` alone (the non-empty condition removed) | exit 1: 24 passed, 1 failed — `case_4_second_newline_after_stays` |
| 10:20:00Z | both conditions, as committed | exit 0: 25 passed, 0 failed |

The wrapped-paste property (`prompt_text_prop_round_trips_a_wrapped_paste`) gained a typed tail after the pair and
is among the 25; it recorded no failing seed, so `proptest-regressions/hook.txt` is unchanged.

## The two guessed cases
- `two_framed_pairs` now carries the measured id on both pairs: the two pairs of one prompt shared one id. Its
  newlines were already the measured ones (three between the pairs).
- `text_then_framed_pair` matches the measured typed-then-paste frame as it stood; it is unchanged.
