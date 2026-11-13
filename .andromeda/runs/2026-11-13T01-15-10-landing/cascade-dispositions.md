# Cascade dispositions — the rows landing of 2026-11-12T03-23-49 (tx 803)

No step-2 sweep: the landing retires every `section` and `fact` of the masters and lands their rows in one TX; the
leaves run once, in the WHOLE mode (`rows-landing.md` §7).

## Leaves

Set: `cascade.py leaves --all`, fired once — `total · whole · 15 group(s) · 27 leaves · 419656 B · unmapped 2`.
Batch: `leaf`, ONE Workflow, 15 comparers — `VERDICT: 15 of 15 pass · failed: none`.
Apply: `total · blocks 40 · applied 40`. Every finding's source row read in the store at tx 803 (36 distinct rows,
each standing); the two `ci.md` findings rest also on `.andromeda/security-plan-detail/` and
`.andromeda/obs-plan-detail/` standing no more.

| group | mode | findings | applied | by hand | declined |
|---|---|---|---|---|---|
| a11y-plan | whole | 1 | 1 | 0 | 0 |
| architecture-1 | whole | 8 | 8 | 0 | 0 |
| architecture-2 | whole | 3 | 3 | 0 | 0 |
| architecture-3 | whole | 2 | 2 | 0 | 0 |
| architecture-4 | whole | 1 | 1 | 0 | 0 |
| architecture+design-system+layout-templates | whole | 1 | 1 | 0 | 0 |
| architecture+security-plan | whole | 2 | 2 | 0 | 0 |
| architecture+test-plan+obs-plan | whole | 1 | 1 | 0 | 0 |
| design-system+layout-templates | whole | 3 | 3 | 0 | 0 |
| masters | whole | 4 | 4 | 0 | 0 |
| obs-plan | whole | 3 | 3 | 0 | 0 |
| security-plan | whole | 4 | 4 | 0 | 0 |
| security-plan+obs-plan | whole | 2 | 2 | 0 | 0 |
| test-plan | whole | 4 | 4 | 0 | 0 |
| test-plan+obs-plan | whole | 1 | 1 | 0 | 0 |

Hygiene: `staged-leaf-.claude__docs__gotchas.md` (the apply's staged bytes, equal to `.claude/docs/gotchas.md`)
was refused P1 at its line 38 — the retired Git Bash example `C:/Program Files/…` the leaf already carries at HEAD —
and moved out of this dir to the session scratchpad; the leaf itself stands as written, its bytes in the landing's
journal file.

Unmapped, no comparer:
- `.claude/rules/host-linux.md` (host) — the host's own leaf: a statement no master states, no step-3 edit.
- `.claude/rules/testing-src.md` — a pointer leaf: each line names a section of `.claude/rules/testing.md` and repeats
  none of its text, so no master statement stands in it to compare.
