# Operator pass — 2026-10-09-inner-cr-and-crlf-in-a-sent-text

The implementer drove this pass on the operator's word, given with the implement invocation of the re-entry ("the
section reading, the whole block fired, then the operator pass with the ci.py conclusion read (leg=operator),
reading the attempt number: the final sha needs green on its first attempt", `inputs#I6`; the same word stood for
the first run, `inputs#I3`, and at the revision, `inputs#I5`). Times are `date -u`, 2026-10-09.

Before it, the re-entry read the tree as the stopped run's (19:29:28Z: HEAD `3c5e012b9a43`, the plan's five files
modified and no other file under `src`, `crates` or `tests`) and the block read green on it in one call of the
gate tool (19:29:37Z to 19:31:48Z): 23 entries, 20 green, none red, 3 not run. Entries 21 to 23 are this pass.
Entry 7, the preservation guard that read red at the stopped run on `tests/channel_paste_validation.rs`, read
green at this first firing after the revision. The three readers of the live records (entries 17 to 19) each
printed `true` with exit 0. No push went out before every entry of the block read green by its own letter.

The counts of that firing: unit 1471 of 1471 (entry 3); the new unit cases 18 of 18 (entry 4); the default
selection unit 1471 and integration 353 of 353 (entry 5); the new cross-process case 3 of 3 (entry 6); the
reader's `selftest: 16 cases, 0 mismatches` (entry 8); `release-check: viola only` with the artifact fresh (entry
9); the smoke session `ready` and its cleanup with `processes_gone` and `endpoint_gone` true (entries 10 to 12 and
15); `g2: clean` (entry 13); schema-check 118 files, 1672 lines, no failure (entry 14); `2.1.287 (Claude Code)` by
path (entry 16).

Step 12's hand reading stood between the block and this pass (19:32:06Z): the rebuilt file reads sha256 prefix
`4057b91d84010eca`, 4 000 416 B, as predicted, and each of its six loaded sections hashes equal to the live
start's build; stop rule S6 did not fire (`live-preconditions.md`, its last section). The two file hashes differ,
and that difference stays recorded there as not explained. No live start was made by the re-entry, and no
rehearsal.

The host, read before the block (`hostwatch.py read --last 15 --for viola`, 19:14:28Z to 19:29:28Z): verdict
`QUIET`, 0 s stalled on IO, load peak 18.2, mean 6.5. The homes' backing read a link on `tmpfs`.

## Before the pass — `pre-push` (entry 20) on the uncommitted tree, 19:32:55Z to 19:33:54Z
- `bash scripts/agent-run.sh pre-push` through the gate tool (`--entry 20`): green, exit 0, 58.9 s. Its document:
  `"ok":true`, `"stage":"linux-tests"`; coverage 1824/1824, doctest 0/0, playwright 1/1, `gate` no breaches.
  Atoms: `exit 0` ✓, `contains "ok":true` ✓, `contains "stage":"linux-tests"` ✓. Load average at its end: 9.18.
- The same entry in the block's whole run (ended 19:31:48Z): green, 58.02 s, the same counts. No product or test
  source changed between the two; the files written between them are this chunk's evidence and the run dir's
  trails.

## Entry 21 — hygiene (by hand)
- Read at 19:34:02Z:
  `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` → exit 0,
  `hygiene: clean — read 73 (runs 53 · evidence 12 · inputs 8) · trails 25 not read · copies 6 not read by P1 — 0
  host paths kept · binary 0 not read by P1`. Atoms: `exit 0` ✓, `contains hygiene: clean` ✓. No row to rewrite.
- Read once more after this file was added, before the commit: the verdict is in the next section's first line.

## Before the pre-CI commit, 19:34:22Z
- Hygiene re-read: `hygiene: clean — read 74 (runs 53 · evidence 13 · inputs 8) · trails 25 not read · copies 6
  not read by P1 — 0 host paths kept · binary 0 not read by P1`, exit 0, this file now among the evidence.
- The scope read (`gate.py scope`): `scope: clean — changed 5 · listed 5 · recorded 0`, with
  `record: listed — tests/channel_paste_validation.rs` (the stopped run's `companion` line, the file now in
  research's list). Its base is HEAD, `3c5e012b9a43`.
- The tree the commit takes: the take-up's and the revision's products, the five changed source and test files,
  this chunk's evidence and inputs, the four run dirs and the bookkeeping the tree carried (88 files). 0 ahead of
  the upstream before it; the remote branch head read live (`git ls-remote`) at `3c5e012b9a43`.
- The commit, the push (entry 22) and the CI read (entry 23) are recorded below after they are made; that part of
  this file rides the next commit.

## The pre-CI commit and entry 22 — the push, 19:34:39Z to 19:34:47Z
- Hygiene read a last time right before the commit (19:34:39Z): `hygiene: clean`, read 74, the same counts.
- `308099b` `chore(2026-10-09-inner-cr-and-crlf-in-a-sent-text): operator pre-CI commit, for the run this chunk's
  verdict reads` at 19:34:39Z (the whole tree, 88 files).
- Entry 22: `git diff --quiet && git diff --cached --quiet && git push origin HEAD` → exit 0 at 19:34:47Z,
  `3c5e012..308099b  HEAD -> build/viola-0.1.0`; 0 ahead of the upstream after it, the remote branch head read
  live at `308099b92f96`. No force push.

## Entry 23 — the CI read, 19:34:51Z to 19:47:17Z
- `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait
  1800` → exit 0: `308099b92f96 verdict: green · checks 15/15 · wall 717 s · runs ci#37981185305
  completed/success`, polled 25 times over 746 s. Atoms: `exit 0` ✓, `contains verdict: green` ✓.
- `run_attempt`, read from the run itself (`gh api …/actions/runs/37981185305`): **1**. Event `push`, head
  `308099b92f96`, started 19:34:49Z, last updated 19:46:50Z, conclusion `success`. It is the only run on the sha
  (`gh run list --commit`, by the full sha). The final sha is green on its first attempt.
- Its fifteen jobs, each `success` at attempt 1: `lint`, `test`, `release` and `perf` on `windows-2025`,
  `macos-latest` and `ubuntu-latest`; `supply-chain`, `msrv`, `fuzz-replay`.
- The run's wall read 717 s; the last chunk's read 416 s. Which job took the longer time was not read.

## After the pass
- No fix commit was made: `308099b` is the final sha of the pass.
- The tree after it carries, for the next commit: this record's last sections, written after the commit, and the
  run dir's trails and ledger lines written after it. No source file is among them.
