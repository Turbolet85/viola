# Curation — 2026-11-05-linux-only-code-and-ci (wrap P3, window 3)

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "A sweep for a `cfg` predicate reads the predicate's parse, never a line grep" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): + "End a retirement cascade with a bare-token census over every master" (confidence 0.8)
  Corrected in place (exempt from the cap): testing.md 2026-10-03, 2026-10-31 · ci.md 2026-10-04 ×2, 2026-10-10 · verification-harness.md 2026-09-25, 2026-10-10
  Filters: 0 dup · 0 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff)
  CLAUDE.md size: 131/200 · T1 3.4 KB, 0 over 600 B

## Applied
- Tier 2 `testing.md`: a `cfg` predicate sweep reads the parse, never a line grep.
  Proof: report Deviations 9 and Decisions & corrections «Sweep hazard: a line grep … cannot see an OS predicate split
  over lines» — the census found a `#[cfg_attr(⏎ not(target_os = "macos"), …)]` pair in `tests/hook_fail_open.rs` that gate
  entry 8's `git grep` read clean.
- Tier 3 `session-learnings.md`: the bare-token census after a retirement cascade.
  Proof: `cascade-dispositions.md` §Residual census — 116 hits over 61 facts read, 19 stale sites in 17 facts
  (`edits-main-9.md`, TX 381) and 8 leaf lines (TX 383–390) that the 551-pattern sweep left.
- Corrections (7): each entry directed a step on a Windows / macOS target, CI leg or `windows-mutants.yml` that no longer
  exists; tagged `[corrected 2026-11-06: …]` in place (TX 397–399).
  Proof: report Changes **Retired** (the `windows-2025` and `macos-latest` legs, `windows-mutants.yml`) and
  `.github/workflows/` holding `ci.yml` and `nightly.yml` alone.

## The cascade's curation rows (cascade-dispositions.md §Curation rows and §Residual census)
- Corrected: `rules/testing.md` :70, :107 · `rules/ci.md` :30, :31, :33 · `rules/verification-harness.md` :56, :68.
- Kept, true: `rules/testing.md` :54 (an I/O-error test fails at one call), :55, :58 (the kill line), :56 (a dated
  Windows fact beside a rule still held), :62 (a crate-local test for a security property), :85 (a test kills only its own
  child); `rules/verification-harness.md` :55 (`--in-diff`); `docs/session-learnings.md` :12, :135 (sweep-pattern
  history), :50–55, :175, :200, :209, :242 (dated Windows facts, true history), :232 (the hygiene read, no OS claim).

## Decisions & corrections (the report's entries)
- Founder rulings, Linux-only (`inputs#I1`, `I5` item 6) — not a learning: the masters carry the ruling (P2).
- Closed words leave with their emitter (`inputs#I2`) — not a learning: obs-plan §6 states the rule (P2).
- RUSTSEC-2026-0441 owned by «Version done-check», `v1-01`'s owner (`inputs#I3`, `I4`) — not a learning: route (P5).
- An absent crate's licence exception dropped (`inputs#I5` item 7) — not a learning: security-plan Dependency Security
  states the rule (P2).
- Sweep hazard, the multi-line `cfg_attr` predicate — applied, Tier 2 `testing.md`.
- Sweep hazard, `rec.py runs --file` printing every run's file of that name — not a learning: pipeline-tool telemetry.
- Plan figure vs measurement, `thiserror::Error` 9 vs 10 — not a learning: a CARRY (P5).
- Guard refusals — not a learning: telemetry.
- Health check 13 (F411, upstream) — not a learning: upstream pipeline.
- Epoch 7's diagnoses and the code audit at the version end (`inputs#I5` item 10) — not a learning: the handoff's.
