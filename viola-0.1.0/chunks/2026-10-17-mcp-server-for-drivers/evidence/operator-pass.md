# Operator pass — 2026-10-17-mcp-server-for-drivers

Fired by the implementer on the operator's word for this chunk ("Then run the operator pass with the ci.py
conclusion read (leg=operator) as usual, and stop ready for the wrap", the /implement invocation of 2026-10-17),
in `.claude/rules/ci.md`'s order. Times UTC.

## 1. The native gate (before the pre-CI commit)

`gate.py run … --only 3,23` on the uncommitted tree (trail `.andromeda/runs/2026-10-17T11-33-11-implement/`):

```
  3 lint        green · exit 0 · 0.22s · 72 B → 3.6.log · CARGO_TARGET_DIR=target/wincheck cargo clippy --workspace … (140 chars)
 23 probe       green · exit 0 · 86.88s · 373768 B → 23.6.log · bash scripts/agent-run.sh pre-push
```

The Windows-target lint read on the host before the push (ci.md, 2026-10-10). pre-push: `"ok":true`,
`"stage":"linux-tests"`, coverage gate `breaches []`.

## 2. Hygiene (entry 24, fired as written)

`python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` — first read
`hygiene: refused 1 files — P1 1` (a `/tmp/…` spelling in this run's outcome file, line 84), respelled by hand; then:

```
hygiene: clean — read 57 (runs 45 · evidence 7 · inputs 5) · trails 14 not read · copies 3 not read by P1 — 0 host paths kept · binary 0 not read by P1
```
