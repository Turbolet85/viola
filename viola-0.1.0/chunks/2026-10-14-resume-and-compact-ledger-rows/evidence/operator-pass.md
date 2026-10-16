# Operator pass — 2026-10-14-resume-and-compact-ledger-rows

This pass was fired by the session on the operator's word: the `/andromeda-implement` invocation arguments,
2026-10-16, «Then the operator pass with the ci.py conclusion read (leg=operator). Stop ready for the wrap».

## Entry 24 — hygiene, fired as written
The run: `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene`.

**First reading.** The verdict was refused:

```
P1 .andromeda/runs/2026-10-14T07-11-44-phase/p5-card-1.md:74 ×1 · tmp
P1 .andromeda/runs/2026-10-16T06-46-40-implement/journal.md:152 ×1 · tmp
P1 viola-0.1.0/chunks/2026-10-14-resume-and-compact-ledger-rows/evidence/live-round.md:73 ×1 · tmp
hygiene: refused 3 files — P1 3 · P2 0 · P3 0 · read 77 (runs 68 · evidence 3 · inputs 6) · trails 23 not read · copies 4 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

**The rewrite.** Each hit was the glob for Run A's OS-temp dir, written in prose. No real host path was involved. In
each of the three files that one token was rewritten to the placeholder form `/tmp/viola-verify-<suffix>`, and
nothing else changed.

**Second reading.** The verdict was clean:

```
hygiene: clean — read 77 (runs 68 · evidence 3 · inputs 6) · trails 23 not read · copies 4 not read by P1 — 0 host paths kept · binary 0 not read by P1
```
