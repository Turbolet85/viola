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

Read again right before the commit, after this file was first written: `hygiene: clean — read 61 (runs 48 · evidence 7 ·
inputs 6) …`.

## 3. The pre-CI commit

`2a3d569123b1e464e64c95193e07e0d27a305417`: `chore(2026-10-18-plugin-precedence-live-readings): operator pre-CI commit,
for the run this chunk's verdict reads`, the whole tree. `git status --short` was empty after it.

## 4. The push (entry 11, `gate.py run … --operator 11`)

```
operator entry 11 · history tripwire: git
 11 probe       green · exit 0 · 2.15s · 92 B → 11.log · history moved: refs/remotes/origin/HEAD bc2e3bf7→2a3d5691; refs/remotes/origin/build/viola-0.1.0 bc2e3bf7→2a3d5691 · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
```

## 5. The CI read (entry 12, fired as written, with the commit's full sha in place of `HEAD`: the same commit)

2026-10-19T06:35Z:

```
ci v1.3 · 2b582a9b
repo Turbolet85/viola (the push remote `origin`) · polled 23× over 685 s
2a3d569123b1 verdict: green · checks 15/15 · wall 654 s · runs ci#39209618442 completed/success
```

The run itself (`gh run list --commit <full sha>`): `attempt` 1, `conclusion` `success`, `event` `push`, `headSha`
`2a3d569123b1e464e64c95193e07e0d27a305417`. No fix commit was needed. This file's steps 3–5 were written after the
push (ci.md 2026-10-13), so they ride the wrap commit.
