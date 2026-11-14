# Session Handoff

**Last Updated:** 2026-11-14T14:32:51Z
**Branch:** build/viola-0.2.0 · 0 ahead of origin/build/viola-0.2.0 as read at this wrap's Setup (this wrap's push lands its commit)
**Status:** clean
**Last Commit:** 2026-11-14-hyprland-exact-frame-bound-pinned — test(…): viola-hyprland's two MAX_FRAME bounds pinned at the exact frame

## Position
- Done: 2026-11-14-hyprland-exact-frame-bound-pinned — two unit tests pin `read.rs:77` and `reply.rs:151` at exactly
  `MAX_FRAME` (both remove-the-guard controls `holds`); CI ci#43602507261 green 7/7 on the pre-CI `aae35c72`.
- Next: «Budget gate removed» (`/andromeda-phase`), Epoch 2's next entry.

## Work done
- Test-only corrective chunk: no product line, no capability claimed; the Epoch 1 audit's two survivors closed (the
  next boundary audit re-grades both sites).

## Drift resolved
- test-plan `decision/fakes-for-external-services` gains its `except` cell (viola-hyprland's unit tests serve
  `hyprctl` by a planted stub; cargo-mutants runs only the mutated package's tests), TX 1142; the cascade carried it
  into `tests-summary.md` and `verification-harness.md`. A twin set on `decision/fake-agent-as-claude` rejected at
  check 2.

## Notes
- New CARRY on «`viola open`»: the `hyprctl reply` census row of `tests/contract_bounded_readers.rs` names a parse
  test as its witness, not one driving the read's drain past its cap (the P5 card's Beyond-the-plan row, placed by the
  wrap directive).
- Not placed: the consumed carry's clause «the declined `viola-e2e` and `viola` mutation units stay out (they wait on a
  pipeline mend)» names no route entry.
- The 2026-11-14 placement of this chunk rests on the founder's selection relayed verbatim by overseer1:
  PROVISIONAL until his own word.
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
