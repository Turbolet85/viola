# Epoch 4 survivors II — the ledger and the hand-applied kills

Chunk 2026-10-13-epoch-4-survivors-ii. The 16 rows are the `owed` rows of
`viola-0.1.0/chunks/2026-10-13-epoch-4-survivors/evidence/survivors.ndjson` (the Epoch 4 boundary audit's
`typed.rs` survivors, coordinates at `dffad3e`), keyed on its coordinate and mutation text; the machine form is
`survivors.ndjson` beside this file. Every row is disposed by a restatement in `src/cmd/verify/typed.rs` and a
case in its `mod tests`. The reader that confirms each kill is the Epoch 5 boundary audit's own mutation run: no
mutation run, witness run or workflow dispatch was made here (`inputs#I1`).

Each mutation was applied by hand at its restated site (one exact replacement, `rustfmt` in the same process,
the edit confirmed by a byte comparison), read through `bash scripts/agent-run.sh run --unit --filter
'test(/(^|::)tests::(<case>)(::|$)/)'`, then the file restored from its saved bytes (confirmed byte-equal) and the
same filter read green. `(::|$)` in place of the plan's `$`: an rstest table case's name ends
`::case_<n>_<label>`. 14 applications cover the 16 rows: `:286:29`, `:651:45` and `:671:53` are the one `+` in
`probe_until` (m03). No red was a compile error or a panic outside an assertion; m03's `Instant - 120 s` did not
panic (the host's monotonic clock was past 120 s).

Written 2026-10-13T17:52:31Z (the readings were taken just before, in this chunk's implement run).

## The ledger

| coordinate (audit) | mutation | at HEAD (restated) | restated mutation | form | by | mutant |
|---|---|---|---|---|---|---|
| `src/cmd/verify/typed.rs:271:84` | replace > with == in framing_turns | `src/cmd/verify/typed.rs:296:48` | replace > with == in added_turn_ended | restatement | src/cmd/verify/typed.rs::tests::added_turn_ended_needs_a_new_prompt_and_a_new_stop::case_2_a_prompt_and_no_stop | m01 |
| `src/cmd/verify/typed.rs:271:84` | replace > with >= in framing_turns | `src/cmd/verify/typed.rs:296:48` | replace > with >= in added_turn_ended | restatement | src/cmd/verify/typed.rs::tests::added_turn_ended_needs_a_new_prompt_and_a_new_stop::case_2_a_prompt_and_no_stop | m02 |
| `src/cmd/verify/typed.rs:286:29` | replace + with - in framing_turns | `src/cmd/verify/typed.rs:308:17` | replace + with - in probe_until | restatement | src/cmd/verify/typed.rs::tests::probe_until_is_the_probe_deadline_after_now | m03 |
| `src/cmd/verify/typed.rs:288:40` | replace > with == in framing_turns | `src/cmd/verify/typed.rs:302:49` | replace > with == in local_command_answered | restatement | src/cmd/verify/typed.rs::tests::local_command_answered_needs_a_new_start_or_a_new_prompt::case_1_none_added | m04 |
| `src/cmd/verify/typed.rs:288:40` | replace > with >= in framing_turns | `src/cmd/verify/typed.rs:302:49` | replace > with >= in local_command_answered | restatement | src/cmd/verify/typed.rs::tests::local_command_answered_needs_a_new_start_or_a_new_prompt::case_1_none_added | m05 |
| `src/cmd/verify/typed.rs:403:9` | replace <impl Pty for SharedPty>::resize -> Result<(), PtyError> with Ok(()) | `src/cmd/verify/typed.rs:419:9` | replace <impl Pty for SharedPty<P>>::resize -> Result<(), PtyError> with Ok(()) | restatement | src/cmd/verify/typed.rs::tests::shared_pty_resize_reaches_its_pty_with_the_size | m06 |
| `src/cmd/verify/typed.rs:415:9` | replace <impl Pty for SharedPty>::close with () | `src/cmd/verify/typed.rs:431:9` | replace <impl Pty for SharedPty<P>>::close with () | restatement | src/cmd/verify/typed.rs::tests::shared_pty_close_reaches_its_pty_once | m07 |
| `src/cmd/verify/typed.rs:419:9` | replace <impl Pty for SharedPty>::child_pid -> Option<u32> with None | `src/cmd/verify/typed.rs:435:9` | replace <impl Pty for SharedPty<P>>::child_pid -> Option<u32> with None | restatement | src/cmd/verify/typed.rs::tests::shared_pty_child_pid_is_its_ptys | m08 |
| `src/cmd/verify/typed.rs:419:9` | replace <impl Pty for SharedPty>::child_pid -> Option<u32> with Some(0) | `src/cmd/verify/typed.rs:435:9` | replace <impl Pty for SharedPty<P>>::child_pid -> Option<u32> with Some(0) | restatement | src/cmd/verify/typed.rs::tests::shared_pty_child_pid_is_its_ptys | m09 |
| `src/cmd/verify/typed.rs:419:9` | replace <impl Pty for SharedPty>::child_pid -> Option<u32> with Some(1) | `src/cmd/verify/typed.rs:435:9` | replace <impl Pty for SharedPty<P>>::child_pid -> Option<u32> with Some(1) | restatement | src/cmd/verify/typed.rs::tests::shared_pty_child_pid_is_its_ptys | m10 |
| `src/cmd/verify/typed.rs:648:12` | delete ! in Run::end_by_ctrl_c | `src/cmd/verify/typed.rs:550:8` | delete ! in ctrl_c_ending | restatement | src/cmd/verify/typed.rs::tests::ctrl_c_ending_sends_again_only_when_the_first_did_not_end_it::case_2_ended_by_the_second | m11 |
| `src/cmd/verify/typed.rs:648:45` | replace + with - in Run::end_by_ctrl_c | `src/cmd/verify/typed.rs:550:27` | replace + with - in ctrl_c_ending | restatement | src/cmd/verify/typed.rs::tests::ctrl_c_ending_sends_again_only_when_the_first_did_not_end_it (4 cases) | m12 |
| `src/cmd/verify/typed.rs:651:12` | delete ! in Run::end_by_ctrl_c | `src/cmd/verify/typed.rs:553:8` | delete ! in ctrl_c_ending | restatement | src/cmd/verify/typed.rs::tests::ctrl_c_ending_sends_again_only_when_the_first_did_not_end_it::case_2_ended_by_the_second | m13 |
| `src/cmd/verify/typed.rs:651:45` | replace + with - in Run::end_by_ctrl_c | `src/cmd/verify/typed.rs:308:17` | replace + with - in probe_until | restatement | src/cmd/verify/typed.rs::tests::probe_until_is_the_probe_deadline_after_now | m03 |
| `src/cmd/verify/typed.rs:659:9` | replace Run::wait_pump -> bool with true | `src/cmd/verify/typed.rs:560:5` | replace ended_by -> bool with true | restatement | src/cmd/verify/typed.rs::tests::ended_by_reads_true_once_finished_and_false_once_the_deadline_passes::case_2_never_finished_at_the_deadline | m14 |
| `src/cmd/verify/typed.rs:671:53` | replace + with - in Run::finish | `src/cmd/verify/typed.rs:308:17` | replace + with - in probe_until | restatement | src/cmd/verify/typed.rs::tests::probe_until_is_the_probe_deadline_after_now | m03 |

## The readings

- **m01** · `src/cmd/verify/typed.rs:271:84` replace > with == in framing_turns · `src/cmd/verify/typed.rs:296:48` replace > with == in added_turn_ended
  - red: Summary 5 tests run: 3 passed, 2 failed, 2013 skipped
    - added_turn_ended_needs_a_new_prompt_and_a_new_stop::case_2_a_prompt_and_no_stop @ src/cmd/verify/typed.rs:977:9 `assertion `left == right` failed` left: true right: false
    - added_turn_ended_needs_a_new_prompt_and_a_new_stop::case_4_both_added @ src/cmd/verify/typed.rs:977:9 `assertion `left == right` failed` left: false right: true
  - green (restored): Summary 5 tests run: 5 passed, 2013 skipped
- **m02** · `src/cmd/verify/typed.rs:271:84` replace > with >= in framing_turns · `src/cmd/verify/typed.rs:296:48` replace > with >= in added_turn_ended
  - red: Summary 5 tests run: 4 passed, 1 failed, 2013 skipped
    - added_turn_ended_needs_a_new_prompt_and_a_new_stop::case_2_a_prompt_and_no_stop @ src/cmd/verify/typed.rs:977:9 `assertion `left == right` failed` left: true right: false
  - green (restored): Summary 5 tests run: 5 passed, 2013 skipped
- **m03** · `src/cmd/verify/typed.rs:286:29` replace + with - in framing_turns, `src/cmd/verify/typed.rs:651:45` replace + with - in Run::end_by_ctrl_c, `src/cmd/verify/typed.rs:671:53` replace + with - in Run::finish · `src/cmd/verify/typed.rs:308:17` replace + with - in probe_until
  - red: Summary 5 tests run: 0 passed, 5 failed, 2013 skipped
    - ctrl_c_ending_sends_again_only_when_the_first_did_not_end_it::case_1_ended_by_the_first @ src/cmd/verify/typed.rs:1108:9 `assertion `left == right` failed` left/right: the wait instants differ
    - ctrl_c_ending_sends_again_only_when_the_first_did_not_end_it::case_2_ended_by_the_second @ src/cmd/verify/typed.rs:1108:9 `assertion `left == right` failed` left/right: the wait instants differ
    - ctrl_c_ending_sends_again_only_when_the_first_did_not_end_it::case_3_never_ended @ src/cmd/verify/typed.rs:1108:9 `assertion `left == right` failed` left/right: the wait instants differ
    - ctrl_c_ending_sends_again_only_when_the_first_did_not_end_it::case_4_ended_then_not @ src/cmd/verify/typed.rs:1108:9 `assertion `left == right` failed` left/right: the wait instants differ
    - probe_until_is_the_probe_deadline_after_now @ src/cmd/verify/typed.rs:1012:9 `assertion `left == right` failed` left/right: the wait instants differ
  - green (restored): Summary 5 tests run: 5 passed, 2013 skipped
- **m04** · `src/cmd/verify/typed.rs:288:40` replace > with == in framing_turns · `src/cmd/verify/typed.rs:302:49` replace > with == in local_command_answered
  - red: Summary 4 tests run: 1 passed, 3 failed, 2014 skipped
    - local_command_answered_needs_a_new_start_or_a_new_prompt::case_1_none_added @ src/cmd/verify/typed.rs:995:9 `assertion `left == right` failed` left: true right: false
    - local_command_answered_needs_a_new_start_or_a_new_prompt::case_2_a_start @ src/cmd/verify/typed.rs:995:9 `assertion `left == right` failed` left: false right: true
    - local_command_answered_needs_a_new_start_or_a_new_prompt::case_4_a_start_already_there @ src/cmd/verify/typed.rs:995:9 `assertion `left == right` failed` left: true right: false
  - green (restored): Summary 4 tests run: 4 passed, 2014 skipped
- **m05** · `src/cmd/verify/typed.rs:288:40` replace > with >= in framing_turns · `src/cmd/verify/typed.rs:302:49` replace > with >= in local_command_answered
  - red: Summary 4 tests run: 2 passed, 2 failed, 2014 skipped
    - local_command_answered_needs_a_new_start_or_a_new_prompt::case_1_none_added @ src/cmd/verify/typed.rs:995:9 `assertion `left == right` failed` left: true right: false
    - local_command_answered_needs_a_new_start_or_a_new_prompt::case_4_a_start_already_there @ src/cmd/verify/typed.rs:995:9 `assertion `left == right` failed` left: true right: false
  - green (restored): Summary 4 tests run: 4 passed, 2014 skipped
- **m06** · `src/cmd/verify/typed.rs:403:9` replace <impl Pty for SharedPty>::resize -> Result<(), PtyError> with Ok(()) · `src/cmd/verify/typed.rs:419:9` replace <impl Pty for SharedPty<P>>::resize -> Result<(), PtyError> with Ok(())
  - red: Summary 1 test run: 0 passed, 1 failed, 2017 skipped
    - shared_pty_resize_reaches_its_pty_with_the_size @ src/cmd/verify/typed.rs:1064:9 `assertion `left == right` failed` left: None right: Some(Size { cols: 120, rows: 40 })
  - green (restored): Summary 1 test run: 1 passed, 2017 skipped
- **m07** · `src/cmd/verify/typed.rs:415:9` replace <impl Pty for SharedPty>::close with () · `src/cmd/verify/typed.rs:431:9` replace <impl Pty for SharedPty<P>>::close with ()
  - red: Summary 1 test run: 0 passed, 1 failed, 2017 skipped
    - shared_pty_close_reaches_its_pty_once @ src/cmd/verify/typed.rs:1077:9 `assertion `left == right` failed` left: 0 right: 1
  - green (restored): Summary 1 test run: 1 passed, 2017 skipped
- **m08** · `src/cmd/verify/typed.rs:419:9` replace <impl Pty for SharedPty>::child_pid -> Option<u32> with None · `src/cmd/verify/typed.rs:435:9` replace <impl Pty for SharedPty<P>>::child_pid -> Option<u32> with None
  - red: Summary 1 test run: 0 passed, 1 failed, 2017 skipped
    - shared_pty_child_pid_is_its_ptys @ src/cmd/verify/typed.rs:1082:9 `assertion `left == right` failed` left: None right: Some(4242)
  - green (restored): Summary 1 test run: 1 passed, 2017 skipped
- **m09** · `src/cmd/verify/typed.rs:419:9` replace <impl Pty for SharedPty>::child_pid -> Option<u32> with Some(0) · `src/cmd/verify/typed.rs:435:9` replace <impl Pty for SharedPty<P>>::child_pid -> Option<u32> with Some(0)
  - red: Summary 1 test run: 0 passed, 1 failed, 2017 skipped
    - shared_pty_child_pid_is_its_ptys @ src/cmd/verify/typed.rs:1082:9 `assertion `left == right` failed` left: Some(0) right: Some(4242)
  - green (restored): Summary 1 test run: 1 passed, 2017 skipped
- **m10** · `src/cmd/verify/typed.rs:419:9` replace <impl Pty for SharedPty>::child_pid -> Option<u32> with Some(1) · `src/cmd/verify/typed.rs:435:9` replace <impl Pty for SharedPty<P>>::child_pid -> Option<u32> with Some(1)
  - red: Summary 1 test run: 0 passed, 1 failed, 2017 skipped
    - shared_pty_child_pid_is_its_ptys @ src/cmd/verify/typed.rs:1082:9 `assertion `left == right` failed` left: Some(1) right: Some(4242)
  - green (restored): Summary 1 test run: 1 passed, 2017 skipped
- **m11** · `src/cmd/verify/typed.rs:648:12` delete ! in Run::end_by_ctrl_c · `src/cmd/verify/typed.rs:550:8` delete ! in ctrl_c_ending
  - red: Summary 4 tests run: 0 passed, 4 failed, 2014 skipped
    - ctrl_c_ending_sends_again_only_when_the_first_did_not_end_it::case_1_ended_by_the_first @ src/cmd/verify/typed.rs:1107:9 `assertion `left == right` failed` left: 2 right: 1
    - ctrl_c_ending_sends_again_only_when_the_first_did_not_end_it::case_2_ended_by_the_second @ src/cmd/verify/typed.rs:1107:9 `assertion `left == right` failed` left: 1 right: 2
    - ctrl_c_ending_sends_again_only_when_the_first_did_not_end_it::case_3_never_ended @ src/cmd/verify/typed.rs:1107:9 `assertion `left == right` failed` left: 1 right: 2
    - ctrl_c_ending_sends_again_only_when_the_first_did_not_end_it::case_4_ended_then_not @ src/cmd/verify/typed.rs:1107:9 `assertion `left == right` failed` left: 2 right: 1
  - green (restored): Summary 4 tests run: 4 passed, 2014 skipped
- **m12** · `src/cmd/verify/typed.rs:648:45` replace + with - in Run::end_by_ctrl_c · `src/cmd/verify/typed.rs:550:27` replace + with - in ctrl_c_ending
  - red: Summary 4 tests run: 0 passed, 4 failed, 2014 skipped
    - ctrl_c_ending_sends_again_only_when_the_first_did_not_end_it::case_1_ended_by_the_first @ src/cmd/verify/typed.rs:1108:9 `assertion `left == right` failed` left/right: the wait instants differ
    - ctrl_c_ending_sends_again_only_when_the_first_did_not_end_it::case_2_ended_by_the_second @ src/cmd/verify/typed.rs:1108:9 `assertion `left == right` failed` left/right: the wait instants differ
    - ctrl_c_ending_sends_again_only_when_the_first_did_not_end_it::case_3_never_ended @ src/cmd/verify/typed.rs:1108:9 `assertion `left == right` failed` left/right: the wait instants differ
    - ctrl_c_ending_sends_again_only_when_the_first_did_not_end_it::case_4_ended_then_not @ src/cmd/verify/typed.rs:1108:9 `assertion `left == right` failed` left/right: the wait instants differ
  - green (restored): Summary 4 tests run: 4 passed, 2014 skipped
- **m13** · `src/cmd/verify/typed.rs:651:12` delete ! in Run::end_by_ctrl_c · `src/cmd/verify/typed.rs:553:8` delete ! in ctrl_c_ending
  - red: Summary 4 tests run: 0 passed, 4 failed, 2014 skipped
    - ctrl_c_ending_sends_again_only_when_the_first_did_not_end_it::case_1_ended_by_the_first @ src/cmd/verify/typed.rs:1112:9 `assertion `left == right` failed` left: 1 right: 0
    - ctrl_c_ending_sends_again_only_when_the_first_did_not_end_it::case_2_ended_by_the_second @ src/cmd/verify/typed.rs:1112:9 `assertion `left == right` failed` left: 1 right: 0
    - ctrl_c_ending_sends_again_only_when_the_first_did_not_end_it::case_3_never_ended @ src/cmd/verify/typed.rs:1112:9 `assertion `left == right` failed` left: 0 right: 1
    - ctrl_c_ending_sends_again_only_when_the_first_did_not_end_it::case_4_ended_then_not @ src/cmd/verify/typed.rs:1112:9 `assertion `left == right` failed` left: 0 right: 1
  - green (restored): Summary 4 tests run: 4 passed, 2014 skipped
- **m14** · `src/cmd/verify/typed.rs:659:9` replace Run::wait_pump -> bool with true · `src/cmd/verify/typed.rs:560:5` replace ended_by -> bool with true
  - red: Summary 4 tests run: 1 passed, 3 failed, 2014 skipped
    - ended_by_reads_true_once_finished_and_false_once_the_deadline_passes::case_2_never_finished_at_the_deadline @ src/cmd/verify/typed.rs:1126:9 `assertion `left == right` failed` left: true right: false
    - ended_by_reads_true_once_finished_and_false_once_the_deadline_passes::case_3_finished_after_the_deadline @ src/cmd/verify/typed.rs:1126:9 `assertion `left == right` failed` left: true right: false
    - ended_by_reads_true_once_finished_and_false_once_the_deadline_passes::case_4_deadline_already_past @ src/cmd/verify/typed.rs:1126:9 `assertion `left == right` failed` left: true right: false
  - green (restored): Summary 4 tests run: 4 passed, 2014 skipped
