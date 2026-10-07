# Operator pass — 2026-10-07-test-homes-off-the-contended-volume

The implementer drove this pass on the operator's word, given with the implement invocation ("Run the operator
pass with the ci.py conclusion read (leg=operator) as usual"). The block had read 17 green, 0 red on the final
tree in its one full run of 08:09Z to 08:10Z (entries 1 to 17; entries 18 to 20 are this pass). No live round
exists in this block.

## Before the pass — `pre-push` (entry 16) on the uncommitted tree, 2026-10-07T08:11:32Z
- `bash scripts/agent-run.sh pre-push` → exit 0 after 42 s: `"ok":true`, `"stage":"linux-tests"`; coverage
  1675/1675, playwright 1/1, `gate` no breaches. Atoms: `exit 0` ✓, `contains "ok":true` ✓,
  `contains "stage":"linux-tests"` ✓. Load average at its start: 18.22, falling from another project's build that
  had ended; no build process of another project was alive at its end.
- The watch item (a coverage merge red over green tests while another build writes to the volume): not seen.
  Three green readings in this chunk: entry 16 in the block's first run (41.8 s, quiet host), entry 16 in the full
  run (43.0 s, started 40 s after the contended window while the other build was still alive), and this one.

## Entry 18 — hygiene (by hand)
- Read at 2026-10-07T08:12:23Z:
  `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` → exit 0,
  `hygiene: clean — read 57 (runs 45 · evidence 7 · inputs 5) · trails 12 not read · copies 3 not read by P1 — 0
  host paths kept · binary 0 not read by P1`. Atoms: `exit 0` ✓, `contains hygiene: clean` ✓. No row to rewrite.
- Read once more after this section was added, before the commit: the verdict is in the next section's first line.
