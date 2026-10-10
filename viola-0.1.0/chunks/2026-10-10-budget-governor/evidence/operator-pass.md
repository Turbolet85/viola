# Operator pass — 2026-10-10-budget-governor

The implementer drove this pass on the operator's word, given with the implement invocation ("Run the operator
pass: pre-push, the hygiene entry, the pre-CI commit, the push, and the ci.py conclusion read (leg=operator).
Fold a red into a fix commit and read CI again; report the run ids", `inputs#I8`). It is plan step 16; step 15
as revised lifted the hold (`inputs#I6`, `inputs#I7`). Times are `date -u` or the gate trail's own `ts`; the
pass ran on 2026-10-10. No mutation run, no workflow dispatch and no start of the real `claude` is part of it
(`inputs#I1`).

## Before it — the block, read green by the run that built steps 1 to 14

This session re-entered at step 16 and did not fire the block again. Its green reading is the earlier implement
run's (`.andromeda/runs/2026-10-10T21-29-10-implement/`, the trail's call stamped 22:01:10Z), on HEAD
`396c108ba64c` plus the chunk's uncommitted edits: 16 entries, 13 green, 0 red, 3 not run. Entries 14 to 16 are
the operator's and are this pass. The same trail holds one earlier whole firing, stamped 21:58:51Z, with the
same summary line.

The block's lines, from that trail:

```
  1 lint        green · exit 0 · 0.76s · 0 B → 1.2.log · cargo fmt --all --check
  2 lint        green · exit 0 · 1.22s · 134 B → 2.2.log · cargo clippy --workspace --all-targets --features fake-age… (75 chars)
  3 unit        green · exit 0 · 8.13s · 242871 B → 3.2.log · bash scripts/agent-run.sh run --unit
  4 unit        green · exit 0 · 0.93s · 10991 B → 4.2.log · bash scripts/agent-run.sh run --unit --filter 'test(/perce… (147 chars)
  5 integration green · exit 0 · 4.74s · 9514 B → 5.2.log · bash scripts/agent-run.sh run --integration --filter 'bina… (205 chars)
  6 probe       green · exit 0 · 0.01s · 0 B → 6.2.log · git diff --quiet 396c108ba64c -- crates/viola-agent-claude… (424 chars)
  7 smoke       green · exit 0 · 0.09s · 47 B → 7.2.log · bash scripts/agent-run.sh cleanup --session p-bg-smoke
  8 smoke       green · exit 0 · 4.13s · 348 B → 8.2.log · bash scripts/agent-run.sh boot --session p-bg-smoke --inst… (70 chars)
  9 smoke       green · exit 0 · 0.09s · 216 B → 9.2.log · bash scripts/agent-run.sh status --session p-bg-smoke
 10 probe       green · exit 0 · 0.02s · 10 B → 10.2.log · bash scripts/g2-zero-panics.sh
 11 probe       green · exit 0 · 0.32s · 108 B → 11.2.log · bash scripts/agent-run.sh schema-check
 12 smoke       green · exit 0 · 0.3s · 175 B → 12.2.log · bash scripts/agent-run.sh cleanup --session p-bg-smoke
 13 probe       green · exit 0 · 62.43s · 295146 B → 13.2.log · bash scripts/agent-run.sh pre-push
 14 probe       not run — leg operator (the letter drives it) · python -X utf8 ~/.claude/skills/andromeda-phase/../androme… (90 chars)
 15 probe       not run — leg operator (the letter drives it) · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
 16 probe       not run — leg operator (the letter drives it) · python -X utf8 ~/.claude/skills/andromeda-phase/../androme… (114 chars)
entries 16 · green 13 · red 0 · recorded 0 · timeout 0 · not-run 3
```

What the logs of that firing hold, read in this session while they still stand:

- entry 3: unit 1865 of 1865; entry 4: the 72 inline cases the seven prefixes select, 72 passed; entry 5: the
  six named binaries, 62 of 62;
- entry 6, the preservation guard against `396c108ba64c`: exit 0, no output;
- entries 7 to 9 and 12, the smoke session `p-bg-smoke`: pre-clean `cleaned:[]`; boot `"ok":true` with one
  instance `builder`; status `state:"ready"`; cleanup `processes_gone:true`, `endpoint_gone:true`,
  `home_removed:true`, `killed:[]`;
- entry 10: `g2: clean`; entry 11: schema-check 160 files, 2220 lines, 0 torn, no failure.

What changed between that firing and this pass: the plan's revision of step 15 (`plan.md`, `research.md`,
`scope.md`, the chunk's inputs I6 and I7, the revision's phase run dir), the friction ledger and the handoff's
session-end stamp. No file under `src`, `crates`, `tests`, `.config`, `scripts`, `fixtures`, `plugin` or
`schemas`, and neither `Cargo.toml` nor `Cargo.lock`, is newer than that trail (a `find -newer` read at 22:18Z,
empty).

The Windows lint, on this host, 22:18:30Z: `cargo clippy --workspace --all-targets --features fake-agent --target
x86_64-pc-windows-msvc -- -D warnings` with `CARGO_TARGET_DIR=target/wincheck` (check only, nothing linked or
run) exited 0 in 0.12 s, its build already current for this tree. It runs no test.

The scope read (`gate.py scope`, 22:20:01Z): `scope: clean — changed 31 · listed 31 · recorded 0 (companion 0 ·
mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 75`, base HEAD. No `scope-record.md` exists
and none is needed.

## Step 1 — `pre-push` (entry 13) on the uncommitted tree, 22:18:34Z to 22:19:37Z

- `bash scripts/agent-run.sh pre-push` through the gate tool (`--entry 13`, this run's dir):

  ```
   13 probe       green · exit 0 · 62.8s · 295147 B → 13.log · bash scripts/agent-run.sh pre-push
  entries 16 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 15
  ```

  Its document: `"ok":true`, `"stage":"linux-tests"`; coverage 2279 of 2279, doctest 0 of 0, playwright 1 of 1,
  `gate` no breaches. Atoms, from the trail (`gate.py show … --n 13`): `exit 0` ✓, `contains "ok":true` ✓,
  `contains "stage":"linux-tests"` ✓.
- The coverage summary it left (`target/agent-run/artifacts/llvm-cov-summary.json`, written 22:19:35Z, inside
  the run's window) reads lines 97.98 (9739 of 9940), functions 97.90 (1166 of 1191), regions 97.76 (18694 of
  19122). The thresholds are 85, 95 and 80.
- The process list read at 22:20:02Z, every process by its executable path: none under this repository's
  `target/` and none under the test-home base.

## Step 2 — entry 14, hygiene (fired as written)

- Read at 22:19:55Z:
  `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` → exit 0,
  `hygiene: clean — read 67 (runs 56 · evidence 1 · inputs 10) · trails 21 not read · copies 8 not read by P1 —
  0 host paths kept · binary 0 not read by P1`, base HEAD. Atoms: `exit 0` ✓, `contains hygiene: clean` ✓. No
  row to rewrite.
- Read once more after this file was added, right before the commit: the verdict is in the next section, which
  was written after the commit and rides the next one.

## Before the pre-CI commit, 22:20:41Z

- Hygiene re-read: `hygiene: clean — read 68 (runs 56 · evidence 2 · inputs 10) · trails 21 not read · copies 8
  not read by P1 — 0 host paths kept · binary 0 not read by P1`, exit 0, this file among the evidence.
- The tree the commit takes, 107 paths by `git status --porcelain --untracked-files=all`: the take-up's products
  (the stamped route line, the master's pending record, the ledger note on `v1-35`, the chunk folder, the two
  phase run dirs), the 28 changed and 3 new source, test and config files, this chunk's evidence (the red and
  green readings, this record) and inputs I1 to I8, the two implement run dirs, the friction ledger, the last
  wrap's evolve trail and the handoff's session-end stamp. 0 ahead of the upstream before it; the remote branch
  head read live (`git ls-remote`) at `396c108ba64c`.
- This section and everything below it was written after the commit; it rides the next commit.

## Step 3 and step 4 — the pre-CI commit and entry 15, the push, 22:20:46Z to 22:20:52Z

- `c85961d` `chore(2026-10-10-budget-governor): operator pre-CI commit, for the run this chunk's verdict reads`
  at 22:20:46Z (the whole tree, 107 files; 31 of them under `src`, `crates`, `tests`, `.config` or
  `Cargo.toml`). `git status --short` read empty after it.
- Entry 15, fired through the gate tool with no run dir (`gate.py run --plan
  viola-0.1.0/chunks/2026-10-10-budget-governor/plan.md --operator 15`), 22:20:50Z to 22:20:52Z. A `--dry-run`
  of the same call at 22:19:06Z fired nothing and printed the same tripwire line. The firing's tripwire line,
  entry line and summary line, as printed:

  ```
  operator entry 15 · history tripwire: git
   15 probe       green · exit 0 · 2.27s · 90 B → 15.log · history moved: refs/remotes/origin/HEAD 396c108b→c85961dc; refs/remotes/origin/build/viola-0.1.0 396c108b→c85961dc · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
  entries 16 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 15
  ```

  The entry carries no `expect` key, so its line asserts `exit 0` alone. Its log
  (`$TMPDIR/andromeda-gate/2026-10-10-budget-governor/run-20261010T222050Z/15.log`, 90 B) holds git's own two
  lines, the second `396c108..c85961d  HEAD -> build/viola-0.1.0`.
- The history reading is the move the entry is for: the two remote-tracking refs went from `396c108b` to
  `c85961dc`, a fast-forward. No local branch, tag or stash moved. No force push.
- After it: 0 ahead of the upstream; the remote branch head read live at `c85961dc62b3`; the tree clean.

## Step 5 — entry 16, the CI read (fired as written), 22:20:56Z to 22:33:20Z: green

- `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait
  1800` → exit 0: `c85961dc62b3 verdict: green · checks 15/15 · wall 726 s · runs ci#38091081413
  completed/success`, polled 25 times over 744 s. Atoms: `exit 0` ✓, `contains verdict: green` ✓.
- `run_attempt`, read from the run itself (`gh api …/actions/runs/38091081413`): **1**. Event `push`, head
  `c85961dc62b3`, started 22:20:53Z, last updated 22:33:03Z, conclusion `success`. It is the only run on the sha
  (`gh run list --commit`, by the full sha). The sha is green on its first attempt, so no fix commit was made.
- Its fifteen jobs, each `success`: `lint`, `test`, `release` and `perf` on `windows-2025`, `macos-latest` and
  `ubuntu-latest`; `supply-chain`, `msrv`, `fuzz-replay`. `test (windows-2025)` was the last to finish.
- The three `test` jobs' own summaries, read from their logs (`gh api …/jobs/<id>/logs
  --allow-escape-sequences`, escapes stripped): `windows-2025` 2280 tests run, 2280 passed; `macos-latest` 2275
  run, 2275 passed; `ubuntu-latest` 2279 run, 2279 passed; none skipped, and no `FAIL`, `TIMEOUT`, `LEAK`,
  `SIGKILL` or `ABORT` status line in any of the three (every status line matched by one pattern over the
  whole log: `PASS` 2280, 2275 and 2279, `SLOW` 5 each). On the last chunk's final sha they read 2169, 2159 and
  2163 (`67ab367`): 111 more on Windows, 116 more on each Unix runner. The five tests each runner marks slow
  are the standing `_window_` ones; none is this chunk's.
- This chunk's cases, counted as `PASS` lines in each of the three logs:

  | cases | `windows-2025` | `macos-latest` | `ubuntu-latest` |
  |---|---|---|---|
  | `viola::cli_budget` | 9 | 9 | 9 |
  | `viola::cli_revive` | 9 | 12 | 12 |
  | `viola::hook_statusline` | 7 | 11 | 11 |
  | `viola::cli_controls_not_disableable` | 19 | 19 | 19 |
  | `viola-state::state_replay` | 7 | 7 | 7 |
  | `viola-state::state_events` | 4 | 4 | 4 |
  | inline cases carrying one of the unit filter's seven prefixes | 71 | 72 | 72 |

  Each count is the whole binary's, the standing cases with the new ones. Which `cli_revive` and
  `hook_statusline` cases Windows lacks was not read name by name; the two witnesses are among them (below).
  The one prefixed inline case Windows lacks was not named either.
- **The nine `cli_budget` cases passed on all three runners**, each from a stamped home: 4.4 to 4.7 s on
  `windows-2025`, 4.6 to 5.1 s on `macos-latest`, 3.6 to 4.0 s on `ubuntu-latest`, under the binary's 20 s kill.
  They are the seven `path6_` cases (one with two rows) and `e5_the_replay_reads_what_the_wrapper_wrote`.
- **The two witnesses**, `cfg(unix)`: `revive_over_a_home_another_user_could_write_is_refused_and_leaves_it_as_found`
  passed in 4.6 s on `macos-latest` and 3.6 s on `ubuntu-latest`;
  `hook_statusline_a_home_another_user_can_write_runs_nothing_however_often` in 0.5 s and 0.3 s. Neither is
  built on `windows-2025`, and neither name has a line in that log.
- In each `test` job the steps `G2 zero panics`, `G4 schema conformance`, `Secret scan` and `Gate verdict`
  concluded `success`: `g2: clean` on all three; the G4 documents read `windows-2025` 328 files, 3002 lines,
  `macos-latest` 338 files, 3104 lines, `ubuntu-latest` 338 files, 3105 lines, 0 torn and no failure on any. CI
  keeps its test homes, so these are the reads over the lines the new cases wrote. Whether each kept home was
  among the files a step read was not read: no job's artifact was opened.
- In each `lint` job the step `G1 bare instrument` concluded `success`. That is the read the plan's obs
  criterion on `run.budget_eval` names. The step's own output was not opened.

## After the pass

- No fix commit was made: `c85961d` is the final sha of the pass, local and remote, and the sha the verdict
  stands on.
- Entry 13 was fired once in this pass, entry 15 once and entry 16 once. Entry 14 was fired once as written and
  re-read before the commit.
- The tree after the pass carries, for the next commit: this record's sections written after the pre-CI commit,
  this run dir's trails and the friction ledger's lines. No source file is among them.
- Not measured in the pass: any mutation score of the new code (no mutation run is part of a chunk,
  `inputs#I1`); the real `claude` against the gate (no live start, `inputs#I1`); the names of the cases
  `windows-2025` does not build.
