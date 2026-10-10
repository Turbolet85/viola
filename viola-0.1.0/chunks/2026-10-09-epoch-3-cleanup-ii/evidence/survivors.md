# The eighteen Linux survivors, read against three witness runs — 2026-10-09-epoch-3-cleanup-ii

Source of the eighteen: the Epoch 3 boundary audit's Linux survivor table
(`.andromeda/runs/2026-10-08T10-08-51-code-audit/`, `c-mutation-viola-state.json`,
`c-mutation-viola-agent-claude.json`, `c-mutation-viola.json`, each unit's `survivors`), the rows of research.md's
table. Fourteen are now caught by a test in the package that owns the mutated file; four are gone from the generated
set because their expression was restated. None is left missed and none is called equivalent.

The machine-readable record is `survivors.ndjson` (eighteen lines; built by `survivors-build.py` from the three
outcome files beside it, never typed by hand). Its `witness` field is the outcome line as cargo-mutants printed it,
or, for a restated row, the lines the function now holds.

## The three witness runs

One-shot measurements through the harness's package arm (plan step 14), one at a time, each with
`TMPDIR="$(dirname "$PWD")/viola-mutants-scratch"`, on the tree that holds every edit of this chunk (HEAD `14f1fb5`
plus the chunk's uncommitted edits; `run --unit` and `run` read green on it first: unit 1484 of 1484, integration
354 of 354). No file was edited while a run was in flight. Host `x86_64-unknown-linux-gnu`, cargo-mutants 27.1.0,
jobs 1. Times are UTC.

| Run | Command (after `bash scripts/agent-run.sh run --mutants`) | Start | End | Wall | Tested | Caught | Missed | Timeout | Unviable | Harness |
|---|---|---|---|---|---|---|---|---|---|---|
| viola-state | `--package viola-state --file crates/viola-state/src/events.rs --file …/stamps.rs --file …/strict.rs --file …/fs.rs` | 2026-10-09T23:34:54Z | 23:49:58Z | 904 s | 203 | 129 | 65 | 0 | 9 | exit 1, `"ok":false` |
| viola-agent-claude | `--package viola-agent-claude --file crates/viola-agent-claude/src/ledger.rs` | 2026-10-09T23:50:29Z | 23:59:17Z | 528 s | 303 | 283 | 0 | 0 | 20 | exit 0, `"ok":true` |
| viola (root) | `--package viola --file src/bin/viola-fake-agent.rs --file src/cmd/mod.rs --file src/cmd/hook.rs --file src/cmd/verify.rs` | 2026-10-09T23:59:52Z | 2026-10-10T01:06:06Z | 3974 s | 263 | 238 | 0 | 0 | 25 | exit 0, `"ok":true` |

Each run reads `timeout == 0` and `unviable <= caught`. Their documents and outcome lines: `witness-viola-state.json`
and `-outcomes.txt` (203 lines), `witness-viola-agent-claude.json` and `-outcomes.txt` (303), `witness-viola.json`
and `-outcomes.txt` (263).

The host over the three windows (`hostwatch.py read --from … --to … --for viola`): `QUIET for viola`, 0 s stalled
on IO, in both readings (23:34:54Z to 23:59:17Z: io some peak 6 %, load peak 14.1, mean 8.7; 23:59:52Z to
01:06:06Z: io some peak 15 %, load peak 20.6, mean 10.1). The test homes' backing read `tmpfs` behind the
`target/e2e-home` link after each run.

## The eighteen rows

`n` is the row of research.md's table. "Site" is the audit's coordinate; "now" is the coordinate in the witness
where the line moved. "Obs" says whether the mutated expression is obs code.

| n | Site (audit) | Mutation (audit) | Disposition | Outcome | Now | Test | Obs |
|---|---|---|---|---|---|---|---|
| 1 | `crates/viola-state/src/events.rs:96:19` | `NotFound` guard → `true` in `current_len` | test | caught | same | `events::tests::events_current_len_of_a_log_that_cannot_be_statted_is_an_error` | no |
| 2 | `crates/viola-state/src/events.rs:149:19` | `NotFound` guard → `true` in `read_within` | test | caught | same | `events::tests::events_read_of_a_log_that_cannot_be_opened_is_an_error` | no |
| 3 | `crates/viola-state/src/events.rs:209:24` | `<` → `>` in `LoggedLines::next_line` | restated | gone | — | — | no |
| 4 | `crates/viola-state/src/fs.rs:290:19` | content guard → `true` in `replace_private_shared` | test | caught | same | `fs::tests::replace_private_shared_in_a_read_only_dir_is_done_only_over_the_same_bytes` (Unix only) | no |
| 5 | `crates/viola-state/src/stamps.rs:28:19` | `NotFound` guard → `true` in `read_capped` | test | caught | same | `stamps::tests::read_stamps_of_a_file_that_cannot_be_opened_is_an_error` | no |
| 6 | `crates/viola-state/src/strict.rs:34:23` | `NotFound` guard → `true` in `check_stamps` | test | caught | same | `strict::tests::check_stamps_of_a_path_that_cannot_be_statted_is_unreadable` | no |
| 7 | `crates/viola-agent-claude/src/ledger.rs:647:81` | `==` → `!=` in `both_parallel_answered` | test | caught | same | `ledger::tests::check_dialog_concurrency_with_a_post_for_a_third_id_is_not_answered` | no |
| 8 | `crates/viola-agent-claude/src/ledger.rs:670:33` | `<` → `<=` in `parallel_both_before_first_post` | restated | gone | — | — | no |
| 9 | `crates/viola-agent-claude/src/ledger.rs:685:51` | `+` → `*` in `clear_start` | restated | gone | — | — | no |
| 10 | `crates/viola-agent-claude/src/ledger.rs:900:76` | `&&` → `\|\|` in `dialog_variants` | test | caught | `898:68`, in `joined_call` | `ledger::tests::dialog_variants_with_a_post_of_an_unknown_id_beside_an_identified_call_records_no_post` | no |
| 11 | `crates/viola-agent-claude/src/ledger.rs:1172:5` | `is_local_char` → `true` | test | caught | `1187:5` | `ledger::tests::has_email_reads_local_at_domain_dot_tld::case_09_space_before_the_at` | no |
| 12 | `crates/viola-agent-claude/src/ledger.rs:1181:74` | `==` → `!=` in `has_email` | test | caught | `1196:74` | `ledger::tests::has_email_reads_local_at_domain_dot_tld::case_10_hyphen_in_the_domain` | no |
| 13 | `src/bin/viola-fake-agent.rs:328:9` | `close_hooks` → `()` | test | caught | same | `tests::run_hook_after_close_hooks_runs_nothing` (the bin's own module) | no |
| 14 | `src/bin/viola-fake-agent.rs:437:74` | `==` → `!=` in `run_hook` | test | caught | same | `tests::run_hook_of_a_stop_with_the_receipt_hold_takes_at_least_the_hold` | no |
| 15 | `src/bin/viola-fake-agent.rs:473:46` | `&&` → `\|\|` in `submit` | restated | gone | — | — | no |
| 16 | `src/bin/viola-fake-agent.rs:473:57` | `==` → `!=` in `submit` | test | caught | `495:28`, in `Agent::end_turn` | `tests::submit_of_the_long_paste_with_a_paste_hint_takes_at_least_the_hint` | no |
| 17 | `src/cmd/mod.rs:133:5` | `cli_sink` → `None` | test | caught | same | `wait_in_a_home_whose_diagnostics_is_a_file_keeps_the_chain_in_the_detail_file` (`tests/cli_wait_last.rs`, cross-process) | **yes** |
| 18 | `src/cmd/hook.rs:214:72` | `>` → `>=` in `handle_dialog` | test | caught | same | `cmd::hook::tests::handle_dialog_with_a_payload_at_the_frame_bound_prints_the_decision_body` | no |

Row 17 is the one obs expression of the eighteen (the `cli` verbs' detail sink): it is caught by a test, not by
restatement or argument. The other seventeen are not obs code: state readers and one state writer (1 to 6), the
verify ledger's checks (7 to 12), the test-side fake agent (13 to 16) and the dialog hook's frame bound (18).

## The fourteen kill rows, quoted

Each line is the witness's own.

```
caught   crates/viola-state/src/events.rs:96:19: replace match guard e.kind() == std::io::ErrorKind::NotFound with true in current_len in 0s build + 3s test
caught   crates/viola-state/src/events.rs:149:19: replace match guard e.kind() == ErrorKind::NotFound with true in read_within in 0s build + 3s test
caught   crates/viola-state/src/fs.rs:290:19: replace match guard fs::read(path).ok().as_deref() != Some(bytes) with true in replace_private_shared in 0s build + 3s test
caught   crates/viola-state/src/stamps.rs:28:19: replace match guard e.kind() == io::ErrorKind::NotFound with true in read_capped in 0s build + 3s test
caught   crates/viola-state/src/strict.rs:34:23: replace match guard e.kind() == io::ErrorKind::NotFound with true in check_stamps in 0s build + 3s test
caught   crates/viola-agent-claude/src/ledger.rs:647:81: replace == with != in both_parallel_answered in 0s build + 0s test
caught   crates/viola-agent-claude/src/ledger.rs:898:68: replace && with || in joined_call in 0s build + 0s test
caught   crates/viola-agent-claude/src/ledger.rs:1187:5: replace is_local_char -> bool with true in 1s build + 0s test
caught   crates/viola-agent-claude/src/ledger.rs:1196:74: replace == with != in has_email in 0s build + 0s test
caught   src/bin/viola-fake-agent.rs:328:9: replace Agent::close_hooks with () in 0s build + 38s test
caught   src/bin/viola-fake-agent.rs:437:74: replace == with != in Agent::run_hook in 0s build + 38s test
caught   src/bin/viola-fake-agent.rs:495:28: replace == with != in Agent::end_turn in 0s build + 49s test
caught   src/cmd/mod.rs:133:5: replace cli_sink -> Option<DetailSink> with None in 2s build + 39s test
caught   src/cmd/hook.rs:214:72: replace > with >= in handle_dialog in 1s build + 38s test
```

**Which test failed under each mutant.** The witness is each case's remove-the-guard reading: the mutant is the
guard neutralised, and `caught` is a test red.

- Rows 7, 10, 11 and 12 (read from the viola-agent-claude run's per-mutant logs): under each mutant exactly one
  test failed, the new case named in the table.
- Rows 13, 14, 16, 17 and 18 (read from the root run's per-mutant logs): under each mutant the new case named in
  the table failed. Under row 16's mutant one more test ended, by the profile's kill:
  `cli_verify verify_window_paste_hint_past_the_gate_maximum_still_stamps` (the mutated agent holds after every
  framed turn but the long one).
- Rows 1, 2, 4, 5 and 6: **not read from a log.** The viola-state run's per-mutant logs were replaced by the next
  run before they were read (the tool keeps one earlier run). The case in the table is named by construction: it
  calls the mutated function with the input the mutant answers differently, and it is one of the only five tests
  added to that package since the audit read these five mutants missed. The `caught` lines above are measured.

**The three rows inside split functions.** Row 10's expression moved from `dialog_variants` into the helper
`joined_call` (`.or_else(|| calls.iter().rposition(|(t, i)| *t == tool && i.is_none()))`, now `898:68`). Rows 15
and 16's expression moved from `submit` into `Agent::end_turn` (the long-paste line, now `495`). The case for row
10 drives `dialog_variants`; the case for row 16 drives `submit`. Neither aims at the helper.

## The four restated rows, shown gone

For each: the audit's mutation text is absent at that expression, and every mutant the run generated for the
restated expression is quoted with its outcome.

- **Row 3, `LoggedLines::next_line`.** The guard is now `if len != self.cap`, resting on the read one line above
  taking at most `cap` bytes. The run holds 13 lines of the function, all `caught`, and none reads `replace < with
  >`. The restated comparison's one mutant:
  `caught   crates/viola-state/src/events.rs:211:24: replace != with == in LoggedLines::next_line in 0s build + 3s test`.
  `events_read_skips_and_counts_a_line_past_the_cap` is in that package's suite, which failed under it; which
  test failed was not read from a log (see rows 1 to 6).
- **Row 8, `parallel_both_before_first_post`.** It now asks whether any PostToolUse stands before the second
  PreToolUse, with no ordering operator between two indexes, resting on a capture being never both. The run holds
  two lines of the function and no `replace < with <=`:
  `caught   crates/viola-agent-claude/src/ledger.rs:663:19: replace != with == in parallel_both_before_first_post in 1s build + 0s test`
  and
  `caught   crates/viola-agent-claude/src/ledger.rs:670:9: delete ! in parallel_both_before_first_post in 0s build + 0s test`.
- **Row 9, `clear_start`.** The `+ 1` is dropped, resting on the capture at the paste's own index being a
  UserPromptSubmit, never the SessionStart searched for. At the base the function held two `+` sites, the `+ 1`
  (`685:51`, the survivor) and the map's `after + at`; the run holds one, the map's `from + at`:
  `caught   crates/viola-agent-claude/src/ledger.rs:694:24: replace + with - in clear_start in 0s build + 0s test`
  and
  `caught   crates/viola-agent-claude/src/ledger.rs:694:24: replace + with * in clear_start in 0s build + 0s test`.
  The function's other three lines (`693:58`, `693:31`, `693:79`) read `caught` too. So a line with the audit's
  mutation text does stand in the function, on the other expression, and it is caught; no mutant of a `+ 1` is
  generated.
- **Row 15, the long-paste test in the fake agent.** `framing.is_some() &&` is dropped from the long-paste line,
  resting on `framing_stem(PROBE_LONG_PASTE)` being a compiled text's stem and always `Some`. `Agent::end_turn`
  holds three operator lines and one whole-function line, all `caught`:
  `caught   src/bin/viola-fake-agent.rs:495:28: replace == with != in Agent::end_turn in 0s build + 49s test`,
  `caught   src/bin/viola-fake-agent.rs:500:37: replace && with || in Agent::end_turn in 0s build + 4s test`,
  `caught   src/bin/viola-fake-agent.rs:500:48: replace == with != in Agent::end_turn in 0s build + 4s test`,
  `caught   src/bin/viola-fake-agent.rs:492:9: replace Agent::end_turn with () in 0s build + 10s test`.
  The one `&& → ||` left is line 500's, the tag-like paste's test, which this chunk did not restate.
  `Agent::submit` holds no `&& → ||` on a framing test any more (`458:48 replace && with ||` is the
  local-command branch, caught).

## Every missed or timed-out line of a witness, accounted for

- **viola-agent-claude:** none. **viola (root):** none.
- **viola-state: 65 missed, 0 timed out.** Each was matched by file and mutation text against the audit's record:
  - 59 are in `c-mutation-viola-state.json` `not_measured`, every one marked `cfg(windows)`: the 20 in `fs.rs`
    (`win::wide`, `win::protected_sddl`, `win::profile_dir`, `win::protect_outside_profile`, `win::protect`,
    `win::dacl_of`) and the 39 in `strict.rs` (`win::wide`, `win::check`, `win::persistent_acls`,
    `win::owner_and_dacl`, `win::aces`, `win::sid_string`, `win::user_sid`). Not measured here; owed to "Windows
    mutation grade".
  - 6 are the `replace_private_with` rows owed to "Windows mutation grade": `fs.rs:270:18` (`+=` → `*=`),
    `274:20` (the match guard → `false`), `275:21` (`&&` → `||`), `274:29` (`<` → `==`, `>`, `<=`).
  - **0 are not accounted for.** No other line of that run reads missed.
- The audit's record lists 77 not measured for the whole unit; the other 18 are in `pin.rs`, which this run did
  not mutate.

So the restated and split files show no new missed mutant: every line of `ledger.rs`, `viola-fake-agent.rs`,
`src/cmd/mod.rs`, `src/cmd/hook.rs` and `src/cmd/verify.rs` reads caught or unviable, the five new functions of
the splits included (`joined_call`, `Agent::end_turn`, `named_payloads`, `scrubbed_payloads`,
`signature_screens`, `write_recording`).

## Observed in the root run, not this chunk's

cargo-mutants' record holds no `timeout` outcome. Inside the per-mutant logs, nextest printed a `TIMEOUT` line for
133 of the 263 mutants (a mutated fake agent or probe leaves waits to run to their kill line), and 234 logs hold a
`FAIL` line. **Four mutants are graded `caught` by the `mutants` profile's 10 s kill alone, with no failing
assertion:**

- `src/bin/viola-fake-agent.rs:378:16: delete ! in Agent::replay_dialogs`
- `src/bin/viola-fake-agent.rs:378:38: delete ! in Agent::replay_dialogs`
- `src/bin/viola-fake-agent.rs:97:17: delete match arm "--screens" in Opts::parse`
- `src/cmd/verify.rs:284:9: replace ProbeDir::typed -> PathBuf with Default::default()`

None is on a line this chunk edited, and none is one of the eighteen. Under each, the stamped-home fixture's
`viola verify` does not finish, so every test that takes a stamped home ends at the kill. They were read against
the backing (`tmpfs`) and the host record (QUIET, above) before anything else, and were not re-run. Whether the
audit's own run graded them the same way was not read. They are named here for the wrap to place.

## Not measured

- Any of this on Windows or macOS. The NUL-byte path (rows 1, 2, 5, 6) fails before any filesystem call on Linux
  (`InvalidInput`); the CI run on `windows-2025` and `macos-latest` is the first reading of those four cases there.
- Row 4's case on Windows: it is `cfg(unix)`. Its Windows grade is owed to "Windows mutation grade", as is
  `strict.rs:34:23`'s.
- Row 4's case under an effective uid of 0, where the replace succeeds and the case passes without killing the
  mutant: not run.
