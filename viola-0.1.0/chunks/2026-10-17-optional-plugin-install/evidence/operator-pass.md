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

Read again right before the commit, after this file was first written: `hygiene: clean — read 65 (runs 49 ·
evidence 7 · inputs 9) …`.

## 3. The pre-CI commit

`43c64109ff05457d53e8c6da0c8a8a4acca86e15`: `chore(2026-10-17-optional-plugin-install): operator pre-CI commit, for the
run this chunk's verdict reads`, the whole tree. `git status --short` was empty after it.

## 4. The push (entry 20, `gate.py run … --operator 20`)

```
operator entry 20 · history tripwire: git
 20 probe       green · exit 0 · 2.33s · 90 B → 20.log · history moved: refs/remotes/origin/HEAD 9afbff44→43c64109; refs/remotes/origin/build/viola-0.1.0 9afbff44→43c64109 · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
```

## 5. The CI read (entry 21, fired as written, with the commit's full sha in place of `HEAD`: the same commit)

2026-10-18T02:25:11Z:

```
ci v1.3 · b795b622
repo Turbolet85/viola (the push remote `origin`) · polled 20× over 589 s
43c64109ff05 verdict: green · checks 15/15 · wall 586 s · runs ci#39090008812 completed/success
```

The run itself (`gh run view 39090008812`): `attempt` 1, `conclusion` `success`, `event` `push`, `headSha`
`43c64109ff05457d53e8c6da0c8a8a4acca86e15`. No fix commit was needed.
