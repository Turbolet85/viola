# The root package's baseline under the nextest `mutants` profile (plan steps 1, 2, 5)

## Red before (step 1) — recorded, never graded

Source: P5's baseline firing of the block entry whose `run` opens `NEXTEST_PROFILE=mutants` (entry 14), on the
untouched tree at HEAD `d35644a58909`, read with `gate.py show --run-dir .andromeda/runs/2026-10-20T07-58-07-phase
--marker 2026-10-20-epoch-5-cleanup --n 14` and its log (`$TMPDIR/andromeda-gate/2026-10-20-epoch-5-cleanup/phase-2026-10-20T07-58-07/14.log`,
228 210 B, written 2026-10-20T08:52:45Z; `$TMPDIR` = `/tmp`).

- `df -h /tmp` before it (P5, entry 1's log): `tmpfs 32G 18G 14G 56% /tmp`.
- Verdict: `exit 0 ✗ (exit 100)` · `lacks 'TIMEOUT' ✗`.
- Summary line: `Summary [  61.115s] 1601 tests run: 1600 passed (115 slow), 1 timed out, 0 skipped`.
- The one TIMEOUT line: `TIMEOUT [  10.003s] (1600/1601) viola::contract_fake_agent_drift fake_agent_dialog_replay_matches_every_recorded_dialog_set`.

The cases at or near the 10 s kill (their PASS lines, same log):

| case | wall (s) | kill (s) |
|---|---|---|
| `contract_fake_agent_drift fake_agent_dialog_replay_matches_every_recorded_dialog_set` | TIMEOUT 10.003 | 10 |
| `contract_fake_agent_drift fake_agent_framing_replay_matches_every_recorded_framing_set` | 9.453 | 10 |
| `contract_fake_agent_drift fake_agent_statusline_replay_matches_every_recorded_statusline_set` | 9.431 | 10 |
| `contract_ledger_probes contract_ledger_probes_pass_over_every_stamped_set` | 9.447 | 10 |
| `cli_send send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins` | 8.708 | 10 |
| `cli_send send_under_the_paste_hint_on_a_verified_cli::case_1_hint` | 8.686 | 10 |
| `cli_verify verify_window_without_screens_fails_every_interactive_row` | 43.034 | 45 |
| `cli_budget path6_the_boundaries_and_the_window_word` | 5.955 | 10 |

The other replay cases the step-2 walks cover: `fake_agent_agents_replay_matches_every_recorded_agents_set` 0.465 s,
`fake_agent_compact_replay_matches_every_recorded_compact_set` 0.090 s, `fake_agent_resume_replay_matches_every_recorded_resume_set`
0.078 s, `fake_agent_print_turn_matches_every_recorded_set` 0.163 s.

## After steps 2–4 (step 5): the same entry, fired by /implement P2

Source: the block's entry 14 in this run (`gate.py show --run-dir .andromeda/runs/2026-10-20T08-57-19-implement --marker
2026-10-20-epoch-5-cleanup --n 14`; log `14.log`, 225 304 B, written 2026-10-20T09:19:01Z), on the chunk's tree.

- `df -h /tmp` before the block (entry 1): `tmpfs 32G 19G 14G 59% /tmp`.
- Verdict: `green · exit 0`. `lacks 'TIMEOUT'` holds.
- Summary line: `Summary [  61.136s] 1605 tests run: 1605 passed (112 slow), 0 skipped`. No TIMEOUT line, so no case
  outside steps 2–4 needed a change.

| case | before (s) | after (s) | kill after (s) |
|---|---|---|---|
| `fake_agent_dialog_replay_matches_every_recorded_dialog_set` (3 dialog sets) | TIMEOUT 10.003 | 4.877 | 10 |
| `fake_agent_framing_replay_matches_every_recorded_framing_set` | 9.453 | 4.836 | 10 |
| `fake_agent_statusline_replay_matches_every_recorded_statusline_set` | 9.431 | 4.836 | 10 |
| `contract_ledger_probes_pass_over_every_stamped_set` (2 stamped sets) | 9.447 | 4.800 | 10 |
| `fake_agent_agents_replay_matches_every_recorded_agents_set` | 0.465 | 0.294 | 10 |
| `fake_agent_compact_replay_matches_every_recorded_compact_set` | 0.090 | 0.048 | 10 |
| `fake_agent_resume_replay_matches_every_recorded_resume_set` | 0.078 | 0.063 | 10 |
| `fake_agent_print_turn_matches_every_recorded_set` | 0.163 | 0.062 | 10 |
| `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins` | 8.708 | 8.681 | 30 (step 3) |
| `send_under_the_paste_hint_on_a_verified_cli::case_1_hint` | 8.686 | 8.795 | 30 (step 3) |
| `verify_window_without_screens_fails_every_interactive_row` | 43.034 | 43.034 | 60 (step 4) |
| `path6_the_boundaries_and_the_window_word` | 5.955 | 5.807 | 10 (unchanged) |

Each step-2 case asserts what it asserted before, per set (the same receipts, drift lists, exits and step lines,
every failing set named by its version in one panic), with the `compared >= N` floors unchanged. Its wall is now the
slowest set's, not the sum of all of them.
