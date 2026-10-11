# Red and green readings — 2026-10-10-session-links (plan step 10)

Implement's one-shot runs on the Linux dev host, each through the harness (`bash scripts/agent-run.sh run …`,
one JSON document per run). All were taken before 2026-10-11T00:16:16Z (the clock read after the last one). No
run here is a gate entry. Part A only: steps 12 to 15 are held by step 11 and nothing of them was built or read.

## 1. Red before the product, green after (steps 2, 4, 6, 7, 8)

**The stub tree.** Steps 1, 3 and 5 as built, and two bodies that held their names and no behaviour:
`viola_state::replay::apply_link_line` returned `false` and changed nothing; `LinkSet::change`
(`src/run/links.rs`) returned `Ok(())` with no line appended, no set change and no snapshot write. The calls in
`send` and `answer` were in place.

| run | filter | on the stub tree | on the built tree |
|---|---|---|---|
| unit | `test(/link_rule_\|links_set_\|send_link_\|answer_link_/)` | exit 1, 34 run, 13 passed, 21 failed | exit 0, 34 run, 34 passed (`target/run-archive/991`) |
| integration | `binary(cli_links) \| binary(cli_budget) \| binary(state_replay) \| binary(state_events)` | exit 1, 28 run, 22 passed, 6 failed (`target/run-archive/990`) | exit 0, 28 run, 28 passed (`target/run-archive/992`) |

**Red on the stub tree, green on the built tree (27 cases):**
- step 2, `link_rule_*` (5): `a_link_adds_its_pair_with_the_lines_ts`, `a_link_after_an_unlink_takes_the_later_ts`,
  `two_drivers_of_one_driven_are_two_pairs`, `a_link_line_without_a_ts_has_since_unknown`,
  `replay_holds_the_set_its_one_pass_derives`;
- step 3, `binary(state_replay)` (2): `state_replay_links_hold_the_first_since_until_an_unlink_and_a_later_one_after_it`,
  `state_replay_a_snapshot_cut_short_is_replayed_with_the_logged_link`;
- step 4, `links_set_*` in `src/run/links.rs` (9): `restore_over_a_log_with_link_unlink_link_holds_the_last_since`,
  `restore_over_an_absent_log_is_empty`, `record_twice_writes_one_line_and_returns_the_same_since`,
  `two_drivers_are_two_pairs_in_the_order_they_linked`, `a_logged_pair_of_another_driven_is_not_this_instances`,
  `remove_of_an_absent_pair_writes_nothing`, `remove_appends_one_unlink_and_a_later_record_takes_a_later_since`,
  `an_append_that_fails_leaves_the_set_without_the_pair`,
  `a_snapshot_write_that_fails_leaves_the_set_with_the_logged_pair`;
- step 5, `links_set_*` in `src/cmd/run.rs` (1): `links_set_at_a_start_over_a_log_with_a_link_is_in_the_first_snapshot`;
- step 6, `send_link_*` (3): `an_issued_send_from_another_instance_appends_link_before_send_issued`,
  `a_second_issued_send_adds_no_link`, `a_link_that_cannot_be_recorded_is_internal_and_issues_nothing`;
- step 7, `answer_link_*` (3): `a_taken_answer_from_another_instance_appends_one_link`,
  `a_second_taken_answer_adds_no_second_line`,
  `a_link_that_cannot_be_recorded_is_internal_and_the_answer_is_not_taken`;
- step 8, `binary(cli_links)` (4): `links_a_send_from_another_instance_links_once`,
  `links_an_answer_from_another_instance_links`, `links_survive_a_wrapper_restart`,
  `e5_the_replay_reads_the_link_the_wrapper_wrote` (the first three at the helper's "exactly one `link` line"
  assertion, `tests/cli_links.rs:48`; the fourth at its replayed length).

**Green on the stub tree too (14 of the new cases): each asserts an absence or an unchanged set, so a tree
without the behaviour cannot read it red.** Their forced-on readings are sections 2 and 3:
- `link_rule_*` (4): `a_repeated_link_keeps_the_first_since`, `an_unlink_of_an_absent_pair_changes_nothing`,
  `a_line_with_a_name_that_is_no_string_changes_nothing`, `a_line_of_another_kind_changes_nothing`. These four
  have no forced-on reading; the function they read is the one the five red cases above read red;
- `links_set_restore_of_a_log_that_cannot_be_read_is_an_error` (1): the read fails on either tree;
- `send_link_*` (5 with their rstest cases): `a_refused_send_adds_none` (2), `a_send_under_the_wrappers_own_name_adds_none`,
  `a_send_with_no_from_adds_none` (2);
- `answer_link_*` (3 with their rstest cases): `a_refused_answer_appends_none` (2),
  `a_wrong_kind_no_from_and_the_wrappers_own_name_append_none`;
- `links_a_refused_send_and_an_own_name_send_record_none` (`binary(cli_links)`).
- `path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone` (step 9) was green on its first run, on the stub
  tree: its property does not rest on the link code.

The other 22 cases of the integration filter are the files' standing cases; all were green on both trees.

## 2. Step 9, forced on: a gate reading added to `DialogSlot::answer`

**The scratch edit** (`src/run/dialog.rs`, in `answer`, directly after `parse_from`): an answer was refused
`budget-paused` / `five-hour` whenever the instance's own snapshot read `budget_paused: true`
(`viola_state::snapshot::read_snapshot(&self.instance_dir).is_some_and(|s| s.budget_paused)`). The plan words
"a gate evaluation"; the slot holds no gate, so the edit read the gate's recorded verdict from the snapshot the
gate writes. In the same build the link record was also moved to the call's arrival in `send` and `answer`
(section 3).

**Reading:** `run --integration --filter 'binary(cli_links) | test(/path6_a_closed_gate_leaves/)'`, exit 1,
6 run, 3 passed, 3 failed. `path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone` failed at
`tests/cli_budget.rs:521`, the exit code of its step 5 (`viola answer builder <id> --json`): left `Some(11)`,
right `Some(0)`. Its steps 1 to 4 passed in that run: the control read exit 11 and the gate records, and `wait`
and `last` read as under the open gate.

**Reverted and read back:** both files were restored from copies taken before the edit, and their SHA-256 read
equal to the readings taken before it (`src/run/send.rs` `70d44733…`, `src/run/dialog.rs` `a3f23d65…`); no line
of either holds the scratch marker. The case then read green in the gate block's integration entry.

## 3. The two guards of the implicit link, removed

`.claude/rules/testing.md` (2026-09-25, the remove-the-guard run; its 2026-10-10 extension on absence cases).

**Guard 1: a refused call links nothing** (the record sits after the last refusal). Forced on: the record moved
to the call's arrival, directly after the params are parsed, in `send` and in `answer`.
- unit, `test(/send_link_|answer_link_/)`: exit 1, 14 run, 9 passed, 5 failed:
  `send_link_a_refused_send_adds_none` (both cases, `src/run/send.rs:2381`),
  `answer_link_a_refused_answer_appends_none` (both cases), 
  `answer_link_a_wrong_kind_no_from_and_the_wrappers_own_name_append_none`;
- root: `links_an_answer_from_another_instance_links` failed at "a refused answer links"
  (`tests/cli_links.rs:169`), `links_a_refused_send_and_an_own_name_send_record_none` at "a refused send links"
  (`tests/cli_links.rs:199`).

**Guard 2: the wrapper's own name links nothing** (`from != name`). Removed in `send` and in `answer`, the
record back in its place.
- unit, same filter: exit 1, 14 run, 12 passed, 2 failed:
  `send_link_a_send_under_the_wrappers_own_name_adds_none`,
  `answer_link_a_wrong_kind_no_from_and_the_wrappers_own_name_append_none`;
- root, `binary(cli_links)`: exit 1, 5 run, 4 passed, 1 failed:
  `links_a_refused_send_and_an_own_name_send_record_none` at "the wrapper's own name links"
  (`tests/cli_links.rs:212`).

**Restored and read back** after each of the two builds: the same two SHA-256 readings as in section 2.

**Not forced on:** `send_link_a_send_with_no_from_adds_none`. With no `from` there is no name to record, so
there is no guard line to remove; the case pins that a send without one is issued with no `link` line.

## Not measured

- No mutation run and no witness run (the operator's direction at take-up, `inputs#I1`, note 1): no mutant of
  the new code was graded.
- The new cases on Windows and macOS: read by the operator pass's CI entry, not here.
- Nothing against the real `claude`: every reading ran under the fake agent.
