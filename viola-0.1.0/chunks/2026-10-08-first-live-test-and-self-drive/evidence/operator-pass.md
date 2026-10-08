# Operator pass — 2026-10-08-first-live-test-and-self-drive

The implementer drove this pass on the operator's word, given with the implement invocation ("Run the operator
pass with the ci.py conclusion read (leg=operator) as usual and read run_attempt: the final sha needs green on
its first attempt", inputs#I19). Times are `date -u`, 2026-10-08.

Before it, the block read green on the final tree in one call of the gate tool (08:53Z to 08:55Z): entries 1 to
15 and 18 to 22, twenty green and none red. Entries 16 and 17, the round, were not fired again: the round was
fired once, by the second implement run (`round-072838Z.txt`, `round: COMPLETE`). Entries 23 to 25 are this
pass. No push went out before every entry of the block read green by its own letter. The live work was over
before the pass: both live sessions closed, the own compositor ended at 08:49:30Z, the desktop lock read locked
after it (`compositor.md`).

Two readings of the live work are named in implement's report and are not gates: a driver text ending in one CR
is delivered but reported not delivered, and takes the wheel (`live-readings.ndjson`, `trailing-cr`); and the
CLI's plan file landed in its default directory, not in the `plans/` beside the home (`live-run.md`).

## Before the pass — `pre-push` (entry 14) on the uncommitted tree, 08:57:52Z to 08:58:48Z
- `bash scripts/agent-run.sh pre-push` through the gate tool (`--entry 14`): green, exit 0, 55.61 s. Its
  document: `"ok":true`, `"stage":"linux-tests"`; coverage 1747/1747 (35 more than the base's 1712: the eight
  reply cases and the 27 negative controls of `src/run/wheel.rs`), playwright 1/1, `gate` no breaches. Atoms:
  `exit 0` ✓, `contains "ok":true` ✓, `contains "stage":"linux-tests"` ✓. Load average at its end: 4.68.
- The same entry in the block's two whole runs (08:35Z and 08:54Z): green, 56.19 s and 56.5 s, the same counts.
  No product source changed between the three; the files written between them are this chunk's evidence, its
  matrix ref and one input copy.

## Entry 23 — hygiene (by hand)
- Read at 08:58:53Z:
  `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` → exit 0,
  `hygiene: clean — read 267 (runs 220 · evidence 26 · inputs 21) · trails 31 not read · copies 19 not read by
  P1 — 0 host paths kept · binary 0 not read by P1`. Atoms: `exit 0` ✓, `contains hygiene: clean` ✓. No row to
  rewrite.
- Read once more after this file was added, before the commit: the verdict is in the next section's first line.

## Before the pre-CI commit, 08:59:13Z
- Hygiene re-read: `hygiene: clean`, read 268 (runs 220 · evidence 27 · inputs 21), this file now among the
  evidence.
- The scope read (`gate.py scope`): `scope: clean — changed 2 · listed 2 · recorded 0`; the two changed files
  are research's two, `src/human.rs` and `src/run/wheel.rs`. Its base is HEAD, `0dafa09`.
- The tree the commit takes: the phase's and both revisions' products, the two source files, this chunk's
  evidence and inputs, the six run dirs and the bookkeeping the tree carried. 0 ahead of the upstream before it.
- The commit, the push (entry 24) and the CI read (entry 25) are recorded below after they are made; that part
  of this file rides the next commit.
