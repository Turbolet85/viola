# Red and green — 2026-10-10-self-healing-state

The one-off readings the plan's steps 3 and 11 ask for. Every run is a firing of block entry 4 (the unit filter)
or entry 5 (the integration filter) through the gate tool, in implement run
`.andromeda/runs/2026-10-10T10-25-01-implement/`; the entry lines are copied from its trail without the header.
Times are the trail's own (`ts`, UTC, 2026-10-10). Each log is named on its line and sits under
`$TMPDIR/andromeda-gate/2026-10-10-self-healing-state/implement-2026-10-10T10-25-01/`.

## 1. Step 3 — the new cases on a tree without the behaviour (red)

**The tree.** Every test of steps 4 to 10 as it stands at the end of the chunk, with one later exception named
below. Step 1's kind table was already in, as the plan orders it. The product side held the new API with no
behaviour behind it, because the cases do not compile without the names:

- `Skipped` with its three fields, and a reader that counted nothing and returned every object as before;
- the three append functions on the old write path: no last-byte read, no heal, no line;
- `read_snapshot_classified` returning `Absent` for everything, with `read_snapshot` on its old body;
- `replay` returning an empty result and `read_snapshot_or_replay` returning `Absent`.

**Entry 4, 10:37:42Z**

```
  4 unit        red · exit 0 ✗ (exit 1) · exit 1 · 8.39s · 44815 B → 4.log · bash scripts/agent-run.sh run --unit --filter 'test(/torn_… (120 chars)
entries 16 · green 0 · red 1 (4) · recorded 0 · timeout 0 · not-run 15
```

nextest: `43 tests run: 10 passed, 33 failed, 1497 skipped`. Each of the 33 failed on an assertion, none on a
compile error:

- `events::tests`, 13: the seven `three_counts` cases that expect a count (`a_line_past_the_cap_is_torn` ×3,
  `an_absent_non_string_or_unlisted_kind_is_unknown_and_unreturned`,
  `an_unlisted_key_counts_once_for_its_line_and_the_line_is_returned`,
  `an_unterminated_last_line_is_torn_unreturned_and_unwritten`,
  `a_terminated_line_that_is_not_one_object_is_torn`) and the six `torn_tail` cases that expect a heal
  (`append_starts_on_a_fresh_line_and_keeps_the_prior_bytes`,
  `heal_writes_one_state_recovered_line_and_the_next_append_none`,
  `append_at_returns_and_hands_its_builder_the_fresh_line_start`,
  `try_append_heals_while_the_lock_is_free_and_writes_nothing_while_held`,
  `reader_started_at_the_old_end_returns_the_healed_line`,
  `counts_one_torn_line_before_the_heal_and_one_after`);
- `replay::tests`, 9: every `replay_recovers` case but the empty-log one;
- `snapshot::tests`, 11: every `snapshot_cause` case but the absent one (the seven `unreadable` rows, the
  `v` 0 case, the unreadable file, the newer `v`, the written snapshot).

The 10 that passed on that tree, and why each could not fail there:

- the four `kind_table` cases of `viola-core`: step 1 lands before step 3, so their behaviour was present;
- six cases that assert something does NOT happen, which a tree with no behaviour satisfies:
  `events_torn_tail_a_log_with_no_fragment_takes_the_line_alone_and_no_record` ×3 (absent, empty, whole),
  `events_three_counts_every_kind_written_with_its_own_keys_reads_zero`,
  `replay_recovers_nothing_from_an_empty_or_an_absent_log`, `snapshot_cause_no_file_is_absent`.

So the plan's sentence "every new case fails for want of the behaviour it guards" held for 33 of the 39 cases of
steps 4 to 7. The other six are shown red by the two controls of section 3.

**Entry 5, 10:38:25Z**

```
  5 integration red · exit 0 ✗ (exit 1) · exit 1 · 11.57s · 13051 B → 5.log · bash scripts/agent-run.sh run --integration --filter 'bina… (166 chars)
entries 16 · green 0 · red 1 (5) · recorded 0 · timeout 0 · not-run 15
```

nextest: `43 tests run: 37 passed (3 slow), 6 failed, 0 skipped`. The six, each on an assertion:

- `viola::chaos_torn_append chaos_torn_tail_left_by_a_stopped_wrapper_is_healed_by_the_next_start` (the reader
  counted `torn_lines` 0 on the shortened log where the case expects 1);
- `viola-state::state_events`, 2 of 3: `a_log_cut_short_takes_the_next_append_on_a_fresh_line`,
  `lines_outside_the_contract_are_counted_and_known_kinds_still_returned`;
- `viola-state::state_replay`, 3 of 3.

Of the 37 that passed, 33 are the older cases of `cli_send` and `cli_answer`, which this chunk does not change.
The other four are new or edited and assert an absence: `state_events_every_kind_round_trips_the_six_key_line`,
`path2_send_confirms_with_cl1_events`, `path4_dialogs_are_logged_once_woken_and_answered_by_id`,
`path4_permission_is_logged_once_woken_and_answered_by_id` (zero on all three counts). Section 3's first control
shows the first two red; the two `path4` cases have no red reading here (see section 3).

**The one later change to a test.** After this reading the helper `assert_reads_clean` in `tests/cli_send.rs`
and `tests/cli_answer.rs` was rewritten from `.map(..).count()` to `.collect()` plus `.len()`, for clippy
(`suspicious_map`). It asserts the same two things.

## 2. Step 11 — the same entries on the finished tree (green)

**Entries 4 and 5, 10:41:00Z**

```
  4 unit        green · exit 0 · 5.54s · 7346 B → 4.2.log · bash scripts/agent-run.sh run --unit --filter 'test(/torn_… (120 chars)
  5 integration green · exit 0 · 11.48s · 6893 B → 5.2.log · bash scripts/agent-run.sh run --integration --filter 'bina… (166 chars)
entries 16 · green 2 · red 0 · recorded 0 · timeout 0 · not-run 14
```

nextest: unit `43 tests run: 43 passed, 1497 skipped`; integration `43 tests run: 43 passed (3 slow), 0 skipped`.

## 3. Two controls on the finished tree, each a guard removed and put back

**The key-table control (the plan's, step 11), 10:41:30Z.** One key removed from one row of
`EventKind::data_keys` in `crates/viola-core/src/lib.rs`: `prompt-submitted` read `["text"]` in place of
`["text", "origin"]`. The edit was read back in the file before the run.

```
  4 unit        red · exit 0 ✗ (exit 1) · exit 1 · 6.43s · 9572 B → 4.3.log · bash scripts/agent-run.sh run --unit --filter 'test(/torn_… (120 chars)
  5 integration red · exit 0 ✗ (exit 1) · exit 1 · 11.5s · 8651 B → 5.3.log · bash scripts/agent-run.sh run --integration --filter 'bina… (166 chars)
entries 16 · green 0 · red 2 (4,5) · recorded 0 · timeout 0 · not-run 14
```

- unit, `41 passed, 2 failed`: `kind_table_data_keys_are_the_contract_rows` and
  `events_three_counts_every_kind_written_with_its_own_keys_reads_zero`;
- integration, `41 passed (3 slow), 2 failed`: `path2_send_confirms_with_cl1_events`, whose product-written log
  read `Skipped { unknown_kinds: 0, unknown_fields: 2, torn_lines: 0 }` where the case expects all zero, and
  `state_events_every_kind_round_trips_the_six_key_line`.

The two `path4` cases stayed green under this control: their logs hold no `prompt-submitted` line. Their zero
count was not shown red by removing a key of a dialog kind; that reading was not taken.

**The absence control, 10:42:02Z.** Three guards neutralised at once, each read back in its file before the run:
`tail` in `events.rs` reported every log as torn (the empty and absent ones included);
`read_snapshot_classified` returned `Unreadable` for a missing file; `replay` started from `wheel` driver and
`budget_paused` false in place of nothing.

```
  4 unit        red · exit 0 ✗ (exit 1) · exit 1 · 6.44s · 25206 B → 4.4.log · bash scripts/agent-run.sh run --unit --filter 'test(/torn_… (120 chars)
entries 16 · green 0 · red 1 (4) · recorded 0 · timeout 0 · not-run 15
```

nextest: `43 tests run: 29 passed, 14 failed, 1497 skipped`. Among the 14 are the five absence cases the first
control did not reach: `events_torn_tail_a_log_with_no_fragment_takes_the_line_alone_and_no_record` ×3,
`replay_recovers_nothing_from_an_empty_or_an_absent_log` and `snapshot_cause_no_file_is_absent`. The other nine
are cases the same three edits break by the way.

**Put back.** The four files the two controls touched hash the same after the last restore as before the first
edit (`sha256sum`, read at 10:42:13Z):

| file | sha256, first 16 |
|---|---|
| `crates/viola-core/src/lib.rs` | `ab8cdfeecaad2196` |
| `crates/viola-state/src/events.rs` | `8db47cfe41d59cf1` |
| `crates/viola-state/src/snapshot.rs` | `c8f6364bd1277b93` |
| `crates/viola-state/src/replay.rs` | `200be7eee0ba3db9` |

**The schema control, 10:44:45Z.** After the first whole-block firing one assertion was added to the chaos case:
its `state-recovered` line must pass `schemas/diag-line.v1.json`. A local run removes its test homes, so the
block's schema-check entry never reads that line; CI keeps the homes. For the control the heal's line was given
one field outside the catalog (`outside_catalog`), read back in the file before the run.

```
  5 integration red · exit 0 ✗ (exit 1) · exit 1 · 15.41s · 7939 B → 5.5.log · bash scripts/agent-run.sh run --integration --filter 'bina… (166 chars)
entries 16 · green 0 · red 1 (5) · recorded 0 · timeout 0 · not-run 15
```

nextest: `43 tests run: 42 passed (3 slow), 1 failed, 0 skipped`. The one is the chaos case, at the new
assertion: `the state-recovered line fails the diag-line schema`. The field was then removed and the four hashes
above read the same again.

The whole block was fired once more after that restore; its entries 4 and 5 are the green reading on the final
tree (the implement report carries the block's lines).

## 4. The root wait count the plan forecast

`grep -rnE 'Instant::now\(\) \+ WITHIN' tests` after step 10: 23 sites in 17 files. The plan read 22 in 16 at
`2f1efe3`; `tests/chaos_torn_append.rs` adds one site and one file.
