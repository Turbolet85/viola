# Session Handoff

**Last Updated:** 2026-11-17T23:51:30Z
**Branch:** build/viola-0.2.0 · 0 ahead of origin/build/viola-0.2.0 as read at this wrap's Setup (this wrap's push lands its commit)
**Status:** clean
**Last Commit:** chore(route): operator-requested adaptation — 0-pending wrap

## Position
- Done: 2026-11-17-trust-record-ledger-row (last complete); 0 pending.
- Next: `/andromeda-phase` on «Trust-record row read live» (chk-e47bgjiy) — the owed live re-read of the
  `trust-record` row alone, no capability claimed; its cap is the founder's R6 (S0′ + two spares on a named cause),
  its predicate the 21 floor rows. «Session folder trusted by viola» (chk-dhecc45p) is taken up again after it; its
  S1/S2 cap is asked at that take-up.

## Work done
- 0-pending wrap `2026-11-17T23-48-56Z-wrap`: the operator-requested route adaptation of the overseer's relay
  (inputs#I1, the founder's R5 «Отдельная запись перед этой» and R6 «1 + 2 запасных», read 23:06:58Z) on the
  cancelled second take-up's P3 block (run 2026-11-17T22-05-41Z-phase, sealed by this wrap; the take-up cancel tx 2554).
  Card in the away form, answered by the overseer (inputs#I2: Q1 a, Q2 a, Q3 a). tx 2558: chk-e47bgjiy minted after
  chk-mjhurken with five carries; `cry-nuv7yj2y` and `cry-fl52zn4o` moved to it whole; `cry-3iuydqyx` added on
  chk-dhecc45p. Record: `.andromeda/runs/2026-11-17T23-48-56Z-wrap/adaptation-record.md`.

## Drift resolved
- None: no master touched (the relay's direction).

## Notes
- **Provisional, owed the founder's word:** the 21:00 wrap's answer card Q1 (25 rows, delegate overseer 21:33:27Z)
  and its three route carries (delegate overseer 21:45:49Z); the 17:24 wrap's answer card confirm; the
  placeholder-hint carry on «The 2.1.325 recorded fixture set»; earlier: `HYPRCTL_DEADLINE` (on «`viola open`»);
  `cry-x5wbcckc` on «Version done-check»; the compact window and dialog-hold bounds. This wrap's placement and cap
  are the founder's own words (R5, R6), not a delegate pin.
- **Unmeasured, named:** why S0's interactive runs B–F never reached the input box (hypothesis: one common cause, not
  trust — carry `cry-qddbztwe` on «Trust-record row read live»); whether cargo-deny keeps ignoring RUSTSEC-2026-0543
  once a safe upgrade exists (the Epoch 4 boundary); the CLI's 13 StopFailure `error` words and the placeholder
  hint's cell attribute (on «The 2.1.325 recorded fixture set»). The `rootOnly` branch is read dead on 2.1.325 by
  the code (p3-block of the run 2026-11-17T22-05-41Z-phase).
- **Stamps:** from 95792f71 on every 22-row stamp reads unverified to a binary built from the tree (the live home's
  2.1.296 · .302 · .312 · .319 · .323 · .325, every rig and scratch home); the pinned product the overseer drives with
  is not rebuilt and keeps its 22-row reading. The live re-stamp (23 rows) rides «The 2.1.325 recorded fixture set»
  (`cry-bb5cy7aq`) and verifies only once the trust-record row passes live.
- **Near-bound:** `cli_verify runs::verify_window_without_screens_fails_every_interactive_row` 52.2–52.3 s of its 60 s
  kill — carry `cry-fl52zn4o`, now on «Trust-record row read live».
- **Stale leaf count:** CARRY `cry-2snj2igc` on «Version done-check» — the earlier stamp-history counts in
  `docs/commands.md:24` and `docs/services/viola-agent-claude.md:26` stand as history; the relied count now reads 23.
- **Tool defects seen (friction family):** `rec.py rows-land` read a never-landed set «landed before (tx 1193)»; the
  leaf comparers missed six stale counts and moved a stale «twenty-four» to «twenty-five».
- The operator's desk list (no route entry): `~/.cache/viola-ctl-ready-deadline-20261117T163750Z`;
  `~/dev/projects/viola-mutants-scratch/cold-*` (two) and `…/hang-mhCgsY`;
  `target/e2e-home/viola-live-20261115T083805Z/`; `target/live-round-screens/`; the two `projects[…]` trust entries in
  `~/.claude.json`; the live home's stale `~/.viola/budget.json`, `~/.viola/instances/*/settings.json` and any
  `~/.viola/statusline-source.json`; the unpin of `vb.sh`'s 2.1.296 shim; `target/live-stopfailure-20261115/` and
  `-s2/`; `~/.cache/viola-path1-measure` (13 GB); `target/class-wall/` and the `target/e2e-home/viola-stamp-*` dirs on
  the tmpfs; the rig's kept frames under `target/live-rig/`.
- **Disk:** /home 186G free, `target/` 218G — larger than the free space: run the clean cadence before a full rebuild.
- **Deferred learnings:** `recurrence-despite-learning: CLAUDE.md «A builder session sets no scheduled wake, timer or
  reminder of its own»` (the implement window's `ScheduleWakeup` `stop: true`, refused by the classifier
  2026-11-17T20:31:41Z); `recurrence-despite-learning: testing.md 2026-11-13 «A gate.py control clone holds no build
  output …»`; `recurrence-despite-learning: CLAUDE.md «A timestamp written into any record … is read from date -u»`;
  `recurrence-despite-learning: testing.md 2026-11-15 «A removal sweeps every test that reads the removed field, row
  or mode …»`; «Never chain `rec.py land` onto a `gate.py hygiene` call», 0.7. Review with
  `/andromeda-wrap-session --review`.
- **Host:** python `yaml` absent; read `df -h /tmp` before a stamped-home gate. claude 2.1.325 (stamps pre-23rd-row);
  `mise ls claude` reads 2.1.326 as `latest` (not used). Hyprland 0.56.2. The host's own Claude sessions rewrite
  `~/.claude.json` about once a minute.
- **Last failed command:** none.

## Session End Status
Completed normally at 2026-11-18 10:15:19
