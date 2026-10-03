# Session Handoff

**Last Updated:** 2026-10-03T08:00Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (this wrap's commit is pushed after this file is written)
**Status:** clean
**Last Commit:** 2026-10-02-epoch-2b-cleanup — Epoch 2b cleanup (M2 cause named, open red; M1 disposed; coverage corrected)

## Position
- Done: 2026-10-02-epoch-2b-cleanup. M2's cause is named with a two-sided witness: the D: dev volume's per-operation
  filesystem latency (D: 22/40 vs a C: copy of the tree 62/0, both orders). It stays an **OPEN red**; its closure (fix
  the volume, or move the test homes) is the founder's.
  - M1: eight survivors disposed (4 · 2 · 1 caught, plus the `restrict` exemption).
  - Coverage correction: 97.59 / 97.67 / 97.50 on the C: copy.
  - P5: the child dies with its wrapper.
  - CI green on `9e3b850` (ci#37107107417).
- Next: **Mutation scoring completion** (working-route :68, the split-off M3 + WSL leg, minted at Epoch 3's head) →
  `/andromeda-phase`. Its WSL leg's possible `TMPDIR` carve-out is a widening that goes to the founder live at its phase.

## Work done
- Test-only code: kill tests for search_restricted, the pin NotFound guards and refuse_stale on Windows;
  `remove_owned` (owner record last); the deadline-below-kill lint; a build-independent usage test; the P5 pin.
- Evidence: M2 diagnosis, leaks, M1, P5 and the coverage correction, all under the chunk's `evidence/`.

## Drift resolved
- 2 detector proposals and 1 cascade hit, all applied:
  - arch §Occupied Resources: the `target/e2e-home/` entry;
  - test-plan §3 → Test data bootstrap: Cleanup;
  - obs-plan §9: the stale deleter name.
- 3 sidecar entries; 2 leaves re-derived. No escalation.

## Notes
- **Owned and owed** (all on the route): M2's closure, leak B, the in-repo coverage gate and the TUI cases go with the
  M2 CARRY on :68 (owner: the founder's decision). The P5 tab-close leg is on the revive entry :85 (owner: the founder).
- **Plan step 4's premise was measured false** (std deletes read-only files); no guard was written.
- **Leftovers:** `evidence/leftovers.py --apply` is the founder's removal command; the `%TEMP%` sweep is a founder desk
  item.
- **Disk:** D: had 139.9 GB free before this wrap's light gate.
- **Viola is paused by the founder** so D: can be fixed (overseer, 2026-10-03): no cold D: builds until that lands.
  The wrap's light gate read the M2 class red on D: (incl. two cases green at /implement: D: grades run to run) and the
  pre-push timed out on its Windows stage; CI green on `9e3b850` is the acceptance leg.
- **Deferred learnings** (max-3 cap):
  - Windows PowerShell 5 parses a BOM-less UTF-8 `.ps1` as ANSI (keep scratch `.ps1` ASCII) (0.8);
  - let a red CI run finish before folding its fix (0.7);
  - carried from before: the doubled-backslash guard recurrence and the cross-drive `git worktree` move.
- **Last failed command:** none.
