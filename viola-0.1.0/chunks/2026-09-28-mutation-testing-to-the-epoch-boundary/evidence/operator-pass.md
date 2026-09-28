# Operator pass — 2026-09-28-mutation-testing-to-the-epoch-boundary

Run on the operator's word (2026-09-28, this session): "run the operator pass now, entries 23-27 and plan steps 8-9,
with the one-measurement-push bound".

## Before the pre-CI commit (uncommitted tree on `537ac36`)
- **Entry 24** `bash scripts/agent-run.sh pre-push` — exit 0 · `"cmd":"pre-push","ok":true` · `"stage":"windows-tests"` ·
  document keys `v cmd ok stage sync cache linux vm windows` (no `legs`, no `union`) · sync `files` 54 · Linux coverage
  917 passed, Windows coverage 932 passed, both gates `ok:true`.
- **Entry 23** `gate.py hygiene` — exit 0 · `hygiene: clean — read 27 (runs 27 · evidence 0)`.

## Push 1 — `d5deb01` (operator pre-CI commit)
- **Entry 25** clean-tree guard held · `537ac36..d5deb01`.
- **Entry 26** `ci.py conclusion --sha HEAD --wait 1800` — `d5deb0102e39 verdict: green · checks 15/15 · wall 463 s ·
  runs ci#36480299135 completed/success` (15 as predicted: 18 minus `mutants` ×2 and `mutants-verdict`).
- **Entry 27** `gh run download 36480299135 … -n junit-macos-latest` — the two tests 101.554 s / 101.475 s; the
  unbuildable-root case 0.350 s.

## Push 2 — `a7c1560` (the ONE measurement-only push, plan step 8)
- **Entry 24** before it: pre-push exit 0, `"stage":"windows-tests"`, `ok:true` (Linux 917, Windows 932).
- **Entry 25** clean-tree guard held · `d5deb01..a7c1560`.
- **Entry 26** — `a7c15600176a verdict: red · checks 15/15 · first-fail +157 s test (ubuntu-latest) · runs
  ci#36481260151`: `cargo llvm-cov report` could not merge a corrupt-header `viola-<pid>-<sig>_1.profraw`. Cause and
  fold: `macos-mutants-phases.md` §A red this push raised. Every other job green, `test (macos-latest)` included.
- **Entry 27** `gh run download 36481260151 … -n junit-macos-latest` plus the job log — the phases in
  `macos-mutants-phases.md`; the two tests 87.539 s / 87.204 s.

## Push 3 — the step-9 commit (measurement removed, runner-side arm)
- Tree vs `d5deb01` in code: only the two `#[cfg(not(target_os = "macos"))]` attributes and their comment in
  `run/mutants.rs` (`.config/nextest.toml` byte-identical to `d5deb01` again).
- **Entry 24** before it: pre-push exit 0, `"stage":"windows-tests"`, `ok:true` (Linux 917, Windows 932).
