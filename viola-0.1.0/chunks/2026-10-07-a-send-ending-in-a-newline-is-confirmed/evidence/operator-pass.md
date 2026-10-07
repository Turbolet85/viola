# Operator pass — 2026-10-07-a-send-ending-in-a-newline-is-confirmed

The implementer drove this pass on the operator's word, given with the implement invocation and again with the
re-entry ("Run the operator pass with the ci.py conclusion read (leg=operator) as usual and read run_attempt: the
final sha needs green on its first attempt", inputs#I4, inputs#I9). Times are `date -u`, 2026-10-07.

Before it, the block's sixteen implement-side entries read green on the final tree, in the re-entry's run of the
gate tool: entries 1 to 15 in one call (15:13Z to 15:15Z), then the census's two one-shot controls (the
must-pass control and the planted control), then entry 16 (15:15:50Z to 15:18:03Z). Their readings are in
`profraw-red-green.md`. Entries 17 to 20 are this pass. No push went out before every entry of the block read
green by its own letter (inputs#I6). No live `claude` session was started.

## Before the pass — `pre-push` (entry 15) on the uncommitted tree, 15:19:44Z to 15:20:41Z
- `bash scripts/agent-run.sh pre-push` through the gate tool (`--entry 15`): green, exit 0, 56.06 s. Its document:
  `"ok":true`, `"stage":"linux-tests"`; coverage 1712/1712 (seventeen more than the base's 1695: the nine cases of
  the rule table, the six new `send` cases and the two cross-process cases; the seventh `send` case of the filter
  is an existing one that turned), playwright 1/1, `gate` no breaches. Atoms: `exit 0` ✓, `contains "ok":true` ✓,
  `contains "stage":"linux-tests"` ✓. Load average at its start: 12.48.
- The same entry in the block's run at 15:14Z: green, 55.51 s, the same counts. No product source changed between
  the two; the files written between them are this chunk's evidence.

## Entry 17 — hygiene (by hand)
- Read at 15:20:45Z:
  `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` → exit 0,
  `hygiene: clean — read 100 (runs 85 · evidence 4 · inputs 11) · trails 23 not read · copies 9 not read by P1 — 0
  host paths kept · binary 0 not read by P1`. Atoms: `exit 0` ✓, `contains hygiene: clean` ✓. No row to rewrite.
- Read once more after this file was added, before the commit: the verdict is in the next section's first line.
