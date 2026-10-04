# Curation — 2026-10-04-dialog-answers-by-dialog-id

Scope: this resumed window's conversation plus the report's *Decisions & corrections*. The P1 window's own conversation
is gone (resume), so a correction only that window held is not curated.

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   extended testing.md (the 2026-09-25 remove-the-guard entry)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 0 task-specific · 0 conflict · 2 below threshold · 0 deferred · 1 recurrence logged
  No-other-home: "run a remove-the-guard control under the nextest `mutants` profile when the neutralised guard can park a Condvar on a fixed test clock"
  Extended: T2/testing.md: "Every new guard test carries its remove-the-guard run…" + the Condvar-hang facet
  CLAUDE.md size: 124/200 · T1 1.8 KB, 0 over 600 B

## Applied
- T2 testing.md, extension of the 2026-09-25 remove-the-guard entry (confidence 0.8: measured +0.4, technical detail
  +0.2, no other durable home +0.2 — not on the route, in no master, no playbook rule).
  Proof: the report's Decisions & corrections sweep hazard (a neutralised guard parked a Condvar on a fixed test clock
  and hung with no kill line; a killed run skipped the restore), the controls recorded in
  `viola-0.1.0/chunks/2026-10-04-dialog-answers-by-dialog-id/evidence/guard-controls.md`.

## Rejected
- "A Windows DACL / strict-modes test homes under `target/e2e-home`, never `%TEMP%` or a `create_dir_all`-made home"
  — Filter 4 at exactly 0.6 (measured +0.4, detail +0.2); neither conditional signal applies because this wrap's P2
  amended the fact into test-plan §6 (the Windows `--home` security-sweep case).
- "The Linux dev host's `grep` is ugrep, which refuses a bounded-repeat context regex such as `.{0,200}` (exceeds
  complexity limits)" — Filter 4 at 0.2 (detail only; one event, no gate or design change).

## Recurrence
- `recurrence-despite-learning: host-win32.md 2026-09-28` (extended 2026-09-29: the Bash guard refuses a heredoc
  redirected to a file) — this wrap tried a `cat <<EOF >>` append to a run-dir file and the guard refused it; the
  text went in through the Edit tool. Carried to the handoff's Deferred learnings.
