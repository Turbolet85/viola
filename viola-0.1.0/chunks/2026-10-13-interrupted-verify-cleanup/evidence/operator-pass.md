# Operator pass — 2026-10-13-interrupted-verify-cleanup

Fired by the implementer on the operator's word in the `/andromeda-implement` invocation ("Run the operator pass with
the ci.py conclusion read (leg=operator) as usual"), in `.claude/rules/ci.md`'s order. Times are the run's
`journal.md` lines (`.andromeda/runs/2026-10-13T18-48-52-implement/journal.md`).

## 1. The native gate (entry 17), on the final uncommitted tree
```
 17 probe       green · exit 0 · 66.18s · 320761 B → 17.4.log · bash scripts/agent-run.sh pre-push
entries 20 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 19
```
The Windows-target lint (entry 3) read green on the same code in the final whole-block run (run 4).

## 2. Hygiene (entry 18, fired as written)
```
control: P1 drive · P1 device · P1 home · P1 users · P1 root · P1 msys · P1 wsl · P1 tmp · P1 in-root home · P1 in-root drive · P2 · P3 rust · P3 ts — each fired on its synthetic known positive
hygiene: clean — read 48 (runs 42 · evidence 1 · inputs 5) · trails 13 not read · copies 3 not read by P1 — 0 host paths kept · binary 0 not read by P1
```
