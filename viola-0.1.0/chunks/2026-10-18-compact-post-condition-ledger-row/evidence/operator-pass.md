# Operator pass — 2026-10-18-compact-post-condition-ledger-row

Run on the operator's word (`inputs#I5`; the two scope additions accepted, `inputs#I6`), after the implement run's
block read `entries 26 · green 17 · red 1 (15)` with `pre-push` (entry 23) green in 84.43 s on this tree's source.

## Entry 24 — hygiene (fired as written)
First read: `hygiene: refused 1 files — P1 1 · P2 0 · P3 0 …` — `evidence/live-round.md:70 ×1 · tmp` (a `/tmp/…` path);
the line was rewritten to `$TMPDIR/…`. Second read:
```
hygiene: clean — read 60 (runs 50 · evidence 2 · inputs 8) · trails 15 not read · copies 6 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

## The pre-CI commit
`3d5ae39` — `chore(2026-10-18-compact-post-condition-ledger-row): operator pre-CI commit, for the run this chunk's
verdict reads` (the whole tree, as earlier pre-CI commits; the tree clean after it).

## Entry 25 — the push (`gate.py run --plan … --operator 25`)
```
operator entry 25 · history tripwire: git
 25 probe       green · exit 0 · 2.24s · 90 B → 25.log · history moved: refs/remotes/origin/HEAD 7f35c8ec→3d5ae39a; refs/remotes/origin/build/viola-0.1.0 7f35c8ec→3d5ae39a · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
entries 26 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 25
```
The history move is the intended push of `3d5ae39`.

## Entry 26 — the CI read (fired as written)
```
3d5ae39ac935 verdict: green · checks 15/15 · wall 658 s · runs ci#39169321430 completed/success
runs: ci#39169321430 push completed/success
checks: fuzz-replay · lint (macos-latest) · lint (ubuntu-latest) · lint (windows-2025) · msrv · perf (macos-latest)
  perf (ubuntu-latest) · perf (windows-2025) · release (macos-latest) · release (ubuntu-latest) · release (windows-2025)
  supply-chain · test (macos-latest) · test (ubuntu-latest) · test (windows-2025)
```
