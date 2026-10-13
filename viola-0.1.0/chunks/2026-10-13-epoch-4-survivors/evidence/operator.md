# Operator pass — 2026-10-13-epoch-4-survivors

On the operator's word in this session's `/andromeda-implement` arguments: "Run the operator pass with the ci.py
conclusion read (leg=operator) as usual." Lines are copied verbatim, the tools' headers left out.

## Entry 17 re-fired before the pre-CI commit

`gate.py run … --only 17` (firing 2) on the final uncommitted tree:

```
 17 probe       green · exit 0 · 66.81s · 315160 B → 17.2.log · bash scripts/agent-run.sh pre-push
entries 20 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 19
```

The document read `"ok":true,"stage":"linux-tests"`: coverage 2431 passed / 0 failed, playwright 1 passed, gate
breaches `[]`.

## Entry 18 — hygiene, fired as written before the pre-CI commit

```
hygiene: clean — read 81 (runs 74 · evidence 2 · inputs 5) · trails 15 not read · copies 3 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

## The pre-CI commit

`chore(2026-10-13-epoch-4-survivors): operator pre-CI commit, for the run this chunk's verdict reads`, the whole tree
(`git add -A`), on `build/viola-0.1.0`. The record above rides it; the sections below are written after it and ride
the wrap commit.
