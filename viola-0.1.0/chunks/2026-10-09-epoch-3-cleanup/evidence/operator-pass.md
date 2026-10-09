# Operator pass — 2026-10-09-epoch-3-cleanup

The implementer drove this pass on the operator's word, given with the implement invocation of the re-entry ("Then
the records, the whole block re-fired, and the operator pass with the ci.py conclusion read (leg=operator), reading
the attempt number: the final sha needs green on its first attempt", `inputs#I8`; the same word stood for the first
run, `inputs#I5`). Times are `date -u`, 2026-10-09.

Before it, the block read green on the final tree in one call of the gate tool (16:48:34Z to 16:52:11Z): 27
entries, 24 green, none red, 3 not run. Entries 25 to 27 are this pass. Entries 22 and 23, the two readers of the
live records that were red at the stopped run by their `artifact` key, read green at this first firing after the
revision (`inputs#I7`): each printed `true` with exit 0. No push went out before every entry of the block read
green by its own letter. The live work was over before the block was fired: start 4 was stopped at 16:47:14Z and
the ledger's last census row was written at 16:47:44Z (`live-run.md`).

Three readings of the live work are named in implement's report and are not gates: the CLI submits a text's inner
CR LF as one LF and its lone inner CR as one LF, and in both cases a send that was delivered and answered is
reported not delivered and takes the wheel (`live-readings.ndjson`, `after-inner-crlf` and `after-inner-cr`;
`live-run.md`).

## Before the pass — `pre-push` (entry 24) on the uncommitted tree, 16:53:07Z to 16:54:18Z
- `bash scripts/agent-run.sh pre-push` through the gate tool (`--entry 24`): green, exit 0, 70.89 s. Its
  document: `"ok":true`, `"stage":"linux-tests"`; coverage 1800/1800, doctest 0/0, playwright 1/1, `gate` no
  breaches. Atoms: `exit 0` ✓, `contains "ok":true` ✓, `contains "stage":"linux-tests"` ✓. Load average at its
  end: 32.20 (another project's mutation build stood on the host).
- The same entry in the block's whole run (16:51:01Z to 16:52:10Z): green, 69.09 s, the same counts. No product
  or test source changed between the two; the files written between them are this chunk's evidence and the run
  dir's trails.

## Entry 25 — hygiene (by hand)
- Read at 16:54:24Z:
  `python -X utf8 ~/.claude/skills/andromeda-phase/../andromeda-tools/scripts/gate.py hygiene` → exit 0,
  `hygiene: clean — read 72 (runs 50 · evidence 13 · inputs 9) · trails 22 not read · copies 7 not read by P1 — 0
  host paths kept · binary 0 not read by P1`. Atoms: `exit 0` ✓, `contains hygiene: clean` ✓. No row to rewrite.
- Read once more after this file was added, before the commit: the verdict is in the next section's first line.

## A restart inside the pass, 16:54:51Z
- A restart ended the implementing session at 16:54:51Z, right after this file's first three sections were
  written and before any commit. The session was resumed at 17:00Z on the operator's word, with the tree as it
  was left: HEAD `59e791e`, nothing committed, 0 ahead of the upstream, the remote branch head read live
  (`git ls-remote`) at the same `59e791e`.
- Read at the resume, 17:00:47Z, that the two readings above still describe the tree: of the files the commit
  takes, two were written after the `pre-push` run began, this file (16:54:49Z) and the handoff's last section
  (the session-end hook's rewrite, 16:54:51Z). No product, test or configuration file is among them; the newest
  of those is `crates/viola-agent-claude/src/hook.rs`, 16:13:32Z. So the `pre-push` reading of 16:53:07Z stands
  for this tree, and it was not run again.
- No process stood at the resume: 0 rig hosts, 0 processes of the product build, the harness, the fake agent or
  `claude` 2.1.287, 0 gate runs. The product build still reads `62bf6028d95fe34e`, and the homes' backing `tmpfs`.

## Before the pre-CI commit, 17:00:58Z
- Hygiene re-read: `hygiene: clean — read 73 (runs 50 · evidence 14 · inputs 9) · trails 22 not read · copies 7
  not read by P1 — 0 host paths kept · binary 0 not read by P1`, exit 0, this file now among the evidence.
- The scope read (`gate.py scope`): `scope: clean — changed 10 · listed 10 · recorded 0`; the ten changed files
  are research's ten source, test and configuration files. Its base is HEAD, `59e791e`.
- The tree the commit takes: the phase's and the revision's products, the ten changed files, the two new rule
  files and the two edited leaves, this chunk's evidence and inputs, the four run dirs and the bookkeeping the
  tree carried (94 files). 0 ahead of the upstream before it.
- The commit, the push (entry 26) and the CI read (entry 27) are recorded below after they are made; that part
  of this file rides the next commit.
