## Addendum — the P7.1 light gate, 2026-10-16
- inputs#I8 · message (the wrap P7.1 ASSERT dialog, the overseer, founder-delegated) · copy · n/a — the treatment of
  round entry 12
  - bears on **Outcome**: adds to it. The light gate (this run dir's `gate-…json`) read `entries 26 · green 17 · red
    0 · recorded 0 · timeout 0 · not-run 9`.
  - The nine not-run are the leg entries, re-verified from `evidence/`:
    - round 1 (7, 8) from `round-071816Z.txt` and round 2 (9, 10, 11) from `round-071942Z.txt`, each `round:
      COMPLETE`;
    - the operator entries 24–26 from `operator-pass.md` (hygiene clean, push green, `ci.py` `verdict: green`);
    - round entry 12 (the census after the `--local-live` round) from its hand record in `live-round.md`.
  - Entry 12 is in no round listing. The round was fired with `--entry 11`, which runs no step after its leg, so the
    plan's exact command was run by hand at 07:19:50Z and printed no pid. That record is accepted as entry 12's
    treatment.
- inputs.py verify, re-fired after this addendum: uncited 1 → 0.

