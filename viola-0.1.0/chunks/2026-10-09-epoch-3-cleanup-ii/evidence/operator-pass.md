# Operator pass — 2026-10-09-epoch-3-cleanup-ii

The implementer drove this pass on the operator's word, given with the implement invocation ("Run the operator
pass with the ci.py conclusion read (leg=operator) as usual, reading the attempt number: the final sha needs green
on its first attempt", `inputs#I3`). Times are `date -u`; the pass ran on 2026-10-10.

## Before it — the block, 01:09:17Z to 01:11Z

The whole block read green in one call of the gate tool, on the tree the three witness runs had measured (HEAD
`14f1fb5f588d` plus the chunk's uncommitted edits): 21 entries, 18 green, 0 red, 3 not run. Entries 19 to 21 are
this pass. No push went out before every entry of the block read green by its own letter.

The readings of that firing:

- entries 1 and 2: `cargo fmt --all --check` and clippy with `-D warnings`, exit 0;
- entry 3: unit 1484 of 1484; entry 4: unit 1484, integration 354 of 354 (base: 1471 and 353);
- entry 5, the count probe: exit 1, last line `0` (19 at the base commit);
- entries 6 to 8, the instrument: `selftest: 0 mismatches · 9 duplication cases · 6 complexity cases`;
  `duplication: pct 2.84 · duplicated 1533 of 53987 · clones 187 · named fragments 0`;
  `complexity: over_ceiling 1 · dialog_variants 10 · record 2 · submit 11 · files 126 of 126`;
- entry 9, the preservation guard against `14f1fb5f588d`: exit 0;
- entries 10 and 11, the survivor reader and the score reader: each printed `true`, exit 0;
- entries 12 to 14 and 17, the smoke session `p-e3c2-smoke`: pre-clean `cleaned:[]`; boot `"ok":true` in 4.8 s on
  the split fake agent; status `state:"ready"`; cleanup `processes_gone:true`, `endpoint_gone:true`,
  `home_removed:true`;
- entry 15: `g2: clean`; entry 16: schema-check 145 files, 1952 lines, 0 torn, no failure;
- entry 18, `pre-push`: `"ok":true`, `"stage":"linux-tests"`; coverage 1838 of 1838, doctest 0 of 0, playwright
  1 of 1, `gate` no breaches; 56.89 s.

The host, read before the block (`hostwatch.py read --last 10 --for viola`, 01:09:17Z): verdict `QUIET`, 0 s
stalled on IO; load average 2.3. The homes' backing read a link on `tmpfs` after each mutation run. The process
list read after the block held no process of this repository (two `viola`-named processes belonged to other trees,
decided by their executable and working directory).

The scope read after the block (`gate.py scope`): `scope: clean — changed 20 · listed 20 · recorded 0`, base HEAD.
No scope record was needed: every edited file is in research's two lists.

## Before the pass — `pre-push` (entry 18) on the uncommitted tree, 01:12:55Z to 01:13:59Z

- `bash scripts/agent-run.sh pre-push` through the gate tool (`--entry 18`): green, exit 0, 63.73 s. Its document:
  `"ok":true`, `"stage":"linux-tests"`; coverage 1838 of 1838, doctest 0 of 0, playwright 1 of 1, `gate` no
  breaches. Atoms: `exit 0` ✓, `contains "ok":true` ✓, `contains "stage":"linux-tests"` ✓.
- Load average at its end: 67.2 over one minute (20.9 over five): the coverage run itself beside the host's other
  builders. It read green under it.
- No product or test source changed between the block's firing and this one; the files written between them are
  the run dir's trails, the friction ledger and the operator's word snapshotted as `inputs#I3`.

## Entry 19 — hygiene (by hand)

- Read at 01:14:03Z:
  `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` → exit 0,
  `hygiene: clean — read 66 (runs 47 · evidence 14 · inputs 5) · trails 14 not read · copies 3 not read by P1 — 0
  host paths kept · binary 0 not read by P1`. Atoms: `exit 0` ✓, `contains hygiene: clean` ✓. No row to rewrite.
- One thing was moved before this read, so that it would read clean: the four mutation runs' stderr files, which
  the plan filed under the run dir. Each opens with cargo's own build lines, and those carry the repository's
  absolute path. They are a tool's output saved verbatim, so they went to the session scratchpad; their outcome
  lines are in `evidence/` (`e2e-score-outcomes.txt` and the three `witness-*-outcomes.txt`) and their other lines
  are quoted in `e2e-score.md` and the run journal.
- Read once more after this file was added, before the commit: the verdict is in the next section's first line.

## Before the pre-CI commit, 01:14:32Z

- Hygiene re-read: `hygiene: clean — read 67 (runs 47 · evidence 15 · inputs 5) · trails 14 not read · copies 3 not
  read by P1 — 0 host paths kept · binary 0 not read by P1`, exit 0, this file now among the evidence.
- The scope read again: `scope: clean — changed 20 · listed 20 · recorded 0`, base HEAD `14f1fb5f588d`.
- The tree the commit takes: the take-up's products (the stamped route line, the master's pending record, the
  chunk folder, the phase run dir), the 18 changed and 2 new source and test files, this chunk's evidence and
  inputs, this implement run dir and the bookkeeping the tree carried (94 files). 0 ahead of the upstream before
  it; the remote branch head read live (`git ls-remote`) at `14f1fb5f588d`.
- This part of the file and everything below it was written after the commit; it rides the next commit.

## The pre-CI commit and entry 20 — the push, 01:14:39Z to 01:14:46Z

- `632f6a7` `chore(2026-10-09-epoch-3-cleanup-ii): operator pre-CI commit, for the run this chunk's verdict reads`
  at 01:14:39Z (the whole tree, 94 files).
- Entry 20: `git diff --quiet && git diff --cached --quiet && git push origin HEAD` → exit 0 at 01:14:46Z,
  `14f1fb5..632f6a7  HEAD -> build/viola-0.1.0`; 0 ahead of the upstream after it, the remote branch head read
  live at `632f6a7edf29`. No force push.

## Entry 21 — the CI read, 01:15:03Z to 01:22:49Z

- `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait
  1800` → exit 0: `632f6a7edf29 verdict: green · checks 15/15 · wall 447 s · runs ci#38012420489
  completed/success`, polled 16 times over 466 s. Atoms: `exit 0` ✓, `contains verdict: green` ✓.
- `run_attempt`, read from the run itself (`gh api …/actions/runs/38012420489`): **1**. Event `push`, head
  `632f6a7edf29`, started 01:14:48Z, last updated 01:22:19Z, conclusion `success`. It is the only run on the sha
  (`gh run list --commit`, by the full sha). The final sha is green on its first attempt.
- Its fifteen jobs, each `success` at attempt 1: `lint`, `test`, `release` and `perf` on `windows-2025`,
  `macos-latest` and `ubuntu-latest`; `supply-chain`, `msrv`, `fuzz-replay`.
- The three `test` jobs' own summaries, read from their logs: `ubuntu-latest` 1838 tests run, 1838 passed;
  `windows-2025` 1861 run, 1861 passed; `macos-latest` 1834 run, 1834 passed; none skipped.
- The new cases on the two runners this chunk could not measure before, each read as its own `PASS` line in that
  job's log:
  - `windows-2025`: 13 of the 14. The four NUL-byte cases pass there
    (`events_current_len_of_a_log_that_cannot_be_statted_is_an_error`,
    `events_read_of_a_log_that_cannot_be_opened_is_an_error`,
    `read_stamps_of_a_file_that_cannot_be_opened_is_an_error`,
    `check_stamps_of_a_path_that_cannot_be_statted_is_unreadable`), and so do the three fake-agent cases (the host
    program is `%SystemRoot%\System32\whoami.exe` there), the frame-bound case and the cross-process case. The
    fourteenth, the read-only directory case, is `cfg(unix)` and does not exist on that runner.
  - `macos-latest`: all 14, the read-only directory case included.
  - A pass of a NUL-byte case says the call failed with a kind other than `NotFound` on that OS: with `NotFound`
    the code under test returns its absent value and the case fails. Which kind it was on each OS was not read.

## After the pass

- No fix commit was made: `632f6a7` is the final sha of the pass.
- The tree after it carries, for the next commit: this record's sections written after the commit, one sentence
  added to `evidence/survivors.md` about the runners, the run journal's last entries, and the gate trail and
  ledger lines written after the commit. No source file is among them.
