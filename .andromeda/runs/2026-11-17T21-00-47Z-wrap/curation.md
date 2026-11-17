# Curation — 2026-11-17-trust-record-ledger-row (wrap 2026-11-17T21-00-47Z-wrap)

CLAUDE.md ecosystem: no new learnings this session.
  Filters: 2 dup · 3 task-specific · 0 conflict · 0 deferred · 1 recurrence (→ handoff)
  CLAUDE.md size: 134/200 · T1 3.9 KB, 0 over 600 B

## Every Decisions & corrections entry, weighed
Implement's *Decisions & corrections*:
- «The Bash guard refused a `cd` out of the project root (twice) and a `cat` heredoc to a file; each re-issued in the
  passing form» — not a learning: each refusal cost its one re-issue (the guard put the call back on the path);
  host-linux.md already carries both (the cwd entry, 2026-09-28's heredoc entry).
- «A `ScheduleWakeup` call was made in error … refused by the permission classifier» — Filter 1 duplicate of the
  Tier 1 entry «A builder session sets no scheduled wake, timer or reminder of its own»; the entry records a defect
  and the act recurred (the call carried `stop: true`, inputs#I3) → handoff `recurrence-despite-learning`.
- «Live start cap: S0 used, S0-2 unspent» — rejected, task-specific (Filter 2: one chunk's cap).
- «Sweep hazard: the `--record` scrub depends on `HOME` …» — Filter 1 duplicate: this wrap's cascade wrote it into
  `.claude/rules/testing.md`'s generated body (leaf finding testing.md:34#1, «a `--record` verify, whose scrub reads
  `HOME`, keeps the real `HOME` …») and test-plan `decision/verify-under-test-owned-user-home` holds it.

`## After implement`'s *Decisions & corrections after implement*:
- The wrap directive (inputs#I2) — not a learning: one wrap's directions; its rule for a confounded live reading is
  now test-plan `decision/live-failing-reading-not-verification` (a master home).
- The slip as it happened (inputs#I3) — the same recurrence as above, logged once.

This conversation:
- The overseer's card condition («no row words it as a measured fact») — rejected, task-specific: applied as a set on
  architecture `decision/trust-record-ledger-row` (TX 2510); the master rule above carries the class.
- rec.py rows-land's false «landed before» and the leaf comparers' missed counts — not candidates: telemetry of the
  tool, recorded in the friction family (evolve records 2026-11-17T21:42:52Z-b, -c).
