# Operator pass — 2026-10-13-epoch-4-survivors

On the operator's word in this session's `/andromeda-implement` arguments: "Run the operator pass with the ci.py
conclusion read (leg=operator) as usual." Lines are copied verbatim, the tools' headers left out.

## Entry 17 re-fired before the pre-CI commit

`gate.py run … --only 17` (firing 2) on the final uncommitted tree:

```
 17 probe       green · exit 0 · 66.81s · 315160 B → 17.2.log · bash scripts/agent-run.sh pre-push
entries 20 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 19
```

The document read `"ok":true,"stage":"linux-tests"`: coverage 2431 passed / 0 failed, playwright 1 passed, gate
breaches `[]`.

## Entry 18 — hygiene, fired as written before the pre-CI commit

```
hygiene: clean — read 81 (runs 74 · evidence 2 · inputs 5) · trails 15 not read · copies 3 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

## The pre-CI commit

`chore(2026-10-13-epoch-4-survivors): operator pre-CI commit, for the run this chunk's verdict reads`, the whole tree
(`git add -A`), on `build/viola-0.1.0`. The record above rides it; the sections below are written after it and ride
the wrap commit.

`ad7fb3c1f7a8`. Its body: "hygiene: clean, nothing rewritten."

## Entry 19 — the push, fired through `gate.py run --operator 19`

```
operator entry 19 · history tripwire: git
 19 probe       green · exit 0 · 2.25s · 90 B → 19.log · history moved: refs/remotes/origin/HEAD c6e13018→ad7fb3c1; refs/remotes/origin/build/viola-0.1.0 c6e13018→ad7fb3c1 · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
entries 20 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 19
```

After it, `git rev-list --count @{u}..HEAD` read 0.

## Entry 20 — the CI read for the pre-CI commit: green

`ci.py conclusion --sha HEAD --wait 1800`, fired as written:

```
repo Turbolet85/viola (the push remote `origin`) · polled 21× over 623 s
ad7fb3c1f7a8 verdict: green · checks 15/15 · wall 610 s · runs ci#38430155254 completed/success
runs: ci#38430155254 push completed/success
checks: fuzz-replay · lint (macos-latest) · lint (ubuntu-latest) · lint (windows-2025) · msrv · perf (macos-latest)
  perf (ubuntu-latest) · perf (windows-2025) · release (macos-latest) · release (ubuntu-latest) · release (windows-2025)
  supply-chain · test (macos-latest) · test (ubuntu-latest) · test (windows-2025)
```

The new cases ran on all three OSes in the `test` jobs, the two `fs.rs` `#[cfg(windows)]` cases on `test
(windows-2025)`, and the `lint (windows-2025)` job linted the `cfg(windows)` half. No fix commit was needed.
