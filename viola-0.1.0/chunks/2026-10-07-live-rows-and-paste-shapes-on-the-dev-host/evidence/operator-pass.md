# Operator pass — 2026-10-07-live-rows-and-paste-shapes-on-the-dev-host

The implementer drove this pass on the operator's word, given with the implement invocation ("Run the operator
pass with the ci.py conclusion read (leg=operator) as usual"). The block had read 17 green, 0 red on the final tree
in its one full run of 2026-10-07T10:30Z to 10:32Z (entries 1 to 17; entries 18 to 20 are this pass). No `live` or
`round` entry exists in this block: the three live sessions are one-off witnesses recorded in this folder.

## Before the pass — `pre-push` (entry 17) on the uncommitted tree, 2026-10-07T10:41:59Z
- `bash scripts/agent-run.sh pre-push` → exit 0 after 43 s: `"ok":true`, `"stage":"linux-tests"`; coverage
  1682/1682 (seven more than the base: the `live_shape` cases), playwright 1/1, `gate` no breaches. Atoms:
  `exit 0` ✓, `contains "ok":true` ✓, `contains "stage":"linux-tests"` ✓. Load average at its start: 0.89.
- The same entry in the block's full run: green, 42.05 s, the same counts.

## Entry 18 — hygiene (by hand)
- Read at 2026-10-07T10:42:42Z:
  `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` → exit 0,
  `hygiene: clean — read 91 (runs 72 · evidence 11 · inputs 8) · trails 14 not read · copies 6 not read by P1 — 0
  host paths kept · binary 0 not read by P1`. Atoms: `exit 0` ✓, `contains hygiene: clean` ✓. No row to rewrite.
- Read once more after this section was added, before the commit: the verdict is in the next section's first line.

## The pre-CI commit and entry 19 — the push, 2026-10-07T10:43:00Z
- Hygiene re-read at 10:42:53Z, just before the commit: `hygiene: clean`, read 92 (runs 72 · evidence 12 ·
  inputs 8), this file now among the evidence.
- The scope read before it (`gate.py scope`, 10:41:50Z): `scope: clean — changed 1 · listed 1 · recorded 0`; the one
  changed file is `crates/viola-agent-claude/src/hook.rs`.
- `33d2084` `chore(2026-10-07-live-rows-and-paste-shapes-on-the-dev-host): operator pre-CI commit, for the run this
  chunk's verdict reads` (the whole tree, 101 files: the phase's products, `hook.rs`, this chunk's evidence and
  inputs, the two run dirs and the bookkeeping the tree carried).
- Entry 19: `git diff --quiet && git diff --cached --quiet && git push origin HEAD` → exit 0 at 10:43:07Z,
  `e7e5bf7..33d2084  HEAD -> build/viola-0.1.0`; 0 ahead of the upstream after it.

## Entry 20 — the CI conclusion: GREEN
- `ci.py conclusion --sha HEAD --wait 1800` → exit 0: `33d20843c8a6 verdict: green · checks 15/15 · wall 449 s ·
  runs ci#37609247992 completed/success` (polled 17× over 499 s, 2026-10-07T10:43:11Z to 10:51:30Z). Atoms:
  `exit 0` ✓, `contains verdict: green` ✓. No fix commit was needed.
- The fifteen jobs, each `success` (`gh run view 37609247992 --json jobs`): `test`, `lint`, `perf` and `release`
  on the three OSes (windows-2025, macos-latest, ubuntu-latest), `supply-chain`, `msrv`, `fuzz-replay`.

## The new unit cases on CI, from the three `test` jobs' logs of ci#37609247992
| leg | `live_shape` cases passed | other verdict lines for them | the leg's tests |
|---|---|---|---|
| ubuntu-latest | 7 | 0 | `1682 tests run: 1682 passed (4 slow)` |
| macos-latest | 7 | 0 | `1678 tests run: 1678 passed (4 slow)` |
| windows-2025 | 7 | 0 | `1706 tests run: 1706 passed (4 slow)` |

Each leg ran seven more tests than at the base commit's run (ci#37592258366: 1675, 1671, 1699): the five
`prompt_text_live_shape_normalises_as_measured` cases and the two `prompt_origin_live_shape_files_as_harness` cases.
CI never runs this chunk's live work; the three live sessions are recorded in this folder.
