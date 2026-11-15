# P4 forks — chunk 2026-11-15-mutation-and-coverage-baselines-green, answered in the question dialog 2026-11-15

Authority: the operator, relaying the overseer's technical answer (delegate overseer; no boundary widening in either fork).

## Q1 — Items 2 and 3 (the mutants kill and the WITHIN receipt wait move together): measured at HEAD, the stamped-home class is 161 tests ≥ 5 s, the stamped ones 6.0–7.3 s warm against the 10 s default kill; M2's case reads 6.2–6.5 s instrumented against WITHIN 7 s. Which form?
Options: (1) Raise default + WITHIN (Recommended) — [profile.mutants] slow-timeout 5 s × 3 = 15 s and WITHIN 7 → 12 s; (2) Per-class override list.
Answer: Raise default + WITHIN (Recommended)
Notes, verbatim: The overseer's technical answer. A class of 161 tests is not a list to keep: an override list misses the next stamped-home binary silently, which is the defect this chunk exists to end. Say in the plan, in words the wrap can amend from, that this departs from the 2026-10-20 'never by raising the default' line and why (the class is the default population now), keep the planted-hang control pair, and measure the cold parallel baseline's slowest case against 15 s with the host load beside it — if any case still crosses 15 s cold, name it on the card rather than raise again.

## Q2 — Item 5 (no `home` component in a home given to `viola verify --record`): where does the rule live?
Options: (1) In-repo helper + carry (Recommended) — tests/support/home.rs gains a record-home helper naming the home `vhome`, the six --record callers moved onto it, the live rig's half carried forward to «Standing live rig with a screen model»; (2) Carry whole to the rig entry.
Answer: In-repo helper + carry (Recommended)
Notes, verbatim: The overseer's technical answer. The helper is the instrument; the live rig's half rides the Carry-forward to «Standing live rig with a screen model».
