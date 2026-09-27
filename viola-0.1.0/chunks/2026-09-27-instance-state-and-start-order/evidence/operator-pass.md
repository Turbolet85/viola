# Operator pass — 2026-09-27 (run by the session on the overseer's word)

Order: rust-analyzer stopped by exact `ExecutablePath` → entry 22 `pre-push` on the uncommitted tree → the pre-CI
commit → entry 23's guarded push → entry 24 → (CI red) a V17-form fix commit → `pre-push` again → guarded push →
entry 24 on the fix sha.

## Entry 22 — `bash scripts/agent-run.sh pre-push` (uncommitted tree)
- 2026-09-27T04:49:17Z → 05:09:04Z, **1187 s**, exit 0 (run dir `op-22.{out,err,rc,start,end,ra}`; rust-analyzer
  `left: 0` at launch).
- `ok:true`, `stage:"union"`; sync 69 files; Linux `coverage` 463/463 + gate ok; `ubuntu-latest` base a892917, 160
  tested; `windows-2025` base a892917, 160 tested; union 0 breaches; cache `scratch_bytes` 322 572 880 found and wiped.

## Pre-CI commit and entry 23
- `39b8297a0c413654f309f696b06ccd8b998e9b4f`
  `chore(2026-09-27-instance-state-and-start-order): operator pre-CI commit, for the run this chunk's verdict reads`
  (`git add -A`; tree clean after).
- `git diff --quiet && git diff --cached --quiet && git push origin build/viola-0.1.0` → exit 0,
  `a892917..39b8297` (fast-forward, no force), ~05:09:53Z.

## CI run 36296402785 on 39b8297 — RED (see `ci-red-36296402785.md`)
- First failure `mutants (ubuntu-latest)` +88 s (baseline TIMEOUT: run_cli canary scan — this chunk's, fixed);
  `test (windows-2025)` +172 s (viola-pty resize test — pre-existing, CARRY proposed). Called red by the overseer
  before the windows mutation leg finished.

## Fix commit and its push
- `ed359cdf9f1e55e5758ada2820907f8377fde321`
  `fix(2026-09-27-instance-state-and-start-order): run_cli's canary scan skips the byte-identical pinned copy,
  measured in run 36296402785`.
- `pre-push` on the committed tree: 05:24:23Z → 05:43:46Z, **1163 s**, exit 0; sync 0 files; Linux 463/463; both
  legs base a892917, 160 tested; union 0 breaches.
- Guarded push → exit 0, `39b8297..ed359cd` (fast-forward), ~05:44:02Z.

## Entry 24 — CI run 36298052174 on ed359cd (`ci`, push, conclusion success)
- `[.check_runs[] | .conclusion] | unique | join(",")` pinned to ed359cd → `success` (exit 0).
- Mutation legs, CI beside local:

  | leg | CI job | CI base | CI tested | local base / tested (fix pre-push) |
  |---|---|---|---|---|
  | `mutants (ubuntu-latest)` | 108560461887, 527 s | a892917 | 160 | a892917 / 160 |
  | `mutants (windows-2025)` | 108560461906, 1539 s | a892917 | 160 | a892917 / 160 |

- The two named tests on this run's `test (windows-2025)` (job 108560461939): viola-pty
  `spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code` PASS 0.022 s (423/464);
  `run_cli run_never_writes_a_claude_canary_anywhere` PASS 0.325 s (90/464).

## Wall-clock — local pre-push vs CI

| | local `pre-push` (Windows host + WSL2 Ubuntu) | CI |
|---|---|---|
| green, end to end | 1187 s (entry 22) · 1163 s (fix commit) | run 36298052174: **1575 s** push → last check (`mutants-verdict`); run created → updated 1573 s |
| time to first failure | none — both local runs were green on trees CI read red | run 36296402785: **+88 s** (`mutants (ubuntu-latest)`), then +172 s (`test (windows-2025)`) |
| critical path | sequential: sync → Linux coverage + gate → ubuntu leg → windows leg → union | parallel jobs; the windows mutation leg (1539 s), then `mutants-verdict` |

CI per-check completion on the green run, seconds after the push: supply-chain +29 · release ubuntu +32 · release
macos +35 · msrv +61 · release windows +63 · fuzz-replay +107 · test ubuntu +134 · lint ubuntu +144 · lint macos +145 ·
test macos +165 · test windows +195 · lint windows +226 · mutants ubuntu +533 · mutants windows +1545 ·
mutants-verdict +1575.

Reading (facts only): the local gate did not reproduce either red of run 36296402785. The canary TIMEOUT needs a
slower machine than the 32-core WSL leg (5.1 s isolated there, 10 s bound), and the windows `test` job (llvm-cov
instrumented) is not among pre-push's stages.

## Outcome
Green on the second push of the pass, after one fix commit.
