# Fan-out results — 2026-11-21-key-after-a-resize

architecture: add 0 · set 2 · retire 0 · none 0 · ops-architecture.md
security-plan: add 3 · set 2 · retire 0 · none 0 · ops-security-plan.md
design-system: no proposer
layout-templates: no proposer
test-plan: add 3 · set 1 · retire 0 · none 1 · ops-test-plan.md
obs-plan: none · ops-obs-plan.md — reason: the chunk changed only the test-only viola-e2e live rig; no logging or telemetry surface, no obs-plan row named by the expected amendments
a11y-plan: add 1 · set 1 · retire 0 · none 0 · ops-a11y-plan.md

ops-architecture.md
decision/human-key-never-held-behind-resize · apply — check 1: no rule; the reason the report's reading gives the standing no-hold decision, judged by fit
decision/no-key-held-behind-resize · reject — check 2: the fact's home is test-plan decision/key-after-resize-waits-for-size (its except names the KR arm); main retires this row, heir decision/human-key-never-held-behind-resize

ops-main-architecture.md
decision/no-key-held-behind-resize · apply — check 2: one fact keeps one home; the human-key half stands in decision/human-key-never-held-behind-resize, the test half in test-plan decision/key-after-resize-waits-for-size

ops-security-plan.md
decision/live-round-rig-keys-own-pty · reject — check 5 (inputs#I11 item 5): the set drops the qualifier «only» and the stop's two members (Ctrl-C into its own pty, a no-key group kill); the cell stands at 299 of its 300 and cannot hold KR's grant: a fork in founder-forks.md, its most conservative option taken (the row stands as it is; KR's grant stated in its own row)
decision/live-rig-kept-stamp-owner-only · reject — superseded by main's set: the proposer's «before any scratch where it makes one» drops which seeded arm refuses before its scratch
decision/live-rig-kr-arm-keeps-attributes-only · reject — superseded by main's add, which carries the grant's resize and key counts the keys row's cell cannot hold
decision/live-rig-kr-seam-options-rehearse-only · reject — check 2: the fact's home is test-plan decision/kr-seams-rehearse-only (the plan's revision entry puts the rehearse-only options in the test plan)
decision/cross-session-fixture-runtime-dir-residual · reject — check 5: the add drops what the fixture keeps (the runtime-dir path and pids inside the CLI's prefix, unnormalised by the scrub); main's add proposes the row again

ops-main-security-plan.md
decision/live-rig-kept-stamp-owner-only · apply — check 1: the operator's recorded direction settles it — relayed verbatim by the overseer, 2026-11-22, inputs#I11: «Строка без меток (Recommended)»
decision/live-rig-kr-arm-keeps-attributes-only · escalate → Q1 — rul-bznresq5: a boundary widening the operator answered himself to fork K3 (inputs#I2 item 3) · resolved: apply (Q1) — delegate the overseer, 2026-11-22: «each lies inside it. PROVISIONAL, for the epoch batch.»
decision/cross-session-fixture-keeps-runtime-path · escalate → Q2 — rul-r74hshif: an accepted residual of a data-protection boundary, fit read not_master at the cross-session wrap; his own answer stands on record (inputs#I11 item 4) · resolved: apply (Q2) — relayed verbatim by the overseer, 2026-11-22, inputs#I11: «Принять как остаток (Recommended)»

ops-test-plan.md
decision/key-after-resize-waits-for-size · escalate → Q1 — rul-bznresq5: a boundary widening the operator answered himself to fork K3 (inputs#I2 item 3), the except naming the KR arm · resolved: apply (Q1) — delegate the overseer, 2026-11-22: «each lies inside it. PROVISIONAL, for the epoch batch.»
decision/kr-after-key-settle · apply — check 1: rul-5hynfkq5 accurate this-chunk addition (Session::settle_since, the report's Symbols / APIs)
decision/kr-late-redraw-not-dirty-box · apply — check 1: rul-5hynfkq5 accurate this-chunk addition (kr_box_word, late-redraw)
decision/refusal-rehearsal-reads-own-effects · apply — check 1: rul-5hynfkq5 accurate this-chunk addition (the report's Deviations and Decisions: the seam case's own checks)

ops-main-test-plan.md
decision/kr-seams-rehearse-only · apply — check 5: the plan's revision entry for test-plan decision/standing-live-rig (the kr rehearsals' rehearse-only options), a row of its own since the standing rig's except holds the SF arm's; no boundary widens (the plan's note)

ops-a11y-plan.md
decision/no-screen-content-parsing · reject — superseded by main's set, which names the KR arm as the plan's entry says (inputs#I11 item 3)
decision/no-wait-after-resize · reject — check 2: the fact's home is architecture decision/human-key-never-held-behind-resize

ops-main-a11y-plan.md
decision/no-screen-content-parsing · escalate → Q1 — rul-bznresq5: a boundary widening the operator answered himself to fork K3 (inputs#I2 item 3), the except naming the KR arm's resize reading · resolved: apply (Q1) — delegate the overseer, 2026-11-22: «each lies inside it. PROVISIONAL, for the epoch batch.»
