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
