# Operator pass — 2026-10-06-local-command-and-paste-framing-rows

The implementer drove this pass on the operator's word after the implement report ("Run the operator pass now, as
usual: entry 26 the hygiene read, the operator pre-CI commit, entry 27 the push, entry 28 the ci.py conclusion
read (leg=operator) on the pushed HEAD"). The block had read 23 green, 0 red on the final tree in its second full
run (`block-reds-host-contention.md` holds the first run's two reds and their cause). The live round was fired
once at step 14 and is not fired again (`record-round-green.md`).

## Before the pass — `pre-push` (entry 25) on the uncommitted tree, 2026-10-06T21:32:30Z
- `bash scripts/agent-run.sh pre-push` → exit 0 after 43.2 s: `"ok":true`, `"stage":"linux-tests"`; coverage
  1638/1638, playwright 1/1, `gate` no breaches. Atoms: `exit 0` ✓, `contains "ok":true` ✓,
  `contains "stage":"linux-tests"` ✓. No other build was linking on the host during it.

## Entry 26 — hygiene (by hand), 2026-10-06
- `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` → exit 0,
  `hygiene: clean — read 84 (runs 61 · evidence 14 · inputs 9)`. Atoms: `exit 0` ✓, `contains hygiene: clean` ✓.
  Re-read after this file was written, before the commit (below).

## The pre-CI commit and entry 27 — the push
- `05b5f2b` `chore(2026-10-06-local-command-and-paste-framing-rows): operator pre-CI commit, for the run this
  chunk's verdict reads` (the whole tree, 116 files, after hygiene read clean again:
  `read 85 (runs 61 · evidence 15 · inputs 9)`).
- Entry 27: `git diff --quiet && git diff --cached --quiet && git push origin HEAD` → exit 0,
  `2fbc954..05b5f2b  HEAD -> build/viola-0.1.0`.

## Entry 28 — the CI conclusion: GREEN
- `ci.py conclusion --sha HEAD --wait 1800` → exit 0: `05b5f2b5371b verdict: green · checks 15/15 · wall 481 s ·
  runs ci#37534758441 completed/success` (polled 17× over 497 s, 2026-10-06T21:33:48Z to 21:42:05Z). Atoms:
  `exit 0` ✓, `contains verdict: green` ✓. No fix commit was needed.
- Seventeen rows stamp through the fake agent on all three `test` legs: ubuntu-latest (254 s, `1638 tests run:
  1638 passed (4 slow)`), macos-latest (348 s, 1634 passed, 4 slow), windows-2025 (431 s, 1673 passed, 4 slow).

## The timing read on CI (step 9's owed half), from the three `test` jobs' logs of ci#37534758441
Seconds; the kill is 20 s for the verify-driving binaries and 45 s for the `verify_window_` class.

| test | ubuntu-latest | macos-latest | windows-2025 | here (`full-run-timing.md`) |
|---|---|---|---|---|
| `fake_agent_dialog_replay_matches_every_recorded_dialog_set` (the watch test) | 9.98 | 9.07 | 8.51 | 7.9 |
| `fake_agent_framing_replay_matches_every_recorded_framing_set` | 4.11 | 4.61 | 4.23 | 3.9 |
| `contract_ledger_probes_pass_over_every_stamped_set` | 4.12 | 4.43 | 4.27 | 4.0 |
| `verify_a_complete_set_prints_seventeen_steps_and_stamps_every_row` | 6.08 | 4.17 | 3.89 | 4.1 |
| `verify_window_paste_hint_past_the_gate_maximum_still_stamps` | 9.70 | 10.18 | 10.07 | 9.9 |
| `verify_window_without_screens_fails_every_interactive_row` | 20.55 | 20.76 | 20.31 | 20.6 |
| `send_long_text_wrapped_by_the_cli_is_confirmed` | 0.90 | 0.56 | 0.53 | 0.45 |

- The watch test's longest reading is 9.98 s, on the ubuntu coverage leg: half its 20 s kill. No timing red was
  read, so the designed-floor rule of `.claude/rules/testing.md` was not invoked and no bound moved.
- On every leg the tests at or over 10 s are the designed waits: the two `harness_lifecycle` boot cases, the
  `verify_window_` cases and the three `send_window_` cases; windows-2025 adds one harness self-test of the
  mutation runner at 11.6 s, which this chunk does not touch. The "here" column is the instrumented local run.
