# Operator pass — 2026-10-16-remove-the-browser-leg

Fired by the implementer on the operator's word for this chunk (2026-10-16: "Run the operator pass with the ci.py
conclusion read (leg=operator) as usual; stop ready for the wrap", and again after the founder removed `e2e-web/`).

## 1. The native gate
Entry 22, `bash scripts/agent-run.sh pre-push`, third firing (the block's second whole firing, after the last code
edit; the tree has taken no code edit since):
`22 probe       green · exit 0 · 75.64s · 330057 B → 22.3.log · bash scripts/agent-run.sh pre-push`
Atoms: `exit 0` ✓ · `contains "ok":true` ✓ · `contains "stage":"linux-tests"` ✓.
The Windows-target lint (`.claude/rules/ci.md`, 2026-10-10), entry 3 of the same firing:
`3 lint        green · exit 0 · 0.6s · 155 B → 3.2.log · CARGO_TARGET_DIR=target/wincheck cargo clippy --workspace …`

## 2. Hygiene (entry 23, fired as written)
`python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` — exit 0:
`hygiene: clean — read 41 (runs 38 · evidence 0 · inputs 3) · trails 12 not read · copies 1 not read by P1 — 0 host paths kept · binary 0 not read by P1`
