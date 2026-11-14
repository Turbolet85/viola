# relay-1 — the probe clash (plan entry 11 vs the cases that spell an old word)

Question put at implement P1: the plan's identifier probe (entry 11) contradicts its own compatibility / retired-row
cases, which must spell the old words; options: keep literals and surface · old shapes to fixture files · drop the
literal-only bits.

Selected: «Old shapes to fixture files»

Notes, verbatim:
The overseer's technical answer (the delegate; the operator is away): the old 0.1.0 shapes go to committed fixture files outside the probed paths (fixtures/viola-0.1.0/), every case that must spell an old word reads it from there — the exit-code case's «budget-paused» word included, nothing dropped. Reason: entry 11 is matrix#v2-04's own acceptance as claimed (the identifier probe exits 1 with no output) and must read green as written; the compatibility cases stay whole. Record the new files as an in-intent addition for the wrap, and add the fixture directory to the hygiene/fixture walk the tree already has if one covers fixtures/..
