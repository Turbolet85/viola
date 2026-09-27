# Operator pass — 2026-09-27 (run by the session on the overseer's word)

Order as the plan's step 9 wires it: rust-analyzer stopped by exact `ExecutablePath` → entry 17 `pre-push` on the
uncommitted tree → the pre-CI commit → entry 18's guarded push → entries 19–20 once the run completed.

## Entry 17 — `bash scripts/agent-run.sh pre-push` (uncommitted tree)
- 2026-09-27T00:12:12Z → 00:26:24Z, **851 s**, exit 0 (run dir `op-17.{out,err,rc,start}`).
- `ok:true`, `stage:"union"`; sync 166 files; linux-tests `coverage` 401/401; `ubuntu-latest` leg base
  `a69c5efb082f1d04f067073a5638618fd709be7e`, 87 tested, ok; `windows-2025` leg base a69c5ef, 87 tested, ok; union
  `ok:true`, 0 breaches.

## Pre-CI commit and entry 18 — the push
- `054ebe48a7f777e258542ba4004fdc23e13cb9ff`
  `chore(2026-09-26-local-linux-pre-push-gate): operator pre-CI commit, for the run this chunk's verdict reads`
  (`git add -A`, the previous chunk's precedent; tree clean after).
- `git diff --quiet && git diff --cached --quiet && git push origin build/viola-0.1.0` → exit 0,
  `a69c5ef..054ebe4  build/viola-0.1.0 -> build/viola-0.1.0` (fast-forward, no force); pushed at 2026-09-27T00:26:45Z
  (`op-18.pushed-at`).

## Entries 19–20 — CI run 36282518379 on 054ebe4 (`ci`, push, conclusion success)
- Entry 19 (`[.check_runs[] | .conclusion] | unique | join(",")`, pinned to the pushed sha): `success`.
- Entry 20 (the two `mutants` legs and `mutants-verdict`): `success,success,success`.
- **Base, CI beside local:**

  | leg | CI base (job log, the harness's run document) | local base (entry 17) | tested (CI / local) |
  |---|---|---|---|
  | `mutants (ubuntu-latest)` (job 108516960075) | `a69c5efb082f1d04f067073a5638618fd709be7e` | a69c5ef | 87 / 87 |
  | `mutants (windows-2025)` (job 108516959964) | `a69c5efb082f1d04f067073a5638618fd709be7e` | a69c5ef | 87 / 87 |

  Equal on both legs: the uncommitted-promotion base rule made the pre-commit local legs derive the flip CI derives
  after the commit.

## Wall-clock — local pre-push vs CI (overseer1's fast-feedback slot, first live measurement)
Measured from the tools' own timestamps (`op-ci-measure.json`: `gh api …/check-runs` and `…/actions/runs`).

| | local `pre-push` (this host: Windows + WSL2 Ubuntu, 32 cores) | CI run 36282518379 |
|---|---|---|
| green, end to end | 846.2 s · 850.2 s · 852.2 s (gate entry 13, three consecutive) · 851 s (entry 17) | 1301 s push → last check (run created → updated: 1296 s) |
| critical path | sequential: sync → Linux `run --coverage` + gate → ubuntu leg → windows leg → union | parallel jobs; the `mutants (windows-2025)` leg, 1261 s, then `mutants-verdict` 27 s |
| time to first failure | none on the green runs; on the earlier reds: **71.43 s** (cold, first clone + first instrumented build) and **33.42 s** (warm), both stopping fail-fast at `linux-tests` | none (all 15 success) |

CI's per-check completion, seconds after the push: release ubuntu +23 · supply-chain +36 · release macos +42 ·
msrv +61 · release windows +63 · fuzz-replay +109 · **test (ubuntu-latest) +115** · lint ubuntu +139 · lint macos +143 ·
test macos +167 · test windows +188 · lint windows +209 · **mutants (ubuntu-latest) +478** · **mutants (windows-2025)
+1270** · mutants-verdict +1301.

Reading (facts only, no design claim): the local gate's first Unix test verdict comes ~33–71 s after launch, against
CI's `test (ubuntu-latest)` at +115 s; its full verdict (both legs and the union) comes at ~850 s, against CI's 1301 s,
whose tail is the windows mutation leg. The local run pays the windows leg on the same host after the ubuntu leg.

## Outcome
Green on the first push of the pass — no fix commit was needed.
