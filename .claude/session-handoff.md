# Session Handoff

**Last Updated:** 2026-11-14T12:52:57Z
**Branch:** build/viola-0.2.0 · 0 ahead of origin/build/viola-0.2.0 as read at this wrap's Setup (this wrap's push lands its commit)
**Status:** clean
**Last Commit:** no-marker — chore(route): operator-requested adaptation — 0-pending wrap (the Epoch 1 boundary of viola-0.2.0)

## Position
- Done: 2026-11-14-hyprland-seam (Epoch 1 — Foundation complete; diagnosis `2026-11-14T11-32-05-evolve-diagnose` and
  code audit `2026-11-14T11-39-19-code-audit` ran at its boundary). CI: ci#43579735207 read the pre-CI `93b2ca1d`,
  ci#43589375075 the wrap commit `25348549`, green 7/7.
- Next: «Hyprland exact-frame bound pinned» (`/andromeda-phase`), Epoch 2's first entry; «Budget gate removed» after it.

## Work done
- 0-pending wrap `2026-11-14T12-51-41-wrap`: the overseer's adaptation relay (inputs#I1) applied — one test-only
  corrective entry `chk-ut3m6zi6` for the audit's two `viola-hyprland` survivors (`read.rs:77`, `reply.rs:151`, `>` →
  `>=` on `MAX_FRAME`), first in Epoch 2, TX 1110; record in the run's `adaptation-record.md`.

## Drift resolved
- none — no chunk wrapped, no master touched.

## Notes
- The placement rests on the founder's selection «Корректирующий чанк первым (Recommended)» (12:48:08Z), relayed
  verbatim by overseer1 via the Viola overseer: PROVISIONAL on a trajectory fork until his own word.
- The 14th CARRY on the 2026-11-13 chunk's line (placement provisional) stays open as a provenance note.
- The unpin of `vb.sh`'s 2.1.296 shim is the operator's (no route entry).
- `HYPRCTL_DEADLINE` (5 s) is PROVISIONAL; «`viola open`» carries its settling and the `exec` / `monitors` requests.
- Two controls witnessed their own cases only in part (`record-email`, `hygiene-email`) — see the 2026-11-14 hyprland
  wrap's report.
- **Owned by 0.2.0** (carried): the 3600 s hold ceiling, PROVISIONAL; the live round's two reds; matrix#v1-27 deferred.
- **Carried, unchanged:** `chunks/2026-10-26-explicit-session-links`'s card confirm and P2 halt;
  `chunks/2026-10-27-bounded-inputs-at-every-boundary`'s P2 halt Q1 and Path 1's runner-variance reading;
  `obs-scope-summary#20.5` and `slo-invariants-telemetry-budgets#2.2` label `max` readings the report labels `mean`.
- **Deferred learnings:** the eight of the 2026-11-06 wrap, the 3 of the 2026-11-13 epoch-8 wrap, and the 2 of the
  2026-11-14 hyprland wrap (`v00.0.0` proptest; dash's default `PATH`) — unchanged. Review with
  `/andromeda-wrap-session --review`.
- **Host:** python `yaml` is absent; read `df -h /tmp` before a stamped-home gate. claude 2.1.323 is `latest` and stamped.
  Hyprland 0.56.2.
- **Last failed command:** none.

## Session End Status
Completed normally at 2026-11-14 14:29:20
