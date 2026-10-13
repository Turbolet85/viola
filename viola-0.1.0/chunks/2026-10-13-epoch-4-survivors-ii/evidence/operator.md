# Operator pass — 2026-10-13-epoch-4-survivors-ii

Run on the operator's word, given in this session's `/andromeda-implement` arguments: "Run the operator pass with
the ci.py conclusion read (leg=operator) as usual." Lines are copied verbatim; the tools' headers are left out.

## Entry 18 re-fired before the pre-CI commit

`gate.py run … --only 18` (firing 2), on the final uncommitted tree:

```
 18 probe       green · exit 0 · 66.61s · 318472 B → 18.2.log · bash scripts/agent-run.sh pre-push
entries 21 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 20
```

The document read `"ok":true,"stage":"linux-tests"`: coverage 2452 passed / 0 failed, playwright 1 passed, gate
breaches `[]`.

## Entry 19 — hygiene, fired as written before the pre-CI commit

```
hygiene: clean — read 43 (runs 38 · evidence 2 · inputs 3) · trails 13 not read · copies 1 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

## The pre-CI commit

`chore(2026-10-13-epoch-4-survivors-ii): operator pre-CI commit, for the run this chunk's verdict reads`, made of the
whole tree (`git add -A`) on `build/viola-0.1.0`. The record above rides this commit. The sections below are written
after it and ride the wrap commit.

`94f0be906ceb`. Its body: "hygiene: clean, nothing rewritten."

## Entry 20 — the push, fired through `gate.py run --operator 20`

```
operator entry 20 · history tripwire: git
 20 probe       green · exit 0 · 2.1s · 90 B → 20.log · history moved: refs/remotes/origin/HEAD 64d7e5f7→94f0be90; refs/remotes/origin/build/viola-0.1.0 64d7e5f7→94f0be90 · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
entries 21 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 20
```

After it, `git rev-list --count @{u}..HEAD` read 0.

## Entry 21 — the CI read for the pre-CI commit: green

`ci.py conclusion --sha HEAD --wait 1800`, fired as written:

```
repo Turbolet85/viola (the push remote `origin`) · polled 19× over 561 s
94f0be906ceb verdict: green · checks 15/15 · wall 559 s · runs ci#38442894573 completed/success
runs: ci#38442894573 push completed/success
checks: fuzz-replay · lint (macos-latest) · lint (ubuntu-latest) · lint (windows-2025) · msrv · perf (macos-latest)
  perf (ubuntu-latest) · perf (windows-2025) · release (macos-latest) · release (ubuntu-latest) · release (windows-2025)
  supply-chain · test (macos-latest) · test (ubuntu-latest) · test (windows-2025)
```

The new cases and `cli_verify` ran on all three OSes in the `test` jobs. The `lint (windows-2025)` job linted the
generic `SharedPty` and the test double on the Windows side. No fix commit was needed.
