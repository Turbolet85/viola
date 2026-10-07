# Operator pass — 2026-10-07-send-waits-out-the-paste-hint

The implementer drove this pass on the operator's word, given with the implement invocation ("Run the operator
pass with the ci.py conclusion read (leg=operator) as usual", inputs#I3). Times are `date -u`, 2026-10-07.

Before it, the block's eighteen implement-side entries read green on the final tree: entries 1 to 16 in the second
whole-block run (12:40Z to 12:43Z, entry 9 red there by its baseline, the record not yet written), the live round
(entries 17 and 18) at 12:44:07Z to 12:44:54Z, and entries 7, 8 and 9 once more after the round (12:46Z), entry 9
now green. Entries 19 to 21 are this pass. The live round was fired before any of it, and no product source
changed between the round and the pre-CI commit (STOP 6).

## Before the pass — `pre-push` (entry 16) on the uncommitted tree, 12:47:13Z to 12:48:09Z
- `bash scripts/agent-run.sh pre-push` through the gate tool (`--entry 16`): green, exit 0, 55.44 s. Its document:
  `"ok":true`, `"stage":"linux-tests"`; coverage 1695/1695 (thirteen more than the base's 1682: the seven
  `verdict_verified_…` cases, the gate's bound case, the four `send` cases and the keystroke case), playwright 1/1,
  `gate` no breaches. Atoms: `exit 0` ✓, `contains "ok":true` ✓, `contains "stage":"linux-tests"` ✓. Load
  average at its start: 2.13.
- The same entry in the block's second full run: green, 56.78 s, the same counts.

## Entry 19 — hygiene (by hand)
- Read at 12:48:15Z:
  `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` → exit 0,
  `hygiene: clean — read 70 (runs 59 · evidence 6 · inputs 5) · trails 14 not read · copies 3 not read by P1 — 0
  host paths kept · binary 0 not read by P1`. Atoms: `exit 0` ✓, `contains hygiene: clean` ✓. No row to rewrite.
- Read once more after this file was added, before the commit: the verdict is in the next section's first line.

## The pre-CI commit and entry 20 — the push, 12:48:44Z to 12:48:48Z
- Hygiene re-read at 12:48:38Z, just before the commit: `hygiene: clean`, read 71 (runs 59 · evidence 7 ·
  inputs 5), this file now among the evidence.
- The scope read before it (`gate.py scope`, 12:48:38Z): `scope: clean — changed 10 · listed 10 · recorded 0`; the
  ten changed files are research's ten, and no `scope-record.md` line was needed.
- `e574e73` `chore(2026-10-07-send-waits-out-the-paste-hint): operator pre-CI commit, for the run this chunk's
  verdict reads` at 12:48:44Z (the whole tree, 89 files: the phase's products, the ten source and test files,
  this chunk's evidence and inputs, the two run dirs and the bookkeeping the tree carried).
- Entry 20: `git diff --quiet && git diff --cached --quiet && git push origin HEAD` → exit 0 at 12:48:48Z,
  `56e67bb..e574e73  HEAD -> build/viola-0.1.0`; 0 ahead of the upstream after it.

## Entry 21 — the CI conclusion: GREEN
- `ci.py conclusion --sha HEAD --wait 1800` → exit 0: `e574e738f814 verdict: green · checks 15/15 · wall 397 s ·
  runs ci#37623727247 completed/success` (polled 14× over 405 s, 12:48:55Z to 12:55:41Z). Atoms: `exit 0` ✓,
  `contains verdict: green` ✓. No fix commit was needed, so no source changed after the live round (STOP 6 did
  not fire).
- The fifteen jobs, each `success` (`gh run view 37623727247 --json jobs`): `test`, `lint`, `perf` and `release`
  on the three OSes (windows-2025, macos-latest, ubuntu-latest), `supply-chain`, `msrv`, `fuzz-replay`.

## The new and turned cases on CI, from the three `test` jobs' logs of ci#37623727247
The twelve new unit cases are those the unit filter entry selects (`waits_for_the_input_box` or
`after_the_gate_wait` in the name): seven in `screen.rs`, one in `gate.rs`, four in `send.rs`.

| leg | new unit cases passed | other verdict lines for them | keystroke case | `hint` | `no_hint` | verify's hint case (9 000 ms hold) | `verify_window_without_screens_…` | the leg's tests |
|---|---|---|---|---|---|---|---|---|
| ubuntu-latest | 12 | 0 | PASS 7.451 s | PASS 7.765 s | PASS 5.011 s | PASS 13.129 s | PASS 35.265 s | `1695 tests run: 1695 passed (5 slow)` |
| macos-latest | 12 | 0 | PASS 8.644 s | PASS 8.251 s | PASS 5.696 s | PASS 13.706 s | PASS 35.235 s | `1691 tests run: 1691 passed (5 slow)` |
| windows-2025 | 12 | 0 | PASS 7.822 s | PASS 7.884 s | PASS 4.821 s | PASS 13.057 s | PASS 34.316 s | `1719 tests run: 1719 passed (5 slow)` |

- Each leg ran thirteen more tests than at the previous chunk's run (ci#37609247992: 1682, 1678, 1706).
- The keystroke case was sequenced on its observed signals on every OS leg (STOP 5 did not fire): 7.5 s to 8.6 s,
  under the CI profile's 20 s kill for `binary(cli_send)`.
- `verify_window_without_screens_fails_every_interactive_row` read 34.3 s to 35.3 s on the runners, the same floor
  as on the dev host (34.3 s), 24.7 s or more under the moved 60 s kill. Its `SLOW [> 20.000s]` line is the moved
  period firing once.
- `feed_past_the_frame_cap_poisons_and_settles_without_rows`, the one case the first block run read red, passes on
  all three.
- CI never runs the live round; its record is `reverify-round.md`.
