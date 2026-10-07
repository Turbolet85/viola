# Operator pass — 2026-10-07-test-homes-off-the-contended-volume

The implementer drove this pass on the operator's word, given with the implement invocation ("Run the operator
pass with the ci.py conclusion read (leg=operator) as usual"). The block had read 17 green, 0 red on the final
tree in its one full run of 08:09Z to 08:10Z (entries 1 to 17; entries 18 to 20 are this pass). No live round
exists in this block.

## Before the pass — `pre-push` (entry 16) on the uncommitted tree, 2026-10-07T08:11:32Z
- `bash scripts/agent-run.sh pre-push` → exit 0 after 42 s: `"ok":true`, `"stage":"linux-tests"`; coverage
  1675/1675, playwright 1/1, `gate` no breaches. Atoms: `exit 0` ✓, `contains "ok":true` ✓,
  `contains "stage":"linux-tests"` ✓. Load average at its start: 18.22, falling from another project's build that
  had ended; no build process of another project was alive at its end.
- The watch item (a coverage merge red over green tests while another build writes to the volume): not seen.
  Three green readings in this chunk: entry 16 in the block's first run (41.8 s, quiet host), entry 16 in the full
  run (43.0 s, started 40 s after the contended window while the other build was still alive), and this one.

## Entry 18 — hygiene (by hand)
- Read at 2026-10-07T08:12:23Z:
  `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` → exit 0,
  `hygiene: clean — read 57 (runs 45 · evidence 7 · inputs 5) · trails 12 not read · copies 3 not read by P1 — 0
  host paths kept · binary 0 not read by P1`. Atoms: `exit 0` ✓, `contains hygiene: clean` ✓. No row to rewrite.
- Read once more after this section was added, before the commit: the verdict is in the next section's first line.

## The pre-CI commit and entry 19 — the push, 2026-10-07T08:12:38Z
- Hygiene re-read just before the commit: `hygiene: clean`, the same counts (read 57).
- `230f5dc` `chore(2026-10-07-test-homes-off-the-contended-volume): operator pre-CI commit, for the run this
  chunk's verdict reads` (the whole tree, 70 files: the phase's products, this chunk's six source, test and script
  files, its evidence, the two run dirs and the bookkeeping the tree carried).
- Entry 19: `git diff --quiet && git diff --cached --quiet && git push origin HEAD` → exit 0,
  `b329083..230f5dc  HEAD -> build/viola-0.1.0`; 0 ahead of the upstream after it.

## Entry 20 — the CI conclusion: GREEN
- `ci.py conclusion --sha HEAD --wait 1800` → exit 0: `230f5dc77eae verdict: green · checks 15/15 · wall 405 s ·
  runs ci#37592258366 completed/success` (polled 14× over 405 s, 2026-10-07T08:12:49Z to 08:19:34Z). Atoms:
  `exit 0` ✓, `contains verdict: green` ✓. No fix commit was needed.
- The fifteen jobs, each `success` (`gh run view 37592258366 --json jobs`): `test`, `lint`, `perf` and `release`
  on the three OSes (windows-2025, macos-latest, ubuntu-latest), `supply-chain`, `msrv`, `fuzz-replay`.
- The three `test` legs: ubuntu-latest `1675 tests run: 1675 passed (4 slow)`, macos-latest `1671 passed
  (4 slow)`, windows-2025 `1699 passed (4 slow)`.

## The keeper cases and G2 on CI, from the three `test` jobs' logs of ci#37592258366
| leg | `home_base_backing_` passed | `e2e_home_backing_` passed | other verdict lines for them | slowest |
|---|---|---|---|---|
| ubuntu-latest | 6 | 7 | 0 | 0.007 s |
| macos-latest | 6 | 7 | 0 | 0.024 s |
| windows-2025 | 1 | 1 | 0 | 0.027 s |

- On windows-2025 only the plain-base case of each keeper exists (the other five, and the `local_live` case, are
  `#[cfg(unix)]`), so 1 and 1 is the whole set there.
- G2 with the edited start point: each `test` log carries `g2-probe: all cases as expected` and `g2: clean`, once
  each. No runner has a link, so there the walk is over a real directory, as before the edit. The three `perf`
  jobs, which run G2 too, concluded `success`; their logs were not read line by line.
