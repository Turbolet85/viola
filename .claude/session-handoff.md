# Session Handoff

**Last Updated:** 2026-11-16T00:11:17Z
**Branch:** build/viola-0.2.0 · 0 ahead of origin/build/viola-0.2.0 as read at this wrap's Setup (this wrap's push lands its commit)
**Status:** clean (once the wrap's commit lands)
**Last Commit:** 2026-11-15-api-error-turn-end — the wrap's chunk commit (StopFailure registered → `turn-ended` with the closed cause `api-error`, the error text crossing into nothing)

## Position
- Done: 2026-11-15-api-error-turn-end — the CLI's `StopFailure` hook registered as `viola hook stop-failure`, ending the
  running turn as `turn-ended` with `cause: "api-error"`; the instance idle, the next send accepted; `v2-16`.
- Next: «`wait` wake condition» (`chk-r4pjfa6x`, `/andromeda-phase`), Epoch 3 — it carries the open CI timing red.

## Work done
- Wrap `2026-11-15T23-35-53-wrap`: report closed (rev 3); masters 23 operations landed (TX 1664–1668: architecture 11,
  security-plan 5, test-plan 5, design-system 1, obs-plan 1); 13 leaves re-derived; curation T2 1 (`rules/testing.md`).
- Route (TX 1685): CARRY on «`wait` wake condition» (the CI timing red) and on «The 2.1.325 recorded fixture set» (the
  live StopFailure reading and the no-ledger-row exception).

## Drift resolved
- The hook-event count nine → ten at its three master sites, `turn-ended`'s `cause`, the StopFailure map row, the
  `TurnEndCause` set, the `cause` re-validation, the error text dropped, the ledger exception at five rows. 13
  duplicate adds rejected, one home each. Every row of the F1/F2 widenings landed PROVISIONAL on the delegate's answer
  by relay (inputs#I4 item 2) — the founder away; the forks in the run dir's `founder-forks.md`.

## Notes
- **Founder's word owed:** F1 (StopFailure with no ledger row / probe) and F2 (the `cause` word crossing; the granular
  cause list the overseer's lean) — `.andromeda/runs/2026-11-15T21-19-45-phase/founder-forks.md` and this wrap's
  `founder-forks.md`; nothing ratified.
- **Open red, not this chunk's:** ci#43773402286 on `7fd86141`, `path1_run_start_one_home_in_order` TIMEOUT at 20 s —
  the `stamped_home` fixture's unbounded `viola verify` inside the test's time; owner the CARRY on «`wait` wake
  condition».
- PROVISIONAL still carried: `HYPRCTL_DEADLINE` (on «`viola open`»); the 3600 s hold ceiling ratified on one reading
  on one CLI version.
- The operator's desk list (no route entry): `~/dev/projects/viola-mutants-scratch/cold-*` (two) and `…/hang-mhCgsY`;
  the kept tmpfs home `target/e2e-home/viola-live-20261115T083805Z/`; the local frames under
  `target/live-round-screens/`; the two `projects[…]` trust entries in `~/.claude.json`; the live home's stale
  `~/.viola/budget.json`, `~/.viola/instances/*/settings.json` and any `~/.viola/statusline-source.json`; the unpin of
  `vb.sh`'s 2.1.296 shim; `target/live-stopfailure-20261115/` and `target/live-stopfailure-20261115-s2/`;
  `~/.cache/viola-path1-measure` (13 GB); live start 1's CLI rewrote `~/.claude.json`.
- **Deferred learnings:** none new this wrap. Earlier: «Never chain `rec.py land` onto a `gate.py hygiene` call: a
  refused hygiene still lands its bytes» (confidence 0.7). Review with `/andromeda-wrap-session --review`.
- **Host:** python `yaml` is absent; read `df -h /tmp` before a stamped-home gate. claude 2.1.325 is `latest` and
  stamped. Hyprland 0.56.2.
- **Last failed command:** none.

## Session End Status
Completed normally at 2026-11-16 04:08:08
