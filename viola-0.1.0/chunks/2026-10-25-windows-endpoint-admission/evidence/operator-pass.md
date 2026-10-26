# Operator pass — 2026-10-25-windows-endpoint-admission

Run by the session on the operator's word at invocation (2026-10-25T23:49Z): «run 2026-10-25-windows-endpoint-admission
through its operator pass and the CI read (supply-chain red is expected, owned by the tokio lock-bump entry; the
windows-2025 leg is this chunk's witness).»

## Entry 21 — hygiene, fired as written

`python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene`, last read 2026-10-26T00:03Z,
right before the pre-CI commit (an earlier read at 00:01Z read the same):

```
control: P1 drive · P1 device · P1 home · P1 users · P1 root · P1 msys · P1 wsl · P1 tmp · P1 in-root home · P1 in-root drive · P2 · P3 rust — each fired on its synthetic known positive
hygiene: clean — read 427 (runs 416 · evidence 3 · inputs 8) · trails 13 not read · copies 6 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

This record is written after that read (plan-template §Test Commands, the `leg` key).

## pre-push, re-run before the push

`gate.py run --plan … --entry 20`, no run dir, 2026-10-26T00:01Z:

```
 20 probe       green · exit 0 · 100.13s · 392516 B → 20.log · bash scripts/agent-run.sh pre-push
entries 24 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 23
```
