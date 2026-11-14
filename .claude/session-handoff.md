# Session Handoff

**Last Updated:** 2026-11-14T11:06:32Z
**Branch:** build/viola-0.2.0 · 0 ahead of origin/build/viola-0.2.0 as read at this wrap's Setup (this wrap's push lands its commit)
**Status:** clean
**Last Commit:** 2026-11-14-hyprland-seam — viola's one Hyprland seam (`viola-hyprland`), measured live at L1/L2, replayed by a fake in CI

## Position
- Done: 2026-11-14-hyprland-seam — the sync crate `viola-hyprland` (closed requests, bounded `hyprctl` read, tolerant
  parsers that drop every reply string, five closed failure words, `hyprctl-probe` / `hyprctl-json` log words), the
  test-only recorder `viola-hypr-tap` (closed keep-list), the fake `hyprctl`, `fixtures/hyprland/0.56.2/` (L1 and L2 on
  the operator's own words), the replay contract, the fuzz target; four controls hold; CI ci#43579735207 green on
  `93b2ca1d`.
- Next: «Budget gate removed» (`/andromeda-phase`).

## Work done
- Code + the operator pass (pre-CI `93b2ca1d`, pushed); wrap: 45 row operations in 4 masters (TX 1070–1075), 20 leaf
  findings + 1 by hand, curation T2 3, 2 CARRY pins on «`viola open`».

## Drift resolved
- The plan's dispatch argv measured wrong on Hyprland 0.56.2 (`dispatch` runs as Lua `hl.dsp.*`): corrected in code at
  implement; architecture `decision/hyprctl-direct-spawn` states the measured form and its closed inserted values.
- The founder's own word on the answer card («Подтвердить обе», 10:58:23Z, relayed by overseer1): `hyprctl-direct-spawn`
  and security-plan `decision/hyprland-recorder-string-allowlist` landed as his ratification.
- The five `contract/hyprland-seam-errors/*` rows landed on the delegated operator's technical answer, corrected once
  (a new boundary in a plain area, no new area).
- Rejected: 13 at Validate (a second home for one fact), 11 by fit (`not_master`), each in the wrap's
  `fanout-results.md`.

## Notes
- Two controls witnessed their own cases only in part (nextest fail-fast): `record-email` never ran step 2's
  `unclean_refuses_an_email_in_a_string_or_a_key_after_a_path`; `hygiene-email` did not reach the status-line and
  script cases. The guards hold; those cases' own reds are unwitnessed (the learning is in session-learnings.md).
- The nine expected amendments that "drop the owed divergence" have no cell to set: the divergence is the rows
  landing's `verified` meta (`v: stale`), which no row operation writes.
- The 14th CARRY on the 2026-11-13 chunk's line (placement provisional) stays open as a provenance note.
- The unpin of `vb.sh`'s 2.1.296 shim is the operator's (no route entry).
- `HYPRCTL_DEADLINE` (5 s) is PROVISIONAL; «`viola open`» carries its settling and the `exec` / `monitors` requests.
- **Owned by 0.2.0** (carried): the 3600 s hold ceiling, PROVISIONAL; the live round's two reds; matrix#v1-27 deferred.
- **Carried, unchanged:** `chunks/2026-10-26-explicit-session-links`'s card confirm and P2 halt;
  `chunks/2026-10-27-bounded-inputs-at-every-boundary`'s P2 halt Q1 and Path 1's runner-variance reading;
  `obs-scope-summary#20.5` and `slo-invariants-telemetry-budgets#2.2` label `max` readings the report labels `mean`.
- **Deferred learnings:** the eight of the 2026-11-06 wrap (unchanged), the 3 of the 2026-11-13 epoch-8 wrap
  (unchanged), and 2 of this wrap (max-3 cap):
  - a proptest property comparing a parsed value's text with its input fails on non-canonical input (`v00.0.0`) —
    compare parsed values (0.8);
  - dash defaults an unset `PATH`, so an env-strip test removes a variable the shell does not default (0.8).
  Review with `/andromeda-wrap-session --review`.
- **Host:** python `yaml` is absent; read `df -h /tmp` before a stamped-home gate. claude 2.1.323 is `latest` and stamped.
  Hyprland 0.56.2; the session was left unlocked after L2 (the operator's direction).
- **Last failed command:** none.
