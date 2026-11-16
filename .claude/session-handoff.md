# Session Handoff

**Last Updated:** 2026-11-16T05:43:03Z
**Branch:** build/viola-0.2.0 · 0 ahead of origin/build/viola-0.2.0 as read at this wrap's Setup (this wrap's push lands its commit)
**Status:** clean (once the wrap's commit lands)
**Last Commit:** 2026-11-16-send-confirmation — the wrap's chunk commit (a send matched whatever trailing whitespace the CLI trims, PROVISIONAL; the readiness-gate turn race closed by the flight lock)

## Position
- Done: 2026-11-16-send-confirmation — `is_submitted_form` takes the typed text, or a non-empty text equal to it less a
  trailing run of ASCII space/TAB/CR/LF (PROVISIONAL); the post-gate reads, link, `send-issued` and paste under one
  flight lock; `v2-18`, `v2-19`. CI ci#43807991445 green 7/7 on `76257d01`.
- Next: «Local-command delivery» (`chk-ooka6oic`, `/andromeda-phase`), Epoch 3.

## Work done
- Wrap `2026-11-16T05-15-05-wrap`: report closed (rev 3, TX 1824–1825); masters 16 operations landed (TX 1829–1834:
  architecture 5, design-system 1, layout-templates 1, obs-plan 2, security-plan 3, test-plan 4); 10 leaf findings in
  9 leaves; curation: no new learnings.
- Route (TX 1846): two CARRYs on «Standing live rig with a screen model» — the live reading of which whitespace the CLI
  trims at submit; the hook-latency window, `v2-19`'s residual.

## Drift resolved
- The exact-match claim retired in architecture, security-plan and test-plan for the trailing-whitespace tolerance
  (each heir `state: provisional`); the relabel, readback, read-back and confirm-window rows state the tolerance; the
  flight lock stated (architecture decision/send-flight-lock, obs turn-running bounded); the fake agent's
  `--trim-prompt-end`. 5 proposals rejected (3 capability-ledger exceptions — the tolerance relies on no CLI
  behaviour; 2 duplicate homes), 1 by fit (`send-refusal-order`, not_master).
- One Boundary-widening escalation (Q1, 13 operations) landed PROVISIONAL on the delegate's answer by relay
  (`inputs#I5`, the wrap directive `inputs#I4`).

## Notes
- **Founder's word owed:** this wrap's Q1 (`.andromeda/runs/2026-11-16T05-15-05-wrap/founder-forks.md`, «either end»
  the candidate); this chunk's phase Q1 (`.andromeda/runs/2026-11-16T04-10-47-phase/founder-forks.md`); the previous
  wrap's Q1 (`.andromeda/runs/2026-11-16T03-24-03-wrap/founder-forks.md`); F1 of 2026-11-16-wait-wake-condition
  (`.andromeda/runs/2026-11-16T00-29-34-phase/founder-forks.md`) and its candidate `session_crons` as a count (no route
  entry); F1/F2 of 2026-11-15-api-error-turn-end (`.andromeda/runs/2026-11-15T21-19-45-phase/founder-forks.md`,
  `.andromeda/runs/2026-11-15T23-35-53-wrap/founder-forks.md`); nothing ratified.
- **Stale leaf count, no edit asked:** `.claude/rules/verification-harness.md:28`, `.claude/docs/commands.md:23`/`:24`,
  `.claude/docs/tests-summary.md:21` say «all twenty-four» relied ledger rows; the code says 21. No master states it,
  so the cascade cannot reach it.
- PROVISIONAL still carried: `HYPRCTL_DEADLINE` (on «`viola open`»); the 3600 s hold ceiling ratified on one reading
  on one CLI version; the send's trailing-whitespace tolerance (this chunk).
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
