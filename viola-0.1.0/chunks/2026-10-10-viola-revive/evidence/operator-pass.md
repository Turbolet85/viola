# Operator pass — 2026-10-10-viola-revive

The implementer drove this pass on the operator's word, given with the implement invocation ("Run the operator
pass with the ci.py conclusion read (leg=operator) as usual", `inputs#I5`). Times are `date -u` or the gate
trail's own `ts`; the pass ran on 2026-10-10. No mutation run and no workflow dispatch is part of it
(`inputs#I1`). The three live `claude` starts of plan step 12 were made before it and are recorded in
`live-revive.md`; the pass itself starts none.

## Before it — the block, read green twice

The whole block read green in one call of the gate tool, on HEAD `00c73fda87ef` plus the chunk's uncommitted
edits: 16 entries, 13 green, 0 red, 3 not run. Entries 14 to 16 are the operator's and are this pass. It was
fired twice. The first firing ended 14:42Z. One test edit followed it: in `tests/cli_revive.rs` the removal of
the recorded directory became a bounded wait on the removal itself (a directory a process still holds cannot be
removed on Windows; that hold was not measured here, and on this host the first try removes it). The second
firing, ended 14:55Z, is the reading on the final tree. No push went out before every entry read green.

The second firing's lines, from the trail:

```
  1 lint        green · exit 0 · 0.7s · 0 B → 1.2.log · cargo fmt --all --check
  2 lint        green · exit 0 · 0.34s · 134 B → 2.2.log · cargo clippy --workspace --all-targets --features fake-age… (75 chars)
  3 unit        green · exit 0 · 6.83s · 207662 B → 3.2.log · bash scripts/agent-run.sh run --unit
  4 unit        green · exit 0 · 0.75s · 12026 B → 4.2.log · bash scripts/agent-run.sh run --unit --filter 'test(/snaps… (155 chars)
  5 integration green · exit 0 · 4.53s · 7744 B → 5.2.log · bash scripts/agent-run.sh run --integration --filter 'bina… (144 chars)
  6 probe       green · exit 0 · 0.01s · 0 B → 6.2.log · git diff --quiet 00c73fda87ef -- crates/viola-channel crat… (503 chars)
  7 smoke       green · exit 0 · 0.09s · 47 B → 7.2.log · bash scripts/agent-run.sh cleanup --session p-rev-smoke
  8 smoke       green · exit 0 · 3.79s · 285 B → 8.2.log · bash scripts/agent-run.sh boot --session p-rev-smoke --ins… (71 chars)
  9 smoke       green · exit 0 · 0.1s · 215 B → 9.2.log · bash scripts/agent-run.sh status --session p-rev-smoke
 10 probe       green · exit 0 · 0.02s · 10 B → 10.2.log · bash scripts/g2-zero-panics.sh
 11 probe       green · exit 0 · 0.32s · 108 B → 11.2.log · bash scripts/agent-run.sh schema-check
 12 smoke       green · exit 0 · 0.31s · 176 B → 12.2.log · bash scripts/agent-run.sh cleanup --session p-rev-smoke
 13 probe       green · exit 0 · 58.35s · 255563 B → 13.2.log · bash scripts/agent-run.sh pre-push
 14 probe       not run — leg operator (the letter drives it) · python -X utf8 ~/.claude/skills/andromeda-phase/../androme… (90 chars)
 15 probe       not run — leg operator (the letter drives it) · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
 16 probe       not run — leg operator (the letter drives it) · python -X utf8 ~/.claude/skills/andromeda-phase/../androme… (114 chars)
entries 16 · green 13 · red 0 · recorded 0 · timeout 0 · not-run 3
```

What the logs of that firing hold:

- entry 3: unit 1624 of 1624; entry 4: the 70 selected inline cases, 70 passed; entry 5: the four named
  binaries, 55 of 55 (`cli_revive` 11, `chaos_revive` 1, `state_replay` 5, `cli_fake_agent` 38);
- entry 6, the preservation guard against `00c73fda87ef`: exit 0, no output;
- entries 7 to 9 and 12, the smoke session `p-rev-smoke`: pre-clean `cleaned:[]`; boot `"ok":true` with one
  instance `builder`; status `state:"ready"`; cleanup `processes_gone:true`, `endpoint_gone:true`,
  `home_removed:true`, `killed:[]`;
- entry 10: `g2: clean`; entry 11: schema-check 154 files, 2142 lines, 0 torn, no failure;
- entry 13, `pre-push`: `"ok":true`, `"stage":"linux-tests"`; coverage 2002 of 2002, doctest 0 of 0, playwright
  1 of 1, `gate` no breaches. The kill of the instrumented wrapper in `tests/chaos_revive.rs` did not break the
  coverage merge: the stop rule of `inputs#I4` did not fire.

One limit of entries 10 and 11 on this host: a local run removes each test home when its test ends, so neither
entry read the refusal cases' `cwd-missing` and `no-session` lines or the cut-short snapshot's `state-recovered`
line here. The cut-short case holds its lines to the schema itself; CI keeps its homes, and its G2 and G4 steps
read them (the CI read below). What entry 11 did read beside the smoke home is the stamped live home, with the
three live sessions' role files of step 12.

The process list read after that firing (14:55:38Z) held no process whose command line lies under this
repository's `target/`.

The scope read (`gate.py scope`, 14:52Z and again 14:56:56Z): `scope: UNPARSED 4 · changed 21 · listed 21 ·
recorded 0 (companion 0 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 86`, base HEAD. Every
edited file is in research's two lists, so no scope record was needed. The four `UNPARSED` rows are
`research.md` lines 171 to 174, the sweep record's four column-0 lines inside the `## Files to modify` section;
research is not edited here, and the rows are the wrap's to settle with the operator.

## Step 1 — `pre-push` (entry 13) on the uncommitted tree, 14:55:49Z to 14:56:47Z

- `bash scripts/agent-run.sh pre-push` through the gate tool (`--entry 13`):

  ```
   13 probe       green · exit 0 · 58.25s · 255563 B → 13.3.log · bash scripts/agent-run.sh pre-push
  entries 16 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 15
  ```

  Its document: `"ok":true`, `"stage":"linux-tests"`; coverage 2002 of 2002, doctest 0 of 0, playwright 1 of 1,
  `gate` no breaches. Atoms: `exit 0` ✓, `contains "ok":true` ✓, `contains "stage":"linux-tests"` ✓.
- No source or test file changed between the block's second firing and this one; the files written between them
  are the friction ledger, the gate trail and this chunk's evidence.

## Step 2 — entry 14, hygiene (fired as written)

- Read at 14:56:51Z:
  `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` → exit 0,
  `hygiene: clean — read 78 (runs 62 · evidence 9 · inputs 7) · trails 14 not read · copies 5 not read by P1 — 0
  host paths kept · binary 0 not read by P1`. Atoms: `exit 0` ✓, `contains hygiene: clean` ✓. No row to rewrite.
- Read once more after this file was added, right before the commit: the verdict is in the next section, which
  was written after the commit and rides the next one.

## Before the pre-CI commit, 14:57:26Z

- Hygiene re-read: `hygiene: clean — read 79 (runs 62 · evidence 10 · inputs 7) · trails 14 not read · copies 5
  not read by P1 — 0 host paths kept · binary 0 not read by P1`, exit 0, this file among the evidence.
- The tree the commit takes: the take-up's products (the stamped route line, the master's pending record, the
  ledger note on `v1-41`, the chunk folder, the phase run dir), the 18 changed and 3 new source, test and schema
  files, this chunk's evidence (the red and green readings, the live record with its ledger and rig scripts) and
  inputs, this implement run dir and the bookkeeping the tree carried. 0 ahead of the upstream before it; the
  remote branch head read live (`git ls-remote`) at `00c73fda87ef`.
- This section and everything below it was written after the commit; it rides the next commit.

## Step 3 and step 4 — the pre-CI commit and entry 15, the push, 14:57:33Z to 14:57:43Z

- `0fad11c` `chore(2026-10-10-viola-revive): operator pre-CI commit, for the run this chunk's verdict reads` at
  14:57:33Z (the whole tree, 108 files). `git status --short` read empty after it.
- Entry 15, fired once through the gate tool with no run dir (`gate.py run --plan
  viola-0.1.0/chunks/2026-10-10-viola-revive/plan.md --operator 15`), 14:57:41Z to 14:57:43Z. A `--dry-run` of
  the same call at 14:57:41Z fired nothing and printed the same tripwire line. The firing's tripwire line, entry
  line and summary line, as printed:

  ```
  operator entry 15 · history tripwire: git
   15 probe       green · exit 0 · 2.04s · 90 B → 15.log · history moved: refs/remotes/origin/HEAD 00c73fda→0fad11c9; refs/remotes/origin/build/viola-0.1.0 00c73fda→0fad11c9 · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
  entries 16 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 15
  ```

  The entry carries no `expect` key, so its line asserts `exit 0` alone. Its log
  (`$TMPDIR/andromeda-gate/2026-10-10-viola-revive/run-20261010T145741Z/15.log`, 90 B) holds git's own two
  lines, the second `00c73fd..0fad11c  HEAD -> build/viola-0.1.0`.
- The history reading is the move the entry is for: the two remote-tracking refs went from `00c73fda` to
  `0fad11c9`, a fast-forward. No local branch, tag or stash moved. No force push.
- After it: 0 ahead of the upstream; the remote branch head read live at `0fad11c9c62d`; the tree clean.

## Step 5 — entry 16, the CI read (fired as written), 14:57:49Z to 15:05:04Z

- `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait
  1800` → exit 0: `0fad11c9c62d verdict: green · checks 15/15 · wall 429 s · runs ci#38061685124
  completed/success`, polled 15 times over 435 s. Atoms: `exit 0` ✓, `contains verdict: green` ✓.
- `run_attempt`, read from the run itself (`gh api …/actions/runs/38061685124`): **1**. Event `push`, head
  `0fad11c9c62d`, started 14:57:45Z, last updated 15:04:57Z, conclusion `success`. It is the only run on the sha
  (`gh run list --commit`, by the full sha). The sha is green on its first attempt.
- Its fifteen jobs, each `success` at attempt 1: `lint`, `test`, `release` and `perf` on `windows-2025`,
  `macos-latest` and `ubuntu-latest`; `supply-chain`, `msrv`, `fuzz-replay`.
- The three `test` jobs' own summaries, read from their logs (`gh api …/jobs/<id>/logs
  --allow-escape-sequences`, escapes stripped): `windows-2025` 2023 tests run, 2023 passed; `macos-latest` 1998
  run, 1998 passed; `ubuntu-latest` 2002 run, 2002 passed; none skipped, and no `FAIL`, `TIMEOUT`, `LEAK`,
  `SIGKILL` or `ABORT` status line in any of the three. Before this chunk they read 1931, 1898 and 1902
  (`0af8283`): 92 more on Windows, 100 more on each Unix runner.
- This chunk's cases, counted as `PASS` lines in each of the three logs:

  | cases | `windows-2025` | `macos-latest` | `ubuntu-latest` |
  |---|---|---|---|
  | `viola::chaos_revive`, the kill-and-revive case | 1 | 1 | 1 |
  | `viola::cli_revive` | 9 | 11 | 11 |
  | `viola-state::state_replay` | 5 | 5 | 5 |
  | `viola::cli_fake_agent`, the two new option cases | 2 | 2 | 2 |
  | lines carrying one of the unit filter's seven tokens | 65 | 71 | 71 |

  The two `cli_revive` cases Windows lacks are the `cfg(unix)` ones (the widened instance directory and the
  widened log under `--list`). The six token lines it lacks are the `cfg(unix)` unit cases (five widened modes of
  `strict_instance`, one `snapshot_cwd` case on a directory name that is not UTF-8). The 71 are the 70 inline
  cases the unit filter selects and one `state_replay` case whose name carries a token.
- **The kill-and-revive case passed on all three runners.** So on `macos-latest` and `ubuntu-latest` too the
  wrapper's child was gone, by pid and start time, inside the case's bound after the wrapper was killed: the
  reading plan step 11 names as made for the first time (the fake agent as the child; no real CLI).
- The check plan step 3 flags as not read at planning, the Windows verdict on a tree viola itself made: the
  nine `cli_revive` cases and the kill-and-revive case each ran a revive over a home and an instance `viola run`
  had just made on `windows-2025`, and none was refused `strict-modes-failed`.
- In each `test` job the steps `G2 zero panics`, `G4 schema conformance`, `Secret scan` and `Gate verdict`
  concluded `success`. The G4 documents: `windows-2025` 277 files, 2553 lines; `macos-latest` and
  `ubuntu-latest` 274 files, 2550 lines each; 0 torn and no failure on any. The secret scan's content canary is
  the word the `cwd-missing` case names its recorded directory with, and the scan fails a role file that holds
  it. Whether each kept home was among the files a step read was not read: no job's artifact was opened.

## After the pass

- No fix commit was made: `0fad11c` is the final sha of the pass, local and remote.
- No entry read red in the pass, so no operator entry was fired a second time.
- The tree after the pass carries, for the next commit: this record's sections written after the commit. No
  source file is among them.
- Not measured in the pass: the bounded removal in `revive_whose_recorded_directory_is_gone_is_cwd_missing` has
  no reading of its loop ever running. The case passed on `windows-2025`; whether the first try removed the
  directory there is not in the log.
