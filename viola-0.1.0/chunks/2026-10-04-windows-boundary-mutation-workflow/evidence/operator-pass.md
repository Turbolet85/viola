# Operator pass — plan steps 7–9, entries 16–18

Run by the session on the overseer's word (founder-delegated, 2026-10-04): "Now run the operator pass: entries 16-18,
then step 8, and step 9 only if needed. Report and stop before the wrap." The leak disposition that preceded it is in
`leak.md` §Disposition.

## Before the pre-CI commit
- `bash scripts/agent-run.sh pre-push` (re-run on the tree the commit carries): exit 0, `ok:true`, `stage:"linux-tests"`;
  coverage 962 passed · 0 failed, doctest 0 · 0, playwright 1 · 0; gate `ok:true`, `breaches:[]`.
- Entry 16, `python -X utf8 <andromeda-tools>/scripts/gate.py hygiene`: exit 0,
  `hygiene: clean — read 35 (runs 32 · evidence 3) · trails 13 not read · binary 0 not read by P1` (the reading taken
  with this file present).

## Pre-CI commit, push, CI read
- Pre-CI commit `60c569b` (`chore(2026-10-04-windows-boundary-mutation-workflow): operator pre-CI commit, for the run
  this chunk's verdict reads`); tree clean after it.
- Entry 17 (`git diff --quiet && git diff --cached --quiet && git push origin HEAD`): exit 0,
  `7aca558..60c569b HEAD -> build/viola-0.1.0`.
- Entry 18 (`ci.py conclusion --sha HEAD --wait 1800`): exit 0,
  `60c569b40ff5 verdict: green · checks 15/15 · wall 309 s · runs ci#37174418732 completed/success`.

## Step 8 — the first dispatch
- `gh workflow list` shows `windows-mutants active 374339639` (dispatchable from the default branch after the push).
- `gh workflow run windows-mutants.yml --ref build/viola-0.1.0`: exit 0. Its run, by `gh run list --workflow
  windows-mutants.yml --json databaseId,headSha`: **37174673472**, `event: workflow_dispatch`, `headSha`
  `60c569b40ff5223b6487168194d602c8fbb1baa2` = the pushed HEAD, created 2026-10-04T03:39:01Z.
- Completed 04:05:54Z, every job under 27 min of its 120. The grades and the survivor list are in `windows-dispatch.md`:
  the 34 owed read 30 caught · 4 unviable · 0 missed.

## Step 9 — not fired
No owed coordinate graded missed, so no kill test, fix commit or re-dispatch. The final HEAD is still `60c569b`
(CI ci#37174418732 `verdict: green`, 15/15). The pass stops here, before the wrap, per the overseer's word.
