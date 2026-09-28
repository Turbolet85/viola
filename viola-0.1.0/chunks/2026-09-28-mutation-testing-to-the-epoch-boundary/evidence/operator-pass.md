# Operator pass — 2026-09-28-mutation-testing-to-the-epoch-boundary

Run on the operator's word (2026-09-28, this session): "run the operator pass now, entries 23-27 and plan steps 8-9,
with the one-measurement-push bound".

## Before the pre-CI commit (uncommitted tree on `537ac36`)
- **Entry 24** `bash scripts/agent-run.sh pre-push` — exit 0 · `"cmd":"pre-push","ok":true` · `"stage":"windows-tests"` ·
  document keys `v cmd ok stage sync cache linux vm windows` (no `legs`, no `union`) · sync `files` 54 · Linux coverage
  917 passed, Windows coverage 932 passed, both gates `ok:true`.
- **Entry 23** `gate.py hygiene` — exit 0 · `hygiene: clean — read 27 (runs 27 · evidence 0)`.
