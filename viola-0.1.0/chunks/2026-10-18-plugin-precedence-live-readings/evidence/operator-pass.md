# Operator pass — 2026-10-18-plugin-precedence-live-readings

Fired by the implementer on the operator's word for this chunk: "Then the operator pass with the ci.py conclusion read
(leg=operator), and stop ready for the wrap" (the /implement invocation, 2026-10-19). It ran after the live round
closed (`evidence/live-round.md`), in `.claude/rules/ci.md`'s order. Times UTC.

## 1. The native gate (before the pre-CI commit)

`gate.py run … --only 9 --void 9:…` on the uncommitted tree, after the round's records were written (trail
`.andromeda/runs/2026-10-19T05-23-09-implement/`):

```
  9 probe       green · exit 0 · 87.73s · 379711 B → 9.2.log · void: walk-class tests/contract_fixture_hygien… (90 chars) · bash scripts/agent-run.sh pre-push
entries 12 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 11 · voided 1 (9)
```

The Windows-target lint, read on the host before the push (ci.md, 2026-10-10; the command from testing.md
2026-10-03 Ext. 2026-10-10): `CARGO_TARGET_DIR=target/wincheck cargo clippy --workspace --all-targets --features
fake-agent --target x86_64-pc-windows-msvc -- -D warnings` — exit 0, `Finished` `dev` profile.

## 2. Hygiene (entry 10, fired as written)

`python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene`:

```
hygiene: clean — read 60 (runs 48 · evidence 6 · inputs 6) · trails 16 not read · copies 4 not read by P1 — 0 host paths kept · binary 0 not read by P1
```
