# Operator pass — 2026-10-27-pinned-copy-race-and-reproducible-builds

Run on the operator's own request in the implement session (2026-10-27, «through its operator pass and the CI
read»), the entries fired in their plan's operator forms; lines copied verbatim, headers left out.

## Entry 23 — hygiene, fired as written, before the pre-CI commit

First firing (2026-10-27T02:31Z):

```
P1 .andromeda/runs/2026-10-27T02-08-46-implement/journal.md:9 ×1 · tmp
hygiene: refused 1 files — P1 1 · P2 0 · P3 0 · read 528 (runs 513 · evidence 8 · inputs 7) · trails 14 not read · copies 5 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

The journal's line 9 named step 3's scratch by its temp-dir path; rewritten as «a `viola-race-*` dir under the OS
temp dir». Re-fired:

```
hygiene: clean — read 528 (runs 513 · evidence 8 · inputs 7) · trails 14 not read · copies 5 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

## Entry 22 — pre-push, re-fired before the push (its note)

```
 22 probe       green · exit 0 · 100.64s · 422769 B → 22.2.log · bash scripts/agent-run.sh pre-push
entries 25 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 24
```
