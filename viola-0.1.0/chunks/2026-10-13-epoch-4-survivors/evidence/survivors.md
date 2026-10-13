# Epoch 4 survivors — the ledger and the hand-applied kills

Chunk 2026-10-13-epoch-4-survivors. The 47 rows are the Epoch 4 boundary audit's `mutation.survivors`
(`.andromeda/runs/2026-10-12T06-54-29-code-audit/record.json`), keyed on its coordinate and mutation text; the
machine form is `survivors.ndjson` beside this file. The reader that confirms each kill is the Epoch 5 boundary
audit's own mutation run: no mutation run, witness run or workflow dispatch was made here (`inputs#I1`).

## The ledger

| coordinate (audit) | mutation | at HEAD | form | by |
|---|---|---|---|---|
| `src/cmd/hook.rs:403:27` | replace || with && in answer | `src/cmd/hook.rs:403:27` | test | src/cmd/hook.rs::tests::answer_reads_no_relative_answers_dir_that_resolves |
| `src/cmd/send.rs:119:12` | delete ! in Out<'_>::open | `src/cmd/send.rs:119:12` | restatement | src/cmd/send.rs::tests::issue_line_shows_only_for_a_human_on_a_terminal (4 cases) |
| `src/cmd/verify/typed.rs:271:84` | replace > with == in framing_turns | `src/cmd/verify/typed.rs:271:84` | owed | Epoch 4 survivors II |
| `src/cmd/verify/typed.rs:271:84` | replace > with >= in framing_turns | `src/cmd/verify/typed.rs:271:84` | owed | Epoch 4 survivors II |
| `src/cmd/verify/typed.rs:286:29` | replace + with - in framing_turns | `src/cmd/verify/typed.rs:286:29` | owed | Epoch 4 survivors II |
| `src/cmd/verify/typed.rs:288:40` | replace > with == in framing_turns | `src/cmd/verify/typed.rs:288:40` | owed | Epoch 4 survivors II |
| `src/cmd/verify/typed.rs:288:40` | replace > with >= in framing_turns | `src/cmd/verify/typed.rs:288:40` | owed | Epoch 4 survivors II |
| `src/cmd/verify/typed.rs:304:54` | replace && with || in max_gap | `src/cmd/verify/typed.rs:304:54` | test | src/cmd/verify/typed.rs::tests::max_gap_leaves_out_a_chunk_after_the_window |
| `src/cmd/verify/typed.rs:403:9` | replace <impl Pty for SharedPty>::resize -> Result<(), PtyError> with Ok(()) | `src/cmd/verify/typed.rs:403:9` | owed | Epoch 4 survivors II |
| `src/cmd/verify/typed.rs:415:9` | replace <impl Pty for SharedPty>::close with () | `src/cmd/verify/typed.rs:415:9` | owed | Epoch 4 survivors II |
| `src/cmd/verify/typed.rs:419:9` | replace <impl Pty for SharedPty>::child_pid -> Option<u32> with None | `src/cmd/verify/typed.rs:419:9` | owed | Epoch 4 survivors II |
| `src/cmd/verify/typed.rs:419:9` | replace <impl Pty for SharedPty>::child_pid -> Option<u32> with Some(0) | `src/cmd/verify/typed.rs:419:9` | owed | Epoch 4 survivors II |
| `src/cmd/verify/typed.rs:419:9` | replace <impl Pty for SharedPty>::child_pid -> Option<u32> with Some(1) | `src/cmd/verify/typed.rs:419:9` | owed | Epoch 4 survivors II |
| `src/cmd/verify/typed.rs:439:23` | replace > with >= in Feed::take | `src/cmd/verify/typed.rs:439:23` | test | src/cmd/verify/typed.rs::tests::feed_reaching_exactly_the_frame_cap_does_not_poison_and_one_byte_past_does |
| `src/cmd/verify/typed.rs:648:12` | delete ! in Run::end_by_ctrl_c | `src/cmd/verify/typed.rs:648:12` | owed | Epoch 4 survivors II |
| `src/cmd/verify/typed.rs:648:45` | replace + with - in Run::end_by_ctrl_c | `src/cmd/verify/typed.rs:648:45` | owed | Epoch 4 survivors II |
| `src/cmd/verify/typed.rs:651:12` | delete ! in Run::end_by_ctrl_c | `src/cmd/verify/typed.rs:651:12` | owed | Epoch 4 survivors II |
| `src/cmd/verify/typed.rs:651:45` | replace + with - in Run::end_by_ctrl_c | `src/cmd/verify/typed.rs:651:45` | owed | Epoch 4 survivors II |
| `src/cmd/verify/typed.rs:659:9` | replace Run::wait_pump -> bool with true | `src/cmd/verify/typed.rs:659:9` | owed | Epoch 4 survivors II |
| `src/cmd/verify/typed.rs:671:53` | replace + with - in Run::finish | `src/cmd/verify/typed.rs:671:53` | owed | Epoch 4 survivors II |
| `src/run/dialog.rs:144:5` | replace millis -> u64 with 1 | `src/run/dialog.rs:144:5` | test | src/run/dialog.rs::tests::millis_is_the_whole_milliseconds_capped_at_u64_max |
| `src/run/dialog.rs:208:42` | replace != with == in DialogSlot::hook_dialog | `src/run/dialog.rs:208:42` | test | src/run/dialog.rs::tests::continuation_survives_a_pre_tool_use_for_another_tool |
| `src/run/send.rs:271:5` | replace close with () | `src/run/send/slot.rs:207:5` | test | src/run/send/slot.rs::tests::send_expired_window_closes_the_send_so_a_late_line_is_not_claimed |
| `src/run/wheel.rs:297:18` | replace && with || in Recorder::record | `src/run/wheel.rs:297:18` | test | src/run/wheel.rs::tests::wheel_pause_whose_record_cannot_be_appended_is_internal |
| `src/run/wheel.rs:393:9` | replace Classifier::abort -> bool with true | `src/run/wheel.rs:393:9` | test | src/run/wheel.rs::tests::classifier_split_reads_carry_the_unfinished_sequence::case_2_csi_cut_by_an_escape_then_a_focus_report |
| `src/run/wheel.rs:427:17` | delete match arm ESC in Classifier::step | `src/run/wheel.rs:427:17` | test | src/run/wheel.rs::tests::classifier_split_reads_carry_the_unfinished_sequence::case_1_esc_key_then_a_focus_report |
| `src/run/wheel.rs:434:32` | replace match guard self.params.len() < BOUND with true in Classifier::step | `src/run/wheel.rs:434:32` | test | src/run/wheel.rs::tests::classifier_typing_is_editing::sgr_mouse_one_past_the_bound (pair: classifier_terminal_reply_is_not_editing::sgr_mouse_at_the_bound) |
| `src/run/wheel.rs:434:50` | replace < with <= in Classifier::step | `src/run/wheel.rs:434:50` | test | src/run/wheel.rs::tests::classifier_typing_is_editing::sgr_mouse_one_past_the_bound (pair: classifier_terminal_reply_is_not_editing::sgr_mouse_at_the_bound) |
| `src/run/wheel.rs:458:24` | replace match guard osc with true in Classifier::step | `src/run/wheel.rs:458:24` | test | src/run/wheel.rs::tests::classifier_typing_is_editing::dcs_ended_by_bel |
| `src/run/wheel.rs:467:35` | replace < with <= in Classifier::step | `src/run/wheel.rs:467:35` | test | src/run/wheel.rs::tests::classifier_typing_is_editing::osc_one_past_the_bound (pair: classifier_terminal_reply_is_not_editing::osc_at_the_bound) |
| `src/run/wheel.rs:516:49` | replace == with != in is_reply | `src/run/wheel.rs:516:49` | test | src/run/wheel.rs::tests::classifier_terminal_reply_is_not_editing::da1_with_no_field, ::da2_with_no_field |
| `src/run/wheel.rs:518:47` | replace && with || in is_reply | `src/run/wheel.rs:518:47` | test | src/run/wheel.rs::tests::classifier_typing_is_editing::kitty_flags_with_two_fields |
| `crates/viola-pty/src/lib.rs:202:9` | replace <impl Pty for PortablePty>::resize -> Result<(), PtyError> with Ok(()) | `crates/viola-pty/src/lib.rs:202:9` | test | crates/viola-pty/src/lib.rs::tests::spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code, ::spawn_reports_a_resize_to_a_child_that_reads_no_key, ::spawn_delivers_a_key_written_right_after_a_resize_to_a_child_not_yet_reading (report files per process) |
| `crates/viola-state/src/board.rs:164:29` | replace += with *= in SkippedTotal::add | `crates/viola-state/src/board.rs:164:29` | test | crates/viola-state/src/board.rs::tests::board_read_sums_the_skip_counts_over_every_instance |
| `crates/viola-state/src/board.rs:164:29` | replace += with -= in SkippedTotal::add | `crates/viola-state/src/board.rs:164:29` | test | crates/viola-state/src/board.rs::tests::board_read_sums_the_skip_counts_over_every_instance |
| `crates/viola-state/src/board.rs:45:19` | replace match guard e.kind() == io::ErrorKind::NotFound with true in check_if_present | `crates/viola-state/src/board.rs:45:19` | test | crates/viola-state/src/board.rs::tests::board_scan_refuses_a_home_that_cannot_be_statted_and_writes_no_line (cfg(unix)) |
| `crates/viola-state/src/board.rs:67:19` | replace match guard e.kind() == io::ErrorKind::NotFound with true in scan | `crates/viola-state/src/board.rs:67:19` | test | crates/viola-state/src/board.rs::tests::board_scan_of_an_instances_dir_that_cannot_be_listed_is_an_error (cfg(unix)) |
| `crates/viola-state/src/fs.rs:281:18` | replace += with *= in replace_private_with | `crates/viola-state/src/fs.rs:281:18` | restatement | crates/viola-state/src/fs.rs::tests::replace_private_retrying_tries_exactly_the_attempts_while_retry_holds |
| `crates/viola-state/src/fs.rs:285:20` | replace match guard attempts < REPLACE_ATTEMPTS && retry_replace(e.error.raw_os_error(), HOST_IS_WINDOWS) with false in replace_private_with | `crates/viola-state/src/fs.rs:285:20` | restatement | crates/viola-state/src/fs.rs::tests::replace_private_retrying_tries_exactly_the_attempts_while_retry_holds |
| `crates/viola-state/src/fs.rs:285:29` | replace < with <= in replace_private_with | `crates/viola-state/src/fs.rs:285:29` | restatement | crates/viola-state/src/fs.rs::tests::replace_private_retrying_tries_exactly_the_attempts_while_retry_holds |
| `crates/viola-state/src/fs.rs:285:29` | replace < with == in replace_private_with | `crates/viola-state/src/fs.rs:285:29` | restatement | crates/viola-state/src/fs.rs::tests::replace_private_retrying_tries_exactly_the_attempts_while_retry_holds |
| `crates/viola-state/src/fs.rs:285:29` | replace < with > in replace_private_with | `crates/viola-state/src/fs.rs:285:29` | restatement | crates/viola-state/src/fs.rs::tests::replace_private_retrying_tries_exactly_the_attempts_while_retry_holds |
| `crates/viola-state/src/fs.rs:286:21` | replace && with || in replace_private_with | `crates/viola-state/src/fs.rs:286:21` | restatement | crates/viola-state/src/fs.rs::tests::replace_private_retrying_returns_at_once_when_retry_refuses |
| `crates/viola-agent-claude/src/statusline.rs:110:5` | replace shell_argv -> Option<[&str; 3]> with Some([""; 3]) | `crates/viola-agent-claude/src/statusline.rs:110:5` | test | crates/viola-agent-claude/src/statusline.rs::tests::statusline_shell_is_bin_sh_with_the_command_as_one_argument |
| `crates/viola-agent-claude/src/statusline.rs:110:5` | replace shell_argv -> Option<[&str; 3]> with Some(["xyzzy"; 3]) | `crates/viola-agent-claude/src/statusline.rs:110:5` | test | crates/viola-agent-claude/src/statusline.rs::tests::statusline_shell_is_bin_sh_with_the_command_as_one_argument |
| `crates/viola-e2e/src/harness/run/mutants/scratch.rs:48:5` | replace prepare -> Result<Option<(PathBuf, u64)>, String> with Ok(None) | `crates/viola-e2e/src/harness/run/mutants/scratch.rs:48:5` | not-measured-here | test-plan §10 Mutation gate record (not measured here: inside the HOST_SCRATCH cfg!(windows) branch; read caught on windows-2025 in run 38036448183, job mutants (viola-e2e)) |
| `crates/viola-e2e/src/harness/run/mutants/scratch.rs:54:8` | delete ! in prepare | `crates/viola-e2e/src/harness/run/mutants/scratch.rs:54:8` | not-measured-here | test-plan §10 Mutation gate record (not measured here: inside the HOST_SCRATCH cfg!(windows) branch; read caught on windows-2025 in run 38036448183, job mutants (viola-e2e)) |

## The kills, red under the hand-applied mutant and green on the restored file

Each mutant was applied at its site by text (`rustfmt` on the file in the same script), the edit confirmed
landed (the file's bytes changed), the case run through `bash scripts/agent-run.sh run --unit --filter <expr>`,
the file restored byte-exact from a copy taken before the edit, the restore confirmed (bytes equal), and the
case run again. `red` names the panic location and the first message line the case printed.

| coordinate (audit) | mutation | test | red | green | read at |
|---|---|---|---|---|---|
| `src/cmd/hook.rs:403:27` | replace || with && in answer | `src/cmd/hook.rs::tests::answer_reads_no_relative_answers_dir_that_resolves` | red: 1 failed, 0 passed — src/cmd/hook.rs:1114:9 assertion failed: answered(HookEvent::PreToolUse, &captures, &relative, TWO_QUESTIONS).is_empty() | green: 1 passed, 0 failed | 2026-10-13T16:14:17Z |
| `src/cmd/send.rs:119:12` | delete ! in Out<'_>::open | `src/cmd/send.rs::tests::issue_line_shows_only_for_a_human_on_a_terminal (4 cases)` | red: 2 failed, 2 passed — src/cmd/send.rs:292:9 assertion `left == right` failed left: false; src/cmd/send.rs:292:9 assertion `left == right` failed left: true | green: 4 passed, 0 failed | 2026-10-13T16:09:52Z |
| `src/cmd/verify/typed.rs:304:54` | replace && with || in max_gap | `src/cmd/verify/typed.rs::tests::max_gap_leaves_out_a_chunk_after_the_window` | red: 1 failed, 0 passed — src/cmd/verify/typed.rs:847:9 assertion `left == right` failed left: 500 | green: 1 passed, 0 failed | 2026-10-13T16:14:21Z |
| `src/cmd/verify/typed.rs:439:23` | replace > with >= in Feed::take | `src/cmd/verify/typed.rs::tests::feed_reaching_exactly_the_frame_cap_does_not_poison_and_one_byte_past_does` | red: 1 failed, 0 passed — src/cmd/verify/typed.rs:839:9 exactly the cap | green: 1 passed, 0 failed | 2026-10-13T16:14:25Z |
| `src/run/dialog.rs:144:5` | replace millis -> u64 with 1 | `src/run/dialog.rs::tests::millis_is_the_whole_milliseconds_capped_at_u64_max` | red: 1 failed, 0 passed — src/run/dialog.rs:762:9 assertion `left == right` failed left: 1 | green: 1 passed, 0 failed | 2026-10-13T16:14:29Z |
| `src/run/dialog.rs:208:42` | replace != with == in DialogSlot::hook_dialog | `src/run/dialog.rs::tests::continuation_survives_a_pre_tool_use_for_another_tool` | red: 1 failed, 0 passed — src/run/dialog.rs:753:9 assertion `left == right` failed left: Ok(Object {"ok": Object {"dialog_id": Number(3), "response": Null}}) | green: 1 passed, 0 failed | 2026-10-13T16:14:34Z |
| `src/run/send.rs:271:5` | replace close with () | `src/run/send/slot.rs::tests::send_expired_window_closes_the_send_so_a_late_line_is_not_claimed` | red: 1 failed, 0 passed — src/run/send/slot.rs:275:9 assertion `left == right` failed left: String("driver") | green: 1 passed, 0 failed | 2026-10-13T16:14:38Z |
| `src/run/wheel.rs:297:18` | replace && with || in Recorder::record | `src/run/wheel.rs::tests::wheel_pause_whose_record_cannot_be_appended_is_internal` | red: 1 failed, 0 passed — src/run/wheel.rs:941:9 assertion `left == right` failed left: Ok(Object {"ok": Object {"wheel": String("human")}}) | green: 1 passed, 0 failed | 2026-10-13T16:14:42Z |
| `src/run/wheel.rs:393:9` | replace Classifier::abort -> bool with true | `src/run/wheel.rs::tests::classifier_split_reads_carry_the_unfinished_sequence::case_2_csi_cut_by_an_escape_then_a_focus_report` | red: 1 failed, 0 passed — src/run/wheel.rs:655:9 assertion `left == right` failed: second: [73] left: true | green: 1 passed, 0 failed | 2026-10-13T16:10:22Z |
| `src/run/wheel.rs:427:17` | delete match arm ESC in Classifier::step | `src/run/wheel.rs::tests::classifier_split_reads_carry_the_unfinished_sequence::case_1_esc_key_then_a_focus_report` | red: 1 failed, 0 passed — src/run/wheel.rs:655:9 assertion `left == right` failed: second: [73] left: true | green: 1 passed, 0 failed | 2026-10-13T16:10:26Z |
| `src/run/wheel.rs:434:32` | replace match guard self.params.len() < BOUND with true in Classifier::step | `src/run/wheel.rs::tests::classifier_typing_is_editing::sgr_mouse_one_past_the_bound (pair: classifier_terminal_reply_is_not_editing::sgr_mouse_at_the_bound)` | red: 1 failed, 1 passed — src/run/wheel.rs:625:9 [27, 91, 60, 48, 59, 49, 48, 59, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 77] | green: 2 passed, 0 failed | 2026-10-13T16:10:30Z |
| `src/run/wheel.rs:434:50` | replace < with <= in Classifier::step | `src/run/wheel.rs::tests::classifier_typing_is_editing::sgr_mouse_one_past_the_bound (pair: classifier_terminal_reply_is_not_editing::sgr_mouse_at_the_bound)` | red: 1 failed, 1 passed — src/run/wheel.rs:625:9 [27, 91, 60, 48, 59, 49, 48, 59, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 77] | green: 2 passed, 0 failed | 2026-10-13T16:10:35Z |
| `src/run/wheel.rs:458:24` | replace match guard osc with true in Classifier::step | `src/run/wheel.rs::tests::classifier_typing_is_editing::dcs_ended_by_bel` | red: 1 failed, 0 passed — src/run/wheel.rs:625:9 [27, 80, 49, 36, 114, 48, 109, 7] | green: 1 passed, 0 failed | 2026-10-13T16:10:39Z |
| `src/run/wheel.rs:467:35` | replace < with <= in Classifier::step | `src/run/wheel.rs::tests::classifier_typing_is_editing::osc_one_past_the_bound (pair: classifier_terminal_reply_is_not_editing::osc_at_the_bound)` | red: 1 failed, 1 passed — src/run/wheel.rs:625:9 [27, 93, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 7] | green: 2 passed, 0 failed | 2026-10-13T16:10:43Z |
| `src/run/wheel.rs:516:49` | replace == with != in is_reply | `src/run/wheel.rs::tests::classifier_terminal_reply_is_not_editing::da1_with_no_field, ::da2_with_no_field` | red: 2 failed, 0 passed — src/run/wheel.rs:642:9 [27, 91, 63, 99]; src/run/wheel.rs:642:9 [27, 91, 62, 99] | green: 2 passed, 0 failed | 2026-10-13T16:10:47Z |
| `src/run/wheel.rs:518:47` | replace && with || in is_reply | `src/run/wheel.rs::tests::classifier_typing_is_editing::kitty_flags_with_two_fields` | red: 1 failed, 0 passed — src/run/wheel.rs:625:9 [27, 91, 63, 49, 59, 50, 117] | green: 1 passed, 0 failed | 2026-10-13T16:09:31Z |
| `crates/viola-pty/src/lib.rs:202:9` | replace <impl Pty for PortablePty>::resize -> Result<(), PtyError> with Ok(()) | `crates/viola-pty/src/lib.rs::tests::spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code, ::spawn_reports_a_resize_to_a_child_that_reads_no_key, ::spawn_delivers_a_key_written_right_after_a_resize_to_a_child_not_yet_reading (report files per process)` | red: 3 failed, 0 passed — crates/viola-pty/src/lib.rs:960:17 child report never showed "byte 79 size=120x40"; crates/viola-pty/src/lib.rs:960:17 child report never showed "size 120x40"; crates/viola-pty/src/lib.rs:960:17 child report never showed "size 120x40" | green: 3 passed, 0 failed | 2026-10-13T16:11:10Z |
| `crates/viola-state/src/board.rs:164:29` | replace += with *= in SkippedTotal::add | `crates/viola-state/src/board.rs::tests::board_read_sums_the_skip_counts_over_every_instance` | red: 1 failed, 0 passed — crates/viola-state/src/board.rs:656:9 assertion `left == right` failed left: SkippedTotal { unknown_kinds: 1, unknown_fields: 0, torn_lines: 2 } | green: 1 passed, 0 failed | 2026-10-13T16:14:56Z |
| `crates/viola-state/src/board.rs:164:29` | replace += with -= in SkippedTotal::add | `crates/viola-state/src/board.rs::tests::board_read_sums_the_skip_counts_over_every_instance` | red: 1 failed, 0 passed — crates/viola-state/src/board.rs:164:9 attempt to subtract with overflow | green: 1 passed, 0 failed | 2026-10-13T16:15:10Z |
| `crates/viola-state/src/board.rs:45:19` | replace match guard e.kind() == io::ErrorKind::NotFound with true in check_if_present | `crates/viola-state/src/board.rs::tests::board_scan_refuses_a_home_that_cannot_be_statted_and_writes_no_line (cfg(unix))` | red: 1 failed, 0 passed — crates/viola-state/src/board.rs:479:9 Err(Io(Os { code: 13, kind: PermissionDenied, message: "Permission denied" })) | green: 1 passed, 0 failed | 2026-10-13T16:18:03Z |
| `crates/viola-state/src/board.rs:67:19` | replace match guard e.kind() == io::ErrorKind::NotFound with true in scan | `crates/viola-state/src/board.rs::tests::board_scan_of_an_instances_dir_that_cannot_be_listed_is_an_error (cfg(unix))` | red: 1 failed, 0 passed — crates/viola-state/src/board.rs:495:9 Ok(Ok(0)) | green: 1 passed, 0 failed | 2026-10-13T16:15:38Z |
| `crates/viola-state/src/fs.rs:281:18` | replace += with *= in replace_private_with | `crates/viola-state/src/fs.rs::tests::replace_private_retrying_tries_exactly_the_attempts_while_retry_holds` | red: 1 failed, 1 passed — crates/viola-state/src/fs.rs:452:17 paused past the last attempt | green: 2 passed, 0 failed | 2026-10-13T16:15:52Z |
| `crates/viola-state/src/fs.rs:285:20` | replace match guard attempts < REPLACE_ATTEMPTS && retry_replace(e.error.raw_os_error(), HOST_IS_WINDOWS) with false in replace_private_with | `crates/viola-state/src/fs.rs::tests::replace_private_retrying_tries_exactly_the_attempts_while_retry_holds` | red: 1 failed, 1 passed — crates/viola-state/src/fs.rs:457:9 assertion `left == right` failed: one pause between two attempts left: 1 | green: 2 passed, 0 failed | 2026-10-13T16:16:07Z |
| `crates/viola-state/src/fs.rs:285:29` | replace < with <= in replace_private_with | `crates/viola-state/src/fs.rs::tests::replace_private_retrying_tries_exactly_the_attempts_while_retry_holds` | red: 1 failed, 1 passed — crates/viola-state/src/fs.rs:452:17 paused past the last attempt | green: 2 passed, 0 failed | 2026-10-13T16:16:21Z |
| `crates/viola-state/src/fs.rs:285:29` | replace < with == in replace_private_with | `crates/viola-state/src/fs.rs::tests::replace_private_retrying_tries_exactly_the_attempts_while_retry_holds` | red: 1 failed, 1 passed — crates/viola-state/src/fs.rs:457:9 assertion `left == right` failed: one pause between two attempts left: 1 | green: 2 passed, 0 failed | 2026-10-13T16:16:35Z |
| `crates/viola-state/src/fs.rs:285:29` | replace < with > in replace_private_with | `crates/viola-state/src/fs.rs::tests::replace_private_retrying_tries_exactly_the_attempts_while_retry_holds` | red: 1 failed, 1 passed — crates/viola-state/src/fs.rs:457:9 assertion `left == right` failed: one pause between two attempts left: 1 | green: 2 passed, 0 failed | 2026-10-13T16:16:49Z |
| `crates/viola-state/src/fs.rs:286:21` | replace && with || in replace_private_with | `crates/viola-state/src/fs.rs::tests::replace_private_retrying_returns_at_once_when_retry_refuses` | red: 2 failed, 0 passed — crates/viola-state/src/fs.rs:452:17 paused past the last attempt; crates/viola-state/src/fs.rs:473:9 assertion `left == right` failed: a refused retry never pauses left: 99 | green: 2 passed, 0 failed | 2026-10-13T16:17:03Z |
| `crates/viola-agent-claude/src/statusline.rs:110:5` | replace shell_argv -> Option<[&str; 3]> with Some([""; 3]) | `crates/viola-agent-claude/src/statusline.rs::tests::statusline_shell_is_bin_sh_with_the_command_as_one_argument` | red: 1 failed, 0 passed — crates/viola-agent-claude/src/statusline.rs:390:9 assertion `left == right` failed left: Some(["", "", ""]) | green: 1 passed, 0 failed | 2026-10-13T16:17:15Z |
| `crates/viola-agent-claude/src/statusline.rs:110:5` | replace shell_argv -> Option<[&str; 3]> with Some(["xyzzy"; 3]) | `crates/viola-agent-claude/src/statusline.rs::tests::statusline_shell_is_bin_sh_with_the_command_as_one_argument` | red: 1 failed, 0 passed — crates/viola-agent-claude/src/statusline.rs:390:9 assertion `left == right` failed left: Some(["xyzzy", "xyzzy", "xyzzy"]) | green: 1 passed, 0 failed | 2026-10-13T16:17:26Z |

## Step 2 — `viola-pty` `resize`, measured before the disposal

- **2a, the mutant alone** (2026-10-13 16:02:57Z–16:03:13Z, one harness call, no other test process of this tree
  running; `report_paths` still named after the test alone): `resize → Ok(())` hand-applied at
  `crates/viola-pty/src/lib.rs:202`, the three resize tests filtered alone. All three read **red**, each at
  `lib.rs:958:17` (`wait_line`'s deadline, `CHILD_WITHIN` 7 s):
  - `spawn_delivers_a_key_written_right_after_a_resize_to_a_child_not_yet_reading` — FAIL 7.006 s, `child report never
    showed "byte 79 size=120x40"`; child report `start … size=100x30` · `byte 78` · `byte 79 size=100x30` ·
    `restored=true`; test report `resize-returned` · `key-written` · `key-flushed` · `dsr-cpr 0`.
  - `spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code` — FAIL 7.056 s, `child report never
    showed "size 120x40"`; child report `start … size=100x30` · `byte 78`; test report `resize-returned` · `dsr-cpr 0`.
  - `spawn_reports_a_resize_to_a_child_that_reads_no_key` — FAIL 7.056 s, `child report never showed "size 120x40"`;
    child report `start … size=100x30`; test report `resize-returned` · `dsr-cpr 0`.
- **Branch b.** Under the mutant, run alone, no child ever saw 120×40. The audit's run saw them pass in 5.35–6.11 s,
  so its miss came from outside the test. The **attribution is a HYPOTHESIS**: the audit's two cargo-mutants jobs
  shared one `TMPDIR` (`mut-driver.py`, `JOBS = 2`, one `TMPDIR`), and the report file was named after the test alone,
  so the other job's unmutated run of the same test could write the awaited `size 120x40` line. That two-job run is
  not re-run here (`inputs#I1`); the Epoch 5 boundary audit's own run reads the outcome.
- **The disposal:** `report_paths` names both files `<test>.<pid>.report` / `<test>.<pid>.test.report` (the `Drop` and
  failure-dump behaviour unchanged), so no other process's run can write the file a test waits on. The three tests
  then read red under the mutant and green restored (the m32 row above).

## Notes on two readings

- **`board.rs:164:29` `+=`→`-=`** reads red as `attempt to subtract with overflow` at the mutated statement itself, not
  at the case's assertion: the sum starts at 0, so subtracting the first nonzero count underflows `u64` in the test
  profile, whatever the case. No case can reach an assertion under this mutant; the `*=` twin reads red at the
  assertion (`unknown_fields: 0` against 2).
- **The first firing of 19 specs selected no test.** Their filter was `test(=tests::name)`, which names only a
  crate-root module's tests (`viola-pty`'s `lib.rs`); for `cmd::hook::tests::…`, `board::tests::…` and the like nextest
  read `no tests to run` (`nextest-exit-4`) for both the red and the green run. Those readings were discarded, and
  all 19 were re-run with `test(/(^|::)tests::name$/)`; the table holds the re-run's readings only. `m35`'s case was
  then changed from `.expect("read")` to an assertion on the whole value, so its red is the assertion's, and re-read.
