# Operator pass — 2026-10-26-tokio-rustsec-2026-0386-lock-bump

Fired by the implementer on the operator's word for this chunk (`inputs#I6`: "please run implement per the skill on
2026-10-26-tokio-rustsec-2026-0386-lock-bump, through its operator pass and the CI read; …" — the operator, live,
2026-10-26). Order per `.claude/rules/ci.md` §The operator pass. Lines copied verbatim, without the tool header.

## 1. The native gate (plan entry 24, on the uncommitted tree)
`python -X utf8 {tools_dir}/gate.py run --plan viola-0.1.0/chunks/2026-10-26-tokio-rustsec-2026-0386-lock-bump/plan.md --entry 24`
```
 24 probe       green · exit 0 · 88.05s · 392253 B → 24.log · bash scripts/agent-run.sh pre-push
entries 27 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 26
```
The Windows-target lint (ci.md 2026-10-10) read green on the host in the implement block, entry 4.

## 2. Hygiene (plan entry 25, fired as written)
`python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` — exit 0
```
hygiene: clean — read 356 (runs 349 · evidence 0 · inputs 7) · trails 16 not read · copies 5 not read by P1 — 0 host paths kept · binary 0 not read by P1
```
Re-read right before the commit: below.
