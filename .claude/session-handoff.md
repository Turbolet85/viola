# Session Handoff

**Last Updated:** 2026-11-16T03:53:33Z
**Branch:** build/viola-0.2.0 · 0 ahead of origin/build/viola-0.2.0 as read at this wrap's Setup (this wrap's push lands its commit)
**Status:** clean (once the wrap's commit lands)
**Last Commit:** 2026-11-16-wait-wake-condition — the wrap's chunk commit (`wait` wake condition documented and pinned; the stamped-home CI red closed)

## Position
- Done: 2026-11-16-wait-wake-condition — every `turn-ended` wakes `wait` whatever its `background_tasks`, no false wake
  after a task-free end (pinned by unit and booted cases); one `viola verify` stamp per test binary, copied; `v2-17`.
- Next: «Send confirmation» (`chk-pihewvzr`, `/andromeda-phase`), Epoch 3.

## Work done
- Wrap `2026-11-16T03-24-03-wrap`: report closed (rev 3); masters 11 operations landed (TX 1765–1768: architecture 3,
  security-plan 1 + its provisional state, test-plan 6); 8 leaf findings in 5 leaves; curation T3 1 extension.
- Route (TX 1777): CARRY on «Version done-check» — the two older wait tests that hang past a failure.

## Drift resolved
- The wake rule (every turn end wakes, by kind alone; activity and a harness `prompt-submitted` never), no scheduled-wake
  count on `turn-ended`, the stamp copied once per build, its sweep by lock, the fake script's step kinds. 9 duplicate
  operations rejected, one home each. One Boundary-widening escalation (security-plan
  `decision/test-removal-own-dirs-only`) landed PROVISIONAL on the delegate's standing direction (inputs#I16 item 1).

## Notes
- **Founder's word owed:** this wrap's Q1 (`.andromeda/runs/2026-11-16T03-24-03-wrap/founder-forks.md`); F1 of this
  chunk (`.andromeda/runs/2026-11-16T00-29-34-phase/founder-forks.md`) and its candidate `session_crons` as a count (no
  route entry); F1/F2 of 2026-11-15-api-error-turn-end (`.andromeda/runs/2026-11-15T21-19-45-phase/founder-forks.md`,
  `.andromeda/runs/2026-11-15T23-35-53-wrap/founder-forks.md`); nothing ratified.
- **Closed:** the CI red ci#43773402286 — ci#43797036340 green 7/7 on `d107a553`.
- **Stale leaf count, no edit asked:** `.claude/rules/verification-harness.md:28`, `.claude/docs/commands.md:23`/`:24`,
  `.claude/docs/tests-summary.md:21` say «all twenty-four» relied ledger rows; the code says 21. No master states it,
  so the cascade cannot reach it.
- PROVISIONAL still carried: `HYPRCTL_DEADLINE` (on «`viola open`»); the 3600 s hold ceiling ratified on one reading
  on one CLI version.
- The operator's desk list (no route entry): `~/dev/projects/viola-mutants-scratch/cold-*` (two) and `…/hang-mhCgsY`;
  the kept tmpfs home `target/e2e-home/viola-live-20261115T083805Z/`; the local frames under
  `target/live-round-screens/`; the two `projects[…]` trust entries in `~/.claude.json`; the live home's stale
  `~/.viola/budget.json`, `~/.viola/instances/*/settings.json` and any `~/.viola/statusline-source.json`; the unpin of
  `vb.sh`'s 2.1.296 shim; `target/live-stopfailure-20261115/` and `target/live-stopfailure-20261115-s2/`;
  `~/.cache/viola-path1-measure` (13 GB); live start 1's CLI rewrote `~/.claude.json`; `target/class-wall/` (a clone
  and two target dirs) and the `target/e2e-home/viola-stamp-*` dirs on the tmpfs.
- **Deferred learnings:** `recurrence-despite-learning`: CLAUDE.md's timestamp learning (a journal stamp estimated,
  then corrected to `date -u`, implement run 01-25-19). Earlier: «Never chain `rec.py land` onto a `gate.py hygiene`
  call: a refused hygiene still lands its bytes» (confidence 0.7). Review with `/andromeda-wrap-session --review`.
- **Host:** python `yaml` is absent; read `df -h /tmp` before a stamped-home gate. claude 2.1.325 is `latest` and
  stamped. Hyprland 0.56.2.
- **Last failed command:** none.
