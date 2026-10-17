# Operator pass — entries 20, 21, 22 (`leg = 'operator'`)

Run on the operator's word in the /andromeda-implement invocation ("Run the operator pass with the ci.py conclusion
read (leg=operator); stop ready for the wrap"), 2026-10-17. Lines copied verbatim, headers left out.

**Before the pre-CI commit — entry 19 re-run on the final uncommitted tree** (`gate.py run … --only 19`, started
01:44:14Z):

```
 19 probe       green · exit 0 · 72.99s · 346879 B → 19.3.log · bash scripts/agent-run.sh pre-push
entries 22 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 21
```

**Entry 20 — `gate.py hygiene`, fired as written:**

```
hygiene: clean — read 56 (runs 49 · evidence 3 · inputs 4) · trails 16 not read · copies 2 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

**Pre-CI commit:** `ae21376` — `chore(2026-10-17-end-to-end-path-binary-and-harness-legs): operator pre-CI commit, for
the run this chunk's verdict reads` (`git add -A`; the tree read 0 changes after it).

**Entry 21 — the push, `gate.py run --plan … --operator 21`** (01:45:55Z):

```
operator entry 21 · history tripwire: git
 21 probe       green · exit 0 · 2.92s · 90 B → 21.log · history moved: refs/remotes/origin/HEAD 6042af63→ae213762; refs/remotes/origin/build/viola-0.1.0 6042af63→ae213762 · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
entries 22 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 21
```

The history move is the one intended (the pre-CI commit onto `origin/build/viola-0.1.0`).

**Entry 22 — `ci.py conclusion --sha HEAD --wait 1800`, fired as written** (started 01:46:05Z):

```
ci v1.3 · c2400620
repo Turbolet85/viola (the push remote `origin`) · polled 19× over 559 s
ae2137622b3a verdict: green · checks 15/15 · wall 549 s · runs ci#38992234808 completed/success
runs: ci#38992234808 push completed/success
checks: fuzz-replay · lint (macos-latest) · lint (ubuntu-latest) · lint (windows-2025) · msrv · perf (macos-latest)
  perf (ubuntu-latest) · perf (windows-2025) · release (macos-latest) · release (ubuntu-latest) · release (windows-2025)
  supply-chain · test (macos-latest) · test (ubuntu-latest) · test (windows-2025)
```

`v1-02`'s CI witness: the `test` job on `macos-latest`, `ubuntu-latest` and `windows-2025` (its `run --coverage` and
the ci-smoke harness lifecycle `boot → status → logs → cleanup ×2`) passed in ci#38992234808 on `ae2137622b3a`. The
watched `cli_send` case's CI reading per runner is in that run's job logs, not read here.
