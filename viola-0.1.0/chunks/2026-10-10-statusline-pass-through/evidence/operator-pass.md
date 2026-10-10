# Operator pass — 2026-10-10-statusline-pass-through

The implementer drove this pass on the operator's word, given with the implement invocation ("Run the operator
pass with the ci.py conclusion read (leg=operator) as usual", `inputs#I4`). Times are `date -u` or the gate
trail's own `ts`; the pass ran on 2026-10-10. No mutation run and no workflow dispatch is part of it
(`inputs#I1`). The three live `claude` starts of plan step 12 were made before it and are recorded in
`live-statusline.md`; the pass itself starts none.

## Before it — the block, read green

The whole block read green in one call of the gate tool, on HEAD `4d77eacd14d6` plus the chunk's uncommitted
edits: 17 entries, 14 green, 0 red, 3 not run. Entries 15 to 17 are the operator's and are this pass. It was
started at 19:15:48Z. One firing of entries 1 and 2 came before it and read entry 2 red: `clippy::
large_enum_variant` on `SnapshotRead` and `Recovered`, once the snapshot gained its field (216 bytes). The
snapshot is boxed in both enums; the whole block ran after that fix. No push went out before every entry read
green.

The block's lines, from the trail:

```
  1 lint        green · exit 0 · 0.73s · 0 B → 1.2.log · cargo fmt --all --check
  2 lint        green · exit 0 · 0.16s · 72 B → 2.2.log · cargo clippy --workspace --all-targets --features fake-age… (75 chars)
  3 unit        green · exit 0 · 18.17s · 229385 B → 3.log · bash scripts/agent-run.sh run --unit
  4 unit        green · exit 0 · 1.06s · 22515 B → 4.log · bash scripts/agent-run.sh run --unit --filter 'test(/statu… (135 chars)
  5 integration green · exit 0 · 4.73s · 14185 B → 5.log · bash scripts/agent-run.sh run --integration --filter 'bina… (206 chars)
  6 probe       green · exit 0 · 0.01s · 0 B → 6.log · git diff --quiet 4d77eacd14d6 -- crates/viola-agent-claude… (380 chars)
  7 integration green · exit 0 · 7.08s · 5494 B → 7.log · bash scripts/agent-run.sh run --fuzz-replay
  8 smoke       green · exit 0 · 0.12s · 47 B → 8.log · bash scripts/agent-run.sh cleanup --session p-sl-smoke
  9 smoke       green · exit 0 · 6.19s · 795 B → 9.log · bash scripts/agent-run.sh boot --session p-sl-smoke --inst… (70 chars)
 10 smoke       green · exit 0 · 0.09s · 216 B → 10.log · bash scripts/agent-run.sh status --session p-sl-smoke
 11 probe       green · exit 0 · 0.03s · 10 B → 11.log · bash scripts/g2-zero-panics.sh
 12 probe       green · exit 0 · 0.32s · 108 B → 12.log · bash scripts/agent-run.sh schema-check
 13 smoke       green · exit 0 · 0.3s · 175 B → 13.log · bash scripts/agent-run.sh cleanup --session p-sl-smoke
 14 probe       green · exit 0 · 62.48s · 279329 B → 14.log · bash scripts/agent-run.sh pre-push
 15 probe       not run — leg operator (the letter drives it) · python -X utf8 ~/.claude/skills/andromeda-phase/../androme… (90 chars)
 16 probe       not run — leg operator (the letter drives it) · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
 17 probe       not run — leg operator (the letter drives it) · python -X utf8 ~/.claude/skills/andromeda-phase/../androme… (114 chars)
entries 17 · green 14 · red 0 · recorded 0 · timeout 0 · not-run 3
```

What the logs of that firing hold:

- entry 3: unit 1765 of 1765; entry 4: the 140 selected inline cases, 140 passed (`viola-agent-claude` 93,
  `viola-core` 5, `viola-state` 9, the root bin 29, the fake agent's own 4); entry 5: the six named binaries, 107
  of 107 (`hook_statusline` 11, `cli_instance_state` 20, `tui_passthrough` 6, `hook_fail_open` 24,
  `cli_fake_agent` 41, `state_replay` 5);
- entry 6, the preservation guard against `4d77eacd14d6`: exit 0, no output;
- entry 7: the five fuzz targets' corpora replayed, 5 passed (the `hook_stdin` corpus holds 12 seeds);
- entries 8 to 10 and 13, the smoke session `p-sl-smoke`: pre-clean `cleaned:[]`; boot `"ok":true` with one
  instance `builder`; status `state:"ready"`; cleanup `processes_gone:true`, `endpoint_gone:true`,
  `home_removed:true`, `killed:[]`;
- entry 11: `g2: clean`; entry 12: schema-check 154 files, 2142 lines, 0 torn, no failure;
- entry 14, `pre-push`: `"ok":true`, `"stage":"linux-tests"`; coverage 2163 of 2163, doctest 0 of 0, playwright
  1 of 1, `gate` no breaches. The coverage summary it left reads lines 97.69, functions 97.63, regions 97.46
  (the thresholds are 85, 95 and 80).

One limit of entries 11 and 12 on this host: a local run removes each test home when its test ends, so neither
entry read the lines the new cases wrote here. Case 1, case 4, case 5 and case 6 of `binary(hook_statusline)`
hold their own lines to the schema themselves; CI keeps its homes, and its G2 and G4 steps read them (the CI
read below).

Entries 11 and 12 were fired once more at 19:29:08Z, after step 12's live sessions, so that they read the rig
home's role files, the real CLI's statusline lines among them:

```
 11 probe       green · exit 0 · 0.03s · 10 B → 11.2.log · bash scripts/g2-zero-panics.sh
 12 probe       green · exit 0 · 0.34s · 108 B → 12.2.log · bash scripts/agent-run.sh schema-check
entries 17 · green 2 · red 0 · recorded 0 · timeout 0 · not-run 15
```

`g2: clean`; schema-check 157 files, 2198 lines, 0 torn, no failure.

The process list read at 19:27:35Z held no process whose executable lies under this repository's `target/`, no
rig host and no `claude` 2.1.287 by path.

The scope read (`gate.py scope`, 19:27:28Z): `scope: clean — changed 36 · listed 35 · recorded 1 (companion 0 ·
mechanical 1 · in-intent 0 · widening 0) · absorbed 0 · excluded 66`, base HEAD. The one recorded file is
`fuzz/Cargo.lock`.

## Step 1 — `pre-push` (entry 14) on the uncommitted tree, 19:27:57Z to 19:28:58Z

- `bash scripts/agent-run.sh pre-push` through the gate tool (`--entry 14`):

  ```
   14 probe       green · exit 0 · 60.7s · 279328 B → 14.2.log · bash scripts/agent-run.sh pre-push
  entries 17 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 16
  ```

  Its document: `"ok":true`, `"stage":"linux-tests"`; coverage 2163 of 2163, doctest 0 of 0, playwright 1 of 1,
  `gate` no breaches. Atoms: `exit 0` ✓, `contains "ok":true` ✓, `contains "stage":"linux-tests"` ✓.
- No source or test file changed between the block's firing and this one; the files written between them are
  the friction ledger, the gate trail and this chunk's evidence (the live record, its ledger and its rig
  scripts).
- That call's printed lines were read through a filter that kept the entry's line and the summary line; the two
  lines above are those, and the atoms are the trail's own (`gate.py show … --n 14`).

## Step 2 — entry 15, hygiene (fired as written)

- Read at 19:29:08Z:
  `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` → exit 0,
  `hygiene: clean — read 57 (runs 42 · evidence 9 · inputs 6) · trails 15 not read · copies 4 not read by P1 — 0
  host paths kept · binary 0 not read by P1`. Atoms: `exit 0` ✓, `contains hygiene: clean` ✓. No row to rewrite.
- Read once more after this file was added, right before the commit: the verdict is in the next section, which
  was written after the commit and rides the next one.

## Before the pre-CI commit, 19:29:54Z

- Hygiene re-read: `hygiene: clean — read 58 (runs 42 · evidence 10 · inputs 6) · trails 15 not read · copies 4
  not read by P1 — 0 host paths kept · binary 0 not read by P1`, exit 0, this file among the evidence.
- The tree the commit takes: the take-up's products (the stamped route line, the master's pending record, the
  ledger notes on `v1-24` and `v1-45`, the chunk folder, the phase run dir), the 29 changed and 7 new source,
  test, lockfile and seed files, this chunk's evidence (the red and green readings, the live record with its
  ledger and rig scripts) and inputs, this implement run dir and the bookkeeping the tree carried. 0 ahead of
  the upstream before it; the remote branch head read live (`git ls-remote`) at `4d77eacd14d6`.
- This section and everything below it was written after the commit; it rides the next commit.

## Step 3 and step 4 — the pre-CI commit and entry 16, the push, 19:30:04Z to 19:30:17Z

- `58f8720` `chore(2026-10-10-statusline-pass-through): operator pre-CI commit, for the run this chunk's verdict
  reads` at 19:30:04Z (the whole tree, 103 files). `git status --short` read empty after it.
- Entry 16, fired through the gate tool with no run dir (`gate.py run --plan
  viola-0.1.0/chunks/2026-10-10-statusline-pass-through/plan.md --operator 16`), 19:30:14Z to 19:30:17Z. A
  `--dry-run` of the same call at 19:30:09Z fired nothing and printed the same tripwire line. The firing's
  tripwire line, entry line and summary line, as printed:

  ```
  operator entry 16 · history tripwire: git
   16 probe       green · exit 0 · 2.27s · 90 B → 16.log · history moved: refs/remotes/origin/HEAD 4d77eacd→58f87207; refs/remotes/origin/build/viola-0.1.0 4d77eacd→58f87207 · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
  entries 17 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 16
  ```

  The entry carries no `expect` key, so its line asserts `exit 0` alone. Its log
  (`$TMPDIR/andromeda-gate/2026-10-10-statusline-pass-through/run-20261010T193015Z/16.log`, 90 B) holds git's own
  two lines, the second `4d77eac..58f8720  HEAD -> build/viola-0.1.0`.
- The history reading is the move the entry is for: the two remote-tracking refs went from `4d77eacd` to
  `58f87207`, a fast-forward. No local branch, tag or stash moved. No force push.
- After it: 0 ahead of the upstream; the remote branch head read live at `58f872077352`; the tree clean.

## Step 5 — entry 17, the CI read (fired as written), 19:30:24Z to 19:31:59Z: red

- `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait
  1800` → exit 0: `58f872077352 verdict: red · checks 15/15 · first-fail +66 s lint (windows-2025) · runs
  ci#38080061631 in_progress/-`, polled 4 times over 94 s. Atoms: `exit 0` ✓, `contains verdict: green` ✗.
- The run was left to finish before anything was folded. Read from the run itself (`gh api
  …/actions/runs/38080061631`): `run_attempt` 1, event `push`, head `58f872077352`, started 19:30:19Z, last
  updated 19:37:34Z, conclusion `failure`. It is the only run on the sha (`gh run list --commit`, by the full
  sha).
- Its fifteen jobs: thirteen `success` (`lint`, `test`, `release` and `perf` on `macos-latest` and
  `ubuntu-latest`; `release` and `perf` on `windows-2025`; `supply-chain`, `msrv`, `fuzz-replay`) and two
  `failure`, both on `windows-2025`: `lint` and `test`.

### The two reds, each read from its job's log

**`lint (windows-2025)`** — one error: `function hex_of is never used`, `tests\hook_statusline.rs:45`. The helper
is called only by the four `cfg(unix)` cases, so on Windows nothing uses it. The host's own clippy cannot see
it (`testing.md`, 2026-10-03).

**`test (windows-2025)`** — 2169 run, 2166 passed, 3 failed, all three in `binary(hook_statusline)`:

| case | what it read |
|---|---|
| `…_without_a_recorded_command_prints_nothing_and_records_the_reading` | no `budget.json` |
| `…_oversize_and_malformed_stdin_write_nothing`, `oversize` | role lines `hook-invoked`, `hook-decision`, where it expects `parse-rejected` between them |
| the same, `malformed` | the same two lines |

Each is what the arm does when its instance check refuses the home: nothing read, nothing written, one decision
line. The fourth case of that binary that runs the arm's whole course on Windows,
`…_changes_neither_the_snapshot_nor_the_stamps`, passed there in 4.6 s, and it is the one that starts from a
stamped home. The three red ones started from a home whose first start is `Wrapper::boot`.

**The cause is the fixture, read in its source.** On Windows x64 `Wrapper::boot` calls `seed_conpty` before the
start, and `seed_conpty` makes `<home>/bin/<key>/conpty/` with `create_dir_all` (`tests/support/home.rs`; its
own comment says "It creates the home"). So such a home is the test's own, with the DACL its parent directory
hands down, not the protected one viola sets on a home it creates. `check_instance` reads the home first and
refuses it. A stamped home is made by `viola verify` before any seeding, so it carries viola's own DACL. The
product did what it is built to do; the cases asked it of a home it must refuse. Not read: the refused home's
DACL itself (no job artifact was opened), so the ACE that tripped the rule is not named here.

What the red run did show green: on `macos-latest` 2159 of 2159 and on `ubuntu-latest` 2163 of 2163, none
skipped, no `FAIL`, `TIMEOUT`, `LEAK`, `SIGKILL` or `ABORT` status line in either; the eleven
`binary(hook_statusline)` cases, the five new `cli_instance_state` cases, the new `tui_passthrough` case and the
three `cli_fake_agent` cases passed on both, and on `windows-2025` every one of those that is not one of the
three above (the `cfg(unix)` ones are not built there). `supply-chain` read the fuzz lockfile's 24 added
packages and passed.

### The fix, 19:38Z to 19:41Z

Test code only, in `tests/hook_statusline.rs`:

- `hex_of` and `unstamped` are `cfg(unix)`, like the cases that use them;
- the three every-OS cases that run the arm's whole course (no recorded command, oversize and malformed stdin,
  the process-log scan) take the `stamped_home` fixture, like the fourth. The four `cfg(unix)` cases keep their
  unstamped homes, which viola itself creates on Unix;
- the process-log scan now also requires its one decision line to read `budget_written` true with no `detail`.
  On the red run it passed on Windows over a refused home, where nothing had been read that could leak: it
  proved nothing there.

Read before the fix commit:

- The Windows lint, on this host: `cargo clippy --workspace --all-targets --features fake-agent --target
  x86_64-pc-windows-msvc -- -D warnings` under `CARGO_TARGET_DIR=target/wincheck` (check only, nothing linked or
  run). On the fixed tree, 19:39:18Z: exit 0. Control, the `cfg(unix)` taken off `hex_of` again, 19:39:27Z:
  exit 101 with the runner's own error (`function hex_of is never used`). Attribute put back, 19:39:45Z: exit 0.
  So this read sees what the runner's lint job saw. It does not run a test: whether the three cases pass on
  Windows is the next CI read's.
- Through the gate tool (`--only 1,2,5,14`), 19:40:03Z to 19:41:12Z:

  ```
    1 lint        green · exit 0 · 0.74s · 0 B → 1.3.log · cargo fmt --all --check
    2 lint        green · exit 0 · 0.19s · 72 B → 2.3.log · cargo clippy --workspace --all-targets --features fake-age… (75 chars)
    5 integration green · exit 0 · 5.42s · 14247 B → 5.2.log · bash scripts/agent-run.sh run --integration --filter 'bina… (206 chars)
   14 probe       green · exit 0 · 61.92s · 279329 B → 14.3.log · bash scripts/agent-run.sh pre-push
  entries 17 · green 4 · red 0 · recorded 0 · timeout 0 · not-run 13
  ```

  Entry 5 read 107 of 107. The eleven `binary(hook_statusline)` cases passed here; the five that start from a
  stamped home took 3.7 s each.
- Hygiene before the fix commit, 19:42:06Z: `hygiene: clean — read 2 (runs 1 · evidence 1 · inputs 0) · trails 1
  not read · copies 0 not read by P1 — 0 host paths kept · binary 0 not read by P1` (base HEAD, the pre-CI
  commit: the gate trail and this file).
- This bullet and everything below it was written after the fix commit; it rides the next commit.

## The fix commit and its push, 19:42:15Z to 19:42:17Z

- `67ab367` `fix(2026-10-10-statusline-pass-through): operator fix after CI run 38080061631, the every-OS
  hook_statusline cases start from a stamped home and two unix-only helpers are cfg(unix)` at 19:42:15Z: three
  files, `tests/hook_statusline.rs`, the gate trail and this record. `git status --short` read empty after it.
- Entry 16 again, through the gate tool with no run dir, 19:42:15Z to 19:42:17Z:

  ```
  operator entry 16 · history tripwire: git
   16 probe       green · exit 0 · 1.51s · 90 B → 16.log · history moved: refs/remotes/origin/HEAD 58f87207→67ab3677; refs/remotes/origin/build/viola-0.1.0 58f87207→67ab3677 · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
  entries 17 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 16
  ```

  A fast-forward from `58f87207` to `67ab3677`. No local branch, tag or stash moved. No force push. After it: 0
  ahead of the upstream; the remote branch head read live at `67ab3677cacf`.

## Step 5 again — entry 17, the CI read on the final sha (fired as written), 19:42:22Z to 19:52:42Z: green

- `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait
  1800` → exit 0: `67ab3677cacf verdict: green · checks 15/15 · wall 616 s · runs ci#38080855246
  completed/success`, polled 21 times over 620 s. Atoms: `exit 0` ✓, `contains verdict: green` ✓.
- `run_attempt`, read from the run itself (`gh api …/actions/runs/38080855246`): **1**. Event `push`, head
  `67ab3677cacf`, started 19:42:19Z, last updated 19:52:39Z, conclusion `success`. It is the only run on the sha
  (`gh run list --commit`, by the full sha). The sha is green on its first attempt.
- Its fifteen jobs, each `success`: `lint`, `test`, `release` and `perf` on `windows-2025`, `macos-latest` and
  `ubuntu-latest`; `supply-chain`, `msrv`, `fuzz-replay`.
- The three `test` jobs' own summaries, read from their logs (`gh api …/jobs/<id>/logs
  --allow-escape-sequences`, escapes stripped): `windows-2025` 2169 tests run, 2169 passed; `macos-latest` 2159
  run, 2159 passed; `ubuntu-latest` 2163 run, 2163 passed; none skipped, and no `FAIL`, `TIMEOUT`, `LEAK`,
  `SIGKILL` or `ABORT` status line in any of the three. Before this chunk they read 2023, 1998 and 2002
  (`0fad11c`): 146 more on Windows, 161 more on each Unix runner. The five tests each runner marks slow are the
  standing `_window_` ones; none is this chunk's.
- This chunk's cases, counted as `PASS` lines in each of the three logs:

  | cases | `windows-2025` | `macos-latest` | `ubuntu-latest` |
  |---|---|---|---|
  | `viola::hook_statusline` | 7 | 11 | 11 |
  | `viola::cli_instance_state`, the five new cases | 4 | 5 | 5 |
  | `viola::tui_passthrough`, the statusline-bearing start | 1 | 1 | 1 |
  | `viola::cli_fake_agent`, the two option cases and the mode case | 3 | 3 | 3 |
  | `viola-state::state_replay` | 5 | 5 | 5 |
  | inline cases carrying one of the unit filter's five tokens | 130 | 140 | 140 |

  The four `hook_statusline` cases Windows lacks are the `cfg(unix)` ones (the pass-through, the payload with
  no `rate_limits`, the failing command, the group-writable home); the `cli_instance_state` case it lacks is
  the end-to-end `path6_…`; the ten token lines it lacks are `cfg(unix)` unit cases (the arm's shell-out and
  link cases, the budget file's modes).
- **The three cases that were red on `windows-2025` passed there**, each from a stamped home, in 4.1 to 4.3 s,
  under the binary's 20 s kill. With them the arm's whole course is read on Windows: a reading written to
  `budget.json` with a command recorded and none run, both rejections with their `parse-rejected` line, and a
  process log holding neither the command nor the payload.
- In each `test` job the steps `G2 zero panics`, `G4 schema conformance`, `Secret scan` and `Gate verdict`
  concluded `success`: `g2: clean` on all three; the G4 documents read `windows-2025` 292 files, 2637 lines,
  `macos-latest` and `ubuntu-latest` 300 files, 2730 lines each, 0 torn and no failure on any. CI keeps its
  test homes, so these are the reads over the lines the new cases wrote. Whether each kept home was among the
  files a step read was not read: no job's artifact was opened.

## After the pass

- One fix commit was made: `67ab367` is the final sha of the pass, local and remote. `58f8720` read red and is
  not the sha the verdict stands on.
- Entry 16 was fired twice (the pre-CI commit, the fix commit) and entry 17 twice (red, then green). Entry 15
  was fired once as written and re-read before each commit.
- The tree after the pass carries, for the next commit: this record's sections written after the fix commit
  and the friction ledger's lines. No source file is among them.
- Not measured in the pass: the DACL of a fixture-booted home on `windows-2025` (the cause above is read from
  the fixture's source and from which cases passed, not from the ACL); any mutation score of the new code (no
  mutation run is part of a chunk, `inputs#I1`).
