# Operator pass — 2026-10-13-epoch-4-cleanup

On the operator's word in this session's `/andromeda-implement` arguments: "Run the operator pass with the ci.py
conclusion read (leg=operator) as usual." Lines are copied verbatim, the tools' headers left out.

## Entry 21 re-fired before the pre-CI commit

`gate.py run … --only 21` (firing 3, journal line 3) on the final uncommitted tree:

```
 21 probe       green · exit 0 · 67.13s · 311675 B → 21.3.log · bash scripts/agent-run.sh pre-push
entries 24 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 23
```

The document read `"ok":true,"stage":"linux-tests"`: coverage 2406 passed / 0 failed, playwright 1 passed, gate
breaches `[]`.

## Entry 22 — hygiene, fired as written before the pre-CI commit

The first read refused two files of the phase's run dir, each quoting the CI runner's scratch path as a Windows drive
path:

```
P1 .andromeda/runs/2026-10-13T07-07-28-phase/obs-history.md:9 ×1 · drive
P1 .andromeda/runs/2026-10-13T07-07-28-phase/tests-history.md:5 ×1 · drive
hygiene: refused 2 files — P1 2 · P2 0 · P3 0 · read 46 (runs 40 · evidence 2 · inputs 4) · trails 12 not read · copies 2 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

Each quoted path was rewritten as `<repo parent>/viola-mutants-scratch` on the runner (the spelling plan.md and
scope.md use), nothing else in either line changed. The re-read:

```
hygiene: clean — read 46 (runs 40 · evidence 2 · inputs 4) · trails 12 not read · copies 2 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

## The pre-CI commit

`chore(2026-10-13-epoch-4-cleanup): operator pre-CI commit, for the run this chunk's verdict reads`, the whole tree
(`git add -A`), on `build/viola-0.1.0`. The record above rides it; the sections below are written after it and ride
the wrap commit.
