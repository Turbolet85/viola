# Operator pass — 2026-10-26-server-verification-before-any-frame

Fired by the implementer on the operator's word for this chunk (`inputs#I6`: "please run implement per the skill on
2026-10-26-server-verification-before-any-frame, through its operator pass and the CI read. …" — the operator, live,
2026-10-26). Order per `.claude/rules/ci.md` §The operator pass. Lines copied verbatim, without the tool header.

## 1. The native gate (plan entry 21, on the uncommitted tree)
`python -X utf8 {tools_dir}/gate.py run --plan viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/plan.md --entry 21`
```
 21 probe       green · exit 0 · 87.67s · 399328 B → 21.log · bash scripts/agent-run.sh pre-push
entries 24 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 23
```
The Windows-target lint (ci.md 2026-10-10) read green on the host in the implement block, run 3, entry 4; no source
file changed after it.

## 2. Hygiene (plan entry 22, fired as written)
`python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` — exit 0
```
hygiene: clean — read 459 (runs 447 · evidence 4 · inputs 8) · trails 15 not read · copies 6 not read by P1 — 0 host paths kept · binary 0 not read by P1
```
