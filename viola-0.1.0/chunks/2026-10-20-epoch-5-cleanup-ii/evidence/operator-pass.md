# Operator pass — 2026-10-20-epoch-5-cleanup-ii

The builder session ran this pass on the operator's word, given at the `/andromeda-implement` invocation (`inputs#I4`,
2026-10-20T15:48Z): «Then the operator pass as usual; stop before the wrap.» It follows implement's run
(`.andromeda/runs/2026-10-20T15-48-36-implement/`), whose outcome is green on the dialog-15 answers (`inputs#I5`).

## `pre-push` on the uncommitted tree (entry 27)

It fired in implement's last block firing, on the uncommitted tree, after the last code edit. Only evidence, the outcome
file and two phase-run respells were written after it:
`27 probe       green · exit 0 · 96.78s · 392118 B → 27.2.log · bash scripts/agent-run.sh pre-push`.

## Entry 28 — `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene`

Fired as written on the uncommitted tree, before 2026-10-20T18:42:59Z (the `date -u` read right after it).

- **First firing:**
  ```
  P1 .andromeda/runs/2026-10-20T10-26-22-phase/p5-card-1.md:75 ×3 · tmp
  P1 .andromeda/runs/2026-10-20T10-26-22-phase/p5-prepush-red.md:10 ×3 · tmp
  hygiene: refused 2 files — P1 2 · P2 0 · P3 0 · read 63 (runs 48 · evidence 8 · inputs 7) · trails 14 not read · copies 5 not read by P1 — 0 host paths kept · binary 0 not read by P1
  ```
- **The rewrite (the letter's, §Hygiene):** the 3 + 3 temp-dir paths in the phase run's two P5 records were respelled
  `/tmp/<segment>` → `$TMPDIR/<segment>`, the run-dir spelling of the OS temp dir. Nothing else in those files
  changed. The bare `/tmp` mount mentions do not fire and stay as written.
- **Second firing:**
  `hygiene: clean — read 63 (runs 48 · evidence 8 · inputs 7) · trails 14 not read · copies 5 not read by P1 — 0 host paths kept · binary 0 not read by P1`.
