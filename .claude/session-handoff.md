# Session Handoff

**Last Updated:** 2026-10-01T12:37Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (this wrap's commit is pushed after this file is written)
**Status:** clean
**Last Commit:** no-marker — chore(route): operator-requested adaptation — 0-pending wrap (Epoch 2b boundary)

## Position
- Done: the Epoch 2b boundary. Both boundary reports are committed: evolve `.andromeda/runs/2026-10-01T09-05-15-evolve-diagnose/`
  and code audit `.andromeda/runs/2026-10-01T09-18-50-code-audit/`. The route adaptation (relay items A–D) landed in a
  0-pending wrap, `.andromeda/runs/2026-10-01T12-19-55-wrap/` (`adaptation-record.md`). Last chunk:
  2026-09-29-fake-agent-drift-contract.
- Next: **Epoch 2b cleanup** (working-route :66, Epoch 3's new head) → `/andromeda-phase`.
  - M2 comes first: the in-repo suite is red at HEAD (80 of 985 failed; CI green). It stays an open red until its cause is known.
  - It also carries M1 (8 survivors), M3 (the viola-e2e mutation form), the WSL leg for 12 `cfg(unix)` mutants, the
    `.tmp*` leak, evolve L4/P6/P2, P5 (can a claude child outlive its wrapper), and the two CARRYs that move with the head
    (host reds, WSL `--install-deps`).
  - If phase sizes it over one window, raise a split of M3 + the WSL leg at its P1 (overseer direction).

## Work done
- Route: three entries inserted (Epoch 2b cleanup at the Epoch 3 head; `viola revive` after Self-healing state; Frontend
  toolchain at the Epoch 8 head), :129 and :133 reworded off Lit.
- Masters: the web front's toolkit is React + TypeScript (founder ruling), with its measured details OPEN and owned by
  the Frontend toolchain entry; arch's GUI reads are cookie-gated (security amendments 1, 5 and 3's URNs folded).

## Drift resolved
- No fan-out on this path. The amendments record ratifications; sweep + dispositions in the run dir's
  `cascade-dispositions.md`; 7 sidecar entries; `registry.py check --all` clean.

## Notes
- **No control was retired.** The Build-system no-bundler guard, every CSP directive and every output-encoding ban stand.
  Retiring or relaxing any of them is the Frontend toolchain entry's boundary widening, which the founder rules live.
- **Open for the founder** (on the revive entry): whether a revived instance starts with the wheel at `human`.
- **Left as history:** `viola-0.1.0/intent.md:118` and v1-08's `observed_gap` still describe Lit (dated route-time
  observations; no writer on the wrap path).
- **Disk:** D: had 74 GB free at session start, with `target/` at 73 GB. `target/baseline-target`, `target/e2e-home/probe-*` (3)
  and `target/conpty-seed/` are M2's suspects; leave them in place until the cleanup chunk has diffed against them.
- **Route:** CARRYs that move with the head now sit on :66. The :76 HYPOTHESIS on hand-back framing stays open.
- **Deferred learnings:** unchanged (the doubled-backslash guard recurrence; the cross-drive `git worktree` move).
- **Last failed command:** none.

## Session End Status
Completed normally at 2026-10-03 06:02:37
