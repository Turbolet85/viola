# Session Handoff

**Last Updated:** 2026-10-27T07:23:30Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (HEAD `607792cb`, the operator pass's pre-CI commit; the wrap commit pushes after this file)
**Status:** clean
**Last Commit:** 2026-10-27-bounded-inputs-at-every-boundary — the wrap commit on `607792cb` (ci#40458923095: `verdict: green · checks 15/15`)

## Position
- Done: **2026-10-27-bounded-inputs-at-every-boundary** → `complete` (this wrap, `.andromeda/runs/2026-10-27T06-19-22-wrap/`).
- Next: the working route's first markerless entry, **"Sanitised error surfaces"** (Epoch 6) → `/andromeda-phase`.
  It now carries a PREREQ (the two test gaps below) and the Path 1 WATCH at 1/3.

## Work done
- `viola mcp`'s stdin is capped at `MAX_FRAME` per line by viola's own reader. A longer line is refused whole and
  ends the session with one `parse-rejected{mcp-stdin, oversize}` line.
- `replace_private_shared`'s compare read is bounded, serde_json's `unbounded_depth` is banned in `deny.toml`
  (deny-probes 14/14), and the feed-panic passthrough is witnessed on the outer terminal.
- `tests/contract_bounded_readers.rs` enumerates every boundary and censuses the product readers. v1-46 was claimed.

## Drift resolved
- 74 proposals (ordinary chunk, no sample): 70 applied across five masters (4 merged), 4 rejected (3 twins, 1 not
  this chunk's), 1 check-7 contradiction escalated as written and answered at the P2 halt (`inputs#I5`, the overseer
  as operator: merged with its companion). 23 sidecar entries. Leaves: 11 written, 5 declined. Citations: 0 moved.
- Route: a PREREQ and the migrated WATCH (1/3) on «Sanitised error surfaces»; a CARRY on «Windows-only live
  measurements» (the Windows byte oracle for the feed-panic case).
- Curation: T1 1 (timestamps from `date -u`, never estimated) · T2 1 (testing.md: a kill deadline on every spawned
  `viola run` / `viola mcp` / wrapper).

## Notes
- **PROVISIONAL, owed the founder's own word:** this chunk's P5 approval (delegate overseer1, `inputs#I3`); the Q1
  answer (`inputs#I5`); the Path 1 WATCH's runner-variance reading; carried from earlier wraps, the red-2 fold
  (`chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/inputs/I4`, `I5`) and the P2 halt answers of
  `chunks/2026-10-26-home-and-code-bearing-file-integrity/inputs/I6-relay-1.md.txt` with the Q3 CARRY resting on them.
- **FOR DISCUSSION (the founder's), carried:** Measurement B — one tree, two release-build files, two pin keys (CARRY
  on «Version done-check»; `chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/evidence/reproducible-builds.md`).
- **Two test gaps owned by «Sanitised error surfaces»** (inputs#I4 item 4): the pre-`initialize` stdin cut (exit 11,
  no test), and five boundary rows whose witness never drives past the cap. Both are stated in test-plan §4 and §6.
- **Deferred learnings (carried):** `recurrence-despite-learning: testing.md 2026-09-24` · `recurrence-despite-learning:
  testing.md 2026-09-25, Extended 2026-10-04` — a check in the owning step, not a third entry.
- **Epoch 6 at 12 entries** (2 markerless) — surfaced, no split, by the founder's word of 2026-10-20T10:17Z.
- **Host:** python `yaml` is absent on the Linux dev host (new-session check 12 read by eye); read `df -h /tmp`
  before a stamped-home gate; `target/` reads 297G against 347G free — a clean is due before the next gate-heavy run.
- **Owed outside this repo (friction log, for the founder's desk):** carried from the last wrap (F350, F351, F363,
  the CARRY on "Version done-check" `v1-13` / `v1-36`).
- **Last failed command:** none.
