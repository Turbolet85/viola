# Operator pass — 2026-10-13-epoch-4-survivors-ii

Run on the operator's word, given in this session's `/andromeda-implement` arguments: "Run the operator pass with
the ci.py conclusion read (leg=operator) as usual." Lines are copied verbatim; the tools' headers are left out.

## Entry 18 re-fired before the pre-CI commit

`gate.py run … --only 18` (firing 2), on the final uncommitted tree:

```
 18 probe       green · exit 0 · 66.61s · 318472 B → 18.2.log · bash scripts/agent-run.sh pre-push
entries 21 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 20
```

The document read `"ok":true,"stage":"linux-tests"`: coverage 2452 passed / 0 failed, playwright 1 passed, gate
breaches `[]`.

## Entry 19 — hygiene, fired as written before the pre-CI commit

```
hygiene: clean — read 43 (runs 38 · evidence 2 · inputs 3) · trails 13 not read · copies 1 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

## The pre-CI commit

`chore(2026-10-13-epoch-4-survivors-ii): operator pre-CI commit, for the run this chunk's verdict reads`, made of the
whole tree (`git add -A`) on `build/viola-0.1.0`. The record above rides this commit. The sections below are written
after it and ride the wrap commit.
