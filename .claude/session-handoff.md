# Session Handoff

**Last Updated:** 2026-10-26T01:58:54Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the wrap commit pushes after this file)
**Status:** clean
**Last Commit:** feat(2026-10-25-windows-endpoint-admission) — the wrap commit on top of `e2bb59c8` (the operator pre-CI commit; ci#40247500584: `test (windows-2025)` green, the row red on `supply-chain` alone)

## Position
- Done: **2026-10-25-windows-endpoint-admission** ("Windows endpoint admission", Epoch 6). Report:
  `viola-0.1.0/chunks/2026-10-25-windows-endpoint-admission/report.md`; implement run
  `.andromeda/runs/2026-10-25T23-49-29-implement/`, wrap run `.andromeda/runs/2026-10-26T00-17-51-wrap/` (two P2 halts,
  both resumed on the operator's word: fanout v7 / F350, then ruling F351).
- Next: **tokio RUSTSEC-2026-0386 lock bump** (Epoch 6, inserted this wrap) → `/andromeda-phase`.

## Work done
- The Windows listener's admission is now asserted on CI `test (windows-2025)`: remote clients rejected (pipe-type
  read-back, two control pipes) and the DACL's effect (an anonymous-token open denied, a default-descriptor control).
- The case_4 CARRY closed at its root cause: every root-bin scoped capture goes through
  `test_support::with_scoped_default`, its witness red before and green after; plain `cargo test` ruled unsupported.

## Drift resolved
- 17 amendments: test-plan 13 (§1 untestable zone narrowed to the other-euid peer, §6 negatives named, §4 the capture
  helper, §2 runner, §5, §8), security-plan 3 (the two Windows witnesses; the open RUSTSEC-2026-0386 line and its
  owner), architecture 1 (the key file Crate dependency direction: windows-sys `Wdk_Storage_FileSystem` +
  `Win32_System_IO`), plus the orchestrator's two writes on ruling F351 (the label, the subject line). 0 escalations;
  29 proposals, 10 rejected. 8 sidecar entries. Leaves: 6 findings, 3 applied, 3 declined.
- Curation: T2 1 (`testing.md` 2026-09-27 extended), T3 1 (windows-sys feature gates).
- Route: «tokio RUSTSEC-2026-0386 lock bump» inserted after this entry (the founder's yes on the P5 card); the macOS
  readiness WATCH retired at 3 green runs.

## Notes
- **Owed outside this repo:** two pipeline defects met at P2 — the newdetail re-spawn (F350, fixed in fanout v7) and
  an apply writer that cannot correct a label it wrote (F351, worked by the operator's ruling) — both in the friction
  log for the founder's desk.
- **Deferred learnings (carried, max-3 cap of the last wrap):** the Windows scope contract's `is_windows_gate` reads
  `cfg!(windows)` as well as `cfg(windows`, so a sweep on `cfg(windows)` alone misses an oracle (confidence 0.8).
- **Carried from the last handoff:** unchanged — the founder's next batch, the CARRYs, the boundary-audit list, the
  founder's desk, the operator's-word items, the standing rules (in git at `d35644a5`'s `.claude/session-handoff.md`);
  the founder's word of 2026-10-20T10:17Z: no epoch boundary inside Epoch 6; read `df -h /tmp` before a stamped-home gate.
- **Last failed command:** none.
