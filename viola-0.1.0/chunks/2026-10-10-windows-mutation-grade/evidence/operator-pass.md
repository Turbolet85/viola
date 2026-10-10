# Operator pass — 2026-10-10-windows-mutation-grade

The implementer drove this pass on the operator's word, given with the implement invocation ("Run the operator
pass up to the ordinary CI read (ci.py conclusion, leg=operator), reading the attempt number", `inputs#I4`). Times
are `date -u`; the pass ran on 2026-10-10.

**The pass ends at the ordinary CI read. It fires no dispatch.** `windows-mutants.yml` was not dispatched, and the
block's last entry (`ci.py conclusion --sha HEAD --name mutants --wait 5400`) was not driven: step 11 holds both
until the founder's own word, given after this dispatch was shown to him, reaches the implementer through the
operator (`inputs#I3`, `inputs#I4`). No such word is recorded here, because none was given.

## Before it — the block, 03:28:58Z to 03:31:09Z

The whole block read green in one call of the gate tool, on the tree the three witness runs had measured (HEAD
`781563cd59a6` plus the chunk's uncommitted edits): 26 entries, 22 green, 0 red, 4 not run. Entries 23 to 26 are
the operator's; 23 to 25 are this pass and 26 is held. Fourteen of the 22 had also read green in a first firing at
03:02Z, ahead of the witness runs (entries 1 to 13 and 15). No push went out before every entry read green.

The readings of the whole-block firing:

- entries 1 to 3: `cargo fmt --all --check`; clippy with `-D warnings`; clippy of viola-state, viola-pty and
  viola-e2e with their tests for `x86_64-pc-windows-msvc` in `target/wincheck`; each exit 0. Entry 3 shows the
  Windows-gated test code compiles and lints. It runs no Windows test;
- entry 4: unit 1502 of 1502; entry 5: unit 1502, integration 355 of 355;
- entry 6, the workflow contract test: 4 of 4, the label case among them;
- entry 7: `zizmor .github/workflows/`, exit 0, no findings; entry 8, the forbidden-key probe: exit 0, no output;
- entries 9 to 11, the counts: four `viola` items; one `timeout-minutes: 120`; one "dispatched only during the";
- entry 12, the preservation guard against `781563cd59a6`: exit 0; entry 13, no `#[ignore]`, retry or skip
  added: exit 0, no output;
- entry 14, the reader of `linux-witness.json`: printed `true`, exit 0;
- entry 15: `cargo deny check`, exit 0, with syn and proc-macro2 as direct viola-e2e dependencies and no new
  ignore;
- entries 16 to 18 and 21, the smoke session `p-wmg-smoke`: pre-clean `cleaned:[]`; boot `"ok":true` in 10.2 s;
  status `state:"ready"`; cleanup `processes_gone:true`, `endpoint_gone:true`, `home_removed:true`, `killed:[]`;
- entry 19: `g2: clean`; entry 20: schema-check 145 files, 1952 lines, 0 torn, no failure;
- entry 22, `pre-push`: `"ok":true`, `"stage":"linux-tests"`; coverage 1857 of 1857, doctest 0 of 0, playwright
  1 of 1, `gate` no breaches; 62.57 s.

The host over the block's window (`hostwatch.py read --from 03:28:58Z --to 03:31:09Z --for viola`): verdict
`QUIET`, 0 s stalled on IO. The process list read after the block held no process of this repository: seven
processes named `viola` were another tree's build, decided by `/proc/<pid>/exe`.

The scope read after the block (`gate.py scope`): `scope: clean — changed 11 · listed 11 · recorded 0`, base HEAD.
No scope record was needed: every edited file is in research's two lists.

## Step 1 — `pre-push` (entry 22) on the uncommitted tree, 03:31:59Z to 03:32:55Z

- `bash scripts/agent-run.sh pre-push` through the gate tool (`--entry 22`): green, exit 0, 56.06 s. Its document:
  `"ok":true`, `"stage":"linux-tests"`; coverage 1857 of 1857, doctest 0 of 0, playwright 1 of 1, `gate` no
  breaches. Atoms: `exit 0` ✓, `contains "ok":true` ✓, `contains "stage":"linux-tests"` ✓.
- No source or test file changed between the block's firing and this one (the last source edit is 03:00:46Z); the
  files written between them are the friction ledger and the gate trail.

## Step 2 — entry 23, hygiene (by hand)

- Read at 03:33:00Z:
  `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` → exit 0,
  `hygiene: clean — read 76 (runs 61 · evidence 9 · inputs 6) · trails 15 not read · copies 4 not read by P1 — 0
  host paths kept · binary 0 not read by P1`. Atoms: `exit 0` ✓, `contains hygiene: clean` ✓. No row to rewrite.
- The three mutation runs' stderr files were never filed under the run dir or `evidence/`: each opens with
  cargo's own build lines, which carry the repository's absolute path. They stay under the ignored
  `target/witness-wmg/`; their outcome lines are the three `*-outcomes.txt` files here.
- Read once more after this file was added, right before the commit: the verdict is in the next section.

## Before the pre-CI commit, 03:33:30Z

- Hygiene re-read: `hygiene: clean — read 77 (runs 61 · evidence 10 · inputs 6) · trails 15 not read · copies 4 not
  read by P1 — 0 host paths kept · binary 0 not read by P1`, exit 0, this file now among the evidence.
- The scope read again: `scope: clean — changed 11 · listed 11 · recorded 0`, base HEAD `781563cd59a6`.
- The tree the commit takes: the take-up's products (the stamped route line, the master's pending record, the
  chunk folder, the phase run dir), the 10 changed and 1 new source, test, manifest and workflow files, this
  chunk's evidence and inputs, this implement run dir and the bookkeeping the tree carried. 0 ahead of the
  upstream before it; the remote branch head read live (`git ls-remote`) at `781563cd59a6`.
- This part of the file and everything below it was written after the commit; it rides the next commit.

## Step 3 and step 4 — the pre-CI commit and entry 24, the push, 03:33:36Z to 03:33:44Z

- `dd5161d` `chore(2026-10-10-windows-mutation-grade): operator pre-CI commit, for the run this chunk's verdict
  reads` at 03:33:36Z (the whole tree, 95 files).
- Entry 24: `git diff --quiet && git diff --cached --quiet && git push origin HEAD` → exit 0 at 03:33:44Z,
  `781563c..dd5161d  HEAD -> build/viola-0.1.0`; 0 ahead of the upstream after it, the remote branch head read
  live at `dd5161d55743`. No force push.

## Step 5 — entry 25, the CI read, 03:33:48Z to 03:45:10Z

- `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait
  1800` → exit 0: `dd5161d55743 verdict: green · checks 15/15 · wall 660 s · runs ci#38021000200
  completed/success`, polled 23 times over 682 s. Atoms: `exit 0` ✓, `contains verdict: green` ✓.
- `run_attempt`, read from the run itself (`gh api …/actions/runs/38021000200`): **1**. Event `push`, head
  `dd5161d55743`, started 03:33:46Z, last updated 03:44:49Z, conclusion `success`. It is the only run on the sha
  (`gh run list --commit`, by the full sha). The sha is green on its first attempt.
- Its fifteen jobs, each `success` at attempt 1: `lint`, `test`, `release` and `perf` on `windows-2025`,
  `macos-latest` and `ubuntu-latest`; `supply-chain`, `msrv`, `fuzz-replay`. The checks hold no `mutants` row:
  no dispatch has joined this sha.
- The three `test` jobs' own summaries, read from their logs (`gh api …/jobs/<id>/logs
  --allow-escape-sequences`, escapes stripped): `windows-2025` 1886 tests run, 1886 passed; `macos-latest` 1853
  run, 1853 passed; `ubuntu-latest` 1857 run, 1857 passed; none skipped, and no `FAIL`, `TIMEOUT` or `LEAK` status
  line in any of the three. Before this chunk they read 1861, 1834 and 1838: 19 more on every runner, and 6 more
  that exist on `windows-2025` alone.
- In `test (windows-2025)` the steps `G2 zero panics`, `G4 schema conformance`, `Secret scan` and `Gate verdict`
  each concluded `success`.

### The first run of the Windows-gated cases, each read as its own `PASS` line in `test (windows-2025)`

The six new cases that exist on that runner alone:

- `strict::win::tests::owner_and_dacl_is_the_named_flags`
- `strict::win::tests::persistent_acls_of_a_volume_that_cannot_be_read_is_none`
- `strict::win::tests::check_stamps_refuses_a_ledger_folder_everyone_may_write`
- `strict::win::tests::check_stamps_refuses_a_stamps_file_everyone_may_write`
- `fs::tests::protected_dacl_is_the_named_flags`
- `fs::tests::set_dacl_refuses_an_sddl_that_carries_no_dacl`

Two older Windows-only cases whose code path the chunk changed passed there too:
`check_stamps_of_a_home_under_the_workspace_target_passes` and
`create_private_dir_outside_the_profile_sets_the_protected_owner_only_dacl` (the protected DACL is now set
through `set_dacl` with the literal flags).

The cases that run on every OS passed on all three runners: the two viola-pty rig cases
(`console_read_keeps_a_ctrl_z_and_goes_on_to_the_next_read`, `piped_stdin_is_read_as_the_bytes_written_to_it`),
the five `volume_keeps_acls` cases, `kill_deadline_lies_five_seconds_after_the_instant_it_is_given`,
`check_stamps_of_a_path_that_cannot_be_statted_is_unreadable`, `workflow_job_labels_are_distinct_one_per_item`,
the seven `harness::run::mutants::host` cases and the three new `harness::run::mutants` cases.

What these passes say, and what they do not:

- they say the five runner-only readings the tests rest on hold on `windows-2025` for the unmutated code
  (`survivors.md`, under the first table): the widened DACL reads `Writable`; a drive letter with no volume reads
  `None`; a descriptor from `O:SY` is refused, so the DACL pointer was left null; a lone `^Z` reached the raw
  child as a read of `1a` and the reads went on; a piped stdin was read as its bytes;
- they do not say any mutant is killed on Windows. That each test fails under its mutant is the reading of a
  `windows-mutants.yml` dispatch, which step 11 holds.

## After the pass

- No fix commit was made: `dd5161d` is the final sha of the pass.
- `windows-mutants.yml`: not dispatched. Its newest run is still 37761947926 of 2026-10-08 on `e304994` (`gh run
  list --workflow windows-mutants.yml`). Entry 26 was not driven.
- The tree after the pass carries, for the next commit: this record's sections written after the commit, the gate
  trail and the ledger lines written after it. No source file is among them.

## Step 11 — the founder's word, recorded before anything was fired (2026-10-10T08:01Z)

Everything above this section stands as written at 03:46Z: at that time no word had been given. This section was
written in a later session, on the same tree and the same pushed commit.

- **The word**, the founder's own, given live in the overseer dialog at 2026-10-10T07:58:27Z, after this dispatch
  was shown to him with three options priced (no dispatch, the next boundary measures; one or two dispatches now;
  a general rule for corrective chunks): «Разрешить 1–2 запуска сейчас» ("Allow 1 to 2 runs now").
- **How it reached the implementer:** relayed verbatim by the operator in the `/andromeda-implement` invocation
  of this session. Written whole to this run's `relay-1.md` and snapshotted at 08:01Z as **`inputs#I5`**
  (`inputs/I5-relay-1.md.txt`). The implementer did not see the overseer dialog itself; the snapshot holds the
  operator's relay of it.
- **Its bounds, as the operator stated them with it:** one or two dispatches of `windows-mutants.yml`, for this
  chunk only; a third comes back to the founder. A red job is read from its log before any second dispatch. It
  amends no sentence of a master that states ruling C2, and it is not a rule for other chunks.
- **Read before the dispatch, 08:00:52Z:** HEAD `dd5161d55743`; the remote branch head, read live
  (`git ls-remote origin build/viola-0.1.0`), `dd5161d55743`; the push's own CI verdict on that sha was read green
  at step 5 above, before any dispatch joined its checks. The workflow's two earlier runs are 37761947926
  (2026-10-08, `e304994`) and 37174673472 (2026-10-04, `60c569b`); none exists on `dd5161d`. The working tree
  holds no source change: its uncommitted files are this record, `survivors.md`, the gate trail, the handoff's
  session-end line, the friction ledger, this session's run dir and the `I5` snapshot.
- Nothing had been dispatched when this section was written. The dispatch and entry 26 are recorded in
  `windows-dispatch.md`.

## Step 11 — the dispatch and entry 26, 08:01:33Z to 08:51:18Z

- The dispatch, the first of at most two: `gh workflow run windows-mutants.yml --ref build/viola-0.1.0` → exit 0 at
  08:01:33Z, run 38036448183 (`workflow_dispatch`, head `dd5161d55743`, attempt 1). No second dispatch was made.
- Entry 26, driven once by hand: `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/ci.py
  conclusion --sha HEAD --name mutants --wait 5400` → exit 0 at 08:51:18Z: `dd5161d55743 verdict: green · checks
  9/24 · wall 2961 s`, polled 97 times over 2975 s. Atoms: `exit 0` ✓, `contains verdict: green` ✓.
- All nine jobs `success` at attempt 1; the run `completed` / `success` at 08:50:59Z. Each job's document and
  counts are in `windows-dispatch.md`, the grades of the sixteen in `survivors.md`.
- No commit, no push and no fix was made in this step: `dd5161d` is still the head, local and remote. The tree
  carries no source change.
