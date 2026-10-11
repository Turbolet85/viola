# Operator pass — 2026-10-10-session-links (Part A)

The implementer drove this pass on the operator's word, given with the implement invocation ("Then run the
operator pass for Part A with the ci.py conclusion read (leg=operator) as usual; fold a red into a fix commit
and read CI again; report the run ids", `inputs#I4`). It is plan step 16, which step 11 does not hold. The tree
it takes to CI is Part A alone: steps 1 to 10. Steps 12 to 15 are held by step 11 and nothing of them is built:
no `src/cmd/link.rs`, no `src/cmd/unlink.rs`, no `tests/channel_links.rs`, no dispatch arm for `link` or
`unlink`. Times are `date -u` readings or the tools' own; the pass ran on 2026-10-11. No mutation run, no
workflow dispatch and no start of the real `claude` is part of it (`inputs#I1`, `inputs#I4`).

## Before it — the block, fired once by this implement run

One whole firing (`gate.py run --plan … --run-dir .andromeda/runs/2026-10-11T00-04-43-implement --marker
2026-10-10-session-links`, backgrounded), on HEAD `f02d54865815` plus the chunk's uncommitted edits. Its entry
lines and summary line, as printed:

```
  1 lint        green · exit 0 · 0.77s · 0 B → 1.log · cargo fmt --all --check
  2 lint        green · exit 0 · 0.13s · 72 B → 2.log · cargo clippy --workspace --all-targets --features fake-age… (75 chars)
  3 unit        green · exit 0 · 8.31s · 247395 B → 3.log · bash scripts/agent-run.sh run --unit
  4 unit        green · exit 0 · 1.06s · 5930 B → 4.log · bash scripts/agent-run.sh run --unit --filter 'test(/link_… (101 chars)
  5 integration green · exit 0 · 5.12s · 4799 B → 5.log · bash scripts/agent-run.sh run --integration --filter 'bina… (139 chars)
  6 integration red · exit 0 ✗ (exit 1) · exit 1 · 0.57s · 1577 B → 6.log · bash scripts/agent-run.sh run --integration --filter 'bina… (76 chars)
  7 probe       green · exit 0 · 0.01s · 0 B → 7.log · git diff --quiet f02d54865815 -- crates/viola-agent-claude… (673 chars)
  8 smoke       green · exit 0 · 0.08s · 47 B → 8.log · bash scripts/agent-run.sh cleanup --session p-links-smoke
  9 smoke       green · exit 0 · 5.45s · 798 B → 9.log · bash scripts/agent-run.sh boot --session p-links-smoke --i… (73 chars)
 10 smoke       green · exit 0 · 0.09s · 216 B → 10.log · bash scripts/agent-run.sh status --session p-links-smoke
 11 probe       green · exit 0 · 0.02s · 10 B → 11.log · bash scripts/g2-zero-panics.sh
 12 probe       green · exit 0 · 0.31s · 108 B → 12.log · bash scripts/agent-run.sh schema-check
 13 smoke       green · exit 0 · 0.3s · 178 B → 13.log · bash scripts/agent-run.sh cleanup --session p-links-smoke
 14 probe       green · exit 0 · 63.12s · 300467 B → 14.log · bash scripts/agent-run.sh pre-push
 15 probe       not run — leg operator (the letter drives it) · python -X utf8 ~/.claude/skills/andromeda-phase/../androme… (90 chars)
 16 probe       not run — leg operator (the letter drives it) · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
 17 probe       not run — leg operator (the letter drives it) · python -X utf8 ~/.claude/skills/andromeda-phase/../androme… (114 chars)
entries 17 · green 13 · red 1 (6) · recorded 0 · timeout 0 · not-run 3
```

What the logs of that firing hold:

- entry 3: unit 1899 of 1899; entry 4: the 34 inline cases the four prefixes select, 34 passed; entry 5: the
  four named binaries, 28 of 28;
- **entry 6 is red, and that red is the hold.** It selects `binary(channel_links)`, a file of step 15. Its log
  reads nextest's own refusal, `operator didn't match any binary names` under `binary(channel_links)`, then
  `failed to parse filterset`: the text of the entry's baseline. The plan's Test Commands prose says the entry
  reads red for as long as step 11 holds, and that this red is the hold and nothing else. No test ran in it;
- entry 7, the preservation guard against `f02d54865815`: exit 0, no output;
- entries 8 to 10 and 13, the smoke session `p-links-smoke`: pre-clean `cleaned:[]`; boot `"ok":true` with one
  instance `builder`; status `state:"ready"`; cleanup `processes_gone:true`, `endpoint_gone:true`,
  `home_removed:true`, `killed:[]`;
- entry 11: `g2: clean`; entry 12: schema-check 160 files, 2220 lines, 0 torn, no failure. On this host that
  read does not reach the new cases' lines: a root test's home is removed when the test ends, so the entry read
  the booted smoke home and the kept live homes (the same two figures the previous chunk's entry read). The new
  root cases validate their own role lines against `schemas/diag-line.v1.json` in the test
  (`tests/cli_links.rs` `assert_logs_clean`); CI keeps its test homes and its G4 step reads them.

Before the block, two hand probes: `cargo fmt --all --check` exit 0, and `cargo clippy --workspace --all-targets
--features fake-agent -- -D warnings`, which read one finding in a new test
(`clippy::cloned_ref_to_slice_refs`, `crates/viola-state/tests/state_replay.rs`), fixed before the block; the
re-run exited 0.

The Windows lint, on this host, after that fix: `cargo clippy --workspace --all-targets --features fake-agent
--target x86_64-pc-windows-msvc -- -D warnings` with `CARGO_TARGET_DIR=target/wincheck` (check only, nothing
linked or run) exited 0. It runs no test.

The scope read (`gate.py scope`, 00:20:41Z): `scope: clean — changed 13 · listed 13 · recorded 0 (companion 0 ·
mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 59`, base HEAD. No `scope-record.md` exists and
none is needed.

## Step 1 — `pre-push` (entry 14) on the uncommitted tree, 00:21:09Z to 00:22:12Z

- `bash scripts/agent-run.sh pre-push` through the gate tool (`--entry 14`, this run's dir), its second firing
  on this tree; no source, test or config file changed between the two:

  ```
   14 probe       green · exit 0 · 63.1s · 300467 B → 14.2.log · bash scripts/agent-run.sh pre-push
  entries 17 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 16
  ```

  Its document: `"ok":true`, `"stage":"linux-tests"`; coverage 2321 of 2321, doctest 0 of 0, playwright 1 of 1,
  `gate` no breaches.
- The coverage summary it left (`target/agent-run/artifacts/llvm-cov-summary.json`, written 00:22:10Z, inside
  the run's window) reads lines 98.08 (10218 of 10418), functions 97.99 (1221 of 1246), regions 97.89 (19922 of
  20351). The thresholds are 85, 95 and 80.
- The process list read at 00:22:17Z, every process by its command line: none under this repository's
  `target/` and none under the test-home base.

## Step 2 — entry 15, hygiene (fired as written)

- Read at 00:22:17Z:
  `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` → exit 0,
  `hygiene: clean — read 51 (runs 44 · evidence 1 · inputs 6) · trails 15 not read · copies 4 not read by P1 —
  0 host paths kept · binary 0 not read by P1`, base HEAD. Atoms: `exit 0` ✓, `contains hygiene: clean` ✓. No
  row to rewrite.
- Read once more after this file was added, right before the commit: the verdict is in the next section, which
  was written after the commit and rides the next one.

## Before the pre-CI commit, 00:22:52Z

- Hygiene re-read: `hygiene: clean — read 52 (runs 44 · evidence 2 · inputs 6) · trails 15 not read · copies 4
  not read by P1 — 0 host paths kept · binary 0 not read by P1`, exit 0, this file among the evidence.
- The tree the commit takes, 73 paths by `git status --porcelain --untracked-files=all`: the take-up's products
  (the stamped route line, the master's pending record, the ledger notes on `v1-13` and `v1-36`, the chunk
  folder, the phase run dir), the 11 changed and 2 new source, test and config files, this chunk's evidence
  (the red and green readings, this record) and inputs I1 to I4, this implement run dir, the friction ledger,
  the last wrap's evolve trail and gate script, and the handoff's session-end stamp. 0 ahead of the upstream
  before it; the remote branch head read live (`git ls-remote`) at `f02d54865815`.
- This section and everything below it was written after the commit; it rides the next commit.

## Step 3 and step 4 — the pre-CI commit and entry 16, the push, 00:22:56Z to 00:23:04Z

- `3cae2ce` `chore(2026-10-10-session-links): operator pre-CI commit, for the run this chunk's verdict reads`
  at 00:22:56Z (the whole tree, 73 files; 13 of them under `src`, `crates`, `tests` or `.config`).
  `git status --short` read empty after it.
- Entry 16, fired through the gate tool with no run dir (`gate.py run --plan
  viola-0.1.0/chunks/2026-10-10-session-links/plan.md --operator 16`), 00:23:02Z to 00:23:04Z. A `--dry-run` of
  the same call just before it fired nothing and printed the same tripwire line. The firing's tripwire line,
  entry line and summary line, as printed:

  ```
  operator entry 16 · history tripwire: git
   16 probe       green · exit 0 · 1.9s · 90 B → 16.log · history moved: refs/remotes/origin/HEAD f02d5486→3cae2cec; refs/remotes/origin/build/viola-0.1.0 f02d5486→3cae2cec · git diff --quiet && git diff --cached --quiet && git push … (69 chars)
  entries 17 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 16
  ```

  The entry carries no `expect` key, so its line asserts `exit 0` alone. Its log
  (`$TMPDIR/andromeda-gate/2026-10-10-session-links/run-20261011T002302Z/16.log`, 90 B) holds git's own two
  lines, the second `f02d548..3cae2ce  HEAD -> build/viola-0.1.0`.
- The history reading is the move the entry is for: the two remote-tracking refs went from `f02d5486` to
  `3cae2cec`, a fast-forward. No local branch, tag or stash moved. No force push.
- After it: 0 ahead of the upstream; the remote branch head read live at `3cae2ceca8f0`; the tree clean.

## Step 5 — entry 17, the CI read (fired as written), 00:23:09Z to 00:31:25Z: green

- `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait
  1800` → exit 0: `3cae2ceca8f0 verdict: green · checks 15/15 · wall 491 s · runs ci#38098238731
  completed/success`, polled 17 times over 496 s. Atoms: `exit 0` ✓, `contains verdict: green` ✓.
- `run_attempt`, read from the run itself (`gh api …/actions/runs/38098238731`): **1**. Event `push`, head
  `3cae2ceca8f0`, started 00:23:06Z, last updated 00:31:21Z, conclusion `success`. It is the only run on the sha
  (`gh run list --commit`, by the full sha). The sha is green on its first attempt, so no fix commit was made.
- Its fifteen jobs, each `success`: `lint`, `test`, `release` and `perf` on `windows-2025`, `macos-latest` and
  `ubuntu-latest`; `supply-chain`, `msrv`, `fuzz-replay`. `test (windows-2025)` was the last to finish.
- The three `test` jobs' own summaries, read from their logs (`gh api …/jobs/<id>/logs
  --allow-escape-sequences`, escapes stripped): `windows-2025` 2322 tests run, 2322 passed; `macos-latest` 2317
  run, 2317 passed; `ubuntu-latest` 2321 run, 2321 passed; none skipped, and no `FAIL`, `TIMEOUT`, `LEAK`,
  `SIGKILL` or `ABORT` status line in any of the three (every status line matched by one pattern over the whole
  log: `PASS` 2322, 2317 and 2321, `SLOW` 5 each). On the last chunk's pushed sha they read 2280, 2275 and 2279
  (`c85961d`): 42 more on each runner.
- This chunk's cases, counted as `PASS` lines in each of the three logs:

  | cases | `windows-2025` | `macos-latest` | `ubuntu-latest` |
  |---|---|---|---|
  | `viola::cli_links` | 5 | 5 | 5 |
  | `viola::cli_budget` | 10 | 10 | 10 |
  | `viola-state::state_replay` | 9 | 9 | 9 |
  | `viola-state::state_events` | 4 | 4 | 4 |
  | inline cases carrying one of the unit filter's four prefixes | 34 | 34 | 34 |

  Each count is the whole binary's, the standing cases with the new ones. No new case is OS-gated, and the
  counts are equal on the three runners.
- **The five `cli_links` cases and the folded budget case passed on all three runners**, each from a stamped
  home: 4.4 to 5.1 s on `windows-2025`, 4.6 to 5.4 s on `macos-latest`, 3.7 to 4.4 s on `ubuntu-latest`, under
  the binary's 20 s kill. The slowest on each is `links_survive_a_wrapper_restart` (two boots);
  `path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone` read 4.7, 4.8 and 3.8 s.
- In each `test` job the steps `G2 zero panics`, `G4 schema conformance`, `Secret scan` and `Gate verdict`
  concluded `success`: `g2: clean` on all three; the G4 documents read `windows-2025` 346 files, 3240 lines,
  `macos-latest` 356 files, 3343 lines, `ubuntu-latest` 356 files, 3343 lines, 0 torn and no failure on any. CI
  keeps its test homes, so these are the reads over the lines the new cases wrote. Whether each kept home was
  among the files a step read was not read: no job's artifact was opened.
- In each `lint` job the step `G1 bare instrument` concluded `success`. The step's own output was not opened.

## After the pass

- No fix commit was made: `3cae2ce` is the final sha of the pass, local and remote, and the sha the verdict
  stands on.
- Entry 14 was fired twice (once in the block, once as the pass's step 1), entry 16 once and entry 17 once.
  Entry 15 was fired once as written and re-read before the commit.
- Entry 6 was fired once, in the block, and reads red: the hold of step 11. It was not fired again.
- The tree after the pass carries, for the next commit: this record's sections written after the pre-CI commit.
  No source file is among them.
- Not measured in the pass: any mutation score of the new code (no mutation run is part of a chunk,
  `inputs#I1`); anything against the real `claude` (no live start, `inputs#I1`, `inputs#I4`); anything of
  Part B, which is not built.
