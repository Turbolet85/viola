# Operator pass — 2026-10-04-wait-and-last

Run 2026-10-04 on the operator's word at the /implement call ("Run the operator pass with the ci.py conclusion read
(leg=operator) as usual"), on the Linux dev host. The native `pre-push` (entry 21) read `ok:true`, stage
`linux-tests` (coverage 1200 passed / 0 failed, browser ok, gate ok) at /implement's full run (gate trail
`.andromeda/runs/2026-10-04T09-55-57-implement/`), again by hand on the unchanged code just before the pre-CI commit,
and again on the folded tree before the fix push (below).

| entry | command (as the plan lists it) | exit | reading |
|---|---|---|---|
| 22 | `python -X utf8 …/andromeda-tools/scripts/gate.py hygiene` | 0 | first read `hygiene: refused 4 files — P1 4` (the raw `gate.py` listings phase P4 left in `.andromeda/runs/2026-10-04T09-25-49-phase/`: `.baseline.txt`, `.dryrun.txt`, `.dryrun2.txt`, `.dryrun3.txt`, untracked, named only in gate delta inventories): removed; re-read `hygiene: clean — read 53 (runs 52 · evidence 1) · trails 14 not read · binary 0 not read by P1` |
| — | the pre-CI commit (`git add -A`, then `chore(2026-10-04-wait-and-last): operator pre-CI commit, for the run this chunk's verdict reads`) | 0 | `db15b2b` on `build/viola-0.1.0`, no line-ending warning |
| 23 | `git diff --quiet && git diff --cached --quiet && git push origin HEAD` | 0 | `bcff692..db15b2b  HEAD -> build/viola-0.1.0` |
| 24 | `python -X utf8 …/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800` | 0 | `db15b2bab1cd verdict: red · checks 15/15 · first-fail +155 s test (macos-latest) · runs ci#37194791251` |
| — | the red folded: `fix(2026-10-04-wait-and-last): update the wait feed under the append's lock, so last agrees with the disk` | 0 | `3de125f`; before it, entries 1 · 2 · 4 · 5 · 7 · 8 · 9 · 21 re-ran green on the folded tree (`pre-push` `ok:true`); hygiene after it `clean` |
| 23 | `git diff --quiet && git diff --cached --quiet && git push origin HEAD` | 0 | `db15b2b..3de125f  HEAD -> build/viola-0.1.0`; 0 ahead after |
| 24 | `python -X utf8 …/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800` | 0 | `3de125ff8ea7 verdict: green · checks 15/15 · wall 303 s · runs ci#37195156240 completed/success` |

## The red folded (ci#37194791251, let finish before the fold)
`test (macos-latest)` 1196 run, 2 failed: `last_survives_a_wrapper_restart` (`last_assistant_message` read `null` right
after the `turn-ended` line was on disk) and `last_human_escapes_controls` (stderr not empty: `no message`). Every other
job, `test (ubuntu-latest)` 1200/1200 and `test (windows-2025)` 1237/1237 included, read success.

Cause: `append_hook_event` appended the line, then told the wait feed in a second step, so a `last` asked by someone
who had already seen the line on disk could land in between and read the turn before it. The macOS runner hit that
window. `WaitFeed::appending` now runs the append inside the feed's lock hold, then updates the newest turn and
wakes the parked `wait`s before releasing it. The window is pinned by the unit test
`run::wait::tests::last_after_a_turn_on_disk_never_reads_the_turn_before_it`, red on the old order (`left: Null`,
the macOS reading) and green on the fix (`guard-controls.md`). Both files were in research's lists.

## The three-OS witness (ci#37195156240)
Every job `success` (15/15). Read from each test job's log (`gh api …/actions/jobs/<id>/logs`): no job logged a FAIL or
TIMEOUT line.

| job | `cli_wait_last` (8) | `chaos_wait_vanish` (1) | `channel_endpoint` | `run::wait::tests` (40) | G2 | G4 | secret scan |
|---|---|---|---|---|---|---|---|
| `test (windows-2025)` 111415378671 | 8 PASS | 1 PASS | 6 PASS | 40 PASS | `g2: clean` | `ok:true` 147 files / 973 lines | `ok:true`, 0 hits |
| `test (macos-latest)` 111415378736 | 8 PASS | 1 PASS | 5 PASS | 40 PASS | `g2: clean` | `ok:true` 138 / 938 | `ok:true`, 0 hits |
| `test (ubuntu-latest)` 111415378624 | 8 PASS | 1 PASS | 5 PASS | 40 PASS | `g2: clean` | `ok:true` 138 / 938 | `ok:true`, 0 hits |

**v1-30**: `path3_wait_parks_until_turn_ended_then_last_reads_it`, `wait_after_a_send_cursor_returns_the_turn`,
`last_survives_a_wrapper_restart` and `chaos_wait_parked_wrapper_killed_exits_21` PASSED on all three OSes.
