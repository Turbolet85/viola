# Operator pass — 2026-10-17-optional-plugin-install

Fired by the implementer on the operator's word for this chunk: "Then run the operator pass with the ci.py
conclusion read (leg=operator) as usual, and stop ready for the wrap" (the /implement invocation, `inputs#I4`). It
waited for the live round (`inputs#I5` Q2) and ran once the round was closed as it stands (`inputs#I7`), in
`.claude/rules/ci.md`'s order. Times UTC.

## 1. The native gate (before the pre-CI commit)

`gate.py run … --only 3,18` on the uncommitted tree, 2026-10-18T02:23:04Z (trail
`.andromeda/runs/2026-10-17T18-08-07-implement/`):

```
  3 lint        green · exit 0 · 1.63s · 134 B → 3.2.log · CARGO_TARGET_DIR=target/wincheck cargo clippy --workspace … (140 chars)
 18 probe       green · exit 0 · 82.87s · 377351 B → 18.2.log · bash scripts/agent-run.sh pre-push
```

The Windows-target lint was read on the host before the push (ci.md, 2026-10-10). pre-push read `"ok":true` and
`"stage":"linux-tests"`; the coverage gate's `breaches` was `[]`.

## 2. Hygiene (entry 19, fired as written)

`python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene`:

```
hygiene: clean — read 64 (runs 49 · evidence 6 · inputs 9) · trails 15 not read · copies 7 not read by P1 — 0 host paths kept · binary 0 not read by P1
```
