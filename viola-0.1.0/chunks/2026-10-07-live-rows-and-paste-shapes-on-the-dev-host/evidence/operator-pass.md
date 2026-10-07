# Operator pass — 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host

The implementer drove this pass on the operator's word, given with the implement invocation ("Run the operator
pass with the ci.py conclusion read (leg=operator) as usual"). The block had read 17 green, 0 red on the final tree
in its one full run of 2026-10-07T10:30Z to 10:32Z (entries 1 to 17; entries 18 to 20 are this pass). No `live` or
`round` entry exists in this block: the three live sessions are one-off witnesses recorded in this folder.

## Before the pass — `pre-push` (entry 17) on the uncommitted tree, 2026-10-07T10:41:59Z
- `bash scripts/agent-run.sh pre-push` → exit 0 after 43 s: `"ok":true`, `"stage":"linux-tests"`; coverage
  1682/1682 (seven more than the base: the `live_shape` cases), playwright 1/1, `gate` no breaches. Atoms:
  `exit 0` ✓, `contains "ok":true` ✓, `contains "stage":"linux-tests"` ✓. Load average at its start: 0.89.
- The same entry in the block's full run: green, 42.05 s, the same counts.

## Entry 18 — hygiene (by hand)
- Read at 2026-10-07T10:42:42Z:
  `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` → exit 0,
  `hygiene: clean — read 91 (runs 72 · evidence 11 · inputs 8) · trails 14 not read · copies 6 not read by P1 — 0
  host paths kept · binary 0 not read by P1`. Atoms: `exit 0` ✓, `contains hygiene: clean` ✓. No row to rewrite.
- Read once more after this section was added, before the commit: the verdict is in the next section's first line.
