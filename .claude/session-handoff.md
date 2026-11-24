# Session Handoff

**Last Updated:** 2026-11-24T17:42:17Z
**Branch:** build/viola-0.2.0 · 0 ahead of origin/build/viola-0.2.0 as read at this wrap's Setup (this wrap's push lands its commit)
**Status:** clean
**Last Commit:** 2026-11-24-viola-focus — gated acceptance — `viola focus`

## Position
- Done: 2026-11-24-viola-focus (`chk-fw7hhw4r`), GATED wrap `2026-11-24T17-14-08Z-wrap`. R3 cleared (green
  17:05:47Z, him away on his typed word, workspace 1). v2-11 is verified, its acceptance in his F-E words. The record
  flips gated → complete.
- Still GATED:
  - «`viola up`» (`chk-eq4ejvc6`): R4a is green and R4b STANDING. Its gated door with the release of v2-12's reboot
    clause is NEXT (`inputs#I10` §B, steps 4–7).
  - «`viola adopt`» (`chk-ep3sqyx7`): R5 STANDING.
- Epoch 5: 7 complete, 2 gated, no markerless entry.
- Next: «`viola up`»'s gated implement, then its gated wrap with the release. After that, «Console scaffold»
  (`chk-eg54zv5h`) once Epoch 5's boundary is passed.

## Work done
- P1: the close landed on `report-gated` (tx 4840–4841). Directive `inputs#I10`, the operator's answers `inputs#I11`.
- P2: the operator's answers landed as his.
  - F-A arm 1: 3 decision rows provisional → standing (tx 4844–4845), on ONE card Q1 confirmed by the delegate
    overseer, PROVISIONAL as a confirm.
  - F-E: obs-plan `contract/process-exit-detail/exit-1-focus` set (tx 4846).
  - F-A, F-B, F-C, F-E and F1: 7 ratification entries, one a master (tx 4847–4853).
  - 2 test-plan adds were rejected (check 3).
  - Leaves: 3 findings (commands.md, observability.md, obs-summary.md by hand).
- P3: host-linux.md got 1 entry, the R4a active-workspace reading.
- P5: carry `cry-r4rsj5ey` on «Console scaffold», placed by the delegate overseer, PROVISIONAL.

## Drift resolved
- R4a's reading: on Hyprland 0.56.2 the move dispatch takes the active workspace with the window. No row promises
  otherwise, and his F4 «no focus change» is not kept by the built move. It is a defect, carried to «Console
  scaffold» at its P1.
- F1 for focus: no new bound. `decision/hyprctl-read-deadline` already names viola focus.

## Notes
- **Still PROVISIONAL:**
  - test-plan `decision/window-raise-proven-live` (F-D shape, not his);
  - the a11y «Eight» leads (up wrap W2);
  - every Q1 card confirm (a delegate's);
  - the earlier chunks' forks, as before.
- **Deferred learnings:**
  - recurrence-despite-learning: CLAUDE.md «A timestamp written into any record … is read from `date -u`». The gated
    implement typed an estimate again, a fourth recurrence.
  - Carried: testing.md 2026-09-24 «wait on the exact line asserted».
  - Review with `/andromeda-wrap-session --review` if any should be applied.
- **Unlanded record:** `viola-up`'s `evidence/r4.md` (R4a), landed by this wrap's P7 on `rec.py check`'s word if
  needed. Otherwise `viola up`'s gated implement lands it.
- **Carries:** `cry-z6ucncyv` (R5) and `cry-tzxxzdkd` (R4) stand. `cry-r4rsj5ey` is new.
- **Desk:** `$XDG_RUNTIME_DIR/viola-r3/` and `~/.cache/viola-r4/live-20261124T170559Z/` (R3's and R4a's leavings).
- **Last failed command:** none.
- **Host:** PATH's claude is 2.1.330. Read `hostwatch.py read` before any stall red.
