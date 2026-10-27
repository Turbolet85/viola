# Session Handoff

**Last Updated:** 2026-10-27T00:15:56Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (HEAD `e3468500`, the operator pass's last fix commit; the wrap commit pushes after this file)
**Status:** clean
**Last Commit:** 2026-10-26-home-and-code-bearing-file-integrity — the wrap commit on `e3468500` (ci#40403389629: `verdict: green · checks 15/15`)

## Position
- Done: **2026-10-26-home-and-code-bearing-file-integrity** → `complete` (this wrap, `.andromeda/runs/2026-10-26T21-34-52-wrap/`,
  resumed at P3 in a second window).
- Next: the working route's first markerless entry, **"Pinned copy race and reproducible builds"** (Epoch 6, inserted by
  this wrap) → `/andromeda-phase`.

## Work done
- The home's strict-modes check runs before every read or open at every entry point: the six frame verbs, `hook`,
  `mcp`, `run` / `revive` (`check_home_tree`), `verify` and `update_stamps`; the Unix read rule and the Windows
  read-grant rule; the `conpty/` companions' owner/DACL check (`strict-modes-failed` fallback); the four readers on the
  replay (the founder's `inputs#I3`). CI green on three OSes after two Windows test fixes.

## Drift resolved
- Forced sample (`inputs#I5`): stage-1 proposals architecture 11 · security-plan 6 · design-system 4 · layout-templates
  4 · test-plan 17 · obs-plan 12 · a11y-plan 0, plus 261 cascade proposals; one P2 halt answered (`inputs#I6`): Q1 confirm
  all 51, Q2 the check-7 contradictions applied with companions, Q3 the `viola list` home-check owner → CARRY.
- Citations: one master claim rewritten (raised at Validate); one route citation re-pointed at P5 (`src/cmd/run.rs:475
  → 521`).
- Route: «Pinned copy race and reproducible builds» inserted after this entry with CARRYs 11 and 16 verbatim (the
  founder's P4 Q4 and P5 «Да»); CARRY 17 closed by ruling; a CARRY on «Unix endpoint and home hardening» for
  `board::scan`'s two home rules.
- Curation: T2 two writes to `host-linux.md` (a truncated process listing; the `(deleted)` exe suffix).

## Notes
- **PROVISIONAL, owed the founder's own word at the epoch boundary:** the P2 halt's three answers (`inputs#I6`) are the
  overseer's, founder-delegated; the widening's ratification stays the founder's own `inputs#I3`. The Q3 CARRY on
  «Unix endpoint and home hardening» rests on it.
- **Deferred learnings:** `recurrence-despite-learning: testing.md 2026-09-24` (a path under a regular file reads
  `NotFound` on Windows; ci#40398601594 red on it) · `recurrence-despite-learning: testing.md 2026-09-25, Extended
  2026-10-04` (a remove-the-guard control hung 27 min with no kill line) — a check in the owning step, not a third entry.
- **Epoch 6 at 12 entries** (4 markerless) — surfaced, no split, by the founder's word of 2026-10-20T10:17Z.
- **Owed outside this repo (friction log, for the founder's desk):** this wrap's records `2026-10-26T21:42:14Z-a`,
  `2026-10-27T00:06:24Z-a`…`-d`, `00:11:43Z-a`…`-d`, `00:14:45Z-a`…`-c` and the P7 checkpoint's (the wrap straddled
  the 23:35Z growth-watch deploy). Carried: F350, F351 and the items listed in git at `d35644a5`'s
  `.claude/session-handoff.md`; a CARRY on "Version done-check" (`v1-13` / `v1-36`, the founder's disposition).
- **Host recipe:** read `df -h /tmp` before a stamped-home gate; macOS-target check-only clippy under
  `target/maccheck` beside `target/wincheck`.
- **Last failed command:** none.
