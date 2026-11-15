# P5 review — «Mutation and coverage baselines green» (`2026-11-15-mutation-and-coverage-baselines-green`)

**The plan to open:** `.andromeda/cache/docs/2026-11-15-mutation-and-coverage-baselines-green/plan.md` (rev 4, 420
lines). No plan stands in a chunk's folder. Writing in that file changes no record until a `put`, and
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-15-mutation-and-coverage-baselines-green`
diffs the chunk's records.

## Goal
Make the next boundary's audit able to read a green coverage suite and a green `viola` mutation baseline under the
scratch. The work is test code and test config only, with no mutation run, no coverage verdict and no live start
(`inputs#I1`).

## What the plan does (11 steps)
1. The impostor's socket moves under the shared `SocketRoot` (`/tmp/vs-*`); its path no longer depends on `TMPDIR`.
   The registered name `viola-test-chan-<pid>-<label>` is kept. A test asserts ≤ 107 B at pid 4194303 with the
   longest label (62 B expected).
2. The `mutants` default kill goes 10 → 15 s and `WITHIN` 7 → 12 s (your answer, `inputs#I2` Q1). The plan states
   the departure from the 2026-10-20 «never by raising the default» line, and why, in words the wrap can amend from.
   It carries a planted-hang pair (16 s plant: TIMEOUT under `mutants`, PASS under default) and a cold parallel
   baseline measured against 15 s with the load beside it. A cold case past 15 s is named, never raised again.
3. The ten hypr-tap survivors are each killed by a test in `viola-e2e` (`tests/hypr_tap.rs` against a planted stub
   `hyprctl` replaying the committed 0.56.2 replies, plus one inline `fault` case). Each kill has a `gate.py control`.
4. `TestHome::for_record()` / `record_home()` names the `--record` home `vhome` and refuses a `home` component. The
   six `--record` callers move onto it (`inputs#I2` Q2).
5. 13 one-shot controls in all (items 1, 3, 4 ×10, 5), each case by its name alone; «holds» = every case `failed`.

## Top acceptance criteria
- Under `TMPDIR=~/dev/projects/viola-mutants-scratch/s2` every `hook_fail_open` case passes; the bound test holds; item
  1's control holds.
- Kill 15 s, `WITHIN` 12 s, the kill-line census green, the planted-hang pair recorded.
- The warm suite passes under `--profile mutants`. The cold baseline and the M2 case's instrumented time are recorded
  in `evidence/timings.md` (report-only).
- The ten hypr-tap controls hold; the record-home control holds; entry 15 reads no product source changed.
- CI green on the operator pass's final HEAD.

## Touchpoints
New `crates/viola-e2e/tests/hypr_tap.rs`. Modified: `tests/hook_fail_open.rs`, `tests/support/home.rs`,
`tests/support/watch.rs`, `.config/nextest.toml`, `tests/cli_verify.rs`, `tests/cli_verify/runs.rs`,
`tests/cli_verify/stamps.rs`, `crates/viola-e2e/src/bin/viola-hypr-tap.rs` (an inline test case only).

## Test commands (20 entries)
- Lint and check: fmt, clippy, check.
- Targeted runs: `hook_fail_open` plain and under the scratch `TMPDIR`; the bound test by name; `contract_lints`;
  two line probes; `viola-e2e`'s hypr-tap tests; `cli_verify`.
- The warm `mutants`-profile suite with the load beside it.
- Two operator measurements, each fired once by /implement: the cold baseline and the M2 instrumented case.
- The product-source guard, `run --unit`, pre-push.
- The operator pass: hygiene, push, CI.
- Baselines on the untouched tree: 10 entries green. Reds the plan predicts: entries 5, 6, 8, 9, 10 (the absent tests
  and lines, and `case_6`'s `sun_path` bind).

## Concretization preview
None: no capability is claimed (the carry claims none and adds no requirement; the pool's 25 planned caps are other
entries').

## Warnings
None. 0 of 7 masters read `picked 0`; planlint 0 hits.

## The directive's item-7 block (verbatim where a tool printed it)
- Union, per master:
  `architecture: rows 938 · packs 10 · picked 96 (37 by every draw) · standing 15 · 32635 B`
  `security-plan: rows 597 · packs 6 · picked 71 (35 by every draw) · standing 12 · 21095 B`
  `design-system: rows 313 · packs 4 · picked 17 (11 by every draw) · standing 0 · 6368 B`
  `layout-templates: rows 221 · packs 3 · picked 13 (5 by every draw) · standing 0 · 3012 B`
  `test-plan: rows 1326 · packs 14 · picked 165 (66 by every draw) · standing 36 · 42508 B`
  `obs-plan: rows 1021 · packs 11 · picked 98 (31 by every draw) · standing 2 · 28325 B`
  `a11y-plan: rows 235 · packs 3 · picked 20 (15 by every draw) · standing 4 · 7360 B`
  `rows-union: 7 master(s) · picked 480 of 4651 · rule any`
- The rows files were read WHOLE: their sum is 141 303 B, under the 250 000 B rule. Every rows, standing and history
  file was read once, at P2's end.
- Every row the plan cites stands in its master's rows file or standing list (read against each citation at P5).
- `[RETYPED]` lines (2), each read by `check` as written below; no `[EVERY_ROW]` re-spawn, no re-spawn of any kind:
  `security-pick-p6.d3 · [RETYPED] line 58: decision/server-verified-before-any-channel-frame is no row of the pack — read as decision/server-verified-before-any-frame, the pack's row 58`
  `tests-pick-p13.d3 · [RETYPED] line 97: contract/coverage-failure/llvm-cov-exit-N is no row of the pack — read as contract/coverage-failure/llvm-cov-exit, the pack's row 97`
- Non-zero exits and refusals:
  - The bash guard refused three calls, each re-issued once in the form it passes: a `cd` into the skill's
    references, a `cd` into the control clone, and a tool's output piped into a filter.
  - `gate.py show` exited 2 once on a `--chunk` it does not take.
  - The P3 impostor run under the scratch `TMPDIR` exited 100 (the reproduction).
  - The baseline reds of entries 5, 6, 8, 9 and 10 are listed above.
  - `rec.py put` printed `UNPARSED: Expected newline or end of document after a statement (at line 38, column 148)` at
    rev 2: two baselines spelled `''` inside a TOML literal. They were re-spelt as basic strings at rev 3, and the fence
    parses since.
- `DOES NOT HOLD`: entry 15's first spelling, `crates/*/src` (a wildcard pathspec matches whole paths, never a
  directory prefix), read 0 with a product file changed. It was corrected to `crates/*/src/*` and now holds both ways
  (`.andromeda/runs/2026-11-15T18-24-10-phase/p5-controls.md`).
- No `INDETERMINATE` or `NOT DERIVED` line was printed.
- Where a letter was wrong or silent:
  - My own scope draft cited the bind at :212. The panic site is `.expect("bound")` at :213:14, as the audit said;
    this was corrected at P3.
  - The audit's «with no subdir at all … 108 B» recomputes to 100 B. It is not load-bearing.
  - The carry's five named slow cases were the audit run's casualties, not the class: 161 tests read ≥ 5 s at HEAD.
  - The plan's first note said `case_6` reads red only at 7-digit pids. By the P5 baseline the counter had wrapped, and
    the case still read red at a 5-digit pid; the note was corrected. Under the 52 B `TMPDIR` the path overflows at any
    pid of 4 digits or more.
  - The carry's «the rig's home-naming helper» named no helper in the tree. That gap was put to you as fork Q2.
  - `gate.py show`'s default reads the LAST run record of the trail, so a show of an earlier entry needs `--firing 1`.
    The letter's `gate.py show … --n {n}` is silent on it.
- Authority: both P4 answers are the overseer's technical answer, relayed by you (delegate overseer, `inputs#I2`). No
  fork met an escalate pattern; no boundary widens.

## Beyond the plan
| # | proposal the word approves | owner |
|---|---|---|
| 1 | Carry-forward: the live rig's half of item 5, the home a live `viola verify --record` takes, never named `home` | «Standing live rig with a screen model» (existing entry) |

Apply? (yes / review / cancel)
