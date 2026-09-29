# Session Handoff

**Last Updated:** 2026-09-29T14:53Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the wrap commit is pushed at P7)
**Status:** clean
**Last Commit:** 2026-09-29-fake-agent-drift-contract — the chunk's wrap commit (P7)

## Position
- Done: 2026-09-29-fake-agent-drift-contract — the last Epoch 2b entry; **Epoch 2b is complete**.
  - The fake agent's print-mode hook traffic is a contract against every recorded `fixtures/claude/<ver>/` set:
    spine order a literal, each hook's `stdin_hex` byte-equal to its fixture (it witnessed, then fixed, a dropped
    trailing newline). The receipt `size` is change-driven by a watcher. The cross-session-message tag (escaped and
    plain) is harness origin, founder-ratified live 15:21:44 (relay: the Viola overseer).
  - CI ci#36583175440 on `055adf4`: green 15/15.
- Next: **Readiness gate and timing constants** (working-route :66, Epoch 3's head) → `/andromeda-phase`. It carries
  the host-reds CARRY (re-read here) and the WSL `--install-deps` CARRY.
  - Epoch boundary first, if the operator wants the cadence: `/andromeda-evolve-diagnose` for Epoch 2b and the
    epoch-boundary `/andromeda-code-audit` (the mutation run lives there).

## Work done
- implement → operator pass (pre-CI `055adf4`, CI green) → this wrap, all in one session.

## Drift resolved
- Detectors: 6 proposals (architecture 3 · test-plan 3; the other five docs 0), all applied.
  - 9 more were raised by the orchestrator from the plan's expected amendments: security 2, test-plan 7.
  - 5 escalations, all one boundary widening (the cross-session harness prefix). They resolved on the founder's
    recorded live ratification.
- 4 sidecar entries; 6 leaves re-derived.
- A security Decisions Log entry and a test Decisions Log entry for 2026-09-29.

## Notes
- **Route:** 4 CARRYs.
  - :66 — the WSL CARRY and the host-reds CARRY migrated (today's control read 44 then 57 on one tree).
  - :72 Dialog answers by dialog_id — matcher evaluation and the S8 `annotations` witness.
  - :76 First live test and self-drive — measure a real cross-session prompt. Beside it, the HYPOTHESIS that
    subagent hand-backs arrive framed `Another Claude session sent a message:` ahead of `<agent-message from=`.
- **Measured, not acted on:** the fake agent's size watcher costs ≈3 ms/s per PTY session plus ≈1 ms/s in
  `OpenConsole` (0 on the control). It did not separate the trees; evidence in that chunk's
  `host-reds-two-sided.md`.
- **Deferred learnings:**
  - `recurrence-despite-learning: host-win32.md Session Additions 2026-09-28 (extended 2026-09-29) — "The Bash guard refuses … ANY command carrying [a doubled backslash]"`
    (met 3× this session).
  - "A `git worktree` cannot be moved across drives on this host" (0.8), still held from the prior wrap.
- **For the operator:** still on disk — `target/baseline-target`, `target/e2e-home/probe-*` (3),
  `target/conpty-seed/`. The same-day control worktree was removed after the CI read.
- **Last failed command:** none.
