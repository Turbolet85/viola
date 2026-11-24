# P5 tool block — 2026-11-24-viola-up (the directive's item 11, `inputs#I1`)

Written after the word on card 2, which did not carry it: the cards omitted this block — a miss of this phase, named
below. Nothing here changes the approved plan.

## The union, verbatim
```
architecture: rows 1028 · packs 11 · picked 345 (252 by every draw) · standing 17 · 92757 B
security-plan: rows 655 · packs 7 · picked 198 (153 by every draw) · standing 13 · 54968 B
design-system: rows 344 · packs 4 · picked 174 (136 by every draw) · standing 5 · 40379 B
layout-templates: rows 232 · packs 3 · picked 81 (59 by every draw) · standing 14 · 18301 B
test-plan: rows 1378 · packs 14 · picked 358 (254 by every draw) · standing 86 · 87890 B
obs-plan: rows 1048 · packs 11 · picked 431 (319 by every draw) · standing 23 · 98829 B
a11y-plan: rows 238 · packs 3 · picked 133 (113 by every draw) · standing 24 · 33533 B
rows-union: 7 master(s) · picked 1720 of 4923 · rule any
```
- Every row the plan cites stands in its rows file: each handle grepped in `*-rows.md` / `*-standing.md` of this run
  dir — all present (the plan's Provenance and Goal handles, the Expected-amendment handles).

## [RETYPED] lines and re-spawns, verbatim
```
security-pick-p3.d3 · [RETYPED] line 13: decision/unwrapped-agent-name-never-target is no row of the pack — read as decision/unwrapped-name-never-target, the pack's row 13
tests-pick-p5.d2 · [RETYPED] line 80: decision/pty-tested-on-real-openpty-spawns is no row of the pack — read as decision/pty-tested-on-real-openpty, the pack's row 80
tests-pick-p6 · [RETYPED] line 51: decision/hook-fails-open-silently is no row of the pack — read as decision/hook-fails-open, the pack's row 51
tests-pick-p6.d3 · [RETYPED] line 91: decision/hook-fails-open-silently is no row of the pack — read as decision/hook-fails-open, the pack's row 91
a11y-pick-p2 · [RETYPED] line 64: decision/time-bounds-fall-on-the-driver-only is no row of the pack — read as decision/time-bounds-fall-on-the-driver, the pack's row 64
```
- Re-spawns: none (task 7/7, pick 159/159, history 7/7 first try; `fanout.py census`).

## Non-zero exits, refusals, UNPARSED / INDETERMINATE / NOT DERIVED
- Three Bash-guard refusals, each re-issued once in the guard's form: a `rec.py find` piped into `grep` («a pipeline
  tool's output piped into a filter»); a `cd` into the run dir («moves the session cwd»); a `gate.py run` loop piped
  into `grep` (the same pipe arm).
- Baseline reds, each the plan's prediction: entries 4 and 6 `exit 4` (no tests to run, 0 selected); 8, 10 and 11
  `exit 127` (no `scripts/r4-up.sh`); 12 `exit 2` (no script to grep); 16 deferred, controlled by hand.
- No `UNPARSED`, `INDETERMINATE` or `NOT DERIVED` line was printed by any tool in this run.

## Where a letter was wrong or silent
- The two P5 cards omitted this block (the directive's item 11): this file stands in for it.
- The away form (`inputs-contract.md` §snap: «the step stops there») and the directive (item 5: «go on to P4 without
  waiting» when the arms build) disagree; the directive was followed.
- `promotion.md` orders the scope's draft before the snap, so the scope cited `inputs#I1` before the snap minted the
  id; it matched.
- Card 1's T3 sentence («only existing code words but `would-open`») was wrong against the plan — corrected after
  card 1 (`inputs#I2` change 4).
