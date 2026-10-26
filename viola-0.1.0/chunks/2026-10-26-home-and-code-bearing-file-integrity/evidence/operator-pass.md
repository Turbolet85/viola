# Operator pass — 2026-10-26-home-and-code-bearing-file-integrity

Run on the operator's own request in the implement session (2026-10-26, «through its operator pass and the CI
read»), the entries fired in their plan's operator forms; lines copied verbatim, headers left out.

## Entry 24 — hygiene, fired as written, before the pre-CI commit

First firing (2026-10-26T20:43Z):

```
P1 .andromeda/runs/2026-10-26T19-02-07-implement/journal.md:12 ×1 · tmp
hygiene: refused 1 files — P1 1 · P2 0 · P3 0 · read 538 (runs 532 · evidence 1 · inputs 5) · trails 16 not read · copies 3 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

The journal's line 12 named the e2e-home link's temp-dir target; rewritten as «the owner-only
`viola-e2e-home-<uid>` dir under the temp dir». Re-fired:

```
hygiene: clean — read 538 (runs 532 · evidence 1 · inputs 5) · trails 16 not read · copies 3 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

## Entry 23 — pre-push, re-fired before the push (its note)

```
 23 probe       green · exit 0 · 101.9s · 421949 B → 23.4.log · bash scripts/agent-run.sh pre-push
entries 26 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 25
```

## Entry 25 — the push, and entry 26 — the CI read

Recorded below once fired.
