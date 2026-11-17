# Fan-out results — 2026-11-17-first-readings-on-the-standing-rig

architecture: add 3 · set 2 · retire 0 · none 0 · ops-architecture.md
security-plan: add 2 · set 2 · retire 0 · none 0 · ops-security-plan.md
design-system: none · ops-design-system.md — the chunk is test-only and adds no product CLI wording or UI token
layout-templates: none · ops-layout-templates.md — test-only; no CLI wording or user-facing surface moved
test-plan: add 3 · set 1 · retire 0 · none 2 · ops-test-plan.md
obs-plan: add 1 · set 0 · retire 0 · none 0 · ops-obs-plan.md
a11y-plan: none · ops-a11y-plan.md — test-only; no interactive element, output or log schema change

Check 5 (the plan's Expected amendments, the coverage floor): test-plan decision/verify-driven-tests-20s-kill — no
operation: the row stays true (a verify-driven test's binary sits on the ci 20 s override; the override grew, the rule
did not move). test-plan contract/harness-boot/timeout — no operation: the CLI `harness boot` keeps 20 s per instance
(report, Symbols: `BootOptions::ready_deadline` defaults to `INSTANCE_DEADLINE`, the CLI `boot` keeps 20 s); the row's
boundary is the CLI's. test-plan decision/standing-live-rig — no operation: the row stays true; the arms' new facts
take their homes in security-plan (the capture, the witness, the stop). The three matrix notes (v2-35, v2-19, v2-32)
are P7.3's. The route carry on «A fired scheduled wake keeps the wheel» is P5's.
Check 6 (Spec claims disproved): the bundle's `source` words on a fired wake — no master states them (sweep
`wake-source`, the close's Sweep reach): routed to P5, v2-36's owner entry. The stamp-only user-config witness —
security-plan decision/live-rig-isolated-child's set (Q1).

ops-architecture.md
decision/claude-shapes-in-one-crate · escalate → Q1 — check 1: no rule; an exception to an architecture invariant is structural; it is the consequence of the founder's own answer inputs#I2 item 3 (the rig reads its own UserPromptSubmit payload's key names and `source`), so it rides the answer card · resolved: apply (Q1) — delegate the overseer, 2026-11-17, inputs#I10: «Confirmed … all four lie inside them — rows 3 and 4 are the capture's own consequence for a test-only rig, never the product» — PROVISIONAL confirm; the ratification is the founder's own answers, relayed verbatim by the overseer: inputs#I2 item 3 «Разрешить захват в риге» (the capture) and inputs#I6 item 1 «Искать в файле следы рига» (the witness)
decision/claude-first-no-adapter-trait · escalate → Q1 — check 1: no rule; as above, the same exception on the companion row; inside inputs#I2 item 3 · resolved: apply (Q1) — delegate the overseer, 2026-11-17, inputs#I10: «Confirmed … all four lie inside them — rows 3 and 4 are the capture's own consequence for a test-only rig, never the product» — PROVISIONAL confirm; the ratification is the founder's own answers, relayed verbatim by the overseer: inputs#I2 item 3 «Разрешить захват в риге» (the capture) and inputs#I6 item 1 «Искать в файле следы рига» (the witness)
decision/live-rig-capture-key-names-only · reject — check 2: the fact's home is security-plan decision/live-rig-prompt-capture-closed
decision/live-rig-user-config-mark-witness · reject — check 2: the fact's home is security-plan decision/live-rig-isolated-child
name/other/viola-live-rig-capture · reject — check 1: the names register holds no verb of viola-live-rig (its `live`, `census` and rehearsal verbs stand unnamed); a hidden verb of a test-only binary is no occupied resource of the register

ops-security-plan.md
decision/live-rig-isolated-child · escalate → Q1 — rul-bznresq5: a boundary widening the founder answered himself, inputs#I6 item 1 («Искать в файле следы рига», relayed verbatim by the overseer, read 2026-11-17T16:50:18Z) · resolved: apply (Q1) — delegate the overseer, 2026-11-17, inputs#I10: «Confirmed … all four lie inside them — rows 3 and 4 are the capture's own consequence for a test-only rig, never the product» — PROVISIONAL confirm; the ratification is the founder's own answers, relayed verbatim by the overseer: inputs#I2 item 3 «Разрешить захват в риге» (the capture) and inputs#I6 item 1 «Искать в файле следы рига» (the witness)
decision/live-round-rig-keys-own-pty · escalate → Q2 — inputs#I8 item 2: whether the row's «anew for each chunk» binds the stop is asked of the founder by the overseer; the row changes only on the answer the overseer relays · resolved: apply (Q2) — relayed verbatim by the overseer, 2026-11-17, inputs#I9: «Ctrl-C стопа — постоянный» · as: the per-chunk word of the founder binds only the closed argv grant (keys into a dialog or a box); the stop — Ctrl-C into the rig's own pty, or a no-key group kill — stands for every chunk, needing no word
decision/live-rig-prompt-capture-closed · escalate → Q1 — rul-bznresq5: a boundary widening the founder answered himself, inputs#I2 item 3 («Разрешить захват в риге», relayed verbatim by the overseer, read 2026-11-17T16:05:02Z) · resolved: apply (Q1) — delegate the overseer, 2026-11-17, inputs#I10: «Confirmed … all four lie inside them — rows 3 and 4 are the capture's own consequence for a test-only rig, never the product» — PROVISIONAL confirm; the ratification is the founder's own answers, relayed verbatim by the overseer: inputs#I2 item 3 «Разрешить захват в риге» (the capture) and inputs#I6 item 1 «Искать в файле следы рига» (the witness)
decision/live-rig-user-config-read-bounded · reject — check 7: not_master type 4 — a coverage line retelling how one test-rig reader is built (its bound, link handling, unread word) with no reason stated, which the code and its tests hold; escalated (Q1), not asked

ops-test-plan.md
decision/ci-stamped-home-kill-line · reject — check 7: not_master type 5 — the set lengthens a row still true with an inventory of which binaries fall under it, which the nextest ci profile holds; the row's rule (stamped-home binaries killed at 20 s on ci) stands true, so nothing follows (the Expected amendment is dispositioned: the row stays)
decision/rig-user-config-mark-witness · reject — check 2: the fact's home is security-plan decision/live-rig-isolated-child
decision/rig-prompt-capture-value-free · reject — check 2: the fact's home is security-plan decision/live-rig-prompt-capture-closed
decision/rig-keys-never-the-stop · reject — check 2: the fact's home is security-plan decision/live-round-rig-keys-own-pty

ops-obs-plan.md
decision/live-rig-prompt-capture-not-telemetry · reject — check 2: the fact's home is security-plan decision/live-rig-prompt-capture-closed
