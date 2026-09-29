# Session Handoff

**Last Updated:** 2026-09-29T12:45Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the wrap commit is pushed at P7)
**Status:** clean
**Last Commit:** 2026-09-29-sideloaded-conpty — the chunk's wrap commit (P7)

## Position
- Done: 2026-09-29-sideloaded-conpty.
  - On Windows x64, `viola run` hosts its child in Microsoft's sideloaded ConPTY: `conpty.dll` + `OpenConsole.exe`
    1.24.260710001, vendored, embedded and pinned, with the handles held across the re-hash. Every process restricts
    its DLL search to System32. Any failure falls back to the inbox ConPTY with no terminal byte.
  - H2 with/without in one windows-2025 run (ci#36563868040): sideloaded 0/200, inbox 14/200.
  - CI ci#36566391084 on `8f643f2`: green 15/15.
- Next: **Fake-agent drift contract** (working-route :63, the last Epoch 2b entry) → `/andromeda-phase`. It carries
  the host-reds CARRY and the WSL `--install-deps` CARRY, both moving with the head. Epoch 2b completes with it.

## Work done
- This resumed wrap ran P2-P7. P1 had been authored in the prior window, which stopped at 63 % context on the
  operator's word.

## Drift resolved
- Detectors: 40 proposals (architecture 14 · security-plan 12 · test-plan 6 · obs-plan 6 · a11y-plan 2 · design
  and layout 0).
  - 32 were applied. 8 were rejected: two detector groups that cited source lines the report does not carry.
  - The rejected facts were re-raised from the report as 2 orchestrator amendments.
  - 3 escalations were resolved:
    - the two security widenings, founder live 2026-09-29 10:41:12 (relay: the Viola overseer)
    - the test-home seeded carve-out, the operator's ruling (the overseer agreeing)
- 10 sidecar entries; 12 leaves re-derived; obs D-36; a security Decisions Log entry and a test Decisions Log entry
  for 2026-09-29.

## Notes
- **Route:** 5 CARRYs pinned.
  - :63 — the host's shared local integration reds (entries 6 and 18's owner), moving with the head, plus the
    migrated WSL CARRY.
  - :76 First live test and self-drive — the DA1 headless stall.
  - :103 Home and code-bearing file integrity — the third interim gap, and the seeded-home DACL.
- **Epoch 2b:** no split (the founder's ruling of 2026-09-28 ~21:50). It closes with the next chunk, and then the
  diagnose/audit cadence resumes.
- **Open product finding:** a `viola run` with no terminal on its stdio stalls ~3 s for the sideload's DA1 answer.
  The owner is "First live test and self-drive". The founder's H2 product question for the real `claude` stays open
  there too.
- **Deferred learnings:**
  - `recurrence-despite-learning: host-win32.md Session Additions 2026-09-28 — "The Bash guard refuses a heredoc whose payload carries a doubled backslash"`
    (the implement session met it again).
  - "A `git worktree` cannot be moved across drives on this host" (0.8), held by the max-3 cap.
- **For the operator:** left on disk from the implement session:
  - `target/baseline-target` (its `rm` was denied)
  - `target/e2e-home/probe-*` (these ride the founder desk item that deletes e2e-home)
  - `target/conpty-seed/` (regenerated on demand)
- **Last failed command:** none.

## Session End Status
Completed normally at 2026-09-29 15:28:26
