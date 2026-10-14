# Curation — 2026-10-13-owed-ledger-rows

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + verification-harness.md: "A side effect added to a fake-agent turn goes after that turn's screen write, never between its Stop and the screen it draws" (confidence 0.8)
                                              + testing.md: "Before a new fixture class's first recording is committed, read its keys for identity-bearing values" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 1 dup · 0 task-specific · 0 conflict · 0 deferred · 5 not a learning · 1 below threshold
  Load-bearing: "fake-agent side effect after the screen write" → Resume and compact ledger rows
  Load-bearing: "read a new fixture class's keys for identity values" → Resume and compact ledger rows
  Load-bearing: "a per-OS set's member named through its deciding function" → Resume and compact ledger rows
  Extended: T2/testing.md: "2026-10-07: When a compiled bound moves …" + "a set whose members differ per OS is read the same way" (confidence 0.8)

The next promotable entry is "Resume and compact ledger rows", which this wrap's P5 mints directly after this entry on
the founder's recorded fork-1 answer (`inputs#I1`, `inputs#I4`). It adds ledger rows (moving `LedgerRow::ALL` and the
relied set again), replays new hook payloads in the fake agent and records a resume payload fixture. None of the three
facts is carried on it at an annotation position, and none was amended into a master.

## Applied
- verification-harness.md — fake-agent side effect after the screen write.
  Proof: CI `test (macos-latest)` red on `9ee83095` (ci#38519633153), `cli_send.rs:919`. On Linux, a 2 s probe sleep
  in the first-Stop statusline replay reproduced the red, and moving the replay after the turn screen made it green
  with the sleep kept (80/80). Recorded in the chunk's `evidence/operator.md`. Signals: measured +0.4, technical
  detail +0.2, load-bearing +0.2.
- testing.md — a new fixture class's keys read for identity values.
  Proof: both recorded `StatusLine` variants carried `workspace.repo.owner`. The scrub (home → `~`, OS user word →
  `<user>`) and `contract_fixture_hygiene` read it green; it was surfaced only by reading the keys at the live
  round (`evidence/live-round.md`; implement outcome). Signals: measured +0.4, technical detail +0.2,
  load-bearing +0.2.
- testing.md, extended — the 2026-10-07 entry, a per-OS set's member.
  Proof: CI `test (windows-2025)` red on `9ee83095`. `verified_needs_every_row_pass_under_that_version` popped
  `LedgerRow::ALL`'s last row, which Windows does not rely on; fixed by naming `relied()`'s last row
  (`evidence/operator.md`). Signals: measured +0.4, technical detail +0.2, load-bearing +0.2.

## Decisions & corrections, one line each
- A per-OS relied set breaks tests that pick by position — extended (testing.md 2026-10-07 entry).
- A fake-agent side effect between a hook and a screen change — applied (verification-harness.md).
- Proving a suspected race by injecting a delay — rejected, duplicate (Filter 1: testing.md 2026-09-27 "To test a
  timing window, force it open with a test-only hold and show red before the fix and green after").
- A product `cfg(windows)` line enrols its file in the Windows mutation scope — rejected, below threshold (0.6:
  measured +0.4, detail +0.2; no conditional signal, because test-plan §3 → 5-command implementation now carries the
  `cfg(not(unix))` spelling).
- A new fixture class can carry identity fields the scrub never met — applied (testing.md).
- The founder's word on the handle — not a learning: a ruling, landed in the masters at P2.
- The founder's route moves — not a learning: P5's input.
- A held dialog timed out — not a learning: the outcome of a dialog, no rule to carry.
- Recurrence of testing.md 2026-10-07 (research missed `merge_stamp`'s literal envelope oracle) — not a new entry:
  logged as `recurrence-despite-learning` under the handoff's Deferred learnings (Filter 1).
- Guard and hook refusals (the `cat` heredoc, the stamp-ahead literal, the sweep id length) — not learnings: each
  guard already states its rule, and each cost one re-issue.
- Context compaction mid-chunk — not a learning: the handoff's standing post-compact rule covers it.
- A journal time written by hand was wrong — rejected, duplicate (the plan's standing note "Every UTC time written
  into an evidence record comes from `date -u` in the writing call").

CLAUDE.md size: see the P7 report (`health.py` check 1).
