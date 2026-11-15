# Implement outcome — 2026-11-14-status-line-interception-removed (resume on plan rev 5)

**Outcome: green + smoke.** Next → the OPERATOR PASS (entries 27 hygiene, 28 push, 29 the CI read), then
`/andromeda-wrap-session`.

Setup line that reads the phase run's approval trail (quoted from `python -X utf8 {tools_dir}/rec.py runs --root .
--kind '*-phase' --file '*-2026-11-14-status-line-interception-removed.json'`, the newest run listed):

    2026-11-15T05-23-58-phase/matrix-2026-11-14-status-line-interception-removed.json · open · 2594 B

## Gates

The whole block, second firing: `entries 29 · green 25 · red 0 · recorded 1 · timeout 0 · not-run 3` (the three
`leg = 'operator'` entries; trail `.andromeda/runs/2026-11-15T05-28-03-implement/gate-2026-11-14-status-line-interception-removed.json`).
Entry 9, the identifier probe (v2-05's acceptance), reads `green · exit 1 · 0 B`; entry 10 (the deleted fixtures,
seeds and schema) `green · exit 0 · 0 B`; entries 11 and 12 green with every `contains` atom held under the rev-5
names; entry 26 (`agent-run.sh pre-push`) green in 98 s.

First firing: green 20 · red 5 (11, 14, 15, 16, 26), from three tests that still pinned the 23-row counts:
`ledger::tests::checks::check_typed_rows_fail_without_a_measurement` (its 23-element verdict vector),
`runs::verify_pastes_nothing_more_once_the_long_turn_screen_shows_a_modal` (`18 pass` → `16 pass`) and
`runs::verify_pastes_no_local_command_once_the_tag_turn_screen_shows_a_modal` (`20 pass` → `18 pass`); entry 15 read
entry 14's suite failure. Restated; `--only 11,14,15,16` green 4; the whole block then green. One iteration.

No deferral: every gate ran (the rust delta is the chunk's own).

## Smoke

`agent-run.sh boot --session p-budget-smoke --instance builder` → `ok`; `status` → `ready`, `builder` `live`;
the child's argv (read from `/proc`) is `--plugin-dir <dir>` then the caller's args, no `--settings`; the instance
directory holds `events.ndjson`, `heartbeat`, `snapshot.json` and their locks, no `settings.json`; the snapshot's keys
are `agent_session_id, child_pid, cli_verified, cli_version, cwd, endpoint, links, pid, pinned_bin, started_at, wheel`.
`cleanup` → `processes_gone: true, endpoint_gone: true, home_removed: true`.

## Matrix

v2-05: `planned → implemented`, ref `src/run/mod.rs::tests::child_launch_passes_no_settings_flag;
tests/cli_instance_state.rs::run_writes_no_settings_file_in_the_instance_dir;
tests/hook_fail_open.rs::hook_retired_word_is_an_unknown_event_and_writes_nothing;
crates/viola-agent-claude/src/ledger/test_cases_stamps.rs::relied_rows_hold_no_retired_row;
tests/contract_ledger_probes.rs; the plan's gate 9 (the identifier probe, exit 1, no output); by-construction: viola
writes nothing under ~/.claude and adds no settings layer`.

## Files changed

72 (`gate.py scope`: `changed 72 · listed 71 · recorded 1 (companion 1)`, `scope: clean`); the eleven deletions the
plan names among them. This run wrote steps 12 and 13 and the rev-5 renames over the tree the stopped run
(`.andromeda/runs/2026-11-15T05-14-17-implement/`) left with steps 1–11.

## Deviations from the plan, with justification

- Scope record: `` - `crates/viola-agent-claude/src/ledger/test_cases_checks.rs` · companion · serves step 1 · self ``
  — a test pinning the relied-row verdict vector (23 → 21), outside research's list.
- Step 3's fixture keys are `retired_*`, not `statusline_*`, on the overseer's word (`inputs#I6`), as plan rev 5
  records.
- `tests/cli_fake_agent.rs::fake_agent_exit_outlives_a_hangup_that_lands_while_it_exits` used the removed
  `statusline-echo` mode only as a Stop hook that drops a marker file; its hook is now `/usr/bin/touch <marker>`, the
  same observable, the hangup assertion unchanged.
- `tests/cli_instance_state.rs`: beyond the three cases the plan removes, two neighbours asserted the removed snapshot
  field. They stay, restated to what still holds and renamed:
  `run_with_home_leaves_the_user_settings_and_makes_no_default_home` and
  `run_in_the_default_home_leaves_the_user_settings_as_found` (the user's settings file byte- and mtime-equal; no
  default home under `--home`; the pinned copy's canonical spelling kept). The user settings the helper plants hold a
  neutral `model` key, so no probe word stands in the file.
- `tests/cli_verify/runs.rs`: `verify_without_framing_fails_the_clear_row_and_on_unix_the_statusline_rows` renamed
  `verify_without_framing_fails_the_clear_and_compact_rows`; every summary count in that file restated on 21 rows.
- The plan's step 1 says "the eight statusline cases" of `ledger/test_cases_runs.rs`; nine `#[test]` functions there
  exercised the removed items, and all nine went (the stopped run, recorded in its outcome).
- `crates/viola-e2e/src/harness/session.rs`: the default-options test renamed
  `session_options_default_to_the_pair_recorded_and_stamped` (the stopped run).
- `hook_retired_word_is_an_unknown_event_and_writes_nothing` runs against a home a wrapper booted in and then stopped:
  the home is quiescent, so "the home's tree byte-equal" is read over every file without a live writer racing it.

No `gate.py guard` entry stands in the block. Nothing surfaced.

## Process hygiene

| process | started by | final state |
|---|---|---|
| `viola run builder` + the fake `claude` of session `p-budget-smoke` (wrapper 2313027, child 2313035) | this run (the P3 smoke) | terminated — `cleanup` `processes_gone: true`; the census holds no `p-budget-smoke` row |
| gate-fired trees (cargo, nextest, the harness's sessions) | this run, through `gate.py run` | terminated — `gate.py census --comm viola --comm claude --comm viola-fake-agent --comm cargo --args projects/viola/target/debug/viola --args p-budget-smoke`: no row of this repository's target |
| 16 host processes the census lists (the overseer sessions, the prototype's, the product's and `~/.viola/bin` builds of `viola`, their `claude` children) | NOT DERIVED — none under this repository's `target/`; the host's fleet | left running — not this run's |

## Decisions & corrections

- The overseer's technical answers carried from the stopped run: neutral `retired_*` fixture keys (`inputs#I6`); stop
  for a re-plan (`inputs#I7`); the revision's word (`inputs#I8`) and its P5 word keeping rev 4's baselines for
  entries 11 and 12 (`inputs#I9`) — the new names' first reading is this run's entries 11 and 12, green.
- The operator's standing directions for this run: no live `claude` start, no write under `~/.viola/` or to
  `~/.claude/settings.json`, the pinned product binary untouched — held (the smoke ran over the fake agent in a
  harness home under `target/`).
