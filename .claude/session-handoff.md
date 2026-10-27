# Session Handoff

**Last Updated:** 2026-10-27T03:56:09Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (HEAD `6c77eadd`, the operator pass's pre-CI commit; the wrap commit pushes after this file)
**Status:** clean
**Last Commit:** 2026-10-27-pinned-copy-race-and-reproducible-builds — the wrap commit on `6c77eadd` (ci#40439188053: `verdict: green · checks 15/15`)

## Position
- Done: **2026-10-27-pinned-copy-race-and-reproducible-builds** → `complete` (this wrap, `.andromeda/runs/2026-10-27T02-51-54-wrap/`).
- Next: the working route's first markerless entry, **"Bounded inputs at every boundary"** (Epoch 6) → `/andromeda-phase`.

## Work done
- The pinned copy's absent-copy write is a no-clobber placement of a closed temp file (`fs::place_private_new`), so a
  concurrent first start can no longer exec it open for write (`ETXTBSY`: 4 of 24 two-instance boots before, 0 of 24
  after); Path 7 back on the default pair; the Windows plain-home oracle reads the write grant by its right.

## Drift resolved
- 33 proposals (ordinary chunk, no sample): 32 applied across architecture, security-plan, test-plan and obs-plan
  (4 merged), 1 rejected (registry over-reach), 0 escalated; 5 check-7 contradictions resolved in-pass (each against
  a stale line a companion proposal amended). Leaves: 10 applied, 2 declined. Citations: 0 moved.
- Route: a WATCH on «Bounded inputs at every boundary» (red 1) and a CARRY on «Version done-check» (measurement B),
  both on the directive (`chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/inputs/I6-…`).
- Curation: one Tier 3 entry (comparing two builds by section).

## FOR DISCUSSION (the founder's)
- **Measurement B:** six clean release builds of one tree gave two files (`57bce92d…`, `ff63f38e…`); the loaded
  sections are equal, the build-id, `.symtab` and `.strtab` differ, so one tree can produce two pin keys. Recorded as
  measured, no cause claimed; acting on it is his (CARRY on «Version done-check»;
  `chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/reproducible-builds.md`).

## Notes
- **PROVISIONAL, owed the founder's own word:** red 1 read as runner variance with its WATCH (the overseer's
  technical answer, `inputs#I6` item 3; retires at a recurrence, which reopens the owner question, or 3 green runs);
  the red-2 fold (`inputs#I4`, `inputs#I5`, delegate overseer1); carried from the last wrap, its P2 halt's three
  answers (`chunks/2026-10-26-home-and-code-bearing-file-integrity/inputs/I6-relay-1.md.txt`) and the Q3 CARRY on
  «Unix endpoint and home hardening» resting on them.
- **Deferred learnings (carried):** `recurrence-despite-learning: testing.md 2026-09-24` · `recurrence-despite-learning:
  testing.md 2026-09-25, Extended 2026-10-04` — a check in the owning step, not a third entry.
- **Epoch 6 at 12 entries** (3 markerless) — surfaced, no split, by the founder's word of 2026-10-20T10:17Z.
- **Owed outside this repo (friction log, for the founder's desk):** carried from the last wrap (its records and F350,
  F351, the items in git at `d35644a5`'s handoff; the CARRY on "Version done-check" `v1-13` / `v1-36`); F363 (the
  phase's second re-spawn past the one-retry limit, friction-log `2026-10-27T01:28:02Z-b`).
- **Host recipe:** read `df -h /tmp` before a stamped-home gate; macOS-target check-only clippy under
  `target/maccheck` beside `target/wincheck`; `target/` reads 294G (a clean is due).
- **Last failed command:** none.
